//! A world built from a picture (decision 0170).
//!
//! A [`WorldPicture`] is plain data of one evening, the clock at 1800
//! between two days: the day, the state of both sides, each side's belief
//! and command state, the orders in force, the counters of units made in
//! play and the objectives. It is not state: it is never saved and never
//! hashed, and a world built from it holds nothing that refers to it.
//!
//! [`World::from_picture`] is the one checked way to make a world that is
//! not at its beginning with nothing believed. It refuses a picture that is
//! not sound, naming what is wrong, and builds nothing. The world it builds
//! has no history: its first event, event 1, is the picture, and what the
//! state and belief held as reported or done before the picture cites it.
//! It draws in a domain of its own (decision 0170, 12), so that no trial
//! reads the live game's dice.
//!
//! The umpire may take the whole picture of a live world
//! ([`World::whole_picture`]) and rebuild a world from it on the live
//! game's own seed with no domain ([`World::rebuild_from_picture`]). Both
//! ask for the umpire's key, which nothing that plays the game holds.
//!
//! The word "picture" here is a picture of a whole world on an evening. A
//! command's picture (decision 0134, [`crate::pictures`]) is something
//! else: what one command believes of the enemy while command levels are on.

use std::collections::{BTreeMap, BTreeSet};

use crate::belief::{BattleClaim, BeliefState, Contact, GroundEstimate, GroundGrade};
use crate::claims::UnitClaim;
use crate::command::{CommandState, Hold, HoldMaker};
use crate::events::{Cause, EventKind, EventLog};
use crate::hex::Hex;
use crate::ids::{AircraftTypeId, EntityRef, EventId, IdAllocator, OrderId, UmpireKey};
use crate::map::Map;
use crate::objectives::Objectives;
use crate::orders::{Order, OrderKind, Origin};
use crate::refdata::{RefData, ShipKind};
use crate::rng::{Domain, Stream};
use crate::state::{
    AirGroup, CargoOrder, Charge, Halt, HitsOnMen, LandUnit, March, Preparation, Side, TaskForce,
    TruthState, YardTrip,
};
use crate::time::Pulse;
use crate::world::{MadeIds, SimError, World, MADE_IDS_FLOOR};

mod half;
pub use half::PictureHalf;

/// One evening of a world, as plain data (decision 0170, 1). Built by
/// whoever may build one; checked and used by [`World::from_picture`].
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct WorldPicture {
    /// The last day resolved. The first day the built world resolves is
    /// the day after. The state's own day must be the same.
    pub day: u32,
    /// Ships, task forces, bases, air groups, land units, formations,
    /// replacements and arrivals of both sides, as the engine's own state
    /// holds them. Every order it refers to is named by its place in
    /// [`WorldPicture::orders`] ([`WorldPicture::order_at`]).
    pub truth: TruthState,
    /// Each side's belief. A side not here believes nothing.
    pub beliefs: BTreeMap<Side, PictureBelief>,
    /// Each side's command state, whole, its next plan's number with it. A
    /// side not here has the empty command state, its next plan 1.
    pub command: BTreeMap<Side, CommandState>,
    /// The orders in force, referred to by their place in this list, from
    /// 1. The built world's order book holds them in this order.
    pub orders: Vec<PictureOrder>,
    /// For each side, the next number of a task force and of an air group
    /// made in play. A side not here starts at its first numbers.
    pub counters: BTreeMap<Side, PictureCounters>,
    /// The objectives, where the world is to have them.
    pub objectives: Option<Objectives>,
}

/// What one side believes in a picture (decision 0170, 1). None of it
/// carries an event's number: the built world holds every entry as
/// reported by its first event.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PictureBelief {
    /// Its contacts, in the order they are numbered, from 1.
    pub contacts: Vec<PictureContact>,
    /// Its claims, in the order they arrived.
    pub claims: Vec<PictureClaim>,
    /// Its estimates of the enemy's men ashore, at most one for a hex.
    pub estimates: Vec<PictureEstimate>,
}

