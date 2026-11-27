//! A strike spread over the ships of the kind it wants (decision 0211, its
//! section "Measured and reviewed again", which governs).
//!
//! Where the air combat figures say `strike_is_spread`, the raids that
//! attack one force in a pulse and want one kind of ship are taken in turn,
//! in the order of their first group's number (the order the raids come in).
//! A raid wants the kind its first group ranks best among the force's ships
//! afloat. The first of them draws its ship: one draw, each ship of the kind
//! that does not look lost as likely as another, or among all of the kind
//! where every one looks lost. Each raid after it takes the next ship of
//! that pool by ship number, going round. The groups of a raid that want the
//! raid's kind attack its ship; a group that wants another kind attacks the
//! first ship of that kind by the same order (those that do not look lost
//! first, by ship number), and takes no turn.
//!
//! "Looks lost" is read once, on the state at the start of the air combat
//! step: her flight deck out, where she has one; her flotation damage at or
//! above `crippled_flotation`; or her fire damage at or above
//! `claimed_sunk_fire`. A fire among her aircraft does not make a carrier
//! look lost (decision 0212, "The designer's word").
//!
//! The draw is on a stream of the air combat subsystem keyed on the first
//! raid's first strike group and the force, [`Draw::StrikeSpread`]: a draw
//! d below one, read in millionths, takes the ship at place floor(n x d) of
//! the n in the pool. Nothing else of the attack draws differently: each
//! group's attack keeps its own stream, keyed on the group and the force.

use super::*;

/// How a raid's ship was chosen, for the breakdown of its attacks.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) enum SpreadChoice {
    /// The first raid of its kind at the force drew among `among`, with
    /// `draw`.
    Drawn { among: Vec<ShipId>, draw: Fixed },
    /// A later raid took the next ship after `after`, the ship the raid
    /// before it took.
    ByTurn { after: ShipId },
}

/// The ship a strike group attacks under the rule, and how it was chosen:
/// `None` for a group of another kind than its raid's, which took the first
/// of its kind by the order.
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) struct Spread {
    pub(super) ship: ShipId,
    pub(super) choice: Option<SpreadChoice>,
}

impl World {
    /// Whether `ship` looks lost to a raid, on the state as it stands
    /// (decision 0211, as measured, 4).
    fn looks_lost(&self, ship: ShipId, surface: &SurfaceCombatFigures) -> bool {
        let state = &self.truth.ships[&ship];
        let has_deck = self
            .ref_data
            .ship_classes
            .get(&state.class)
            .is_some_and(|class| class.has_system(ShipSystem::FlightDeck));
        (has_deck && state.systems.get(ShipSystem::FlightDeck) == SystemState::Out)
            || state.flotation_damage >= surface.crippled_flotation
            || state.fire_damage >= surface.claimed_sunk_fire
    }

    /// The ships of `kind` afloat in `force`, those that do not look lost
    /// first, each part by ship number, and how many do not.
    fn spread_order(
        &self,
        force: TaskForceId,
        kind: ShipKind,
        surface: &SurfaceCombatFigures,
    ) -> (Vec<ShipId>, usize) {
        let (sound, lost): (Vec<ShipId>, Vec<ShipId>) = self.truth.task_forces[&force]
            .ships
            .iter()
            .copied()
            .filter(|ship| self.kind_of(*ship) == kind)
            .partition(|ship| !self.looks_lost(*ship, surface));
        let count = sound.len();
        (sound.into_iter().chain(lost).collect(), count)
    }

    /// The kind `group` ranks best among the ships afloat in `force`.
    fn wanted_kind(&self, group: AirGroupId, force: TaskForceId) -> ShipKind {
        let group = &self.truth.air_groups[&group];
        self.truth.task_forces[&force]
            .ships
            .iter()
            .map(|ship| (group.target_rank(self.kind_of(*ship)), self.kind_of(*ship)))
            .min()
            .map(|(_, kind)| kind)
            .expect("the force the raid found has a ship afloat")
    }

