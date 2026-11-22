//! A unit's condition and its fighting share (decision 0208): hungry men
//! are slow to mend. Worked when asked for from the unit's nights of hunger
//! and fed share; no state of its own.

use crate::events::Factor;
use crate::fixed::Fixed;
use crate::refdata::GroundFigures;

/// A unit's fighting share and what it is made of (decision 0208, 3 to 5),
/// as [`fighting_share`] gives it.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FightingShare {
    /// The share of the unit's men fed at last night's charge, as the state
    /// holds it (decision 0208, 9).
    pub fed_share: Fixed,
    /// The parts decision 0208 adds, where the scenario's ground figures give
    /// its two figures; `None` where they do not, and then `share` is the
    /// fed share.
    pub mending: Option<Mending>,
    /// What the unit's firing men and its work are scaled by (decision 0208,
    /// 5 to 7), from 0 to 1.
    pub share: Fixed,
}

/// The parts of a fighting share under decision 0208.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Mending {
    /// The unit's nights of hunger (decision 0205, 7).
    pub hunger_nights: u32,
    /// One less the nights times `hunger_weakness_per_night`, not below 0
    /// (decision 0208, 3).
    pub condition: Fixed,
    /// `unfed_fighting_share` plus one less it times the fed share
    /// (decision 0208, 4).
    pub meal_share: Fixed,
}

impl FightingShare {
    /// The factors of a breakdown that give the share (decision 0208, 15):
    /// `fed_share` always, then `hunger_nights`, `condition` and
    /// `fighting_share` where the figures of 0208 are given, so that a
    /// breakdown without them is as it was.
    pub fn factors(&self) -> Vec<Factor> {
        let mut factors = vec![Factor {
            name: "fed_share",
            value: self.fed_share,
        }];
        if let Some(mending) = self.mending {
            factors.push(Factor {
                name: "hunger_nights",
                value: Fixed::from_int(i64::from(mending.hunger_nights)),
            });
            factors.push(Factor {
                name: "condition",
                value: mending.condition,
            });
            factors.push(Factor {
                name: "fighting_share",
                value: self.share,
            });
        }
        factors
    }
}

/// A unit's fighting share (decision 0208, 3 to 5): its condition, one less
/// its nights of hunger times `hunger_weakness_per_night` and not below 0,
/// times its share from last night's meal, `unfed_fighting_share` plus one
/// less that figure times its fed share. Where the scenario's ground
/// figures do not give the two figures of 0208 it is the fed share, as
/// before. The one function the fight, the work, the side's own count of
/// firing men, the staff's count of work and the agent's line all read. It
/// is worked when asked for and is no state of its own (rule 3).
pub fn fighting_share(
    fed_share: Fixed,
    hunger_nights: u32,
    figures: &GroundFigures,
) -> FightingShare {
    let Some(figures) = figures.hunger.and_then(|hunger| hunger.mending) else {
        return FightingShare {
            fed_share,
            mending: None,
            share: fed_share,
        };
    };
    let condition = (Fixed::ONE - figures.hunger_weakness_per_night * i64::from(hunger_nights))
        .max(Fixed::ZERO);
    let unfed = figures.unfed_fighting_share;
    let meal_share = unfed + (Fixed::ONE - unfed) * fed_share;
    FightingShare {
        fed_share,
        mending: Some(Mending {
            hunger_nights,
            condition,
            meal_share,
        }),
        share: condition * meal_share,
    }
}

/// [`fighting_share`] of a land unit, from its fed share and its nights of
/// hunger.
pub fn unit_fighting_share(
    unit: &crate::state::LandUnit,
    figures: &GroundFigures,
) -> FightingShare {
    fighting_share(unit.fed_share, unit.hunger_nights, figures)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::refdata::{HungerFigures, MendingFigures};

    fn fx(text: &str) -> Fixed {
        text.parse().expect("a number")
    }

    fn figures(mending: Option<MendingFigures>) -> GroundFigures {
        GroundFigures {
            hunger: Some(HungerFigures {
                rations_landed_nights: Fixed::from_int(3),
                hungry_below_share: fx("0.6"),
                hunger_nights_before_loss: 14,
                mending,
                fight_supply: None,
            }),
            ..GroundFigures::FIRST
        }
    }

    fn with_two() -> GroundFigures {
        figures(Some(MendingFigures {
            hunger_weakness_per_night: fx("0.01"),
            unfed_fighting_share: fx("0.5"),
        }))
    }

    #[test]
    fn the_share_is_the_condition_times_the_share_from_the_meal() {
        // Decision 0208, 3 to 5, with the figures of scenario 122.
        let share = fighting_share(Fixed::ZERO, 1, &with_two());
        let mending = share.mending.expect("the figures are given");
        assert_eq!(mending.condition, fx("0.99"));
        assert_eq!(mending.meal_share, fx("0.5"));
        assert_eq!(share.share, fx("0.495"));
        assert_eq!(
            fighting_share(fx("0.5"), 1, &with_two()).share,
            fx("0.7425")
        );
        assert_eq!(fighting_share(Fixed::ONE, 0, &with_two()).share, Fixed::ONE);
        // Not below nothing past a hundred nights (rule 11).
        let past = fighting_share(Fixed::ONE, 120, &with_two());
        assert_eq!(past.mending.expect("given").condition, Fixed::ZERO);
        assert_eq!(past.share, Fixed::ZERO);
    }

    #[test]
    fn without_the_two_figures_the_share_is_the_fed_share() {
        for figures in [figures(None), GroundFigures::FIRST] {
            let share = fighting_share(fx("0.3"), 7, &figures);
            assert_eq!(share.share, fx("0.3"));
            assert_eq!(share.mending, None);
            let names: Vec<&str> = share.factors().iter().map(|f| f.name).collect();
            assert_eq!(names, vec!["fed_share"]);
        }
        let names: Vec<&str> = fighting_share(fx("0.3"), 7, &with_two())
            .factors()
            .iter()
            .map(|f| f.name)
            .collect();
        assert_eq!(
            names,
            vec!["fed_share", "hunger_nights", "condition", "fighting_share"]
        );
    }
}
