//! A side's own half of a picture (decision 0171).
//!
//! A [`PictureHalf`] is plain data: what one side's view gives it whole on
//! an evening, its own units, its orders in force, its command state, its
//! belief and its counters, with no number of a live game's order or event
//! in it. [`SideView::own_half`] copies it from the view and nothing else;
//! [`WorldPicture::from_halves`] joins one half of each side into a picture
//! of a world (decision 0170).
//!
//! The copying is done here, in `sim-core`, for two reasons (decision 0170,
//! the architect's rulings, 4). The walk over every number a unit holds
//! ([`walk_held_ids`]) names each field, so a field added to a unit that
//! holds a number cannot be left out; and the event a halted force or a
//! force on a yard trip cites can only be named inside the crate.
//!
//! Reads: a side's view of a whole side, on an evening. Writes nothing.

use std::collections::{BTreeMap, BTreeSet};

use crate::command::CommandState;
use crate::entering::Entering;
use crate::ids::{
    AirGroupId, BaseId, EntityRef, EventId, FormationId, LandUnitId, OrderId, ShipId, SubmarineId,
    TaskForceId,
};
use crate::orders::Order;
use crate::state::{
    AirGroup, Arrival, Base, Formation, LandUnit, MenReplacement, MenStream, Replacement, Ship,
    Side, Submarine, SupplyLine, TaskForce, TruthState,
};
use crate::view::SideView;
use crate::world::SimError;

use super::{
    belief_of, walk_held_ids, HeldId, PictureBelief, PictureCounters, PictureOrder, WorldPicture,
};

/// One side's own half of a picture of a world on an evening (decision
/// 0171, 1): plain data, every field public, so that a half can also be
/// made by rule. It holds no number of a live game's order or event: its
/// units refer to its orders by their place in [`PictureHalf::orders`],
/// from 1 ([`WorldPicture::order_at`]), and every event they cite is the
/// first of a world built from a picture.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PictureHalf {
    /// The last day resolved.
    pub day: u32,
    /// The side whose half it is.
    pub side: Side,
    pub ships: BTreeMap<ShipId, Ship>,
    pub task_forces: BTreeMap<TaskForceId, TaskForce>,
    pub bases: BTreeMap<BaseId, Base>,
    pub air_groups: BTreeMap<AirGroupId, AirGroup>,
    pub land_units: BTreeMap<LandUnitId, LandUnit>,
    pub formations: BTreeMap<FormationId, Formation>,
    /// What is held apart for the side until its day (decision 0195, 20),
    /// keyed by the unit that enters: the side's own entries only.
    pub entering: BTreeMap<EntityRef, Entering>,
    /// The side's replacement streams, in the order they are applied.
    pub replacements: Vec<Replacement>,
    /// The side's own supply lines (decision 0196, 7), in the order they
    /// give.
    pub supply_lines: Vec<SupplyLine>,
    /// The side's own streams of replacement men, in the order they give,
    /// and its own land units' strengths, those held apart among them
    /// (decision 0197).
    pub men_replacement: MenReplacement,
    /// The side's own submarines, as they are (decision 0218, 36): the
    /// enemy's half holds none of the other side's, and a world built from
    /// a picture has no enemy boat.
    pub submarines: BTreeMap<SubmarineId, Submarine>,
    /// The arrivals due to the side's air groups, in the order they are
    /// delivered.
    pub arrivals: Vec<Arrival>,
    /// The orders in force, each referred to by its place in this list,
    /// from 1.
    pub orders: Vec<PictureOrder>,
    /// The side's command state, whole.
    pub command: CommandState,
    /// What the side believes of the enemy, as a picture holds it.
    pub belief: PictureBelief,
    /// The side's next number of a task force and of an air group made in
    /// play.
    pub counters: PictureCounters,
    /// The number of days the game has, as the side's view gives it
    /// (decision 0194, 11): configuration, the same for both sides.
    pub game_days: Option<u32>,
}

fn refuse(words: String) -> SimError {
    SimError::InvalidSetup(words)
}

