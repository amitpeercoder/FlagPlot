//! What the air combat step notes for the wear step (decision 0035).

use super::raids::out_and_back;
use super::*;

impl World {
    /// Notes for the wear step the patrol each fighter group keeps over its
    /// hex (decision 7 of 0035): the aircraft it kept, no distance, caused by
    /// its orders. A group with no position, or no patrol, notes nothing, and
    /// nothing flies where it is dark.
    pub(super) fn note_patrols(&mut self, fighters: &[FighterGroup], pulse: Pulse) {
        for group in fighters {
            if group.patrol > 0 && group.hex.is_some_and(|hex| !time::is_dark(hex, pulse)) {
                self.sorties.push(Sortie {
                    air_group: group.air_group,
                    kind: SortieKind::Patrol,
                    aircraft: group.patrol,
                    sent: group.patrol,
                    distance_nm: Fixed::ZERO,
                    causes: group.orders.clone(),
                });
            }
        }
    }

    /// Notes for the wear step each strike launched: out to the hex it was
    /// sent to and back, caused by its launch.
    pub(super) fn note_strikes(&mut self, strikes: &[StrikeInFlight]) {
        for strike in strikes {
            let Some(from) = self
                .truth
                .air_groups
                .get(&strike.air_group)
                .and_then(|group| self.truth.air_group_hex(group))
            else {
                continue;
            };
            let kind = match strike.aim {
                Aim::Contact(_) => SortieKind::Strike,
                Aim::Base(_) => SortieKind::BaseStrike,
            };
            self.sorties.push(Sortie {
                air_group: strike.air_group,
                kind,
                aircraft: strike.aircraft,
                sent: strike.aircraft,
                distance_nm: out_and_back(from, strike.hex),
                causes: vec![Cause::Event(strike.launched)],
            });
        }
    }

    /// Notes for the wear step the escorts a fighter group sent with a raid
    /// to `raid_hex`: out and back, caused by the event that says they were
    /// sent.
    pub(super) fn note_escorts(
        &mut self,
        group: AirGroupId,
        aircraft: u32,
        from: Option<Hex>,
        raid_hex: Hex,
        sent: EventId,
    ) {
        let Some(from) = from else {
            return;
        };
        self.sorties.push(Sortie {
            air_group: group,
            kind: SortieKind::Escort,
            aircraft,
            sent: aircraft,
            distance_nm: out_and_back(from, raid_hex),
            causes: vec![Cause::Event(sent)],
        });
    }

    pub(super) fn kind_of(&self, ship: ShipId) -> ShipKind {
        self.ref_data.ship_classes[&self.truth.ships[&ship].class].kind
    }
}
