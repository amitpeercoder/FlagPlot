//! `measure`'s `choices` question: what the rules would choose now for one of
//! the side's own units (decision 0100).
//!
//! The answer is every question `SideView::questions` lists for the unit, each
//! with `SideView::ask`'s answer written as plain JSON. There is no rule here:
//! the figures are the engine's own choice functions, asked through the view,
//! so an answer is what the staff would do now with what the side holds. Every
//! unit, ship, base and contact is written as the side's own handle, never as
//! an engine id.

use std::collections::BTreeSet;

use serde::Serialize;
use serde_json::{json, Map, Value};
use sim_core::choices::air::{
    AirAnswer, AirQuestion, BaseStrikeLaunch, EscortChoice, FighterSplit, GroupLanding, RestChoice,
};
use sim_core::choices::ground::{GroundAnswer, GroundQuestion, UnitDay};
use sim_core::choices::movement::{MovementAnswer, MovementQuestion};
use sim_core::choices::surface::{
    AlertChoice, StragglerAction, StragglerPlan, SurfaceAnswer, SurfaceQuestion,
};
use sim_core::choices::{Answer, Question};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, ContactId, EntityRef, LandUnitId, ShipId, TaskForceId};
use sim_core::state::AirGroupLocation;
use sim_core::StrikeChoice;

use crate::answer::{FORMAT, VERSION};
use crate::common::{Ctx, Dec, Handle};

const NOTE: &str = "Each answer is what the rules would do now with what your side holds, not a forecast. The five choices made at the point of contact (which enemy force a raid attacks, which ship a strike group attacks, how a patrol divides between raids, whether a force that avoids slips away, and who fires at whom) are not asked.";

/// One question of the unit, as the rules ask it, and the answer.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct ChoiceLine {
    /// The choice, in snake_case: `strike`, `fighter_split`, `halt`, ...
    pub choice: &'static str,
    /// The parameters of the question (an object, empty if it has none).
    pub asked: Value,
    /// The answer's fields.
    pub answer: Value,
}

/// The answer to `measure` `choices`.
#[derive(Clone, PartialEq, Eq, Debug, Serialize)]
pub struct Choices {
    pub format: &'static str,
    pub version: u32,
    pub kind: &'static str,
    pub question: &'static str,
    pub unit: Handle,
    pub note: &'static str,
    /// In the order of `SideView::questions`: air, surface, movement, ground.
    pub choices: Vec<ChoiceLine>,
}

impl Choices {
    /// The answer as a few plain lines: the choice, what was asked and the
    /// answer, the last two as the JSON the data holds.
    pub fn to_text(&self) -> String {
        let mut text = format!(
            "FLAG PLOT MEASURE: CHOICES OF {}\n- {}\n",
            self.unit.text(),
            self.note
        );
        if self.choices.is_empty() {
            text.push_str("- No choice arises for this unit now.\n");
        }
        for line in &self.choices {
            text.push_str(&format!(
                "- {}: asked {}, answer {}\n",
                line.choice, line.asked, line.answer
            ));
        }
        text
    }
}

/// The unit a handle names, if it is one of the side's own air groups, task
/// forces or land units.
enum Unit {
    Group(AirGroupId),
    Force(TaskForceId),
    Land(LandUnitId),
}

fn unit_of(ctx: &Ctx<'_>, handle: &str) -> Option<(Unit, Handle)> {
    let (kind, number) = handle.split_once(':')?;
    let number: u32 = number.parse().ok()?;
    match kind {
        "air_group" => {
            let group = ctx.own_group(number)?;
            Some((Unit::Group(group.id), ctx.group(group.id)))
        }
        "task_force" => {
            let force = ctx.own_force(number)?;
            Some((Unit::Force(force.id), ctx.force(force.id)))
        }
        "land_unit" => {
            let unit = ctx.own_land_unit(number)?;
            Some((Unit::Land(unit.id), ctx.land_unit(unit.id)))
        }
        _ => None,
    }
}

