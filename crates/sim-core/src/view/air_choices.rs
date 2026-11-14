//! A side's questions of the air steps: aircraft able to fly, a base strike's launch, a fighter group's patrol and escort, the raid the escorts go with, where aircraft land when their deck is gone, and a tired group's stand-down or return (decision 0100, 2).
//!
//! Each answer is the engine's own function (`airstrike.rs`, `landing.rs`,
//! `upkeep.rs`) fed from what the side holds: its own air groups and ships,
//! its own bases and the public facts of the others, the raids it launched
//! this pulse, and the figures. Nothing of the enemy is read.

use crate::airstrike::{
    able_to_fly, base_strike_launch, escort_choice, fighter_split, raids_of_side,
};
use crate::choices::air::{AirAnswer, AirQuestion};
use crate::hex::Hex;
use crate::ids::{AirGroupId, BaseId, ShipId};
use crate::landing::{carriers_in_hex, landing_radius, place_landings, Airborne};
use crate::orders::AirMission;
use crate::state::{AirGroup, AirGroupLocation, ShipLocation, ShipSystem, Side, SystemState};
use crate::time::Pulse;
use crate::upkeep::rest_choice;
use crate::view::SideView;

impl SideView<'_> {
    /// One of the side's own air groups, by number.
    fn own_air_group(&self, id: AirGroupId) -> Option<&AirGroup> {
        self.air_groups().find(|group| group.id == id)
    }

    /// How many of `group`'s aircraft can fly this pulse
    /// ([`able_to_fly`]). `group` is one of the side's own: its base, if it
    /// is at one, is the side's own, and so is its ship.
    pub(crate) fn flyable_aircraft(&self, group: &AirGroup) -> u32 {
        let base_grounded = match group.location {
            AirGroupLocation::Base(base) => self.pulse.grounded_bases.contains(&base),
            _ => false,
        };
        let flight_deck = match group.location {
            AirGroupLocation::Ship(ship) => self
                .ships()
                .find(|own| own.id == ship)
                .map(|ship| ship.systems.get(ShipSystem::FlightDeck)),
            _ => None,
        };
        able_to_fly(
            group,
            base_grounded,
            flight_deck,
            self.ref_data.surface_combat.as_ref(),
            self.ref_data.fatigue.is_some(),
            self.truth.day + 1,
        )
    }

    /// The side and hex of a base, from what every side has: its own
    /// bases, and the public facts of the others (decision 0034, 2). `None`
    /// for a base that is not on the side's map.
    fn public_base_target(&self, base: BaseId) -> Option<(Side, Hex)> {
        if let Some(own) = self.bases().find(|own| own.id == base) {
            return Some((own.side, own.hex));
        }
        self.enemy_bases()
            .find(|other| other.id == base)
            .map(|other| (other.side, other.hex))
    }

    /// The hex of one of the side's own ships afloat: its task force's
    /// position, or its port's.
    fn own_ship_hex(&self, ship: ShipId) -> Option<Hex> {
        match self.ships().find(|own| own.id == ship)?.location {
            ShipLocation::TaskForce(force) => self
                .task_forces()
                .find(|own| own.id == force)
                .map(|force| force.position()),
            ShipLocation::Port(base) => self.bases().find(|own| own.id == base).map(|b| b.hex),
            ShipLocation::Sunk | ShipLocation::Yard(_) => None,
        }
    }

    /// Whether the flight deck of one of the side's own ships cannot take
    /// aircraft: she has sunk, or her deck is out. `None` if she is not the
    /// side's.
    fn own_deck_is_lost(&self, ship: ShipId) -> Option<bool> {
        let ship = self.ships().find(|own| own.id == ship)?;
        Some(
            ship.location == ShipLocation::Sunk
                || ship.systems.get(ShipSystem::FlightDeck) == SystemState::Out,
        )
    }

    /// Whether `group` is a fighter group on combat air patrol that has
    /// aircraft and is not away at the yard: the groups the patrol and
    /// escort choices are made for.
    fn is_patrol_group(&self, group: &AirGroup) -> bool {
        group.mission == AirMission::CombatAirPatrol
            && group.ready > 0
            && !self.truth.air_group_at_yard(group)
    }

    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise.
    pub(crate) fn ask_air(&self, question: &AirQuestion) -> Option<AirAnswer> {
        match question {
            AirQuestion::Strike { group } => {
                let group = self.own_air_group(*group)?;
                if group.mission != AirMission::NavalStrike {
                    return None;
                }
                Some(AirAnswer::Strike(self.strike_choice(group.id)))
            }
            AirQuestion::AircraftAbleToFly { group } => {
                let group = self.own_air_group(*group)?;
                Some(AirAnswer::AircraftAbleToFly(self.flyable_aircraft(group)))
            }
            AirQuestion::BaseStrike { group, pulse } => {
                let group = self.own_air_group(*group)?;
                let AirMission::BaseStrike(base) = group.mission else {
                    return None;
                };
                let origin = self.truth.air_group_hex(group)?;
                let weapon = self.aircraft_type(group.aircraft_type)?.attack_weapon;
                let launch = base_strike_launch(
                    group,
                    base,
                    self.truth.day + 1,
                    *pulse,
                    origin,
                    weapon,
                    self.flyable_aircraft(group),
                    self.reach_nm(group),
                    self.public_base_target(base),
                );
                Some(AirAnswer::BaseStrike(launch))
            }
            AirQuestion::FighterSplit { group } => {
                let group = self.own_air_group(*group)?;
                if !self.is_patrol_group(group) {
                    return None;
                }
                Some(AirAnswer::FighterSplit(fighter_split(
                    self.flyable_aircraft(group),
                    group.patrol_share,
                )))
            }
            AirQuestion::Escort { group } => {
                let group = self.own_air_group(*group)?;
                if !self.is_patrol_group(group) {
                    return None;
                }
                let split = fighter_split(self.flyable_aircraft(group), group.patrol_share);
                if split.escorts == 0 {
                    return None;
                }
                let raids = raids_of_side(self.pulse.strikes_in_flight, self.side);
                Some(AirAnswer::Escort(escort_choice(
                    &raids,
                    group.location,
                    self.truth.air_group_hex(group),
                    self.reach_nm(group),
                )))
            }
            AirQuestion::Landing { groups } => self.ask_landing(groups),
            AirQuestion::Rest { group } => {
                let group = self.own_air_group(*group)?;
                let threshold = group.rest_above?;
                if self.ref_data.fatigue.is_none() || self.truth.air_group_at_yard(group) {
                    return None;
                }
                Some(AirAnswer::Rest(rest_choice(
                    group.fatigue,
                    threshold,
                    group.stood_down,
                )))
            }
        }
    }

    /// Where the aircraft of `groups` (each with the aircraft it has in the
    /// air) land. A group that is not the side's, is not aboard a ship whose
    /// deck is gone, has no position or no aircraft in the air is left out,
    /// as the engine leaves it out.
    fn ask_landing(&self, groups: &[(AirGroupId, u32)]) -> Option<AirAnswer> {
        let air_groups: Vec<&AirGroup> = self.air_groups().collect();
        let mut airborne: Vec<Airborne> = Vec::new();
        for (id, in_air) in groups {
            let Some(group) = self.own_air_group(*id) else {
                continue;
            };
            let AirGroupLocation::Ship(own_ship) = group.location else {
                continue;
            };
            if self.own_deck_is_lost(own_ship) != Some(true) {
                continue;
            }
            // The last noted: a ship whose deck went out and who then sank is
            // closed by her sinking.
            let hex = self
                .pulse
                .deck_losses
                .iter()
                .rfind(|loss| loss.ship == own_ship)
                .map(|loss| loss.hex)
                .or_else(|| self.own_ship_hex(own_ship));
            let Some(hex) = hex else {
                continue;
            };
            if *in_air == 0 {
                continue;
            }
            let aircraft_type = self.aircraft_type(group.aircraft_type)?;
            airborne.push(Airborne {
                group: group.id,
                hex,
                radius_nm: landing_radius(aircraft_type),
                in_air: *in_air,
                carriers: carriers_in_hex(
                    self.ships(),
                    self.side,
                    own_ship,
                    hex,
                    |ship| self.own_ship_hex(ship),
                    self.ref_data,
                    air_groups.iter().copied(),
                ),
            });
        }
        let bases: Vec<(BaseId, Hex)> = self
            .bases()
            .filter(|base| base.airfield)
            .map(|base| (base.id, base.hex))
            .collect();
        Some(AirAnswer::Landing(place_landings(&airborne, &bases)))
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to. The landing question is one for the
    /// side: the carriers' room is shared between its groups. It names each
    /// own group aboard a ship whose deck is gone with the aircraft it has
    /// in the air, taken from the sorties noted this pulse, else its ready
    /// aircraft.
    pub(crate) fn air_questions(&self) -> Vec<AirQuestion> {
        let mut questions = Vec::new();
        for group in self.air_groups() {
            let id = group.id;
            questions.push(AirQuestion::AircraftAbleToFly { group: id });
            match group.mission {
                AirMission::NavalStrike => questions.push(AirQuestion::Strike { group: id }),
                AirMission::BaseStrike(_) => {
                    for pulse in [Pulse::Morning, Pulse::Afternoon] {
                        questions.push(AirQuestion::BaseStrike { group: id, pulse });
                    }
                }
                AirMission::CombatAirPatrol => {
                    questions.push(AirQuestion::FighterSplit { group: id });
                    questions.push(AirQuestion::Escort { group: id });
                }
                _ => {}
            }
            if group.rest_above.is_some() {
                questions.push(AirQuestion::Rest { group: id });
            }
        }
        let landing: Vec<(AirGroupId, u32)> = self
            .air_groups()
            .filter_map(|group| {
                let AirGroupLocation::Ship(ship) = group.location else {
                    return None;
                };
                if self.own_deck_is_lost(ship) != Some(true) {
                    return None;
                }
                let flown: u32 = self
                    .pulse
                    .sorties
                    .iter()
                    .filter(|sortie| sortie.air_group == group.id)
                    .map(|sortie| sortie.aircraft)
                    .sum();
                Some((group.id, if flown > 0 { flown } else { group.ready }))
            })
            .collect();
        if !landing.is_empty() {
            questions.push(AirQuestion::Landing { groups: landing });
        }
        questions
    }
}