/// A contact in a picture: where, how many ships, taken for what kind, and
/// how many pulses since its last fresh report.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PictureContact {
    pub hex: Hex,
    pub ship_count: u32,
    pub kind: ShipKind,
    pub age_pulses: u32,
}

/// A claim in a picture: a [`BattleClaim`] without its event.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PictureClaim {
    pub hex: Hex,
    pub day: u32,
    pub pulse: Pulse,
    pub claimed_sunk: u32,
    pub claimed_hits: u32,
    pub claimed_kind: Option<ShipKind>,
    pub claimed_shot_down: u32,
    pub claimed_on_ground: u32,
    pub from_air: bool,
    pub from_bombardment: bool,
    /// The types claimed on the ground (decision 0179).
    pub ground_types: Vec<(AircraftTypeId, u32)>,
    /// Who claims what (decision 0180): every unit named is of the claim's
    /// side.
    pub by_unit: Vec<UnitClaim>,
}

/// An estimate of the enemy's men ashore in a picture: a
/// [`GroundEstimate`] without its event.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PictureEstimate {
    pub hex: Hex,
    pub men: u32,
    pub day: u32,
    pub grade: GroundGrade,
}

/// An order in force in a picture: an [`Order`] without its number, which
/// is its place in the picture's list.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PictureOrder {
    pub side: Side,
    /// The day the order took effect.
    pub day_issued: u32,
    pub kind: OrderKind,
    pub origin: Origin,
}

/// One side's counters of units made in play: the number its next task
/// force and its next air group will have.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PictureCounters {
    pub next_task_force: u32,
    pub next_air_group: u32,
}

impl PictureCounters {
    /// The counters of a side that has made nothing in play.
    pub const fn first(side: Side) -> PictureCounters {
        PictureCounters {
            next_task_force: MadeIds::first(side),
            next_air_group: MadeIds::first(side),
        }
    }
}

impl WorldPicture {
    /// The order at `place` in a picture's list, counted from 1: what the
    /// state and a command state hold to refer to it. This is the one way
    /// to name an order outside the engine, and it names a place in a
    /// picture, nothing of any live game.
    pub const fn order_at(place: u32) -> OrderId {
        OrderId::new(place)
    }
}