/// Whether `question` is about `unit`.
fn is_about(question: &Question, unit: &Unit) -> bool {
    match (question, unit) {
        (Question::Air(q), Unit::Group(id)) => match q {
            AirQuestion::Strike { group }
            | AirQuestion::AircraftAbleToFly { group }
            | AirQuestion::BaseStrike { group, .. }
            | AirQuestion::FighterSplit { group }
            | AirQuestion::Escort { group }
            | AirQuestion::Rest { group } => group == id,
            AirQuestion::Landing { groups } => groups.iter().any(|(group, _)| group == id),
        },
        (Question::Surface(q), Unit::Force(id)) => match q {
            SurfaceQuestion::Alert { task_force }
            | SurfaceQuestion::Ready { task_force, .. }
            | SurfaceQuestion::BreakOff { task_force, .. }
            | SurfaceQuestion::Stragglers { task_force }
            | SurfaceQuestion::HeadHome { task_force } => task_force == id,
        },
        (Question::Movement(q), Unit::Force(id)) => match q {
            MovementQuestion::Reroute { force, .. }
            | MovementQuestion::Halt { force }
            | MovementQuestion::SteamOn { force } => force == id,
        },
        (Question::Ground(q), Unit::Land(id)) => match q {
            GroundQuestion::BackToDefend { unit, .. }
            | GroundQuestion::UnitDay { unit, .. }
            | GroundQuestion::AttackSpent { unit, .. } => unit == id,
        },
        _ => false,
    }
}

/// The choices of one own unit, or `None` if the handle is not one of the
/// side's own air groups, task forces or land units.
pub(crate) fn choices(ctx: &Ctx<'_>, handle: &str) -> Option<Choices> {
    let (unit, shown) = unit_of(ctx, handle)?;
    let writer = Writer { ctx };
    let mut lines = Vec::new();
    for question in ctx.view.questions() {
        if !is_about(&question, &unit) {
            continue;
        }
        let Some(answer) = ctx.view.ask(&question) else {
            continue;
        };
        lines.push(writer.line(&question, &answer));
    }
    Some(Choices {
        format: FORMAT,
        version: VERSION,
        kind: "measure",
        question: "choices",
        unit: shown,
        note: NOTE,
        choices: lines,
    })
}

struct Writer<'a, 'v> {
    ctx: &'a Ctx<'v>,
}

fn hex(hex: Hex) -> Value {
    json!({"q": hex.q, "r": hex.r})
}

fn opt_hex(hex_: Option<Hex>) -> Value {
    hex_.map_or(Value::Null, hex)
}

fn handle(h: Handle) -> Value {
    serde_json::to_value(h).expect("a handle is plain data")
}

impl Writer<'_, '_> {
    fn ship(&self, id: ShipId) -> Value {
        handle(self.ctx.ship(id))
    }

    fn force(&self, id: TaskForceId) -> Value {
        handle(self.ctx.force(id))
    }

    fn group(&self, id: AirGroupId) -> Value {
        handle(self.ctx.group(id))
    }

    fn ships(&self, ids: &BTreeSet<ShipId>) -> Value {
        Value::Array(ids.iter().map(|id| self.ship(*id)).collect())
    }

    /// A contact as the side's contact number names it.
    fn contact(&self, id: ContactId) -> Value {
        handle(self.ctx.entity(EntityRef::Contact(id)))
    }

    fn line(&self, question: &Question, answer: &Answer) -> ChoiceLine {
        match (question, answer) {
            (Question::Air(q), Answer::Air(a)) => self.air(q, a),
            (Question::Surface(q), Answer::Surface(a)) => self.surface(q, a),
            (Question::Movement(q), Answer::Movement(a)) => self.movement(q, a),
            (Question::Ground(q), Answer::Ground(a)) => self.ground(q, a),
            _ => unreachable!("the view answers a question with an answer of its own kind"),
        }
    }

    fn air(&self, question: &AirQuestion, answer: &AirAnswer) -> ChoiceLine {
        let mut asked = Map::new();
        let (choice, answer) = match (question, answer) {
            (AirQuestion::Strike { .. }, AirAnswer::Strike(launch)) => {
                ("strike", self.strike(launch.as_ref()))
            }
            (AirQuestion::AircraftAbleToFly { .. }, AirAnswer::AircraftAbleToFly(n)) => {
                ("aircraft_able_to_fly", json!({"aircraft": n}))
            }
            (AirQuestion::BaseStrike { pulse, .. }, AirAnswer::BaseStrike(launch)) => {
                asked.insert("pulse".into(), json!(pulse.name()));
                ("base_strike", self.base_strike(launch.as_ref()))
            }
            (AirQuestion::FighterSplit { .. }, AirAnswer::FighterSplit(split)) => {
                let FighterSplit { patrol, escorts } = split;
                (
                    "fighter_split",
                    json!({"patrol": patrol, "escorts": escorts}),
                )
            }
            (AirQuestion::Escort { .. }, AirAnswer::Escort(escort)) => {
                ("escort", self.escort(escort))
            }
            (AirQuestion::Landing { groups }, AirAnswer::Landing(landings)) => {
                asked.insert(
                    "groups".into(),
                    Value::Array(
                        groups
                            .iter()
                            .map(|(group, in_air)| {
                                json!({"group": self.group(*group), "aircraft_in_air": in_air})
                            })
                            .collect(),
                    ),
                );
                (
                    "landing",
                    json!({"landings": landings.iter().map(|l| self.landing(l)).collect::<Vec<_>>()}),
                )
            }
            (AirQuestion::Rest { .. }, AirAnswer::Rest(rest)) => (
                "rest",
                json!({"rest": match rest {
                    RestChoice::StandDown => "stand_down",
                    RestChoice::Return => "return",
                    RestChoice::NoChange => "no_change",
                }}),
            ),
            _ => unreachable!("the view answers a question with an answer of its own kind"),
        };
        ChoiceLine {
            choice,
            asked: Value::Object(asked),
            answer,
        }
    }

