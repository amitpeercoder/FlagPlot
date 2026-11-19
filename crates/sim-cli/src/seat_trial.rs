//! What plays a seat's trials (decision 0178, rules 5 and 6): a value with,
//! for each side, what the scenario says that side believed, the trial's
//! figures, the reference data and the map. It implements `sim-agent`'s
//! [`TrialRunner`] by playing a trial over `sim-trial` (decisions 0175 and
//! 0177: the standing commands apart from the course, at one date), reading
//! its estimate (decision 0176) and filling the narrow answer a seat may be
//! shown. `serve::open` hands one to the session where the scenario names a
//! file of the trial's figures, as it hands in the staffs.

use std::collections::BTreeMap;

use sim_agent::{
    AssumedNoteShown, AssumedShown, AssumedThing, BaseShown, CountShown, ForceShown, LiveIds,
    MadeFrom, NeedsShown, OtherPlanShown, PartShown, PartingShown, PlanTaskShown, SizeAsked,
    TaskShown, TrialAnswer, TrialAsked, TrialCosts, TrialRefused, TrialRunner, TrialSpread,
    TrialStep, TrialUnderWay, TrialWent, WaitShown,
};
use sim_core::believed::Standing;
use sim_core::command::{Awaited, IntentKind, WaitedFor};
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::map::Map;
use sim_core::refdata::{RefData, ShipKind, TrialFigures, TrialSize};
use sim_core::state::Side;
use sim_core::view::SideView;
use sim_core::world_picture::PictureContact;
use sim_data::Scenario;
use sim_trial::{
    begin_window, estimate, AssumedNote, AssumedOf, Believed, Costs, CourseCommand, Estimate,
    PartSide, PartedBy, Played, PlayedWorld, Spread, Went, WindowAsked, WindowPlaying,
};

/// What plays a seat's trials in one game.
pub struct SeatTrials {
    believed: BTreeMap<Side, Believed>,
    figures: TrialFigures,
    ref_data: RefData,
    map: Map,
}

impl SeatTrials {
    /// The runner of a scenario that names a file of the trial's figures;
    /// `None` for one that names none, whose sessions have no trial.
    pub fn of_scenario(scenario: &Scenario) -> Option<SeatTrials> {
        let figures = scenario.trial_figures()?.clone();
        Some(SeatTrials::with_figures(scenario, figures))
    }

    /// The runner of a scenario with the trial's figures handed in: for
    /// `time`, which may try a scenario that names none.
    pub fn with_figures(scenario: &Scenario, figures: TrialFigures) -> SeatTrials {
        let believed = Side::ALL
            .into_iter()
            .map(|side| {
                (
                    side,
                    Believed {
                        forces: scenario.believed_forces(side).cloned().collect(),
                        intents: scenario.believed_intents(side).cloned().collect(),
                        men: scenario.believed_men(side).cloned().collect(),
                    },
                )
            })
            .collect();
        SeatTrials {
            believed,
            figures,
            ref_data: scenario.world.ref_data().clone(),
            map: scenario.world.map().clone(),
        }
    }

    /// The runner, boxed for the session, where the scenario has a trial.
    pub fn runner_of(scenario: &Scenario) -> Option<Box<dyn TrialRunner>> {
        SeatTrials::of_scenario(scenario).map(|runner| Box::new(runner) as Box<dyn TrialRunner>)
    }
}

/// What the answer is filled with, beside the trial: the live game's own
/// things, as the view gave them when the trial began.
struct Context {
    side: Side,
    live: LiveIds,
    size: TrialSize,
    size_name: Option<String>,
    first_day: u32,
    /// The side's contacts, in the order the pictures number them.
    contacts: Vec<PictureContact>,
    believed: Believed,
}

/// A window being played, or played out.
enum Stage {
    Playing(Box<WindowPlaying>),
    Played(Played),
}

