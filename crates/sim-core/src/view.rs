//! What one side is allowed to see.
//!
//! A [`SideView`] is the only thing a player, the staff planner or an agent
//! is ever handed. It exposes the side's own units exactly and the enemy only
//! as contacts. It has no method that returns enemy truth, so code holding a
//! `SideView` cannot read it.
//!
//! The events a side may see come from [`SideView::events`], each cut down to
//! what the side may know (decision 0047, [`crate::side_events`]). The full log
//! stays with the umpire.
//!
//! The umpire's view is [`crate::World::umpire`]. It is for tests, the
//! command-line runner and hashing, and must never be passed to anything that
//! plays the game.

mod air_choices;
mod command_view;
pub use command_view::CommandView;
mod known_view;
pub use known_view::KnownView;
mod ground_choices;
mod seat_view;
mod supply;

pub use supply::{DaysOfSupply, PlannedUse};
mod movement_choices;
pub mod strike_review;
mod surface_choices;

use std::collections::{BTreeMap, BTreeSet};

use crate::airstrike::{NavalStrike, StrikeChoice, StrikeInFlight};
use crate::belief::{BattleClaim, BeliefState, Contact, GroundEstimate};
use crate::choices::{Answer, Question};
use crate::events::EventLog;
use crate::fixed::Fixed;
use crate::hex::Hex;
use crate::ids::{
    AirGroupId, AircraftTypeId, BaseId, ContactId, EntityRef, EventId, LandUnitId, OrderId,
    ShipClassId, ShipId, TaskForceId,
};
use crate::landing::DeckLoss;
use crate::map::{HexData, Map};
use crate::movement::{self, PassageForecast};
use crate::objectives::{self, KnownProgress, Objective, SideObjectives};
use crate::orders::{AirMission, Order, SpeedSetting};
use crate::refdata::{
    AirCombatFigures, AirSupplyFigures, AirUpkeepFigures, AircraftType, BombardmentFigures,
    FatigueFigures, GroundFigures, PortSize, RefData, SearchFigures, ShipClass,
    SurfaceCombatFigures,
};
use crate::side_events::{self, Owners, SideEvent};
use crate::state::{
    AirGroup, Arrival, Base, Formation, LandUnit, Replacement, Ship, Side, TaskForce, TruthState,
};
use crate::upkeep::Sortie;

/// An air raid on one of the side's own bases, as its events show it
/// ([`SideView::air_raids_on`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaidSeen {
    pub day: u32,
    pub pulse: Option<crate::time::Pulse>,
    /// Whether a patrol of the side's, from a group now based there, fired
    /// in the same pulse.
    pub fought: bool,
}

/// Which optional rules have their figures loaded: air wear and repair
/// (decision 0035), fatigue and rest (0036), and base supply use (0039).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RulesInPlay {
    pub wear: bool,
    pub fatigue: bool,
    pub supply: bool,
}

/// What every side knows of a base that is not its own: where it is and which
/// side holds it, and its number and name. That is the map, not intelligence
/// (decision 0034, 2). The type has no field for supply, fuel, guns, repairs or
/// aircraft, so nothing more can be carried in it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PublicBase {
    pub id: BaseId,
    pub name: String,
    pub hex: Hex,
    /// The side that holds it.
    pub side: Side,
}

/// One side's view of the game.
///
/// `Debug` is written by hand and prints the side only. A derived one would
/// print the whole truth state the view holds, enemy included, into whatever
/// log, briefing or prompt the view was formatted for.
#[derive(Clone, Copy)]
pub struct SideView<'a> {
    side: Side,
    truth: &'a TruthState,
    belief: &'a BeliefState,
    /// The side's own commands and their pictures, while command levels are
    /// on (decision 0134). The other side's are never handed over.
    commands: Option<&'a crate::pictures::SideCommands>,
    /// What the side's area command holds of the commands under it, while
    /// reports travel (decision 0136); the other side's is never handed over.
    held: Option<&'a BTreeMap<crate::pictures::CommandRef, crate::held::HeldState>>,
    /// Whom this view is the truth of: `None` for the side's own view, whole;
    /// `Some(Area)` for a view built over what the area command holds
    /// ([`SideView::known_to`], decision 0136, 9).
    known_to: Option<crate::pictures::CommandRef>,
    /// The command this view is cut to, for a seat at a task force or a base
    /// ([`SideView::at_seat`], `view/seat_view.rs`): its own units, its
    /// events and its picture are the command's and nothing else of the
    /// side's. `None` for the side's own view and for the area seat.
    seat: Option<crate::pictures::CommandRef>,
    /// Whether [`SideView::seat_picture`] is the side's own belief, every
    /// report at once, not a command's picture (the easier setting of
    /// decision 0119, answer 5).
    side_picture: bool,
    /// The task forces on radio silence, both sides' (decision 0136, 15).
    radio_silent: &'a BTreeSet<TaskForceId>,
    map: &'a Map,
    ref_data: &'a RefData,
    log: &'a EventLog,
    orders: &'a BTreeMap<OrderId, Order>,
    objectives: Option<&'a crate::objectives::Objectives>,
    /// The side's own command state, whole (decision 0107, 6). The other
    /// side's is never handed over.
    command: &'a crate::command::CommandState,
    /// The number the side's next made task force will take (its own
    /// counter; the other side's is never handed over).
    next_task_force: TaskForceId,
    /// The number the side's next air group made in play will take (its own
    /// counter; the other side's is never handed over, decision 0171, 6).
    next_air_group: AirGroupId,
    /// The scratch of the pulse in hand, which the choices of the air steps
    /// read (decision 0100): the bases grounded at the pulse's start, the
    /// strikes launched and not landed, what each group flew and the decks
    /// lost. Each holds both sides' entries, so whatever reads them keeps
    /// this side's own and nothing else.
    pulse: PulseScratch<'a>,
    /// The day open and the pulse next, while a day is resolved in parts
    /// (decision 0156): the clock, which both sides know, and no truth of
    /// either side. Copied from the world when the view is made. `None`
    /// between days.
    day_in_hand: Option<crate::world::DayInHand>,
}

/// The world's scratch state of one pulse, by reference (see
/// [`SideView`]'s field `pulse`). Every list holds both sides.
#[derive(Clone, Copy)]
pub(crate) struct PulseScratch<'a> {
    pub(crate) grounded_bases: &'a BTreeSet<crate::ids::BaseId>,
    pub(crate) field_damage: &'a BTreeMap<crate::ids::BaseId, Fixed>,
    pub(crate) strikes_in_flight: &'a [StrikeInFlight],
    pub(crate) sorties: &'a [Sortie],
    pub(crate) deck_losses: &'a [DeckLoss],
}

impl std::fmt::Debug for SideView<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SideView")
            .field("side", &self.side)
            .finish_non_exhaustive()
    }
}

