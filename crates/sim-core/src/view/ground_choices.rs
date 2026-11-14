//! A side's questions of the land units: going back to defend, and what a unit's day is (decision 0100, 2).

use crate::choices::ground::{GroundAnswer, GroundQuestion};
use crate::ground_combat::{goes_back_to_defend, is_with_force, unit_day};
use crate::hex::Hex;
use crate::ids::{LandUnitId, TaskForceId};
use crate::orders::Posture;
use crate::preparation::{qualifies_for_loading, rostered_among};
use crate::state::LandUnit;
use crate::time::{pulse_start, Pulse};
use crate::view::SideView;

impl SideView<'_> {
    /// The answer to `question` from what this side holds now. `None`
    /// where the question names no unit of this side, or the choice does not
    /// arise (a unit with no man ashore does not tire; a force that is not
    /// the side's cannot be rostered with).
    pub(crate) fn ask_ground(&self, question: &GroundQuestion) -> Option<GroundAnswer> {
        let figures = self.ref_data.ground.as_ref()?;
        match *question {
            GroundQuestion::BackToDefend { unit, fatigue } => {
                let land_unit = self.own_land_unit(unit)?;
                if land_unit.men_ashore == 0 {
                    return None;
                }
                Some(GroundAnswer::BackToDefend(goes_back_to_defend(
                    land_unit.posture,
                    fatigue,
                    figures.attack_fatigue_limit,
                )))
            }
            GroundQuestion::UnitDay {
                unit,
                marched,
                contested,
                rostered_with,
            } => {
                let land_unit = self.own_land_unit(unit)?;
                if let Some(force) = rostered_with {
                    self.truth
                        .task_forces
                        .get(&force)
                        .filter(|force| force.side == self.side)?;
                }
                Some(GroundAnswer::UnitDay(unit_day(
                    land_unit.men_ashore,
                    self.truth.men_aboard(unit),
                    land_unit.working,
                    land_unit.activity,
                    marched,
                    contested,
                    rostered_with,
                    |force| is_with_force(self.truth, unit, force),
                )))
            }
        }
    }

    /// Every question of this kind this side could ask now, one for each of
    /// its units the choice applies to: whether each unit with men ashore
    /// goes back to defend, asked with its present fatigue (the engine tests
    /// the fatigue after the day's update, so the answer is for the unit as
    /// it stands now); and what each unit with men ashore or aboard does
    /// with its day, asked with the inputs the staff can fill from what it
    /// holds:
    ///
    /// - `marched`: whether the unit has a march under way (the engine knows
    ///   it only when the day's marches have run);
    /// - `contested`: whether a fight in its hex is certain, which is when a
    ///   fighting unit of its own there is on attack and the side holds an
    ///   estimate of enemy men in the hex (it cannot know that the enemy has
    ///   fighting men there, nor that the enemy attacks, so a fight it only
    ///   fears is not counted);
    /// - `rostered_with`: the force whose roster the unit is on, if that
    ///   force would qualify to gain loading on the day in hand
    ///   (decision 4 of 0095).
    pub(crate) fn ground_questions(&self) -> Vec<GroundQuestion> {
        if self.ref_data.ground.is_none() {
            return Vec::new();
        }
        let day_start = pulse_start(self.day(), Pulse::Night);
        let qualifying: Vec<TaskForceId> = self
            .truth
            .task_forces
            .values()
            .filter(|force| force.side == self.side)
            .filter(|force| qualifies_for_loading(self.truth, force, day_start))
            .map(|force| force.id)
            .collect();
        let mut questions = Vec::new();
        for unit in self.land_units() {
            if unit.men_ashore > 0 {
                questions.push(GroundQuestion::BackToDefend {
                    unit: unit.id,
                    fatigue: unit.fatigue,
                });
            }
            if unit.men_ashore > 0 || self.truth.men_aboard(unit.id) > 0 {
                questions.push(GroundQuestion::UnitDay {
                    unit: unit.id,
                    marched: unit.march.is_some(),
                    contested: self.fight_certain_at(unit),
                    rostered_with: rostered_among(self.truth, qualifying.iter().copied(), unit.id),
                });
            }
        }
        questions
    }

    /// One of the side's own land units, or `None`.
    fn own_land_unit(&self, unit: LandUnitId) -> Option<&LandUnit> {
        self.truth
            .land_units
            .get(&unit)
            .filter(|land_unit| land_unit.side == self.side)
    }

    /// Whether a fight in the hex of `unit` is certain from what the side
    /// holds: the unit has men ashore, a fighting unit of the side's own
    /// there is on attack, and the side holds an estimate of enemy men in
    /// the hex.
    fn fight_certain_at(&self, unit: &LandUnit) -> bool {
        let Some(hex) = unit.hex.filter(|_| unit.men_ashore > 0) else {
            return false;
        };
        self.own_attack_with_fighting_men_at(hex)
            && self
                .ground_estimate(hex)
                .is_some_and(|estimate| estimate.men > 0)
    }

    /// Whether the side has fighting men (not working units) ashore in
    /// `hex` and a unit with men ashore there on attack.
    fn own_attack_with_fighting_men_at(&self, hex: Hex) -> bool {
        let here = |unit: &&LandUnit| unit.men_ashore > 0 && unit.hex == Some(hex);
        let fighting = self.land_units().filter(here).any(|unit| !unit.working);
        let attacks = self
            .land_units()
            .filter(here)
            .any(|unit| unit.posture == Posture::Attack);
        fighting && attacks
    }
}