    fn strike(&self, launch: Option<&StrikeChoice>) -> Value {
        let Some(choice) = launch else {
            return json!({"launches": false});
        };
        let report = self
            .ctx
            .view
            .events()
            .iter()
            .find(|event| event.id == choice.report_event)
            .map(|event| event.number);
        json!({
            "launches": true,
            "contact": self.contact(choice.contact),
            "taken_for": choice.kind.name(),
            "distance_nm": Dec(choice.distance_nm),
            "hex": hex(choice.hex),
            "report_event": report,
        })
    }

    fn base_strike(&self, launch: Option<&BaseStrikeLaunch>) -> Value {
        let Some(launch) = launch else {
            return json!({"launches": false});
        };
        json!({
            "launches": true,
            "base": handle(self.ctx.base(launch.base)),
            "hex": hex(launch.hex),
            "distance_nm": Dec(launch.distance_nm),
            "aircraft": launch.aircraft,
        })
    }

    fn escort(&self, escort: &EscortChoice) -> Value {
        match escort {
            EscortChoice::NoRaid => json!({"escort": "no_raid"}),
            EscortChoice::Stays => json!({"escort": "stays"}),
            EscortChoice::Goes { lead, hex: to } => json!({
                "escort": "goes",
                "raid_led_by": self.group(*lead),
                "hex": hex(*to),
            }),
        }
    }

    fn landing(&self, landing: &GroupLanding) -> Value {
        let places: Vec<Value> = landing
            .places
            .iter()
            .map(|(place, aircraft)| {
                let place = match place {
                    AirGroupLocation::Base(base) => handle(self.ctx.base(*base)),
                    AirGroupLocation::Ship(ship) => self.ship(*ship),
                    AirGroupLocation::Lost => json!("lost"),
                };
                json!({"place": place, "aircraft": aircraft})
            })
            .collect();
        json!({
            "group": self.group(landing.group),
            "places": places,
            "into_the_sea": landing.sea,
        })
    }

    fn surface(&self, question: &SurfaceQuestion, answer: &SurfaceAnswer) -> ChoiceLine {
        let mut asked = Map::new();
        let (choice, answer) = match (question, answer) {
            (SurfaceQuestion::Alert { .. }, SurfaceAnswer::Alert(alert)) => (
                "alert",
                match alert {
                    AlertChoice::NoChange => json!({"alert": "no_change"}),
                    AlertChoice::AlertedBy(contact) => {
                        json!({"alert": "alerted_by", "contact": self.contact(*contact)})
                    }
                    AlertChoice::StandDown => json!({"alert": "stand_down"}),
                },
            ),
            (SurfaceQuestion::Ready { day, pulse, .. }, SurfaceAnswer::Ready(ready)) => {
                asked.insert("day".into(), json!(day));
                asked.insert("pulse".into(), json!(pulse.name()));
                ("ready", json!({"ready": ready}))
            }
            (
                SurfaceQuestion::BreakOff {
                    began_with,
                    fired_on,
                    already_avoiding,
                    ..
                },
                SurfaceAnswer::BreakOff(breaks),
            ) => {
                asked.insert(
                    "began_with".into(),
                    Value::Array(began_with.iter().map(|ship| self.ship(*ship)).collect()),
                );
                asked.insert("fired_on".into(), json!(fired_on));
                asked.insert("already_avoiding".into(), json!(already_avoiding));
                ("break_off", json!({"breaks_off": breaks}))
            }
            (SurfaceQuestion::Stragglers { .. }, SurfaceAnswer::Stragglers(plan)) => {
                ("stragglers", self.stragglers(plan))
            }
            (SurfaceQuestion::HeadHome { .. }, SurfaceAnswer::HeadHome(home)) => {
                ("head_home", json!({"hex": opt_hex(*home)}))
            }
            _ => unreachable!("the view answers a question with an answer of its own kind"),
        };
        ChoiceLine {
            choice,
            asked: Value::Object(asked),
            answer,
        }
    }