impl SideView<'_> {
    /// The side's own half of a picture of this evening (decision 0171, 2
    /// to 6), copied from this view and nothing else:
    ///
    /// - its own units as the view gives them;
    /// - the orders in force, the orders its units and its command state
    ///   refer to and no other, in the order of the side's own numbers for
    ///   them, every reference re-pointed to the order's place in the list;
    /// - every event a unit cites set to the first event of a world built
    ///   from a picture;
    /// - its belief: its contacts in the order of its own numbers for them,
    ///   its claims in the order held, its estimates in the order of their
    ///   hexes, none with the event that reported it;
    /// - its command state, whole, and its two counters.
    ///
    /// Refused while a day is open, since a picture is of an evening; and at
    /// a view cut to a seat, or one whose units are as a command holds them,
    /// since only a whole side has a half.
    pub fn own_half(&self) -> Result<PictureHalf, SimError> {
        if let Some(in_hand) = self.day_in_hand() {
            return Err(refuse(format!(
                "a picture is of an evening, and day {} is open",
                in_hand.day
            )));
        }
        if self.seat().is_some() || self.is_held_view() {
            return Err(refuse(
                "only a whole side has a half: this view is cut to a command".to_string(),
            ));
        }
        let side = self.side();
        let mut truth = TruthState {
            day: self.day(),
            ships: self.ships().map(|ship| (ship.id, ship.clone())).collect(),
            task_forces: self
                .task_forces()
                .map(|force| (force.id, force.clone()))
                .collect(),
            bases: self.bases().map(|base| (base.id, base.clone())).collect(),
            air_groups: self
                .air_groups()
                .map(|group| (group.id, group.clone()))
                .collect(),
            replacements: self.replacements().cloned().collect(),
            arrivals: self.arrivals().copied().collect(),
            land_units: self
                .land_units()
                .map(|unit| (unit.id, unit.clone()))
                .collect(),
            // Hits on men wait only inside a day; an evening holds none.
            hits_on_men: BTreeMap::new(),
            formations: self
                .formations()
                .map(|formation| (formation.id, formation.clone()))
                .collect(),
            entering: self
                .entering()
                .map(|entry| (entry.unit(), entry.clone()))
                .collect(),
            supply_lines: self.supply_lines().copied().collect(),
            men_replacement: self.own_men_replacement(),
            // The waters a side is yet to be told are not its own to know:
            // a world built from a picture has none waiting (decision 0218,
            // 47).
            waters_waiting: Vec::new(),
            submarines: self
                .own_submarines()
                .map(|boat| (boat.id, boat.clone()))
                .collect(),
        };
        let mut commands = BTreeMap::from([(side, self.command_state().clone())]);

        // The orders referred to, by the side's own numbers for them.
        let mut referred = BTreeSet::new();
        let mut not_own = None;
        walk_held_ids(&mut truth, &mut commands, &mut |where_, held| {
            if let HeldId::Order(order) = held {
                match self.order_number(*order) {
                    Some(number) => {
                        referred.insert(number);
                    }
                    None => {
                        not_own.get_or_insert(where_);
                    }
                }
            }
        });
        if let Some(where_) = not_own {
            return Err(refuse(format!(
                "{where_} refers to an order that is not the {} side's own",
                side.name()
            )));
        }
        let places: BTreeMap<u32, OrderId> = referred
            .iter()
            .zip(1..)
            .map(|(number, place)| (*number, WorldPicture::order_at(place)))
            .collect();
        walk_held_ids(&mut truth, &mut commands, &mut |_, held| match held {
            HeldId::Order(order) => {
                let number = self
                    .order_number(*order)
                    .expect("every order referred to is the side's own");
                *order = places[&number];
            }
            HeldId::Event(event) => *event = EventId::new(1),
        });
        let orders = referred
            .iter()
            .map(|number| {
                let Order {
                    id: _,
                    side,
                    day_issued,
                    kind,
                    origin,
                } = self
                    .order_by_number(*number)
                    .expect("the side holds every order it numbers")
                    .clone();
                PictureOrder {
                    side,
                    day_issued,
                    kind,
                    origin,
                }
            })
            .collect();

        let TruthState {
            day,
            ships,
            task_forces,
            bases,
            air_groups,
            replacements,
            arrivals,
            land_units,
            hits_on_men: _,
            formations,
            entering,
            supply_lines,
            men_replacement,
            submarines,
            waters_waiting: _,
        } = truth;
        let command = commands.remove(&side).expect("the side's command state");
        Ok(PictureHalf {
            day,
            side,
            ships,
            task_forces,
            bases,
            air_groups,
            land_units,
            formations,
            entering,
            replacements,
            supply_lines,
            men_replacement,
            submarines,
            arrivals,
            orders,
            command,
            // The enemy's half reads no report of a submarine as a force: its
            // own reports are the side's (decision 0218, 44).
            belief: belief_of(
                self.contacts(),
                self.claims(),
                self.ground_estimates(),
                self.submarine_reports(),
                self.suspected_waters(),
            ),
            counters: PictureCounters {
                next_task_force: self.next_task_force_id().raw(),
                next_air_group: self.next_air_group_id().raw(),
            },
            game_days: self.game_days(),
        })
    }
}