    /// The ship each strike group of `raids` attacks, where the rule is on
    /// (decision 0211, as measured), on the state at the start of the air
    /// combat step. Empty where it is off: every group then chooses as
    /// [`World::ship_to_attack`] does. Reads truth, which the umpire may;
    /// writes nothing.
    pub(super) fn spread_raids(
        &self,
        day: u32,
        pulse: Pulse,
        raids: &[Raid],
    ) -> BTreeMap<AirGroupId, Spread> {
        let mut spread = BTreeMap::new();
        let on = self
            .ref_data
            .air_combat
            .as_ref()
            .is_some_and(|air| air.strike_is_spread);
        if !on {
            return spread;
        }
        let surface = &self.ref_data.surface_combat;
        // (force, kind) -> the ship the last raid of that kind took.
        let mut last: BTreeMap<(TaskForceId, ShipKind), ShipId> = BTreeMap::new();
        // The raids come in the order of their first group (`group_raids`).
        for raid in raids {
            let first = &raid.strikes[0];
            let Some(RaidTarget::Ships(force)) = self.raid_target(first) else {
                continue;
            };
            let kind = self.wanted_kind(first.air_group, force);
            let (order, sound) = self.spread_order(force, kind, surface);
            // The pool: those that do not look lost, or all of the kind
            // where every one does.
            let pool: &[ShipId] = if sound > 0 { &order[..sound] } else { &order };
            let (ship, choice) = match last.get(&(force, kind)) {
                None => {
                    let mut stream = self.rng_pair(
                        day,
                        Some(pulse),
                        Subsystem::AirCombat,
                        Draw::StrikeSpread,
                        EntityRef::AirGroup(first.air_group),
                        EntityRef::TaskForce(force),
                    );
                    let raw = stream.below(Fixed::SCALE.unsigned_abs());
                    let draw = Fixed::from_raw(i64::try_from(raw).expect("a draw fits in i64"));
                    let n = u64::try_from(pool.len()).expect("few ships");
                    let place = usize::try_from(n * raw / Fixed::SCALE.unsigned_abs())
                        .expect("a place among few ships");
                    (
                        pool[place],
                        SpreadChoice::Drawn {
                            among: pool.to_vec(),
                            draw,
                        },
                    )
                }
                Some(after) => {
                    let next = pool
                        .iter()
                        .copied()
                        .find(|ship| ship > after)
                        .unwrap_or(pool[0]);
                    (next, SpreadChoice::ByTurn { after: *after })
                }
            };
            last.insert((force, kind), ship);
            for strike in &raid.strikes {
                let group_kind = self.wanted_kind(strike.air_group, force);
                let chosen = if group_kind == kind {
                    Spread {
                        ship,
                        choice: Some(choice.clone()),
                    }
                } else {
                    // Another kind: the first of it by the same order.
                    let (other, _) = self.spread_order(force, group_kind, surface);
                    Spread {
                        ship: other[0],
                        choice: None,
                    }
                };
                spread.insert(strike.air_group, chosen);
            }
        }
        spread
    }
}

/// The factors a spread choice adds to an attack's breakdown: for a draw,
/// each ship it could have drawn (`spread_could_draw`, the ship's number),
/// the draw and the ship drawn; for a raid by turn, the ship it took and the
/// ship the raid before it took.
pub(super) fn spread_factors(ship: ShipId, choice: &SpreadChoice) -> Vec<Factor> {
    let number = |ship: ShipId| Fixed::from_int(i64::from(ship.raw()));
    match choice {
        SpreadChoice::Drawn { among, draw } => among
            .iter()
            .map(|candidate| Factor {
                name: "spread_could_draw",
                value: number(*candidate),
            })
            .chain([
                Factor {
                    name: "spread_draw",
                    value: *draw,
                },
                Factor {
                    name: "spread_ship_drawn",
                    value: number(ship),
                },
            ])
            .collect(),
        SpreadChoice::ByTurn { after } => vec![
            Factor {
                name: "spread_ship_by_turn",
                value: number(ship),
            },
            Factor {
                name: "spread_after_ship",
                value: number(*after),
            },
        ],
    }
}