impl Stage {
    /// Plays one world; `true` where one was played.
    fn step(&mut self) -> bool {
        match self {
            Stage::Playing(playing) => {
                if playing.step().is_some() {
                    return true;
                }
                let Stage::Playing(playing) =
                    std::mem::replace(self, Stage::Played(empty_played()))
                else {
                    unreachable!("the stage was playing")
                };
                let mut window = playing.play_out();
                *self = Stage::Played(window.played.remove(0).played);
                false
            }
            Stage::Played(_) => false,
        }
    }

    fn played(&self) -> Option<&Played> {
        match self {
            Stage::Played(played) => Some(played),
            Stage::Playing(_) => None,
        }
    }
}

fn empty_played() -> Played {
    Played {
        side: None,
        worlds: Vec::new(),
        uneven: false,
        leaves_given: Vec::new(),
    }
}

/// A seat's trial under way: the trial asked, then the second trial of
/// things as they stand where one was asked, a world at a time.
struct UnderWay {
    with: Stage,
    without: Option<Stage>,
    intent: IntentId,
    context: Context,
}

impl TrialRunner for SeatTrials {
    fn begin(
        &self,
        view: &SideView<'_>,
        asked: &TrialAsked,
    ) -> Result<Box<dyn TrialUnderWay>, TrialRefused> {
        let side = view.side();
        let (size, size_name) = match &asked.size {
            SizeAsked::Named(name) => {
                let found = self
                    .figures
                    .sizes()
                    .into_iter()
                    .find(|(n, _)| n == name)
                    .map(|(_, size)| size);
                let Some(size) = found else {
                    return Err(TrialRefused(format!(
                        "the trial's figures name no size \"{name}\": they name small, standard and full"
                    )));
                };
                (size, Some(name.clone()))
            }
            SizeAsked::Numbers { worlds, days } => (
                TrialSize {
                    worlds: *worlds,
                    days: *days,
                },
                None,
            ),
        };
        let believed = self.believed.get(&side).cloned().unwrap_or_default();
        let course: Vec<CourseCommand> = asked
            .course
            .iter()
            .map(|step| CourseCommand {
                day: step.day,
                command: step.command.clone(),
            })
            .collect();
        let window = |standing, course| WindowAsked {
            trial: asked.trial,
            worlds: size.worlds,
            days: size.days,
            intent: asked.intent,
            standing,
            course,
            dates: Some(asked.dates.clone()),
        };
        let refused = |refused: sim_trial::Refused| {
            TrialRefused(format!("it cannot be tried: {}", refused.0))
        };
        let with = begin_window(
            view,
            &believed,
            &self.figures,
            &self.ref_data,
            &self.map,
            window(asked.standing.clone(), course),
        )
        .map_err(refused)?;
        let without = match &asked.as_they_stand {
            Some(standing) => Some(Stage::Playing(Box::new(
                begin_window(
                    view,
                    &believed,
                    &self.figures,
                    &self.ref_data,
                    &self.map,
                    window(standing.clone(), Vec::new()),
                )
                .map_err(refused)?,
            ))),
            None => None,
        };
        let contacts = view
            .own_half()
            .map(|half| half.belief.contacts)
            .unwrap_or_default();
        Ok(Box::new(UnderWay {
            with: Stage::Playing(Box::new(with)),
            without,
            intent: asked.intent,
            context: Context {
                side,
                live: LiveIds::of(view),
                size,
                size_name,
                first_day: asked.dates.first().copied().unwrap_or(view.day() + 1),
                contacts,
                believed,
            },
        }))
    }
}

impl TrialUnderWay for UnderWay {
    fn step(&mut self) -> TrialStep {
        if self.with.step() {
            return TrialStep::Going;
        }
        if let Some(without) = &mut self.without {
            if without.step() {
                return TrialStep::Going;
            }
        }
        let with = self.with.played().expect("played out");
        let without = self.without.as_ref().and_then(Stage::played);
        let estimate = estimate(with, self.intent, without);
        TrialStep::Done(Box::new(fill(&self.context, &estimate, with)))
    }
}

// ------------------------------------------------------------ The answer

fn spread(s: Spread) -> TrialSpread {
    TrialSpread {
        lowest: s.lowest,
        middle: s.middle,
        highest: s.highest,
    }
}