impl<'a> SideView<'a> {
    // The world hands over each part of its fixed and changing state by
    // reference; one more argument (the objectives) is clearer than a bundle.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        side: Side,
        truth: &'a TruthState,
        belief: &'a BeliefState,
        commands: Option<&'a crate::pictures::SideCommands>,
        held: Option<&'a BTreeMap<crate::pictures::CommandRef, crate::held::HeldState>>,
        radio_silent: &'a BTreeSet<TaskForceId>,
        map: &'a Map,
        ref_data: &'a RefData,
        log: &'a EventLog,
        orders: &'a BTreeMap<OrderId, Order>,
        objectives: Option<&'a crate::objectives::Objectives>,
        command: &'a crate::command::CommandState,
        next_task_force: TaskForceId,
        next_air_group: AirGroupId,
        pulse: PulseScratch<'a>,
        day_in_hand: Option<crate::world::DayInHand>,
    ) -> SideView<'a> {
        SideView {
            side,
            truth,
            belief,
            commands,
            held,
            known_to: None,
            seat: None,
            side_picture: false,
            radio_silent,
            map,
            ref_data,
            log,
            orders,
            objectives,
            command,
            next_task_force,
            next_air_group,
            pulse,
            day_in_hand,
        }
    }

    /// The day open and the pulse next, while a day is resolved in parts:
    /// at 0600 or 1200 of a game that stops between pulses (decision 0156).
    /// `None` between days. It is the clock, known to both sides.
    ///
    /// For the agent interface's words only: no rule of the engine and
    /// nothing of the staff reads it.
    pub fn day_in_hand(&self) -> Option<crate::world::DayInHand> {
        self.day_in_hand
    }

    /// The last pulse resolved (decision 0156): with no day open, what
    /// [`SideView::pulse_just_ended`] is; with day D open, the afternoon of
    /// D-1 while its night is next, the night of D while its morning is
    /// next, the morning of D while its afternoon is next, and the afternoon
    /// of D once that is resolved and the day not yet closed.
    ///
    /// For the agent interface's words only: no rule of the engine and
    /// nothing of the staff reads it; they read `pulse_just_ended`.
    pub fn pulse_last_resolved(&self) -> crate::held::AsOf {
        use crate::held::AsOf;
        let Some(in_hand) = self.day_in_hand else {
            return self.pulse_just_ended();
        };
        let day = in_hand.day;
        match in_hand.next_pulse {
            Some(crate::time::Pulse::Night) => match day - 1 {
                0 => AsOf::Start,
                before => AsOf::Pulse {
                    day: before,
                    pulse: crate::time::Pulse::Afternoon,
                },
            },
            Some(crate::time::Pulse::Morning) => AsOf::Pulse {
                day,
                pulse: crate::time::Pulse::Night,
            },
            Some(crate::time::Pulse::Afternoon) => AsOf::Pulse {
                day,
                pulse: crate::time::Pulse::Morning,
            },
            None => AsOf::Pulse {
                day,
                pulse: crate::time::Pulse::Afternoon,
            },
        }
    }

    /// The number the side's next made task force will take: what a
    /// `FormTaskForce` order given now would make, if no other forming
    /// order is taken in first. Its own series only.
    pub fn next_task_force_id(&self) -> TaskForceId {
        if self.seat.is_some() {
            return TaskForceId::new(0);
        }
        self.next_task_force
    }

    /// The number the side's next air group made in play will take
    /// (decision 0171, 6): its own series only. A side knows its own air
    /// groups, and the number says nothing of the enemy. A view cut to a
    /// seat gives 0, as [`SideView::next_task_force_id`] does there.
    pub fn next_air_group_id(&self) -> AirGroupId {
        if self.seat.is_some() {
            return AirGroupId::new(0);
        }
        self.next_air_group
    }

    /// The side's own command state, whole: its intents with their limits,
    /// its leaves, its plans with their baselines, amendments and runs, and
    /// the commitments of its ships (decision 0107, 6). There is no way to
    /// the other side's.
    pub fn command_state(&self) -> &'a crate::command::CommandState {
        if self.seat.is_some() {
            // A seat at a command states no intent and holds none of the
            // side's (decision 0138, 5).
            return seat_view::blank_command_state();
        }
        self.command
    }

    pub fn side(&self) -> Side {
        self.side
    }

    /// The phase of a "take a place" intent of the side (decision 0142, 5),
    /// read from its plan and the side's own men: prepare while the plan is
    /// preparing; move while it is released and no man of the side is ashore
    /// at the place; land while it is released and one is. `None` for any
    /// other intent, an intent not in force, or a plan in any other state.
    pub fn place_phase(&self, intent: crate::ids::IntentId) -> Option<crate::command::PlacePhase> {
        use crate::command::{PlacePhase, PlanState};
        let state = self.command_state();
        let found = state.intents.get(&intent).filter(|i| i.in_force())?;
        let place = found.kind.place()?;
        match state.plans.get(&found.plan)?.state {
            PlanState::Preparing => Some(PlacePhase::Prepare),
            PlanState::Released => {
                let ashore = self
                    .land_units()
                    .any(|unit| unit.men_ashore > 0 && unit.hex == Some(place));
                Some(if ashore {
                    PlacePhase::Land
                } else {
                    PlacePhase::Move
                })
            }
            _ => None,
        }
    }

    /// The last day resolved.
    pub fn day(&self) -> u32 {
        self.truth.day
    }

    /// The side's own ships, known exactly. Supply aboard is visible only for
    /// the own side, because the filter hides every enemy ship.
    pub fn ships(&self) -> impl Iterator<Item = &'a Ship> + '_ {
        let view = *self;
        self.truth
            .ships
            .values()
            .filter(move |ship| view.owns_ship(ship))
    }

    /// The side's own task forces, known exactly.
    pub fn task_forces(&self) -> impl Iterator<Item = &'a TaskForce> + '_ {
        let view = *self;
        self.truth
            .task_forces
            .values()
            .filter(move |task_force| view.owns_force(task_force))
    }

    /// The side's own bases. Supply and fuel stock are visible only for the own side,
    /// because the filter hides every enemy base.
    pub fn bases(&self) -> impl Iterator<Item = &'a Base> + '_ {
        let view = *self;
        self.truth
            .bases
            .values()
            .filter(move |base| view.owns_base(base))
    }

    /// The public facts of every base on the map the side does not own, in
    /// number order (see [`PublicBase`]). Nothing else of an enemy base is
    /// visible, and a base the enemy formed by a landing is not on the map
    /// (decision 0066, 12): it is not here at all.
    pub fn enemy_bases(&self) -> impl Iterator<Item = PublicBase> + '_ {
        let view = *self;
        self.truth
            .bases
            .values()
            .filter(move |base| view.is_public_base(base))
            .map(|base| PublicBase {
                id: base.id,
                name: base.name.clone(),
                hex: base.hex,
                side: base.side,
            })
    }

    /// The side's own land units, known exactly: men ashore and where
    /// (decision 0066, 21). Their men aboard are in [`SideView::ships`], each
    /// ship's `troops`. The enemy's land units are not here, nor anywhere
    /// else in the view (decision 0066, 22). A destroyed unit is listed with
    /// no men.
    pub fn land_units(&self) -> impl Iterator<Item = &'a LandUnit> + '_ {
        let view = *self;
        self.truth
            .land_units
            .values()
            .filter(move |unit| view.owns_land_unit(unit))
    }

    /// The day from which the side may use one of its own units (decision
    /// 0140, 5): a task force, a ship, an air group or a land unit, the
    /// later of its own day and those of what carries it, as intake reads it
    /// ([`TruthState::available_from`]). `None` for a unit with no day, which
    /// may be used from day 1, and for a unit that is not the side's: no
    /// enemy's day is ever given.
    pub fn available_from(&self, unit: EntityRef) -> Option<u32> {
        if !self.owns_unit(unit) {
            return None;
        }
        self.truth.available_from(unit)
    }

    /// Whether `unit` is a task force, ship, air group or land unit of the
    /// side.
    fn owns_unit(&self, unit: EntityRef) -> bool {
        let side = self.side;
        let own = match unit {
            EntityRef::Ship(id) => self.truth.ships.get(&id).is_some_and(|x| x.side == side),
            EntityRef::TaskForce(id) => self
                .truth
                .task_forces
                .get(&id)
                .is_some_and(|x| x.side == side),
            EntityRef::AirGroup(id) => self
                .truth
                .air_groups
                .get(&id)
                .is_some_and(|x| x.side == side),
            EntityRef::LandUnit(id) => self
                .truth
                .land_units
                .get(&id)
                .is_some_and(|x| x.side == side),
            _ => false,
        };
        own
    }

    /// Whether the side may use one of its own units on the day it now gives
    /// orders for, the day after [`SideView::day`] (decision 0140, 3): it has
    /// no day, or its day has come. The one test every doctrine reads.
    /// False for a unit that is not the side's.
    pub fn may_use(&self, unit: EntityRef) -> bool {
        self.owns_unit(unit)
            && self
                .truth
                .available_from(unit)
                .is_none_or(|from| self.day() + 1 >= from)
    }

    /// The side's own formations of land units, in number order (decision
    /// 0140, 8): each with its parent, so that the tree can be read from any
    /// unit's [`LandUnit::formation`] up. The enemy's are not here.
    pub fn formations(&self) -> impl Iterator<Item = &'a Formation> + '_ {
        let side = self.side;
        self.truth
            .formations
            .values()
            .filter(move |formation| formation.side == side)
    }

    /// The cover of one of the side's own land units (decision 1 of 0106):
    /// 0 if it is marching or on attack, else the higher of its own
    /// fieldworks and the defences of its side's lowest-numbered base in its
    /// hex. It reads only the side's own unit and bases. `None` for a unit
    /// that is not the side's. The engine reads the same function.
    pub fn cover(&self, unit: LandUnitId) -> Option<Fixed> {
        let unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|unit| self.owns_land_unit(unit))?;
        Some(crate::cover::unit_cover(self.truth, unit))
    }

    /// The men of one of the side's own land units who would fire if it
    /// fought now (decision 6 of 0090): its men ashore, cut by its fed share
    /// and by its fatigue, by the function the fight calls. The side's own
    /// unit only: `None` for a unit that is not the side's, or where the
    /// scenario has no ground figures.
    pub fn firing_men(&self, unit: LandUnitId) -> Option<u32> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|unit| self.owns_land_unit(unit))?;
        self.ref_data.ground.as_ref()?;
        Some(
            crate::ground_combat::firing_men(
                land_unit.men_ashore,
                land_unit.fed_share,
                land_unit.fatigue,
            )
            .0,
        )
    }

    /// The fatigue one of the side's own land units would have after one
    /// more day of attack (decision 12 of 0090): its fatigue now plus the
    /// ground figure `fatigue_per_attack_day`, no more than 100. It leaves
    /// out a march, a failed attack and strain, which add to it. The side's
    /// own unit only: `None` for a unit that is not the side's, or where
    /// the scenario has no ground figures.
    pub fn fatigue_after_attack_day(&self, unit: LandUnitId) -> Option<Fixed> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|unit| self.owns_land_unit(unit))?;
        let figures = self.ref_data.ground.as_ref()?;
        Some((land_unit.fatigue + figures.fatigue_per_attack_day).min(Fixed::from_int(100)))
    }

    /// The men of one of the side's own land units aboard its ships. `None`
    /// for a unit that is not the side's.
    pub fn men_aboard(&self, unit: LandUnitId) -> Option<u32> {
        let own = self
            .truth
            .land_units
            .get(&unit)
            .is_some_and(|land_unit| self.owns_land_unit(land_unit));
        own.then(|| self.men_aboard_in_view(unit))
    }

    /// The side's own air groups, known exactly.
    pub fn air_groups(&self) -> impl Iterator<Item = &'a AirGroup> + '_ {
        let view = *self;
        self.truth
            .air_groups
            .values()
            .filter(move |group| view.owns_group(group))
    }

    /// The side's own replacement streams (decision 6 of 0038), known
    /// exactly. The enemy's are not visible.
    pub fn replacements(&self) -> impl Iterator<Item = &'a Replacement> + '_ {
        let view = *self;
        self.truth
            .replacements
            .iter()
            .filter(move |stream| view.owns_stream(stream))
    }

    /// The arrivals due to the side's own air groups (decision 0041), known
    /// exactly. The enemy's are not visible.
    pub fn arrivals(&self) -> impl Iterator<Item = &'a Arrival> + '_ {
        let view = *self;
        self.truth.arrivals.iter().filter(move |arrival| {
            view.truth
                .air_groups
                .get(&arrival.air_group)
                .is_some_and(|group| view.owns_group(group))
        })
    }

    /// What the side believes about the enemy.
    pub fn contacts(&self) -> impl Iterator<Item = &'a Contact> + '_ {
        self.own_belief().contacts()
    }

    /// What the side believes came of its fights: the enemy ships it claims
    /// sank, in the order the claims arrived. Real damage is truth and is not
    /// shown.
    pub fn claims(&self) -> &'a [BattleClaim] {
        self.own_belief().claims()
    }

    /// What the side believes of the enemy's men ashore: one estimate for a
    /// hex, in hex order, each the count of the last ground report or
    /// opening estimate there, with its day (decision 0092, 11 and 19). It
    /// may be wrong; the truth is not shown.
    pub fn ground_estimates(&self) -> impl Iterator<Item = &'a GroundEstimate> + '_ {
        self.own_belief().ground_estimates()
    }

    /// The side's estimate of the enemy's men ashore in `hex`, if it has one.
    pub fn ground_estimate(&self, hex: Hex) -> Option<&'a GroundEstimate> {
        self.own_belief().ground_estimate(hex)
    }

    /// The figures of shore bombardment, if a force can bombard (decision
    /// 0092): public rules, not intelligence.
    pub fn bombardment_figures(&self) -> Option<&'a BombardmentFigures> {
        self.ref_data
            .surface_combat
            .as_ref()
            .and_then(|surface| surface.bombardment.as_ref())
    }

    /// The side's own orders, accepted or not, in the order they were taken
    /// in. This includes the orders a rule gave for the side's units. An
    /// order's `id` is opaque; the side's own count of its orders is
    /// [`SideView::order_number`] (decisions 0055 and 0086).
    pub fn orders(&self) -> impl Iterator<Item = &'a Order> + '_ {
        let view = *self;
        self.orders
            .values()
            .filter(move |order| order.side == view.side && view.owns_order(order))
    }

    /// The events the side may see, each cut down to what it may know, in
    /// log order (decision 0047). Each carries the side's own `number`, 1, 2,
    /// 3 and so on, so a gap in the numbers is not something the side can
    /// see (decision 0086); its `id` is opaque. This is the only way events leave for anything that plays the
    /// game; the full log stays with the umpire.
    pub fn events(&self) -> Vec<SideEvent> {
        let owners = Owners::from_state(
            self.truth,
            self.orders,
            self.ref_data
                .surface_combat
                .as_ref()
                .map_or(0, |figures| figures.round_minutes),
        );
        let all = side_events::project(self.side, self.log.from_index(0), &owners);
        match self.seat {
            None => all,
            Some(_) => self.cut_events(all),
        }
    }

    /// The events of one day the side may see, in log order.
    pub fn events_on_day(&self, day: u32) -> Vec<SideEvent> {
        let mut events = self.events();
        events.retain(|event| event.day == day);
        events
    }

    /// One event the side may see, by its (opaque) id. `None` if there is no
    /// such event or the side may not see it. Its causes are the
    /// ones the side may see, and `hidden_causes` says if there are more.
    pub fn event(&self, id: EventId) -> Option<SideEvent> {
        self.events().into_iter().find(|event| event.id == id)
    }

    /// The side's number of an event the side may see: the `number` its
    /// [`SideEvent`] carries (decision 0086). `None` if there is no such
    /// event or the side may not see it.
    pub fn event_number(&self, id: EventId) -> Option<u32> {
        self.event(id).map(|event| event.number)
    }

    /// One event the side may see, by the side's own number of it (1 for the
    /// first [`SideView::events`] gives). `None` if there is no such event.
    pub fn event_by_number(&self, number: u32) -> Option<SideEvent> {
        self.events()
            .into_iter()
            .find(|event| event.number == number)
    }

    /// The side's number of one of its own orders: its place among the
    /// side's own orders, 1 for the first [`SideView::orders`] gives
    /// (decision 0086). `None` if the order is not the side's own.
    pub fn order_number(&self, id: OrderId) -> Option<u32> {
        let own = self.orders().position(|order| order.id == id)?;
        u32::try_from(own + 1).ok()
    }

    /// One of the side's own orders by the side's number of it. `None` if
    /// the side has no such order.
    pub fn order_by_number(&self, number: u32) -> Option<&'a Order> {
        let place = usize::try_from(number.checked_sub(1)?).ok()?;
        self.orders().nth(place)
    }

    /// The side's number of a contact: the count of contacts the side has
    /// made, which is already the side's own (each side numbers its own
    /// contacts from 1).
    pub fn contact_number(&self, id: ContactId) -> u32 {
        id.raw()
    }

    /// One contact the side holds, by the side's number of it. `None` if the
    /// side holds no such contact.
    pub fn contact_by_number(&self, number: u32) -> Option<&'a Contact> {
        self.own_belief()
            .contacts()
            .find(|contact| contact.id.raw() == number)
    }

    /// How many of the side's own air group `group`'s aircraft can fly now:
    /// the engine's own count ([`crate::airstrike`]'s `able_to_fly`), on the
    /// group's ready aircraft, pilots, deck, field and fatigue. `None` for a
    /// number that is not one of the side's groups. Nothing of the enemy is
    /// read.
    pub fn able_to_fly(&self, group: AirGroupId) -> Option<u32> {
        let found = self.air_groups().find(|own| own.id == group)?;
        Some(self.flyable_aircraft(found))
    }

    /// The air raids on the side's own base `base` that the side knows of,
    /// from its own events (decision 0117, 14, and 0115, 6): each daylight
    /// pulse in which the base was bombed, in order, with whether a patrol of
    /// the side's fought in that pulse (a patrol's fire, by a group of the
    /// side's now based at `base`). Empty for a base that is not the side's.
    /// What the side reads is what its events show it: the bombs that fell on
    /// its base and its own fighters' fire, nothing of the raiders.
    pub fn air_raids_on(&self, base: BaseId) -> Vec<RaidSeen> {
        use crate::events::{AirFirer, EventKind};
        use crate::side_events::SideEventKind;
        if !self.bases().any(|own| own.id == base) {
            return Vec::new();
        }
        let here: BTreeSet<AirGroupId> = self
            .air_groups()
            .filter(|group| group.location == crate::state::AirGroupLocation::Base(base))
            .map(|group| group.id)
            .collect();
        let mut raids: BTreeMap<(u32, Option<crate::time::Pulse>), bool> = BTreeMap::new();
        let mut patrols: BTreeSet<(u32, Option<crate::time::Pulse>)> = BTreeSet::new();
        for event in self.events() {
            let at = (event.day, event.pulse);
            match &event.kind {
                SideEventKind::BaseAttackedAgainst { base: hit, .. } if *hit == base => {
                    raids.entry(at).or_insert(false);
                }
                SideEventKind::AirFight {
                    firer,
                    firing: AirFirer::Patrol,
                    ..
                }
                | SideEventKind::Whole(EventKind::AirFight {
                    firer,
                    firing: AirFirer::Patrol,
                    ..
                }) if here.contains(firer) => {
                    patrols.insert(at);
                }
                _ => {}
            }
        }
        raids
            .into_iter()
            .map(|((day, pulse), _)| RaidSeen {
                day,
                pulse,
                fought: patrols.contains(&(day, pulse)),
            })
            .collect()
    }

    fn own_objectives(&self) -> Option<&'a SideObjectives> {
        // A seat at a command has no mission or objectives: they are the
        // side's, and the area command's to hold (decision 0138, 5).
        if self.seat.is_some() {
            return None;
        }
        self.objectives.map(|o| o.side(self.side))
    }

    /// The side's own mission (decision 6 of 0054), or `None` if the
    /// scenario has no objectives. Never the enemy's.
    pub fn mission(&self) -> Option<&'a str> {
        self.own_objectives().map(|o| o.mission.as_str())
    }

    /// The three margins of the scenario that make a minor, a major and a
    /// decisive victory, or `None` if the scenario has no objectives. One set
    /// serves both sides (the margin is one number seen from both), so it
    /// says nothing of the enemy's mission or objectives; it is part of how
    /// the side's own objectives are scored.
    pub fn levels(&self) -> Option<crate::objectives::Levels> {
        if self.seat.is_some() {
            return None;
        }
        self.objectives.map(|o| o.levels)
    }

    /// The days the scenario's result is judged over (decision 0088), or
    /// `None` if the scenario has no objectives. It is the scenario's
    /// configuration, common to both sides like [`SideView::levels`], and
    /// says nothing of the enemy's objectives or of any damage.
    pub fn horizon_days(&self) -> Option<u32> {
        if self.seat.is_some() {
            return None;
        }
        self.objectives.map(|o| o.horizon_days)
    }

    /// The side's own objectives, in the order the scenario gives them
    /// (decision 6 of 0054). Never the enemy's.
    pub fn objectives(&self) -> &'a [Objective] {
        self.own_objectives()
            .map_or(&[], |o| o.objectives.as_slice())
    }

    /// What the other side is scored for (decision 0183): its objectives
    /// alone, in the order the scenario gives them, and never its mission.
    /// `None` if the scenario has no objectives, and at a seat at a command,
    /// which is given no objectives; `Some` of an empty slice where the
    /// other side has none.
    ///
    /// This is the scenario's scoring, fixed before play and read by no rule
    /// of the simulation, of the same kind as [`SideView::levels`] and
    /// [`SideView::horizon_days`]: a rule of the game both players may read.
    /// It is no truth of the enemy's state and no precedent for a reading of
    /// anything of the enemy's state, his progress or what he knows. No
    /// staff, trial or rule reads it; a commander reads it and decides.
    pub fn enemy_objectives(&self) -> Option<&'a [Objective]> {
        if self.seat.is_some() {
            return None;
        }
        let other = Side::ALL
            .into_iter()
            .find(|side| *side != self.side)
            .expect("two sides");
        self.objectives.map(|o| o.side(other).objectives.as_slice())
    }

    /// The side's progress on each of its objectives, as far as it can know
    /// it (decision 7 of 0054): counted from its own units and from the
    /// events it may see, and from nothing else. In the order of
    /// [`SideView::objectives`]. What no unit or event of the side shows is
    /// marked not observable and counts 0. The umpire's result is not here.
    pub fn known_progress(&self) -> Vec<KnownProgress> {
        let Some(own) = self.own_objectives() else {
            return Vec::new();
        };
        let bases: Vec<BaseId> = self.bases().map(|base| base.id).collect();
        let none = std::collections::BTreeMap::new();
        let hexes = self.objectives.map_or(&none, |o| &o.base_hexes);
        objectives::known_progress(own, &bases, hexes, &self.events())
    }

    /// The hex of a base an objective of either side names, from the
    /// scenario's objectives (decision 0186, 17): where an objective is is
    /// part of the victory conditions (decision 0183), so a side is told it
    /// even for a base not on its map. `None` for a base no objective names,
    /// and at a seat at a command, which is given no objectives.
    pub fn objective_base_hex(&self, base: BaseId) -> Option<Hex> {
        if self.seat.is_some() {
            return None;
        }
        self.objectives
            .and_then(|o| o.base_hexes.get(&base))
            .copied()
    }

    /// The map is common knowledge.
    pub fn hex(&self, hex: Hex) -> Option<&'a HexData> {
        self.map.get(hex)
    }

    /// The island of that number (decision 0151, 1): on both sides' maps, as
    /// the map is.
    pub fn island(&self, number: u32) -> Option<&'a crate::map::Island> {
        self.map.island(number)
    }

    /// Every island the scenario names, by number.
    pub fn islands(&self) -> impl Iterator<Item = &'a crate::map::Island> + 'a {
        self.map.islands()
    }

    /// The hexes joined to `hex` by a land link, in hex order (decision 1 of
    /// 0094). The map is common knowledge. Empty on a map with no land
    /// links.
    pub fn land_neighbours(&self, hex: Hex) -> Vec<Hex> {
        self.map.land_neighbours(hex)
    }

    /// Where one of the side's own land units can be ordered to march: the
    /// hexes joined to its hex by a land link, in hex order (decision 3 of
    /// 0094). `None` for a unit that is not the side's or has no man ashore.
    pub fn march_destinations(&self, unit: LandUnitId) -> Option<Vec<Hex>> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|land_unit| self.owns_land_unit(land_unit) && land_unit.men_ashore > 0)?;
        land_unit.hex.map(|hex| self.map.land_neighbours(hex))
    }

    /// The factor a day's march counts as for one of the side's own land
    /// units marching from its hex to `to` (decision 4 of 0098): the factor
    /// of the lower of its movement skills for the terrain of the two hexes.
    /// `None` for a unit that is not the side's or has no man ashore, or
    /// where the scenario has no ground figures.
    pub fn movement_factor(&self, unit: LandUnitId, to: Hex) -> Option<Fixed> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|land_unit| self.owns_land_unit(land_unit) && land_unit.men_ashore > 0)?;
        let from = land_unit.hex?;
        let figures = self.ref_data.ground.as_ref()?;
        let skill = crate::march::march_skill(self.map, &land_unit.skills, from, to);
        Some(crate::state::Skills::factor(
            skill,
            figures.movement_skill_strength,
        ))
    }

    /// The supply one of the side's own land units' men ashore cost a day,
    /// in tons (decision 16 of 0066 and decision 5 of 0098): its men ashore
    /// times the daily charge times its weight factor. `None` for a unit that
    /// is not the side's, or where the scenario has no ground figures.
    pub fn daily_charge_t(&self, unit: LandUnitId) -> Option<Fixed> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|land_unit| self.owns_land_unit(land_unit))?;
        let figures = self.ref_data.ground.as_ref()?;
        Some(crate::ground::daily_charge_t(
            figures,
            &land_unit.skills,
            land_unit.men_ashore,
        ))
    }

    /// The factor one of the side's own forces' work is multiplied by while
    /// it lands men at `hex` (decision 5 of 0095): the share of the rate an
    /// unprepared force works at plus the rest of 1 times its loading for that
    /// hex over 100. `None` for a force that is not the side's, or where the
    /// scenario has no ground figures.
    pub fn landing_rate_factor(&self, task_force: TaskForceId, hex: Hex) -> Option<Fixed> {
        let force = self
            .truth
            .task_forces
            .get(&task_force)
            .filter(|force| self.owns_force(force))?;
        let figures = self.ref_data.ground.as_ref()?;
        Some(crate::preparation::landing_rate_factor(figures, force.preparation.as_ref(), hex).0)
    }

    /// The fatigue one of the side's own land units would gain when its first
    /// men land at `hex` (decision 6 of 0095), from its rehearsal for that
    /// hex. `None` for a unit that is not the side's, or where the scenario
    /// has no ground figures.
    pub fn landing_fatigue(&self, unit: LandUnitId, hex: Hex) -> Option<Fixed> {
        let land_unit = self
            .truth
            .land_units
            .get(&unit)
            .filter(|land_unit| self.owns_land_unit(land_unit))?;
        let figures = self.ref_data.ground.as_ref()?;
        Some(
            crate::preparation::landing_fatigue_gain(
                figures,
                land_unit.rehearsal,
                land_unit.rehearsal_hex,
                hex,
            )
            .0,
        )
    }

    /// The force whose roster one of the side's own land units is on, if any
    /// (decision 3c of 0099). `None` for a unit that is not the side's.
    pub fn roster_of(&self, unit: LandUnitId) -> Option<TaskForceId> {
        self.truth
            .land_units
            .get(&unit)
            .filter(|land_unit| self.owns_land_unit(land_unit))?;
        self.truth
            .task_forces
            .values()
            .filter(|force| self.owns_force(force))
            .find(|force| {
                force
                    .preparation
                    .as_ref()
                    .is_some_and(|preparation| preparation.roster.contains(&unit))
            })
            .map(|force| force.id)
    }

    /// The fewest steps by sea from every hex that can reach `target` to it
    /// (decision 0048, warnings). The map is common knowledge, so this is
    /// not enemy truth; it counts steps, not miles (a step is charged by
    /// [`SideView::sea_charged_nm_in`], decision 0058). Empty if ships cannot
    /// enter `target`.
    pub fn sea_distances_to(&self, target: Hex) -> BTreeMap<Hex, u32> {
        self.map.sea_distances_to(target)
    }

    /// The miles a force is charged for steaming from `from` to `to` by the
    /// map's shortest sea path (decision 0058): [`Map::sea_charged_nm`]. The
    /// straight-line air distance where the way is clear, more where land
    /// forces a detour. `None` when ships cannot get from one to the other.
    pub fn sea_charged_nm(&self, from: Hex, to: Hex) -> Option<u32> {
        self.map.sea_charged_nm(from, to)
    }

    /// [`SideView::sea_charged_nm`] with the field of
    /// [`SideView::sea_distances_to`] of `to` already worked out.
    pub fn sea_charged_nm_in(&self, field: &BTreeMap<Hex, u32>, from: Hex, to: Hex) -> Option<u32> {
        self.map.sea_charged_nm_in(field, from, to)
    }

    /// The hexes a force steams through from `from` to `to`, in order and
    /// ending at `to` (decision 0017), or `None` when ships cannot get from
    /// one to the other. The map is common knowledge, so this is not enemy
    /// truth; what the steps are charged is [`SideView::sea_charged_nm`].
    pub fn sea_path(&self, from: Hex, to: Hex) -> Option<Vec<Hex>> {
        self.map.sea_path(from, to)
    }

    /// What the movement rules would do with one of the side's own task
    /// forces if it were given a route to `destination` now (decision 0049),
    /// at `speed` if given and otherwise at the speed it has, leaving at the
    /// start of the day after [`SideView::day`]. `None` if `task_force` is
    /// not one of the side's own. It reads only the side's own force and
    /// ships, the map and the reference data, and writes nothing.
    pub fn forecast_passage(
        &self,
        task_force: TaskForceId,
        destination: Hex,
        speed: Option<SpeedSetting>,
    ) -> Option<PassageForecast> {
        let force = self.truth.task_forces.get(&task_force)?;
        if !self.owns_force(force) {
            return None;
        }
        let ships: BTreeMap<ShipId, Ship> = force
            .ships
            .iter()
            .filter_map(|id| self.truth.ships.get(id).map(|ship| (*id, ship.clone())))
            .collect();
        Some(movement::forecast_passage(
            self.map,
            self.ref_data,
            force,
            &ships,
            destination,
            speed,
            self.truth.day + 1,
        ))
    }

    /// What one of the side's own task forces lacks in ammunition and how it
    /// would rearm where it lies (decision 0122, 10): each ship's rounds and
    /// torpedoes lacked, the rate it would take them at and the hours its
    /// fuel would take first, and the base's stock. `None` if the force is
    /// not the side's own. It reads only the side's own forces, ships and
    /// bases.
    pub fn ammunition_work(&self, task_force: TaskForceId) -> Option<crate::rearm::AmmunitionWork> {
        let force = self.truth.task_forces.get(&task_force)?;
        if !self.owns_force(force) {
            return None;
        }
        crate::rearm::ammunition_work(self.truth, self.map, self.ref_data, self.side, task_force)
    }

    /// As [`SideView::forecast_passage`], for a route through `waypoints` in
    /// turn with no mark (decision 0124, 11): what the movement rules would
    /// do with the force given that route now, steaming on past each
    /// waypoint but the last. `None` if `task_force` is not one of the side's
    /// own.
    pub fn forecast_route(
        &self,
        task_force: TaskForceId,
        waypoints: &[Hex],
        speed: Option<SpeedSetting>,
    ) -> Option<PassageForecast> {
        let force = self.truth.task_forces.get(&task_force)?;
        if !self.owns_force(force) {
            return None;
        }
        let ships: BTreeMap<ShipId, Ship> = force
            .ships
            .iter()
            .filter_map(|id| self.truth.ships.get(id).map(|ship| (*id, ship.clone())))
            .collect();
        Some(movement::forecast_route(
            self.map,
            self.ref_data,
            force,
            &ships,
            waypoints,
            speed,
            self.truth.day + 1,
        ))
    }

    /// What a yard order given to one of the side's own task forces now would
    /// do (decision 0069): the port, the passage, each ship's days of repair
    /// and the days away, worked by the function the order itself uses.
    /// `None` if `task_force` is not one of the side's own, or has no ship
    /// afloat in it (a force away has none) or no port for the yard here. It
    /// reads only the side's own force, ships and bases and checks nothing
    /// else; whether the order would be accepted is `World::check_order`'s.
    pub fn yard_plan(&self, task_force: TaskForceId) -> Option<crate::repair::YardPlan> {
        let force = self.truth.task_forces.get(&task_force)?;
        if !self.owns_force(force) {
            return None;
        }
        crate::repair::yard_plan_in(self.truth, self.ref_data, task_force)
    }

    /// Reference data is common knowledge.
    pub fn ship_class(&self, id: ShipClassId) -> Option<&'a ShipClass> {
        self.ref_data.ship_classes.get(&id)
    }

    /// Aircraft types are reference data, so common knowledge.
    pub fn aircraft_type(&self, id: AircraftTypeId) -> Option<&'a AircraftType> {
        self.ref_data.aircraft_types.get(&id)
    }

    /// Every class of ship the scenario's reference data gives, in the order
    /// of their numbers (decision 0174, 8): reference data, so common
    /// knowledge.
    pub fn ship_classes(&self) -> impl Iterator<Item = &'a ShipClass> + 'a {
        self.ref_data.ship_classes.values()
    }

    /// Every type of aircraft the scenario's reference data gives, in the
    /// order of their numbers (decision 0174, 9): reference data, so common
    /// knowledge.
    pub fn aircraft_types(&self) -> impl Iterator<Item = &'a AircraftType> + 'a {
        self.ref_data.aircraft_types.values()
    }

    /// Whose a class of ship is (decision 0173, 4): reference data, so common
    /// knowledge. `None` when the scenario names no file of sides.
    pub fn ship_class_side(&self, id: ShipClassId) -> Option<crate::refdata::ClassSide> {
        self.ref_data.ship_class_side(id)
    }

    /// Whose a type of aircraft is (decision 0173, 4): reference data, so
    /// common knowledge. `None` when the scenario names no file of sides.
    pub fn aircraft_type_side(&self, id: AircraftTypeId) -> Option<crate::refdata::ClassSide> {
        self.ref_data.aircraft_type_side(id)
    }

    /// How far `group`'s strikes and escorts may go with the reach it has in
    /// force, in miles: [`RefData::reach_nm`], the one function every test of
    /// the distance of a strike or an escort uses (decision 2 of 0059). The
    /// figures are rules, so common knowledge.
    pub fn reach_nm(&self, group: &AirGroup) -> Fixed {
        self.ref_data.reach_nm(group.aircraft_type, group.reach)
    }

    /// How far `group` may fly to another base in one move:
    /// [`RefData::transfer_range_nm`], the one function the rebase order
    /// reads (decision 0091, 3). The figures are rules, so common knowledge.
    pub fn transfer_range_nm(&self, group: &AirGroup) -> Fixed {
        self.ref_data.transfer_range_nm(group.aircraft_type)
    }

    /// The contact `group` would launch a strike at now (decision 6 of 0063):
    /// the contact of [`SideView::naval_strike`]. The group is named by its
    /// number and looked up among this side's own groups, so a caller cannot
    /// hand in a group of its own making. `None` for a number that is not one
    /// of this side's groups, a group that is not on the naval strike
    /// mission, has no position, or has no contact that qualifies (or, sized,
    /// none that is not served). It reads belief and the side's own units and
    /// nothing of the enemy.
    pub fn strike_choice(&self, group: AirGroupId) -> Option<StrikeChoice> {
        self.naval_strike(group).map(|strike| strike.choice)
    }

    /// The naval strike `group` would launch now, contact and aircraft
    /// (decisions 6 of 0063, 0114 and 0118): `naval_strike_launch`
    /// (`airstrike.rs`), which the strike decision calls, on this side's
    /// belief and the group's own state, with what the side's groups before
    /// it in group number order would send in a daylight pulse, each worked
    /// the same way, and what has already been launched in the pulse in
    /// hand. A group before it launches as the decision launches it: on the
    /// naval strike mission, with ready aircraft, a position, a weapon and at
    /// least one aircraft to send. The aircraft are what it is able to fly
    /// less its held share, and may be none. `None` as for
    /// [`SideView::strike_choice`].
    pub fn naval_strike(&self, group: AirGroupId) -> Option<NavalStrike> {
        let asked = self.air_groups().find(|own| own.id == group)?;
        if asked.mission != AirMission::NavalStrike {
            return None;
        }
        let figures = self.ref_data.air_combat.as_ref()?;
        self.truth.air_group_hex(asked)?;
        let sent = self.sent_before_group(group, figures);
        self.naval_strike_of(asked, figures, &sent)
    }

    /// What the side's groups before `group`, in group number order, would
    /// launch at each contact in the pulse, with what is already in flight
    /// (decision 0118, 3): each on the naval strike mission with ready
    /// aircraft and a weapon launching what its own walk gives. The one walk
    /// [`SideView::naval_strike`] and the strike review both read.
    pub(crate) fn sent_before_group(
        &self,
        group: AirGroupId,
        figures: &crate::refdata::AirCombatFigures,
    ) -> BTreeMap<crate::ids::ContactId, u32> {
        let mut sent = crate::airstrike::sent_at_contacts(self.pulse.strikes_in_flight, self.side);
        let mut before: Vec<&AirGroup> = self
            .air_groups()
            .filter(|own| own.id < group && own.mission == AirMission::NavalStrike)
            .collect();
        before.sort_by_key(|own| own.id);
        for own in before {
            let armed = self
                .aircraft_type(own.aircraft_type)
                .is_some_and(|kind| kind.attack_weapon != crate::refdata::AttackWeapon::None);
            if own.ready == 0 || !armed {
                continue;
            }
            let Some(launch) = self.naval_strike_of(own, figures, &sent) else {
                continue;
            };
            if launch.aircraft == 0 {
                continue;
            }
            *sent.entry(launch.choice.contact).or_insert(0) += launch.aircraft;
        }
        sent
    }

    /// The naval strike of one of the side's own groups with `sent` already
    /// launched at each contact: `None` where it has no position or no
    /// contact to launch at.
    fn naval_strike_of(
        &self,
        group: &AirGroup,
        figures: &crate::refdata::AirCombatFigures,
        sent: &BTreeMap<crate::ids::ContactId, u32>,
    ) -> Option<NavalStrike> {
        let origin = self.truth.air_group_hex(group)?;
        let can_send = crate::airstrike::strike_aircraft(
            self.flyable_aircraft(group),
            group.strike_held_share,
        );
        // The picture of the group's command (decision 0134, 13).
        let command = crate::pictures::command_of_group(self.truth, group.id)?;
        crate::airstrike::naval_strike_launch(
            self.picture(command),
            group,
            origin,
            self.reach_nm(group),
            crate::airstrike::strike_age_limit(group, figures),
            can_send,
            figures,
            sent,
        )
    }

    /// The air combat figures' extended reach share and ditching chance are
    /// reference data, so common knowledge: the chance an aircraft runs out
    /// of fuel on the way home from a sortie of one-way distance
    /// `distance_nm`, for a type of strike radius `radius_nm`
    /// (`AirCombatFigures::ditch_chance`). `None` without the figures.
    pub fn ditch_chance(&self, distance_nm: Fixed, radius_nm: Fixed) -> Option<Fixed> {
        self.ref_data
            .air_combat
            .as_ref()
            .map(|figures| figures.ditch_chance(distance_nm, radius_nm))
    }

    /// The search figures are reference data, so common knowledge.
    pub fn search_figures(&self) -> Option<&'a SearchFigures> {
        self.ref_data.search.as_ref()
    }

    /// Which of the optional rules have their figures loaded for this game
    /// (decisions 0035, 0036 and 0039). The figures are rules, not enemy
    /// truth, so they are common knowledge, as the search figures are.
    /// The ground figures (decision 0066), if the scenario names them.
    pub fn ground_figures(&self) -> Option<&'a GroundFigures> {
        self.ref_data.ground.as_ref()
    }

    /// The base supply figures (decision 0039), if the scenario names them.
    pub fn air_supply_figures(&self) -> Option<&'a AirSupplyFigures> {
        self.ref_data.air_supply.as_ref()
    }

    pub fn rules_in_play(&self) -> RulesInPlay {
        RulesInPlay {
            wear: self.ref_data.air_upkeep.is_some(),
            fatigue: self.ref_data.fatigue.is_some(),
            supply: self.ref_data.air_supply.is_some(),
        }
    }

    /// Port handling figures are reference data, so common knowledge.
    pub fn port_size(&self, size: u8) -> Option<&'a PortSize> {
        self.ref_data.port_sizes.get(&size)
    }

    /// The surface combat figures, if the scenario names them: among them
    /// the ones a side's own choices use (how near a contact alerts a force,
    /// when a ship counts as crippled, the escorts a straggler takes). They
    /// are rules, the same for both sides, so common knowledge (decision
    /// 0100, 3).
    pub fn surface_combat_figures(&self) -> Option<&'a SurfaceCombatFigures> {
        self.ref_data.surface_combat.as_ref()
    }

    /// The air combat figures, if the scenario names them. Rules, so common
    /// knowledge (decision 0100, 3).
    pub fn air_combat_figures(&self) -> Option<&'a AirCombatFigures> {
        self.ref_data.air_combat.as_ref()
    }

    /// The air wear and repair figures, if the scenario names them. Rules,
    /// so common knowledge (decision 0100, 3).
    pub fn air_upkeep_figures(&self) -> Option<&'a AirUpkeepFigures> {
        self.ref_data.air_upkeep.as_ref()
    }

    /// The fatigue figures, if the scenario names them. Rules, so common
    /// knowledge (decision 0100, 3).
    pub fn fatigue_figures(&self) -> Option<&'a FatigueFigures> {
        self.ref_data.fatigue.as_ref()
    }

    /// The answer to `question`: what the engine would choose now for one of
    /// this side's own units, from what this side holds (decision 0100, 2).
    /// The engine makes the same choice by the same function. `None` where
    /// the question names no unit of this side or the choice does not arise.
    /// It reads nothing of the enemy but what the side holds of it.
    pub fn ask(&self, question: &Question) -> Option<Answer> {
        match question {
            Question::Air(question) => self.ask_air(question).map(Answer::Air),
            Question::Surface(question) => self.ask_surface(question).map(Answer::Surface),
            Question::Movement(question) => self.ask_movement(question).map(Answer::Movement),
            Question::Ground(question) => self.ask_ground(question).map(Answer::Ground),
        }
    }

    /// Every question this side could ask now, one for each of its units
    /// each choice applies to, in a fixed order: air, surface, movement,
    /// ground (decision 0100, 12: a staff's daily decisions at their
    /// dearest).
    pub fn questions(&self) -> Vec<Question> {
        let mut questions: Vec<Question> = self
            .air_questions()
            .into_iter()
            .map(Question::Air)
            .collect();
        questions.extend(self.surface_questions().into_iter().map(Question::Surface));
        questions.extend(
            self.movement_questions()
                .into_iter()
                .map(Question::Movement),
        );
        questions.extend(self.ground_questions().into_iter().map(Question::Ground));
        questions
    }
}

