use std::collections::{BTreeMap, BTreeSet};

use crate::choices::ground::{GroundAnswer, GroundQuestion, UnitDay};
use crate::choices::{Answer, Question};
use crate::events::{Breakdown, Cause, EventKind, ExperienceSource, Factor};
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{BaseId, EventId, LandUnitId, TaskForceId};
use crate::orders::Activity;
use crate::refdata::GroundFigures;
use crate::state::{LandUnit, TruthState};
use crate::world::World;

/// What a land unit's day is (decision 3b of 0099): none if no man is
/// ashore or aboard; none if it marched or its hex was contested; else work
/// if it is a working unit (always) or was set to work, training if it was
/// set to train, and else, standing ready, rehearsal if it is on the roster
/// of a force that qualified today (`rostered_with`) and `with_force` says
/// it is in that force's hex or aboard its ships. `with_force` is asked only
/// then. Decision 1 of 0100: the engine and a side's view both ask it.
// The inputs are the unit's state and the three facts of the day; a struct
// would only rename them.
#[allow(clippy::too_many_arguments)]
pub(crate) fn unit_day(
    men_ashore: u32,
    men_aboard: u32,
    working: bool,
    activity: Activity,
    marched: bool,
    contested: bool,
    rostered_with: Option<TaskForceId>,
    with_force: impl FnOnce(TaskForceId) -> bool,
) -> UnitDay {
    if men_ashore == 0 && men_aboard == 0 {
        return UnitDay::NoMen;
    }
    if marched {
        return UnitDay::Marched;
    }
    if contested {
        return UnitDay::Contested;
    }
    // A working unit always works (decision 1 of 0099).
    let activity = if working {
        Activity::Work(activity.job())
    } else {
        activity
    };
    match activity {
        Activity::Work(job) => UnitDay::Work(job),
        Activity::Train => UnitDay::Train,
        Activity::Ready => match rostered_with {
            Some(force) if with_force(force) => UnitDay::Rehearse(force),
            _ => UnitDay::Ready,
        },
    }
}

/// Whether every man of `unit_id` is ashore in the hex of `force` or aboard
/// its ships, with at least one man (decision 3b of 0099, and 4 of 0095).
/// Reads only the unit and the force, which a side holds of its own.
pub(crate) fn is_with_force(truth: &TruthState, unit_id: LandUnitId, force: TaskForceId) -> bool {
    let unit = &truth.land_units[&unit_id];
    let force = &truth.task_forces[&force];
    let aboard_here: u32 = force
        .ships
        .iter()
        .filter_map(|ship| truth.ships[ship].troops)
        .filter(|troops| troops.unit == unit_id)
        .map(|troops| troops.men)
        .sum();
    let ashore_ok = unit.men_ashore == 0 || unit.hex == Some(force.from_hex);
    ashore_ok && aboard_here == truth.men_aboard(unit_id) && unit.men_ashore + aboard_here > 0
}

impl World {
    /// The day's activity, once every hex is resolved (decision 3b of 0099).
    /// First the loading of each force with a preparation; then each land
    /// unit's one activity, in unit order: none if it marched or its hex was
    /// contested, else work if it is a working unit or was set to work
    /// (decisions 3 and 3a of 0099), else training if it was ordered to
    /// train and the conditions of decision 3 of 0097 hold, else rehearsal if
    /// it is on the roster of a force that qualified today and is in its hex
    /// or aboard its ships; then the airfields the day's work built; then the
    /// day's count of fighting for each unit that fired.
    /// Each gain is given once because the phase runs once a day.
    pub(crate) fn day_activity(
        &mut self,
        day: u32,
        marched: &BTreeMap<LandUnitId, EventId>,
        contested_hexes: &BTreeSet<Hex>,
        fired: &BTreeMap<LandUnitId, EventId>,
        figures: &GroundFigures,
    ) {
        let qualifying = self.gain_loading(day, figures);
        let rostered = self.rostered_with(&qualifying);

        let units: Vec<LandUnitId> = self.truth.land_units.keys().copied().collect();
        let mut on_airfields: BTreeMap<BaseId, Vec<EventId>> = BTreeMap::new();
        for unit_id in units {
            let unit = &self.truth.land_units[&unit_id];
            let aboard = self.truth.men_aboard(unit_id);
            // A unit that marched, or whose hex was contested, marches or
            // fights: none of the three.
            let contested =
                unit.men_ashore > 0 && unit.hex.is_some_and(|hex| contested_hexes.contains(&hex));
            let marched_today = marched.contains_key(&unit_id);
            let rostered_with = rostered.get(&unit_id).copied();
            let side = unit.side;
            let answer = unit_day(
                unit.men_ashore,
                aboard,
                unit.working,
                unit.activity,
                marched_today,
                contested,
                rostered_with,
                |force| is_with_force(&self.truth, unit_id, force),
            );
            self.trace_choice(side, || {
                (
                    Question::Ground(GroundQuestion::UnitDay {
                        unit: unit_id,
                        marched: marched_today,
                        contested,
                        rostered_with,
                    }),
                    Answer::Ground(GroundAnswer::UnitDay(answer)),
                )
            });
            match answer {
                UnitDay::NoMen | UnitDay::Marched | UnitDay::Contested | UnitDay::Ready => {}
                UnitDay::Work(job) => self.work_day(day, unit_id, job, figures, &mut on_airfields),
                UnitDay::Train => self.train_day(day, unit_id, figures),
                UnitDay::Rehearse(force) => self.rehearse_day(day, unit_id, force, figures),
            }
        }
        // The airfields the day's work built (decision 5 of 0099).
        self.finish_airfields(day, &on_airfields, figures);

        // The days of fighting (decision 5 of 0097).
        for (unit_id, fire_event) in fired {
            let unit = &self.truth.land_units[unit_id];
            if unit.experience >= LandUnit::MAX_EXPERIENCE
                || (unit.men_ashore == 0 && self.truth.men_aboard(*unit_id) == 0)
            {
                continue;
            }
            let before = unit.fight_days;
            let experience = unit.experience;
            let days = before + 1;
            let breakdown = self.config.record_breakdowns.then(|| Breakdown {
                factors: vec![
                    Factor {
                        name: "days_before",
                        value: Fixed::from_int(i64::from(before)),
                    },
                    Factor {
                        name: "fight_days_per_level",
                        value: Fixed::from_int(i64::from(figures.fight_days_per_level)),
                    },
                ],
                result: Fixed::from_int(i64::from(days)),
            });
            let counted = self.events.record(
                day,
                None,
                EventKind::FightDayCounted {
                    unit: *unit_id,
                    days,
                    experience,
                },
                vec![Cause::Event(*fire_event)],
                breakdown,
            );
            self.count_toward_experience(
                day,
                *unit_id,
                days,
                figures.fight_days_per_level,
                ExperienceSource::Fighting,
                counted,
            );
        }
    }