fn costs(c: &Costs) -> TrialCosts {
    TrialCosts {
        men_to_enemy: spread(c.men_to_enemy),
        men_unfed: spread(c.men_unfed),
        ships_sunk: spread(c.ships_sunk),
        ships_out_of_action: spread(c.ships_out_of_action),
        aircraft: spread(c.aircraft),
    }
}

fn went(w: &Went) -> TrialWent {
    TrialWent {
        played: w.worlds.clone(),
        met_kept: w.met_kept.clone(),
        met_days: w.met_days.map(spread),
        lost_again: w.lost_again.clone(),
        not_met: w.not_met.clone(),
    }
}

/// The world a task's things are read from, where the staff formed them
/// inside the trial: the world whose plan order it is, as its play left it.
fn world_of(played: &Played, number: Option<u32>) -> Option<&PlayedWorld> {
    let number = number?;
    played.worlds.iter().find(|w| w.number == number)
}

impl Context {
    fn force(&self, id: TaskForceId, world: Option<&PlayedWorld>) -> ForceShown {
        if let Some(own) = self.live.force(id) {
            return ForceShown::Live(own);
        }
        // A force the staff would form: its ships, as the trial's world has
        // them, each a ship of the side's the live game has.
        let ships = world
            .and_then(|w| w.world.as_ref())
            .and_then(|built| {
                built
                    .side_view(self.side)
                    .task_forces()
                    .find(|f| f.id == id)
                    .map(|f| f.ships.iter().filter_map(|s| self.live.ship(*s)).collect())
            })
            .unwrap_or_default();
        ForceShown::Formed { ships }
    }

    fn base(&self, id: BaseId, world: Option<&PlayedWorld>) -> BaseShown {
        if let Some(base) = self.live.base(id) {
            return BaseShown::Live(base);
        }
        let place = world.and_then(|w| w.world.as_ref()).and_then(|built| {
            built
                .side_view(self.side)
                .bases()
                .find(|b| b.id == id)
                .map(|b| b.hex)
        });
        BaseShown::Formed { place }
    }

    fn task(&self, kind: IntentKind, world: Option<&PlayedWorld>) -> TaskShown {
        let base = |id| self.base(id, world);
        let force = |id| self.force(id, world);
        match kind {
            IntentKind::KeepSupplied {
                base: b,
                floor_days,
                ..
            } => TaskShown::KeepSupplied {
                base: base(b),
                floor_days,
            },
            IntentKind::Cover { base: b } => TaskShown::Cover { base: base(b) },
            IntentKind::Hold { base: b } => TaskShown::Hold { base: base(b) },
            IntentKind::Neutralise { target } => TaskShown::Neutralise {
                target: base(target),
            },
            IntentKind::Raid { base: b, target } => TaskShown::Raid {
                base: base(b),
                target: base(target),
            },
            IntentKind::Deny { hex, distance } => TaskShown::Deny { hex, distance },
            IntentKind::StrikeShips { base: b } => TaskShown::StrikeShips { base: base(b) },
            IntentKind::SearchWaters { base: b } => TaskShown::SearchWaters { base: base(b) },
            IntentKind::StandAt { force: f, hex } => TaskShown::StandAt {
                force: force(f),
                hex,
            },
            IntentKind::OperateCarriers { force: f, hex } => TaskShown::OperateCarriers {
                force: force(f),
                hex,
            },
            IntentKind::ReadyLanding {
                force: f,
                hex,
                by_day,
            } => TaskShown::ReadyLanding {
                force: force(f),
                hex,
                by_day,
            },
            IntentKind::BuildUp { base: b, wants } => TaskShown::BuildUp {
                base: base(b),
                wants: wants.iter().collect(),
            },
            IntentKind::TakePlace { hex, by_day } => TaskShown::TakePlace { hex, by_day },
            IntentKind::Bombard { force: f, target } => TaskShown::Bombard {
                force: force(f),
                target: base(target),
            },
            IntentKind::ControlIsland { island, by_day } => {
                TaskShown::ControlIsland { island, by_day }
            }
        }
    }

