//! A trial's own draws (decision 0174, 3): in the trial subsystem, with no
//! pulse, on the picture's day, in the domain of the trial and the world,
//! keyed on the thing drawn for by its place in the side's own list. The
//! list is told by the draw's own value. Nothing of a live game's stream is
//! read: a live game has no domain and draws in no trial subsystem.

use sim_core::ids::EntityRef;
use sim_core::rng::{Domain, Draw, RngKey, RngStream, Subsystem};

use crate::Trial;

/// The stream of one thing drawn for in one world of a trial: `place` in
/// the list `draw` tells, and for a pair the second's place as `other`.
pub(crate) fn stream(
    trial: Trial,
    world: u32,
    day: u32,
    draw: Draw,
    place: u32,
    other: Option<u32>,
) -> RngStream {
    RngKey {
        seed: trial.seed,
        day,
        pulse: None,
        subsystem: Subsystem::Trial,
        entity: EntityRef::Place(place),
        other: other.map(EntityRef::Place),
        draw,
        domain: Some(Domain {
            trial: trial.trial,
            world,
        }),
    }
    .stream()
}
