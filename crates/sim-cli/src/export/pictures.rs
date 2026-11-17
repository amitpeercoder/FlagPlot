//! Each command's picture as JSON (replay version 6, decision 0138, item 6):
//! for a scenario with command levels on, the contacts each command of each
//! side holds, with that picture's own numbers and whether each is the
//! command's own or was told it (decision 0137, 5). The umpire's record, as
//! everything in a replay file is.
//!
//! Written only while command levels are on (`World::commands` is `Some`):
//! with them off there is no `pictures` field anywhere, so the replay of
//! such a run is byte for byte what it was, version apart.

use serde::Serialize;
use sim_core::ids::UmpireKey;
use sim_core::state::Side;
use sim_core::World;

use super::events::command_word;
use super::state::PerSide;

/// One contact of a command's picture: what a side's contact carries, the
/// picture's own number, and `own`.
#[derive(Serialize)]
struct PictureContactOut {
    id: u32,
    hex: (i32, i32),
    ship_count: u32,
    kind: &'static str,
    age_pulses: u32,
    /// The event of the report that last made or refreshed it there.
    report_event: u32,
    /// The command's own sighting (true) or one it was told (false).
    own: bool,
}

/// One command's picture: `command` is `area`, `task_force:N` or `base:N`.
#[derive(Serialize)]
pub struct PictureOut {
    pub command: String,
    contacts: Vec<PictureContactOut>,
}

fn pictures_of(world: &World, side: Side, key: &UmpireKey) -> Vec<PictureOut> {
    let Some(commands) = world.commands(side) else {
        return Vec::new();
    };
    commands
        .iter()
        .map(|(command, post)| PictureOut {
            command: command_word(*command),
            contacts: post
                .picture()
                .contacts()
                .map(|contact| PictureContactOut {
                    id: contact.id.umpire_raw(key),
                    hex: (contact.estimated_hex.q, contact.estimated_hex.r),
                    ship_count: contact.estimated_ship_count,
                    kind: contact.estimated_kind.name(),
                    age_pulses: contact.age_pulses,
                    report_event: contact.last_report_event.umpire_raw(key),
                    own: post.own().contains(&contact.id),
                })
                .collect(),
        })
        .collect()
}

/// The pictures of every command of both sides, each side's in command order
/// (area, task forces, bases). Empty for a side while command levels are off.
pub fn pictures(world: &World) -> PerSide<PictureOut> {
    let key = world.umpire_key();
    PerSide {
        allied: pictures_of(world, Side::Allied, &key),
        japanese: pictures_of(world, Side::Japanese, &key),
    }
}