    fn contact_hex(&self, place: u32) -> Option<Hex> {
        let index = usize::try_from(place).ok()?.checked_sub(1)?;
        self.contacts.get(index).map(|c| c.hex)
    }

    fn contact_kind(&self, place: u32) -> Option<(ShipKind, u32)> {
        let index = usize::try_from(place).ok()?.checked_sub(1)?;
        self.contacts.get(index).map(|c| (c.kind, c.ship_count))
    }

    /// What a thing assumed was, and what it was made from (rule 16).
    fn thing(&self, of: &AssumedOf) -> (AssumedThing, MadeFrom) {
        match of {
            AssumedOf::EnemyBase {
                base,
                hex,
                supply_t,
                fuel_t,
            } => match self.live.base(*base) {
                Some(base) => (
                    AssumedThing::EnemyBase {
                        base,
                        place: *hex,
                        supply_t: *supply_t,
                        fuel_t: *fuel_t,
                    },
                    MadeFrom::Map,
                ),
                None => (
                    AssumedThing::Force {
                        kind: None,
                        ships: None,
                        place: Some(*hex),
                    },
                    MadeFrom::Map,
                ),
            },
            AssumedOf::MenAshore { hex, men } => (
                AssumedThing::Men {
                    place: *hex,
                    men: *men,
                },
                MadeFrom::Estimate,
            ),
            AssumedOf::BelievedForce { place, name } => self.believed_force(*place, name),
            AssumedOf::BelievedMen {
                name,
                hex,
                men,
                standing,
                ..
            } => (
                AssumedThing::Men {
                    place: *hex,
                    men: *men,
                },
                MadeFrom::Believed {
                    name: name.clone(),
                    standing: *standing,
                },
            ),
            AssumedOf::Contact { place } => {
                let kind = self.contact_kind(*place);
                let at = self.contact_hex(*place);
                (
                    AssumedThing::Force {
                        kind: kind.map(|(k, _)| k),
                        ships: kind.map(|(_, n)| n),
                        place: at,
                    },
                    match at {
                        Some(at) => MadeFrom::Contact { at },
                        None => MadeFrom::Trial,
                    },
                )
            }
            AssumedOf::Pair { first, second } => {
                let kind = self.contact_kind(*first);
                let (a, b) = (self.contact_hex(*first), self.contact_hex(*second));
                (
                    AssumedThing::Force {
                        kind: kind.map(|(k, _)| k),
                        ships: None,
                        place: a,
                    },
                    match (a, b) {
                        (Some(first), Some(second)) => MadeFrom::Pair { first, second },
                        _ => MadeFrom::Trial,
                    },
                )
            }
            AssumedOf::EnemyGivenNoBelief => (AssumedThing::NoBelief, MadeFrom::Trial),
            AssumedOf::EnemyIntent { kind } => {
                let (words, place, base) = enemy_intent(*kind);
                (
                    AssumedThing::EnemyIntent {
                        words: words.to_string(),
                        place,
                        base: base.and_then(|b| self.live.base(b)),
                    },
                    MadeFrom::Trial,
                )
            }
            AssumedOf::EnemyGivenNoIntent => (AssumedThing::NoIntent, MadeFrom::Trial),
        }
    }

    fn believed_force(&self, place: u32, name: &str) -> (AssumedThing, MadeFrom) {
        let found = usize::try_from(place)
            .ok()
            .and_then(|p| p.checked_sub(1))
            .and_then(|i| self.believed.forces.get(i));
        let (kind, ships, at, standing) = match found {
            Some(force) => (
                Some(force.kind),
                Some(force.ships),
                match force.last_known {
                    sim_core::believed::LastKnown::NotKnown => None,
                    sim_core::believed::LastKnown::AtBase { hex, .. }
                    | sim_core::believed::LastKnown::AtHex { hex, .. } => Some(hex),
                },
                force.standing,
            ),
            None => (None, None, None, Standing::Unknown),
        };
        (
            AssumedThing::Force {
                kind,
                ships,
                place: at,
            },
            MadeFrom::Believed {
                name: name.to_string(),
                standing,
            },
        )
    }