    /// A day of training for a unit set to train, if the conditions of
    /// decision 3 of 0097 hold: all its men ashore at a base of its side,
    /// fed in full. At or above the ceiling nothing changes and nothing is
    /// recorded: the unit still spent the day.
    fn train_day(&mut self, day: u32, unit_id: LandUnitId, figures: &GroundFigures) {
        let unit = &self.truth.land_units[&unit_id];
        let Some(hex) = unit.hex.filter(|_| unit.men_ashore > 0) else {
            return;
        };
        let side = unit.side;
        let in_base = self
            .truth
            .bases
            .values()
            .any(|base| base.side == side && base.hex == hex);
        if self.truth.men_aboard(unit_id) > 0 || !in_base || unit.fed_share != Fixed::ONE {
            return;
        }
        if u32::from(unit.experience) >= figures.training_ceiling {
            return;
        }
        let before = unit.training_days;
        let experience = unit.experience;
        let days = before + 1;
        let mut causes: Vec<Cause> = unit.activity_order.map(Cause::Order).into_iter().collect();
        let fed = self.events.latest_where(|kind| {
            matches!(kind, EventKind::MenFed { side: owner, hex: at_hex, .. }
                if *owner == side && *at_hex == hex)
        });
        causes.extend(
            fed.filter(|event| {
                self.events
                    .get(*event)
                    .is_some_and(|event| event.day == day)
            })
            .map(Cause::Event),
        );
        let breakdown = self.config.record_breakdowns.then(|| Breakdown {
            factors: vec![
                Factor {
                    name: "days_before",
                    value: Fixed::from_int(i64::from(before)),
                },
                Factor {
                    name: "training_days_per_level",
                    value: Fixed::from_int(i64::from(figures.training_days_per_level)),
                },
            ],
            result: Fixed::from_int(i64::from(days)),
        });
        let counted = self.events.record(
            day,
            None,
            EventKind::TrainingDay {
                unit: unit_id,
                days,
                experience,
            },
            causes,
            breakdown,
        );
        self.count_toward_experience(
            day,
            unit_id,
            days,
            figures.training_days_per_level,
            ExperienceSource::Training,
            counted,
        );
    }

    /// Stores a day counted toward a step of experience (`days` is the count
    /// after the day); at `per_level` the experience rises by one and the
    /// count starts again, with `ExperienceRaised` citing the event of the
    /// day.
    fn count_toward_experience(
        &mut self,
        day: u32,
        unit_id: LandUnitId,
        days: u32,
        per_level: u32,
        source: ExperienceSource,
        counted: EventId,
    ) {
        let unit = self
            .truth
            .land_units
            .get_mut(&unit_id)
            .expect("the land unit exists");
        let raised = days >= per_level;
        let (stored, experience) = if raised {
            (0, unit.experience + 1)
        } else {
            (days, unit.experience)
        };
        match source {
            ExperienceSource::Training => unit.training_days = stored,
            ExperienceSource::Fighting => unit.fight_days = stored,
        }
        unit.experience = experience;
        if raised {
            self.events.record(
                day,
                None,
                EventKind::ExperienceRaised {
                    unit: unit_id,
                    experience,
                    by: source,
                },
                vec![Cause::Event(counted)],
                None,
            );
        }
    }
}