impl SideView<'_> {
    /// The side's own streams of replacement men and its own land units'
    /// strengths, those held apart until their day among them (decision
    /// 0197), for its half.
    fn own_men_replacement(&self) -> MenReplacement {
        let held = self
            .entering()
            .flat_map(|entry| entry.land_units.keys().copied());
        MenReplacement {
            streams: self.men_streams().copied().collect(),
            units: self
                .land_units()
                .map(|unit| unit.id)
                .chain(held)
                .filter_map(|unit| self.strength(unit).map(|strength| (unit, *strength)))
                .collect(),
        }
    }
}

/// Moves every unit of `from` into `into`, refusing a number already there.
fn join_units<K: Ord + Copy + std::fmt::Display, V>(
    into: &mut BTreeMap<K, V>,
    from: BTreeMap<K, V>,
) -> Result<(), SimError> {
    for (id, unit) in from {
        if into.insert(id, unit).is_some() {
            return Err(refuse(format!(
                "the halves cannot be joined: both hold {id}, two units of one number"
            )));
        }
    }
    Ok(())
}

impl WorldPicture {
    /// Joins one half of each side, of the same day, into a picture of a
    /// world (decision 0171, 7): the units of both; the orders of the
    /// Allied half and then of the Japanese, each re-pointed to its new
    /// place; each side's belief, command state and counters; no
    /// objectives. The halves may be given in either order. The
    /// replacement streams and arrivals of both are put in the one order
    /// the state keeps them in (decision 0043, 4c).
    ///
    /// The picture carries the game's days the halves give (decision 0194,
    /// 11).
    ///
    /// Refuses two halves of one side, halves of different days, halves of
    /// games of different lengths, two units
    /// of one number, and a half that refers to an order not in its own
    /// list (which, joined, would name an order of the other half).
    pub fn from_halves(one: PictureHalf, other: PictureHalf) -> Result<WorldPicture, SimError> {
        if one.side == other.side {
            return Err(refuse(format!(
                "the halves cannot be joined: both are of the {} side, two halves of one side",
                one.side.name()
            )));
        }
        if one.day != other.day {
            return Err(refuse(format!(
                "the halves cannot be joined: one is of day {} and the other of day {}, halves \
                 of different days",
                one.day, other.day
            )));
        }
        if one.game_days != other.game_days {
            let words = |days: Option<u32>| match days {
                Some(days) => format!("a game of {days} days"),
                None => "a game of no length given".to_string(),
            };
            return Err(refuse(format!(
                "the halves cannot be joined: one is of {} and the other of {}",
                words(one.game_days),
                words(other.game_days)
            )));
        }
        let game_days = one.game_days;
        let (allied, japanese) = match one.side {
            Side::Allied => (one, other),
            Side::Japanese => (other, one),
        };
        let day = allied.day;
        let mut truth = TruthState {
            day,
            ships: BTreeMap::new(),
            task_forces: BTreeMap::new(),
            bases: BTreeMap::new(),
            air_groups: BTreeMap::new(),
            replacements: Vec::new(),
            arrivals: Vec::new(),
            land_units: BTreeMap::new(),
            hits_on_men: BTreeMap::new(),
            formations: BTreeMap::new(),
            entering: BTreeMap::new(),
            supply_lines: Vec::new(),
            men_replacement: MenReplacement::default(),
            submarines: BTreeMap::new(),
            waters_waiting: Vec::new(),
        };
        let mut orders = Vec::new();
        let mut beliefs = BTreeMap::new();
        let mut command = BTreeMap::new();
        let mut counters = BTreeMap::new();
        for half in [allied, japanese] {
            let PictureHalf {
                day: _,
                side,
                ships,
                task_forces,
                bases,
                air_groups,
                land_units,
                formations,
                entering,
                replacements,
                supply_lines,
                men_replacement,
                submarines,
                arrivals,
                orders: own_orders,
                command: own_command,
                belief,
                counters: own_counters,
                game_days: _,
            } = half;
            let mut own = TruthState {
                day,
                ships,
                task_forces,
                bases,
                air_groups,
                replacements,
                arrivals,
                land_units,
                hits_on_men: BTreeMap::new(),
                formations,
                entering,
                supply_lines,
                men_replacement,
                submarines,
                waters_waiting: Vec::new(),
            };
            let mut commands = BTreeMap::from([(side, own_command)]);
            // Re-point the half's orders past those already joined, after
            // checking that each is in the half's own list.
            let before = u32::try_from(orders.len()).expect("a count of orders");
            let listed = u32::try_from(own_orders.len()).expect("a count of orders");
            let mut unlisted = None;
            walk_held_ids(&mut own, &mut commands, &mut |where_, held| {
                if let HeldId::Order(order) = held {
                    let place = order.raw();
                    if (1..=listed).contains(&place) {
                        *order = WorldPicture::order_at(before + place);
                    } else {
                        unlisted.get_or_insert(format!(
                            "the halves cannot be joined: in the {} half, {where_} refers to \
                             order {place}, which is not in its list of {listed} orders",
                            side.name()
                        ));
                    }
                }
            });
            if let Some(words) = unlisted {
                return Err(refuse(words));
            }
            let TruthState {
                day: _,
                ships,
                task_forces,
                bases,
                air_groups,
                replacements,
                arrivals,
                land_units,
                hits_on_men: _,
                formations,
                entering,
                supply_lines,
                men_replacement,
                submarines,
                waters_waiting: _,
            } = own;
            join_units(&mut truth.ships, ships)?;
            join_units(&mut truth.task_forces, task_forces)?;
            join_units(&mut truth.bases, bases)?;
            join_units(&mut truth.air_groups, air_groups)?;
            join_units(&mut truth.land_units, land_units)?;
            join_units(&mut truth.formations, formations)?;
            join_units(&mut truth.entering, entering)?;
            join_units(&mut truth.submarines, submarines)?;
            truth.replacements.extend(replacements);
            truth.supply_lines.extend(supply_lines);
            truth
                .men_replacement
                .streams
                .extend(men_replacement.streams);
            join_units(&mut truth.men_replacement.units, men_replacement.units)?;
            truth.arrivals.extend(arrivals);
            orders.extend(own_orders);
            beliefs.insert(side, belief);
            command.extend(commands);
            counters.insert(side, own_counters);
        }
        // Replacements and arrivals are lists in the state, kept in one
        // order whatever order they are given in (decision 0043, 4c): the
        // order a scenario's loader puts them in.
        truth.replacements.sort_by_key(|stream| {
            (
                stream.side.code(),
                stream.base,
                stream.aircraft_type,
                stream.aircraft_per_day,
                stream.pilots_per_day,
            )
        });
        truth.supply_lines.sort_by_key(SupplyLine::order_key);
        truth
            .men_replacement
            .streams
            .sort_by_key(MenStream::order_key);
        truth.arrivals.sort_by_key(|arrival| {
            (
                arrival.day,
                arrival.air_group,
                arrival.aircraft,
                arrival.pilots,
            )
        });
        Ok(WorldPicture {
            day,
            truth,
            beliefs,
            command,
            orders,
            counters,
            objectives: None,
            game_days,
        })
    }
}