/// One number the state or a command state holds: of an order or of an
/// event, with words for where it is held.
enum HeldId<'a> {
    Order(&'a mut OrderId),
    Event(&'a mut EventId),
}

/// Walks every number of an order or an event that the state and the
/// command states hold, in a fixed order, handing each to `visit` with the
/// words for where it is held. Each struct that holds one is named here
/// with all its fields, so a field added to it does not compile until it
/// is walked or said to hold none (decision 0170, 8 and 9).
fn walk_held_ids(
    truth: &mut TruthState,
    commands: &mut BTreeMap<Side, CommandState>,
    visit: &mut dyn FnMut(String, HeldId<'_>),
) {
    let TruthState {
        day: _,
        // A ship holds no order and no event.
        ships: _,
        task_forces,
        // A base holds no order and no event.
        bases: _,
        air_groups,
        replacements: _,
        arrivals: _,
        land_units,
        hits_on_men,
        formations: _,
    } = truth;
    for force in task_forces.values_mut() {
        walk_task_force(force, visit);
    }
    for group in air_groups.values_mut() {
        walk_air_group(group, visit);
    }
    for unit in land_units.values_mut() {
        walk_land_unit(unit, visit);
    }
    for ((hex, side), HitsOnMen { hits: _, causes }) in hits_on_men.iter_mut() {
        for cause in causes {
            visit(
                format!("the {} hits on men at {hex}", side.name()),
                HeldId::Event(cause),
            );
        }
    }
    for (side, command) in commands.iter_mut() {
        let CommandState {
            intents: _,
            leaves: _,
            plans: _,
            commitments: _,
            holds,
            next_plan: _,
            interrupts: _,
            standing: _,
            standing_off: _,
        } = command;
        for (target, Hold { day: _, by }) in holds.iter_mut() {
            match by {
                HoldMaker::Command => {}
                HoldMaker::Order(order) => visit(
                    format!("the {} hold on {target}", side.name()),
                    HeldId::Order(order),
                ),
            }
        }
    }
}

fn walk_task_force(force: &mut TaskForce, visit: &mut dyn FnMut(String, HeldId<'_>)) {
    let TaskForce {
        id,
        name: _,
        side: _,
        ships: _,
        mission: _,
        speed: _,
        route: _,
        from_hex: _,
        toward_hex: _,
        progress_nm: _,
        leg_nm: _,
        status: _,
        route_order,
        speed_order,
        cargo_order,
        arrived_at_hours: _,
        arrival_event,
        readiness: _,
        readiness_order,
        alerted: _,
        alerted_by,
        rules_of_engagement: _,
        rules_order,
        home_port: _,
        fatigue: _,
        yard_trip,
        halt,
        preparation,
        landing_at: _,
        charge,
        available_from: _,
    } = force;
    let id = *id;
    let mut order = |what: &str, order: &mut OrderId| {
        visit(format!("{id}'s {what} order"), HeldId::Order(order));
    };
    for (what, held) in [
        ("route", route_order),
        ("speed", speed_order),
        ("readiness", readiness_order),
        ("rules of engagement", rules_order),
    ] {
        if let Some(held) = held {
            order(what, held);
        }
    }
    if let Some(CargoOrder {
        action: _,
        order: held,
        unit: _,
        weight_t: _,
        heeds_reserve: _,
        each_t: _,
    }) = cargo_order
    {
        order("cargo", held);
    }
    if let Some(Preparation {
        hex: _,
        roster: _,
        loading: _,
        order: held,
    }) = preparation
    {
        order("prepare-landing", held);
    }
    if let Some(Charge {
        force: _,
        order: held,
    }) = charge
    {
        order("escort", held);
    }
    let mut event = |what: &str, event: &mut EventId| {
        visit(format!("{id}'s {what}"), HeldId::Event(event));
    };
    if let Some(held) = arrival_event {
        event("arrival", held);
    }
    if let Some(held) = alerted_by {
        event("alert", held);
    }
    if let Some(YardTrip {
        base: _,
        return_day: _,
        departure,
    }) = yard_trip
    {
        event("departure for the yard", departure);
    }
    if let Some(Halt {
        work: _,
        at_hours: _,
        event: halted,
        bombarded,
        cargo_completed,
    }) = halt
    {
        event("halt", halted);
        if let Some(held) = bombarded {
            event("bombardment since its halt", held);
        }
        if let Some(held) = cargo_completed {
            event("cargo completed since its halt", held);
        }
    }
}

fn walk_air_group(group: &mut AirGroup, visit: &mut dyn FnMut(String, HeldId<'_>)) {
    let AirGroup {
        id,
        name: _,
        side: _,
        location: _,
        aircraft_type: _,
        ready: _,
        damaged: _,
        mission: _,
        search: _,
        mission_order,
        arc_order,
        patrol_share: _,
        share_order,
        fatigue: _,
        rest_above: _,
        rest_order,
        stood_down: _,
        pilots: _,
        strength: _,
        reach: _,
        reach_order,
        target_priority: _,
        target_order,
        last_base_strike_day: _,
        rebased_day: _,
        strike_contact_age: _,
        strike_age_order,
        strike_held_share: _,
        strike_share_order,
        strike_sizing: _,
        strike_sizing_order,
        search_once: _,
        search_once_order,
        available_from: _,
        // A tally holds no order and no event (decision 0180).
        claims: _,
    } = group;
    for (what, held) in [
        ("mission", mission_order),
        ("search arc", arc_order),
        ("patrol share", share_order),
        ("rest", rest_order),
        ("reach", reach_order),
        ("target priority", target_order),
        ("strike contact age", strike_age_order),
        ("strike share", strike_share_order),
        ("strike sizing", strike_sizing_order),
        ("search once", search_once_order),
    ] {
        if let Some(held) = held {
            visit(format!("{id}'s {what} order"), HeldId::Order(held));
        }
    }
}

fn walk_land_unit(unit: &mut LandUnit, visit: &mut dyn FnMut(String, HeldId<'_>)) {
    let LandUnit {
        id,
        name: _,
        side: _,
        men_ashore: _,
        hex: _,
        posture: _,
        posture_order,
        fatigue: _,
        fed_share: _,
        march,
        rehearsal: _,
        rehearsal_hex: _,
        activity: _,
        activity_order,
        experience: _,
        training_days: _,
        fight_days: _,
        skills: _,
        working: _,
        when_spent: _,
        fieldworks: _,
        available_from: _,
        formation: _,
    } = unit;
    for (what, held) in [("posture", posture_order), ("activity", activity_order)] {
        if let Some(held) = held {
            visit(format!("{id}'s {what} order"), HeldId::Order(held));
        }
    }
    if let Some(March {
        to: _,
        days: _,
        counted: _,
        cause,
    }) = march
    {
        match cause {
            Cause::Order(held) => visit(format!("{id}'s march"), HeldId::Order(held)),
            Cause::Event(held) => visit(format!("{id}'s march"), HeldId::Event(held)),
        }
    }
}

impl World {
    /// Builds a world from a picture of an evening (decision 0170, 5 to
    /// 11), drawing on `stream`: its seed and its domain, both given by
    /// whoever builds the world. The world stands on the picture's evening
    /// with nothing pending, no day in hand and command levels off; the
    /// first day it resolves is the day after the picture's.
    ///
    /// Refuses a picture that is not sound, naming what is wrong, and
    /// builds nothing: what [`World::new`] refuses of the reference data
    /// and the state, the state's invariants, an order referred to that is
    /// not in the picture, a unit made in play at or above its side's
    /// counter or a counter below its side's first number, a next plan not
    /// above every plan held, a contact off the map or with no ship, a
    /// claim or an estimate after the picture's day, two estimates for one
    /// hex, hits on men waiting to be worked, a command state that breaks
    /// the invariants of the day's end, and objectives the world would
    /// refuse.
    pub fn from_picture(
        picture: WorldPicture,
        ref_data: RefData,
        map: Map,
        stream: Stream,
    ) -> Result<World, SimError> {
        build(picture, ref_data, map, stream.seed, Some(stream.domain))
    }

    /// The umpire's rebuild (decision 0170, 13): a world built from a
    /// picture, as [`World::from_picture`] builds it, on `seed` with no
    /// domain, so that it draws what a live game on that seed draws. For
    /// the test that the constructor loses nothing and for the umpire's
    /// tools. It asks for the umpire's key, which nothing that plays the
    /// game holds.
    pub fn rebuild_from_picture(
        picture: WorldPicture,
        ref_data: RefData,
        map: Map,
        seed: u64,
        _key: &UmpireKey,
    ) -> Result<World, SimError> {
        build(picture, ref_data, map, seed, None)
    }

    /// The whole picture of this world on its evening (decision 0170, 13),
    /// for the umpire: both sides' state, belief and command state as they
    /// are. The orders in force are those the state and the command states
    /// refer to, in the order of their numbers here, renumbered from 1;
    /// the contacts are renumbered from 1 in the order of their numbers.
    /// What a picture does not hold is left out: the event log, the orders
    /// no longer referred to, the daily hashes, the seed and the domain,
    /// what waits for the next day, and command levels (decision 0170, 4).
    /// The event numbers the state holds are set to 1, the event a world
    /// built from the picture begins with: no number of this world's log is
    /// in the picture.
    ///
    /// Refuses while a day is open: a picture is of an evening.
    pub fn whole_picture(&self, _key: &UmpireKey) -> Result<WorldPicture, SimError> {
        if let Some(in_hand) = self.day_in_hand {
            return Err(SimError::InvalidSetup(format!(
                "a picture is of an evening, and day {} is open",
                in_hand.day
            )));
        }
        let mut truth = self.truth.clone();
        let mut command = self.command.clone();
        // The orders referred to, in the order of their numbers here.
        let mut referred = BTreeSet::new();
        walk_held_ids(&mut truth, &mut command, &mut |_, held| {
            if let HeldId::Order(order) = held {
                referred.insert(*order);
            }
        });
        let places: BTreeMap<OrderId, OrderId> = referred
            .iter()
            .zip(1..)
            .map(|(live, place)| (*live, OrderId::new(place)))
            .collect();
        // Orders by their place; every event number the state holds names
        // the picture's own event, the first of a world built from it, so
        // that no number of the live log is in the picture.
        walk_held_ids(&mut truth, &mut command, &mut |_, held| match held {
            HeldId::Order(order) => *order = places[order],
            HeldId::Event(event) => *event = EventId::new(1),
        });
        let orders = referred
            .iter()
            .map(|live| {
                let Order {
                    id: _,
                    side,
                    day_issued,
                    kind,
                    origin,
                } = self.orders[live].clone();
                PictureOrder {
                    side,
                    day_issued,
                    kind,
                    origin,
                }
            })
            .collect();
        let beliefs = self
            .beliefs
            .iter()
            .map(|(side, belief)| (*side, picture_belief(belief)))
            .collect();
        let counters = Side::ALL
            .into_iter()
            .map(|side| {
                (
                    side,
                    PictureCounters {
                        next_task_force: self.made_ids.next_task_force(side),
                        next_air_group: self.made_ids.next_air_group(side),
                    },
                )
            })
            .collect();
        Ok(WorldPicture {
            day: self.truth.day,
            truth,
            beliefs,
            command,
            orders,
            counters,
            objectives: self.objectives.clone(),
        })
    }
}

/// A side's belief as a picture holds it: everything but the event
/// numbers, the contacts in the order of their numbers.
fn picture_belief(belief: &BeliefState) -> PictureBelief {
    belief_of(
        belief.contacts(),
        belief.claims(),
        belief.ground_estimates(),
    )
}

/// A belief as a picture holds it, from its parts as a belief or a side's
/// view gives them: the contacts in the order of their numbers, the claims
/// in the order held, the estimates in the order of their hexes, each
/// without the event that reported it.
fn belief_of<'a>(
    contacts: impl Iterator<Item = &'a Contact>,
    claims: &[BattleClaim],
    estimates: impl Iterator<Item = &'a GroundEstimate>,
) -> PictureBelief {
    PictureBelief {
        contacts: contacts
            .map(|contact| PictureContact {
                hex: contact.estimated_hex,
                ship_count: contact.estimated_ship_count,
                kind: contact.estimated_kind,
                age_pulses: contact.age_pulses,
            })
            .collect(),
        claims: claims
            .iter()
            .map(|claim| {
                let BattleClaim {
                    hex,
                    day,
                    pulse,
                    claimed_sunk,
                    claimed_hits,
                    claimed_kind,
                    claimed_shot_down,
                    claimed_on_ground,
                    from_air,
                    from_bombardment,
                    received_event: _,
                    ground_types,
                    by_unit,
                } = claim.clone();
                PictureClaim {
                    hex,
                    day,
                    pulse,
                    claimed_sunk,
                    claimed_hits,
                    claimed_kind,
                    claimed_shot_down,
                    claimed_on_ground,
                    from_air,
                    from_bombardment,
                    ground_types,
                    by_unit,
                }
            })
            .collect(),
        estimates: estimates
            .map(|estimate| PictureEstimate {
                hex: estimate.hex,
                men: estimate.men,
                day: estimate.day,
                grade: estimate.grade,
            })
            .collect(),
    }
}

/// Whether a claim of `side` may name `unit` (decision 0180, 12): an air
/// group or a task force the picture holds as the side's, or a base the
/// picture holds of either side, since a base may change hands after its
/// guns claimed. A unit of another kind is never named by a claim.
fn claimer_of(truth: &TruthState, unit: EntityRef, side: Side) -> bool {
    match unit {
        EntityRef::AirGroup(group) => truth
            .air_groups
            .get(&group)
            .is_some_and(|group| group.side == side),
        EntityRef::TaskForce(force) => truth
            .task_forces
            .get(&force)
            .is_some_and(|force| force.side == side),
        EntityRef::Base(base) => truth.bases.contains_key(&base),
        _ => false,
    }
}

fn refuse(words: String) -> SimError {
    SimError::InvalidSetup(format!("the picture is not sound: {words}"))
}

/// The one build behind [`World::from_picture`] and
/// [`World::rebuild_from_picture`].
fn build(
    picture: WorldPicture,
    ref_data: RefData,
    map: Map,
    seed: u64,
    domain: Option<Domain>,
) -> Result<World, SimError> {
    let WorldPicture {
        day,
        mut truth,
        beliefs,
        command,
        orders,
        counters,
        objectives,
    } = picture;

    // The state's own day is the picture's: one day, not two.
    if truth.day != day {
        return Err(refuse(format!(
            "the state is of day {}, and the picture of day {day}",
            truth.day
        )));
    }
    // A side not given has the empty command state, its next plan 1.
    let mut command: BTreeMap<Side, CommandState> = Side::ALL
        .into_iter()
        .map(|side| {
            let state = command.get(&side).cloned();
            (side, state.unwrap_or_else(CommandState::new))
        })
        .collect();

    // Every order referred to is in the list (item 6).
    let listed = orders.len();
    let mut unlisted = None;
    walk_held_ids(&mut truth, &mut command, &mut |where_, held| {
        if let HeldId::Order(order) = held {
            let place = order.raw();
            let in_list = usize::try_from(place).is_ok_and(|place| (1..=listed).contains(&place));
            if !in_list && unlisted.is_none() {
                unlisted = Some(format!(
                    "{where_} refers to order {place}, which is not in the picture's list of \
                     {listed} orders"
                ));
            }
        }
    });
    if let Some(words) = unlisted {
        return Err(refuse(words));
    }

    // The counters (item 6): none below its side's first number, and no
    // unit made in play at or above its side's.
    let next = |side: Side| {
        counters
            .get(&side)
            .copied()
            .unwrap_or(PictureCounters::first(side))
    };
    for side in Side::ALL {
        let first = MadeIds::first(side);
        let given = next(side);
        for (what, counter) in [
            ("task forces", given.next_task_force),
            ("air groups", given.next_air_group),
        ] {
            if counter < first {
                return Err(refuse(format!(
                    "the {} counter of {what}, {counter}, is below its side's first number, \
                     {first}",
                    side.name()
                )));
            }
        }
    }
    let made = |raw: u32| raw > MADE_IDS_FLOOR;
    for force in truth.task_forces.values() {
        let counter = next(force.side).next_task_force;
        if made(force.id.raw()) && force.id.raw() >= counter {
            return Err(refuse(format!(
                "{}, made in play, has a number at or above the {} counter of task forces, \
                 {counter}",
                force.id,
                force.side.name()
            )));
        }
    }
    for group in truth.air_groups.values() {
        let counter = next(group.side).next_air_group;
        if made(group.id.raw()) && group.id.raw() >= counter {
            return Err(refuse(format!(
                "{}, made in play, has a number at or above the {} counter of air groups, \
                 {counter}",
                group.id,
                group.side.name()
            )));
        }
    }

    // Each command state's next plan is above every plan it holds.
    for (side, state) in &command {
        if let Some(plan) = state
            .plans
            .keys()
            .find(|plan| plan.raw() >= state.next_plan)
        {
            return Err(refuse(format!(
                "the {} next plan, {}, is not above {plan}, which it holds",
                side.name(),
                state.next_plan
            )));
        }
    }

    // Each side's belief: a contact on the map with a ship; a claim and an
    // estimate of no day after the picture's; one estimate for a hex.
    for (side, belief) in &beliefs {
        let belief_side = *side;
        let side = side.name();
        for (place, contact) in (1..).zip(&belief.contacts) {
            if contact.ship_count == 0 {
                return Err(refuse(format!("the {side} contact {place} has no ship")));
            }
            if map.get(contact.hex).is_none() {
                return Err(refuse(format!(
                    "the {side} contact {place} is at {}, off the map",
                    contact.hex
                )));
            }
        }
        for (place, claim) in (1..).zip(&belief.claims) {
            if claim.day > day {
                return Err(refuse(format!(
                    "the {side} claim {place} is of day {}, after the picture's day, {day}",
                    claim.day
                )));
            }
            // Every unit a claim names is of the claim's side (decision
            // 0180, 12).
            if let Some(unit) = claim
                .by_unit
                .iter()
                .map(|line| line.unit)
                .find(|unit| !claimer_of(&truth, *unit, belief_side))
            {
                return Err(refuse(format!(
                    "the {side} claim {place} names {unit}, which is not of the {side} side"
                )));
            }
        }
        let mut hexes = BTreeSet::new();
        for estimate in &belief.estimates {
            if estimate.day > day {
                return Err(refuse(format!(
                    "the {side} estimate at {} is of day {}, after the picture's day, {day}",
                    estimate.hex, estimate.day
                )));
            }
            if !hexes.insert(estimate.hex) {
                return Err(refuse(format!(
                    "the {side} side holds two estimates for {}",
                    estimate.hex
                )));
            }
        }
    }

    // Nothing half done: no hits on men waiting to be worked. A picture
    // holds no order pending and no day in hand.
    if let Some((hex, side)) = truth.hits_on_men.keys().next() {
        return Err(refuse(format!(
            "hits on {} men at {hex} wait to be worked",
            side.name()
        )));
    }

    // What `World::new` checks of the reference data and the state, and
    // the state's invariants.
    let mut world = World::new(seed, ref_data, map, truth)?;

    // Each command state by the invariants of the day's end.
    let mut violations = Vec::new();
    for (side, state) in &command {
        violations.extend(crate::invariants::check_command(
            *side,
            state,
            &world.truth,
            day,
        ));
    }
    if !violations.is_empty() {
        return Err(SimError::InvariantsViolated { day, violations });
    }
    if let Some(objectives) = objectives {
        world.set_objectives(objectives)?;
    }

    // The first event of the world is the picture (item 7), and what
    // would cite an earlier cause cites it (item 8).
    let mut events = EventLog::default();
    let picture_event = events.record(
        day,
        None,
        EventKind::BuiltFromPicture { day },
        Vec::new(),
        None,
    );
    walk_held_ids(&mut world.truth, &mut command, &mut |_, held| {
        if let HeldId::Event(event) = held {
            *event = picture_event;
        }
    });
    world.events = events;
    world.beliefs = Side::ALL
        .into_iter()
        .map(|side| {
            let given = beliefs.get(&side).cloned().unwrap_or_default();
            let belief = BeliefState::from_picture(
                side,
                given
                    .contacts
                    .iter()
                    .map(|c| (c.hex, c.ship_count, c.kind, c.age_pulses)),
                given.claims.iter().map(|c| BattleClaim {
                    hex: c.hex,
                    day: c.day,
                    pulse: c.pulse,
                    claimed_sunk: c.claimed_sunk,
                    claimed_hits: c.claimed_hits,
                    claimed_kind: c.claimed_kind,
                    claimed_shot_down: c.claimed_shot_down,
                    claimed_on_ground: c.claimed_on_ground,
                    from_air: c.from_air,
                    from_bombardment: c.from_bombardment,
                    received_event: picture_event,
                    ground_types: c.ground_types.clone(),
                    by_unit: c.by_unit.clone(),
                }),
                given.estimates.iter().map(|e| GroundEstimate {
                    hex: e.hex,
                    men: e.men,
                    day: e.day,
                    report_event: picture_event,
                    grade: e.grade,
                }),
                picture_event,
            );
            (side, belief)
        })
        .collect();

    // Orders keep their place (item 9).
    let mut order_ids = IdAllocator::new();
    world.orders = (1..)
        .zip(orders)
        .map(|(place, given)| {
            let id = OrderId::new(place);
            order_ids.reserve(id);
            let PictureOrder {
                side,
                day_issued,
                kind,
                origin,
            } = given;
            (
                id,
                Order {
                    id,
                    side,
                    day_issued,
                    kind,
                    origin,
                },
            )
        })
        .collect();
    world.order_ids = order_ids;

    // The counters stand where the picture put them (item 10).
    world.made_ids = MadeIds::at(
        &Side::ALL
            .into_iter()
            .map(|side| {
                let given = next(side);
                (side, (given.next_task_force, given.next_air_group))
            })
            .collect(),
    );
    world.command = command;
    world.domain = domain;
    Ok(world)
}