    fn note(&self, note: &AssumedNote) -> AssumedNoteShown {
        match note {
            AssumedNote::PlacedByReach { worlds } => {
                AssumedNoteShown::PlacedByReach { worlds: *worlds }
            }
            AssumedNote::AtEnemyBase { worlds } => {
                AssumedNoteShown::AtEnemyBase { worlds: *worlds }
            }
            AssumedNote::TakenForContact { contact, worlds } => AssumedNoteShown::TakenForContact {
                at: self.contact_hex(*contact),
                worlds: *worlds,
            },
            AssumedNote::NotMade { worlds, .. } => AssumedNoteShown::NotMade { worlds: *worlds },
            AssumedNote::KindRaised {
                from, to, worlds, ..
            } => AssumedNoteShown::KindRaised {
                from: *from,
                to: *to,
                worlds: *worlds,
            },
            AssumedNote::ShipsLeftOut {
                kind,
                count,
                worlds,
            } => AssumedNoteShown::ShipsLeftOut {
                kind: *kind,
                count: *count,
                worlds: *worlds,
            },
            AssumedNote::GroupLeftOut {
                aircraft, worlds, ..
            } => AssumedNoteShown::GroupLeftOut {
                aircraft: *aircraft,
                worlds: *worlds,
            },
            AssumedNote::RuledOut { worlds } => AssumedNoteShown::RuledOut { worlds: *worlds },
            AssumedNote::SetAside { why, worlds } => match why {
                sim_trial::SetAside::CountSince { day } => AssumedNoteShown::SetAsideForACount {
                    day: *day,
                    worlds: *worlds,
                },
                sim_trial::SetAside::PlaceIsTheSides => {
                    AssumedNoteShown::SetAsideThePlaceIsYours { worlds: *worlds }
                }
            },
        }
    }

    fn part(&self, part: &PartSide) -> PartShown {
        PartShown {
            went: went(&part.went),
            side: part.side.as_ref().map(costs),
            enemy: part.enemy.as_ref().map(costs),
        }
    }

    fn parted_by(&self, by: &PartedBy) -> (AssumedThing, MadeFrom) {
        match by {
            PartedBy::BelievedForce { place, name } => self.believed_force(*place, name),
            PartedBy::BelievedMen { place, name } => {
                let found = usize::try_from(*place)
                    .ok()
                    .and_then(|p| p.checked_sub(1))
                    .and_then(|i| self.believed.men.get(i));
                match found {
                    Some(men) => (
                        AssumedThing::Men {
                            place: men.hex,
                            men: men.men,
                        },
                        MadeFrom::Believed {
                            name: name.clone(),
                            standing: men.standing,
                        },
                    ),
                    None => (
                        AssumedThing::NoBelief,
                        MadeFrom::Believed {
                            name: name.clone(),
                            standing: Standing::Unknown,
                        },
                    ),
                }
            }
            PartedBy::Pair { first, second } => self.thing(&AssumedOf::Pair {
                first: *first,
                second: *second,
            }),
        }
    }
}

/// An intent the enemy is taken to hold, in a commander's words, with its
/// place or base; never a force of his by number.
fn enemy_intent(kind: IntentKind) -> (&'static str, Option<Hex>, Option<BaseId>) {
    match kind {
        IntentKind::TakePlace { hex, .. } => ("take the place", Some(hex), None),
        IntentKind::Hold { base } => ("hold", None, Some(base)),
        IntentKind::Neutralise { target } => ("neutralise", None, Some(target)),
        IntentKind::Raid { target, .. } => ("raid", None, Some(target)),
        IntentKind::KeepSupplied { base, .. } => ("keep supplied", None, Some(base)),
        IntentKind::Cover { base } => ("cover from the air", None, Some(base)),
        IntentKind::Deny { hex, .. } => ("deny the waters", Some(hex), None),
        IntentKind::ControlIsland { .. } => ("control an island", None, None),
        IntentKind::Bombard { target, .. } => ("bombard", None, Some(target)),
        IntentKind::StandAt { hex, .. } | IntentKind::OperateCarriers { hex, .. } => {
            ("put ships at sea", Some(hex), None)
        }
        IntentKind::ReadyLanding { hex, .. } => ("land", Some(hex), None),
        IntentKind::StrikeShips { base } | IntentKind::SearchWaters { base } => {
            ("strike ships", None, Some(base))
        }
        IntentKind::BuildUp { base, .. } => ("build up", None, Some(base)),
    }
}