    fn stragglers(&self, plan: &StragglerPlan) -> Value {
        let mut answer = Map::new();
        match &plan.action {
            StragglerAction::None => {
                answer.insert("action".into(), json!("none"));
            }
            StragglerAction::Leave { stragglers } => {
                answer.insert("action".into(), json!("leave"));
                answer.insert("stragglers".into(), self.ships(stragglers));
            }
            StragglerAction::Detach {
                sent_home,
                escorts,
                left_behind,
            } => {
                answer.insert("action".into(), json!("detach"));
                answer.insert("sent_home".into(), self.ships(sent_home));
                answer.insert("escorts".into(), self.ships(escorts));
                answer.insert("left_behind".into(), self.ships(left_behind));
            }
            StragglerAction::Retire { left_behind } => {
                answer.insert("action".into(), json!("retire"));
                answer.insert("left_behind".into(), self.ships(left_behind));
            }
        }
        answer.insert("sound_ships".into(), json!(plan.sound_ships));
        answer.insert("escorts_needed".into(), json!(plan.escorts_needed));
        answer.insert("escorts_taken".into(), json!(plan.escorts_taken));
        answer.insert("kept".into(), json!(plan.kept));
        Value::Object(answer)
    }

    fn movement(&self, question: &MovementQuestion, answer: &MovementAnswer) -> ChoiceLine {
        let mut asked = Map::new();
        let (choice, answer) = match (question, answer) {
            (MovementQuestion::Reroute { first_waypoint, .. }, MovementAnswer::Reroute(r)) => {
                asked.insert("first_waypoint".into(), opt_hex(*first_waypoint));
                (
                    "reroute",
                    match r {
                        None => json!({"route_possible": false}),
                        Some(r) => json!({
                            "route_possible": true,
                            "turns_back": r.turns_back,
                            "steers_from": hex(r.steers_from),
                        }),
                    },
                )
            }
            (MovementQuestion::Halt { .. }, MovementAnswer::Halt(index)) => {
                // The index is the waypoint's place in the route, from 0.
                ("halt", json!({"halts_at_waypoint": index}))
            }
            (MovementQuestion::SteamOn { .. }, MovementAnswer::SteamOn(on)) => {
                ("steam_on", json!({"steams_on": on}))
            }
            _ => unreachable!("the view answers a question with an answer of its own kind"),
        };
        ChoiceLine {
            choice,
            asked: Value::Object(asked),
            answer,
        }
    }

    fn ground(&self, question: &GroundQuestion, answer: &GroundAnswer) -> ChoiceLine {
        let mut asked = Map::new();
        let (choice, answer) = match (question, answer) {
            (GroundQuestion::BackToDefend { fatigue, .. }, GroundAnswer::BackToDefend(back)) => {
                asked.insert("fatigue".into(), json!(Dec(*fatigue)));
                ("back_to_defend", json!({"goes_back": back}))
            }
            // What the unit would do when its attack is spent (decisions 6
            // and 7 of 0105), at the hex it stands in.
            (GroundQuestion::AttackSpent { hex, .. }, GroundAnswer::AttackSpent(choice)) => {
                asked.insert("hex".into(), json!(self.ctx.place(*hex)));
                ("attack_spent", json!({"does": choice.name()}))
            }
            (
                GroundQuestion::UnitDay {
                    marched,
                    contested,
                    rostered_with,
                    ..
                },
                GroundAnswer::UnitDay(day),
            ) => {
                asked.insert("marched".into(), json!(marched));
                asked.insert("contested".into(), json!(contested));
                asked.insert(
                    "rostered_with".into(),
                    rostered_with.map_or(Value::Null, |force| self.force(force)),
                );
                ("unit_day", self.unit_day(day))
            }
            _ => unreachable!("the view answers a question with an answer of its own kind"),
        };
        ChoiceLine {
            choice,
            asked: Value::Object(asked),
            answer,
        }
    }

    fn unit_day(&self, day: &UnitDay) -> Value {
        match day {
            UnitDay::NoMen => json!({"day": "no_men"}),
            UnitDay::Marched => json!({"day": "marched"}),
            UnitDay::Contested => json!({"day": "contested"}),
            // No job named is the default job: the airfield if the base has
            // none, else the port.
            UnitDay::Work(job) => {
                json!({"day": "work", "job": job.map_or("default", |job| job.name())})
            }
            UnitDay::Train => json!({"day": "train"}),
            UnitDay::Rehearse(force) => json!({"day": "rehearse", "force": self.force(*force)}),
            UnitDay::Ready => json!({"day": "ready"}),
        }
    }
}