#[cfg(test)]
mod tests {
    use crate::fixed::Fixed;
    use crate::hex::Hex;
    use crate::ids::{AirGroupId, AircraftTypeId, BaseId, ShipClassId, ShipId, TaskForceId};
    use crate::map::{HexData, Map};
    use crate::orders::{Mission, Readiness};
    use crate::refdata::{
        AircraftRole, AircraftType, PortSize, RefData, SearchFigures, ShipClass, ShipKind,
        SurfaceCombatFigures,
    };
    use crate::resolve::PulsePhase;
    use crate::state::{
        AirGroup, AirGroupLocation, Base, Ship, ShipLocation, ShipSystems, Side, TaskForce,
        TruthState,
    };
    use crate::time::Pulse;
    use crate::world::World;

    fn group(id: u32, side: Side, base: u32) -> AirGroup {
        AirGroup {
            ready: 4,
            strength: 4,
            ..AirGroup::new(
                AirGroupId::new(id),
                format!("Group {id}"),
                side,
                AirGroupLocation::Base(BaseId::new(base)),
                AircraftTypeId::new(1),
            )
        }
    }

    #[test]
    fn a_side_sees_only_its_own_air_groups() {
        let mut map = Map::new();
        map.insert(Hex::new(0, 0), HexData::open_sea());
        map.insert(Hex::new(5, 0), HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
                fuel_berth_rate_t_per_h: None,
                fuel_lighter_rate_t_per_h: None,
                ammunition_berth_rate_t_per_h: None,
                ammunition_lighter_rate_t_per_h: None,
            },
        );
        ref_data.aircraft_types.insert(
            AircraftTypeId::new(1),
            AircraftType {
                id: AircraftTypeId::new(1),
                name: "Test".to_string(),
                role: AircraftRole::Patrol,
                search_range_nm: Fixed::from_int(600),
                sweep_width_nm: Fixed::from_int(40),
                strike_range_nm: Fixed::ZERO,
                attack_weapon: crate::refdata::AttackWeapon::None,
                attack_damage: Fixed::ZERO,
                pilot_armour: false,
                self_sealing_tanks: false,
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        let mut truth = TruthState::default();
        for (id, side, hex) in [(1, Side::Allied, 0), (2, Side::Japanese, 5)] {
            truth.bases.insert(
                BaseId::new(id),
                Base::new(
                    BaseId::new(id),
                    format!("Base {id}"),
                    side,
                    Hex::new(hex, 0),
                ),
            );
            truth
                .air_groups
                .insert(AirGroupId::new(id), group(id, side, id));
        }
        let world = World::new(1, ref_data, map, truth).expect("valid setup");

        let allied: Vec<AirGroupId> = world
            .side_view(Side::Allied)
            .air_groups()
            .map(|g| g.id)
            .collect();
        let japanese: Vec<AirGroupId> = world
            .side_view(Side::Japanese)
            .air_groups()
            .map(|g| g.id)
            .collect();
        assert_eq!(allied, vec![AirGroupId::new(1)]);
        assert_eq!(japanese, vec![AirGroupId::new(2)]);

        let view = world.side_view(Side::Allied);
        assert!(view.aircraft_type(AircraftTypeId::new(1)).is_some());
        assert!(view.search_figures().is_some());

        // Formatting a view must not print the truth it holds.
        for printed in [format!("{view:?}"), format!("{view:#?}")] {
            assert!(printed.contains("Allied"), "{printed}");
            assert!(!printed.contains("Base 2"), "{printed}");
            assert!(!printed.contains("Group 2"), "{printed}");
            assert!(!printed.contains("Japanese"), "{printed}");
        }
    }
    /// A day fight in one hex: Allied task force 1 (ships 1 and 2) against
    /// Japanese task force 2 (ship 3). Every gun hit lands on flotation, there
    /// are no torpedoes, and ships 1 and 3 are one hit from sinking, so both
    /// forces lose a ship in round 1.
    fn fought_world() -> World {
        let hex = Hex::new(0, 0);
        let mut map = Map::new();
        map.insert(hex, HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
                fuel_berth_rate_t_per_h: None,
                fuel_lighter_rate_t_per_h: None,
                ammunition_berth_rate_t_per_h: None,
                ammunition_lighter_rate_t_per_h: None,
            },
        );
        ref_data.ship_classes.insert(
            ShipClassId::new(1),
            ShipClass {
                id: ShipClassId::new(1),
                name: "Test cruiser".to_string(),
                kind: ShipKind::Cruiser,
                cruise_speed_kn: Fixed::from_int(16),
                max_speed_kn: Fixed::from_int(32),
                fuel_capacity_t: Fixed::from_int(2000),
                endurance_nm: Fixed::from_int(8000),
                cargo_capacity_t: Fixed::ZERO,
                main_guns: 8,
                gun_rounds_per_min: Fixed::from_int(3),
                gun_rounds_per_gun: 100,
                gun_damage: Fixed::from_int(20),
                torpedo_tubes: 0,
                torpedo_tubes_one_side: 0,
                torpedo_damage: Fixed::ZERO,
                toughness: Fixed::from_int(400),
                radar: false,
                aircraft_capacity: 0,
                anti_aircraft: 0,
                troop_capacity_men: 0,
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        let mut figures = SurfaceCombatFigures::from_surface_combat_toml();
        figures.gun_hit_flotation = Fixed::ONE;
        ref_data.surface_combat = Some(figures);

        let mut truth = TruthState::default();
        for (force, side, ships) in [(1, Side::Allied, vec![1, 2]), (2, Side::Japanese, vec![3])] {
            let force_id = TaskForceId::new(force);
            let mut task_force = TaskForce::at(
                force_id,
                format!("Force {force}"),
                side,
                hex,
                Mission::SurfaceCombat,
            );
            // Both forces were alert by default before decision 0021.
            task_force.readiness = Readiness::Battle;
            for ship in ships {
                let id = ShipId::new(ship);
                task_force.ships.insert(id);
                truth.ships.insert(
                    id,
                    Ship {
                        id,
                        name: format!("Ship {ship}"),
                        class: ShipClassId::new(1),
                        side,
                        fuel_t: Fixed::from_int(2000),
                        supply_t: Fixed::ZERO,
                        cargo_fuel_t: Fixed::ZERO,
                        location: ShipLocation::TaskForce(force_id),
                        // Ships 1 and 3 sink to the first flotation hit.
                        flotation_damage: if ship == 2 {
                            Fixed::ZERO
                        } else {
                            Fixed::from_int(95)
                        },
                        fire_damage: Fixed::ZERO,
                        gun_rounds: Fixed::from_int(800),
                        torpedoes_ready: false,
                        torpedoes_one_side_fired: false,
                        torpedoes_fired: 0,
                        systems: ShipSystems::ALL_WORKING,
                        experience: Ship::DEFAULT_EXPERIENCE,
                        system_repair_points: Fixed::ZERO,
                        troops: None,
                        available_from: None,
                    },
                );
            }
            truth.task_forces.insert(force_id, task_force);
        }
        let mut world = World::new(1, ref_data, map, truth).expect("valid setup");
        world.run_pulse_phase(1, Pulse::Morning, PulsePhase::Movement);
        world
    }

    #[test]
    fn a_side_sees_its_own_ships_damage_and_ammunition_its_sunk_ships_and_its_claims() {
        // Day, both sight in round 1. Allied ships afloat: 1, 2. Japanese: 3.
        // Matching: ship 1 -> 3, ship 2 -> 3 (3 mod 1 targets), ship 3 -> 1.
        // Each Allied gun salvo is 8 x 3 x 5 = 120 shots at day chance 0.05
        // = 6 expected hits, all on flotation at 100 x 20 / 400 = 5 points
        // each, so 30 points: ship 3 (95) sinks, and ship 1 (95) sinks from
        // ship 3's 30 points. Ship 2 is not fired at.
        let world = fought_world();
        let allied = world.side_view(Side::Allied);
        let japanese = world.side_view(Side::Japanese);

        // Own ships only, and a sunk own ship is still shown, as sunk.
        let ids: Vec<u32> = allied.ships().map(|ship| ship.id.raw()).collect();
        assert_eq!(ids, vec![1, 2]);
        let ids: Vec<u32> = japanese.ships().map(|ship| ship.id.raw()).collect();
        assert_eq!(ids, vec![3]);
        let ship1 = allied.ships().find(|s| s.id == ShipId::new(1)).unwrap();
        assert_eq!(ship1.location, ShipLocation::Sunk);
        assert_eq!(ship1.flotation_damage, Fixed::from_int(100));
        let sunk3 = japanese.ships().next().unwrap();
        assert_eq!(sunk3.location, ShipLocation::Sunk);

        // Own damage and ammunition are visible: ship 2 fired 120 rounds of
        // its 800 and was not hit; ship 1 fired in the round it sank.
        let ship2 = allied.ships().find(|s| s.id == ShipId::new(2)).unwrap();
        assert_eq!(ship2.gun_rounds, Fixed::from_int(680));
        assert_eq!(ship2.flotation_damage, Fixed::ZERO);
        assert_eq!(ship1.gun_rounds, Fixed::from_int(680));

        // Claims: each side fired and the other side lost one ship.
        for (view, other) in [(&allied, Side::Japanese), (&japanese, Side::Allied)] {
            let claims = view.claims();
            assert_eq!(claims.len(), 1, "{other:?} side lost one ship");
            assert_eq!(claims[0].claimed_sunk, 1);
            assert_eq!(claims[0].hex, Hex::new(0, 0));
            assert_eq!(claims[0].day, 1);
        }
    }

    #[test]
    fn a_side_with_no_fight_has_no_claims() {
        let mut map = Map::new();
        map.insert(Hex::new(0, 0), HexData::open_sea());
        let mut ref_data = RefData::default();
        ref_data.port_sizes.insert(
            0,
            PortSize {
                berths: 0,
                berth_rate_t_per_h: Fixed::ZERO,
                lighter_rate_t_per_h: Fixed::from_int(20),
                fuel_berth_rate_t_per_h: None,
                fuel_lighter_rate_t_per_h: None,
                ammunition_berth_rate_t_per_h: None,
                ammunition_lighter_rate_t_per_h: None,
            },
        );
        ref_data.search = Some(SearchFigures::from_search_toml());
        ref_data.surface_combat = Some(SurfaceCombatFigures::from_surface_combat_toml());
        let world = World::new(1, ref_data, map, TruthState::default()).expect("valid setup");
        assert!(world.side_view(Side::Allied).claims().is_empty());
    }
}
