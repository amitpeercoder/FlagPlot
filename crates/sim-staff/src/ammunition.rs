//! Ammunition as policy (decision 0127): whether a surface force is short of
//! ammunition by its risk level, and from which day it may be used.
//!
//! A ship is short when its rounds aboard are below the share of its outfit
//! the risk gives, or more of its tubes have fired than the share of its
//! tubes the risk allows; a ship whose class carries the marker for
//! ammunition that is not limiting is never short of rounds. A force is
//! short when any ship of it afloat is. Where the force lies at a base of
//! its side that can rearm it, it may be used from the day after the last of
//! its ships will be no longer short, by the engine's measure of the hours
//! rearming takes there behind its fuel (`SideView::ammunition_work`);
//! where it cannot, there is no day.

use sim_core::command::Risk;
use sim_core::fixed::Fixed;
use sim_core::ids::TaskForceId;
use sim_core::rearm::ShipAmmunition;
use sim_core::view::SideView;

/// The share of its outfit below which a ship is short of rounds (the
/// table): three quarters, a half, a quarter.
pub fn rounds_share(risk: Risk) -> Fixed {
    match risk {
        Risk::Preserve => Fixed::from_ratio(3, 4),
        Risk::Balanced => Fixed::from_ratio(1, 2),
        Risk::Press => Fixed::from_ratio(1, 4),
    }
}

/// The tubes of `tubes` a ship may have fired and not be short of
/// torpedoes (the table): none, a half, or every one (never short).
pub fn tubes_allowed(risk: Risk, tubes: u32) -> u32 {
    match risk {
        Risk::Preserve => 0,
        Risk::Balanced => tubes / 2,
        Risk::Press => tubes,
    }
}

/// A force short of ammunition: its reason's words and the hours from the
/// start of the day until the last of its ships is no longer short, `None`
/// where it cannot rearm where it lies.
pub(crate) struct Short {
    pub(crate) words: String,
    pub(crate) hours: Option<Fixed>,
}

/// One ship's shortness: its words, and the tons it must take, rounds first,
/// to be short no longer.
fn ship_short(ship: &ShipAmmunition, risk: Risk) -> Option<(String, Fixed)> {
    let mut words: Vec<String> = Vec::new();
    let mut rounds_needed: u32 = 0;
    if let Some(outfit) = ship.outfit {
        let level = rounds_share(risk) * Fixed::from_int(i64::from(outfit));
        if ship.rounds_aboard < level {
            let lack = level - ship.rounds_aboard;
            let mut whole = lack.trunc();
            if Fixed::from_int(whole) < lack {
                whole += 1;
            }
            rounds_needed = u32::try_from(whole.max(0)).unwrap_or(u32::MAX);
            words.push(format!(
                "{} has {} rounds against the {level} of a {} of her outfit of {outfit}",
                ship.ship,
                ship.rounds_aboard,
                rounds_share(risk)
            ));
        }
    }
    let allowed = tubes_allowed(risk, ship.tubes);
    let mut torpedoes_needed = 0;
    if ship.torpedoes > allowed {
        torpedoes_needed = ship.torpedoes - allowed;
        words.push(format!(
            "{} has {} of her {} tubes fired, more than the {allowed} allowed",
            ship.ship, ship.torpedoes, ship.tubes
        ));
    }
    if words.is_empty() {
        return None;
    }
    // Rounds are taken before torpedoes: to load a torpedo she takes every
    // round she lacks first.
    let rounds_before = if torpedoes_needed > 0 {
        ship.rounds
    } else {
        rounds_needed
    };
    let tons = ship.round_t * Fixed::from_int(i64::from(rounds_before))
        + ship.torpedo_t * Fixed::from_int(i64::from(torpedoes_needed));
    Some((words.join(", and "), tons))
}

/// Whether `force` is short of ammunition at `risk` (decision 0127, 1), and
/// if so the words and hours of rules 2 to 4. `None` where it is not short.
pub(crate) fn short(view: &SideView<'_>, force: TaskForceId, risk: Risk) -> Option<Short> {
    let work = view.ammunition_work(force)?;
    let mut words: Vec<String> = Vec::new();
    let mut hours: Option<Fixed> = Some(Fixed::ZERO);
    let mut tons_all = Fixed::ZERO;
    for ship in &work.ships {
        let Some((text, tons)) = ship_short(ship, risk) else {
            continue;
        };
        words.push(text);
        tons_all += tons;
        hours = match (hours, ship.rate) {
            (Some(most), Some(rate)) if rate > Fixed::ZERO => {
                Some(most.max(ship.hours_before + tons / rate))
            }
            _ => None,
        };
    }
    if words.is_empty() {
        return None;
    }
    if work.base.is_none_or(|(_, stock)| stock < tons_all) {
        hours = None;
    }
    Some(Short {
        words: format!(
            "at {} risk {}: the force is short of ammunition",
            risk.name(),
            words.join("; ")
        ),
        hours,
    })
}