/// The answer, filled from the estimate and the trial as played.
fn fill(context: &Context, estimate: &Estimate, played: &Played) -> TrialAnswer {
    let world = world_of(played, estimate.plan_order_world);
    let needs = match (
        &estimate.plan_order,
        estimate.plan_order_world,
        estimate.plan_order_day,
    ) {
        (Some(order), Some(number), Some(day)) => Some(NeedsShown {
            world: number,
            day,
            men: order.men.map(|m| (m.wanted, m.to_hand)),
            waits_for: order
                .waits_for
                .iter()
                .filter(|w| !w.met)
                .filter_map(|w| match w.thing {
                    WaitedFor::Men { wanted, to_hand } => Some(WaitShown::Men { wanted, to_hand }),
                    WaitedFor::Leave(leave) => Some(WaitShown::Leave(leave)),
                    WaitedFor::Day(day) => Some(WaitShown::Day(day)),
                    WaitedFor::Force(Awaited::TaskForce(id)) => {
                        Some(WaitShown::Force(context.force(id, world)))
                    }
                    WaitedFor::Force(Awaited::LandUnit(id)) => {
                        context.live.land_unit(id).map(WaitShown::LandUnit)
                    }
                    WaitedFor::Intent(intent) => Some(WaitShown::Intent(intent)),
                    WaitedFor::Knowledge(knowledge) => Some(WaitShown::Knowledge(knowledge)),
                })
                .collect(),
            expected_ashore: order.expected_ashore,
            count: order.need_to_know.as_ref().map(|need| CountShown {
                place: need.place,
                asked: need.asked,
                held: need.held.map(|h| (h.men, h.grade, h.day)),
                met: need.met,
            }),
            tasks: order
                .tasks
                .iter()
                .map(|task| PlanTaskShown {
                    task: context.task(task.kind, world),
                    phase: task.phase,
                })
                .collect(),
            differs_in: estimate.plan_order_differs_in.clone(),
        }),
        _ => None,
    };
    let other_plans = estimate.other_plans.as_ref().map(|plans| {
        plans
            .iter()
            .map(|plan| OtherPlanShown {
                intent: plan.intent,
                task: context.task(plan.kind, world),
                with: (plan.with_course.worlds.clone(), plan.with_course.middle_day),
                without: (plan.without.worlds.clone(), plan.without.middle_day),
                fewer: plan.displaced == Some(sim_trial::Displaced::InFewerWorlds),
                later: plan.displaced == Some(sim_trial::Displaced::Later),
            })
            .collect()
    });
    let assumed = estimate
        .assumed
        .iter()
        .map(|line| {
            let (thing, from) = context.thing(&line.of);
            AssumedShown {
                thing,
                from,
                made_in: line.made_in.clone(),
                not_in: line.not_in.clone(),
                notes: line.notes.iter().map(|n| context.note(n)).collect(),
            }
        })
        .collect();
    let partings = estimate
        .partings
        .iter()
        .map(|parting| {
            let (by, from) = context.parted_by(&parting.by);
            PartingShown {
                by,
                from,
                with: context.part(&parting.with),
                without: context.part(&parting.without),
            }
        })
        .collect();
    TrialAnswer {
        worlds: context.size.worlds,
        days: context.size.days,
        size: context.size_name.clone(),
        first_day: context.first_day,
        last_day: estimate.last_day,
        has_end: estimate.end.is_some(),
        went: went(&estimate.went),
        could_not: estimate.could_not.iter().map(|f| f.world).collect(),
        side_costs: estimate.side_costs.as_ref().map(costs),
        enemy_costs: estimate.enemy_costs.as_ref().map(costs),
        needs,
        other_plans,
        assumed,
        leaves_given: estimate.leaves_given.clone(),
        uneven: estimate.uneven,
        partings,
    }
}
