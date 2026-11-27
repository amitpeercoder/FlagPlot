use crate::commands::BelievedIntentSpec;
use crate::reference::parse_kind;
use serde::Deserialize;
use sim_core::believed::{BelievedForce, BelievedIntent, BelievedMen, LastKnown, Standing};
use sim_core::hex::Hex;
use sim_core::ids::BaseId;
use sim_core::map::HexKind;
use sim_core::state::Side;
use sim_core::World;

use super::commanders::SuperiorSpec;
use super::orders::SideSpec;

/// A contact a side holds when the game begins (decision 8 of
/// `docs/decisions/0021-readiness-and-alarm.md`).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ContactSpec {
    pub(super) side: SideSpec,
    pub(super) q: i32,
    pub(super) r: i32,
    pub(super) ship_count: u32,
    /// A ship kind spelt as in the ship class file: `carrier`, `battleship`,
    /// `cruiser`, `destroyer`, `transport` or `auxiliary`.
    pub(super) kind: String,
    /// How many pulses old the contact is at the start. Defaults to 0.
    pub(super) age_pulses: Option<u32>,
    /// The one command that saw it (decision 0137, 6), as in
    /// `seen_by = { task_force = 1 }`: with command levels on it is in that
    /// command's picture alone, as its own, and sent to no one. With them
    /// off it is the side's, as any other. Defaults to none: every command
    /// was told of it.
    pub(super) seen_by: Option<SuperiorSpec>,
}

/// An estimate of the enemy's men ashore in a hex that a side holds when the
/// game begins (decision 16 of 0092).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct EstimateSpec {
    pub(super) side: SideSpec,
    pub(super) q: i32,
    pub(super) r: i32,
    pub(super) men: u32,
}

/// The waters where a side believes enemy submarines work (decision 0218, 23
/// and 47): a hex and a distance round it, the day the side is told (the
/// first day by default), the words of the belief, the rows it rests on and
/// the architect's reading of the rows into this hex and distance, marked.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SuspectedWatersSpec {
    pub(super) side: SideSpec,
    pub(super) q: i32,
    pub(super) r: i32,
    pub(super) distance: u32,
    pub(super) day: Option<u32>,
    pub(super) words: String,
    pub(super) rows: String,
    pub(super) reading: String,
}

/// A force of the enemy's a side believes in and holds no contact of
/// (decision 0173, 5): the side that believes it, a name, the kind of its
/// most important ship, its ships, its standing (`present`, `absent` or
/// `unknown`), and where it was last known: `at_base`, an enemy base on the
/// side's map, or `q` and `r`, with `day` (0, the default, for before the
/// first day); or neither, where the side does not know where. `placeholder`
/// is true where the belief rests on no source, and then `note` is required.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BelievedForceSpec {
    pub(super) side: SideSpec,
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) ships: u32,
    pub(super) standing: String,
    pub(super) at_base: Option<u32>,
    pub(super) q: Option<i32>,
    pub(super) r: Option<i32>,
    pub(super) day: Option<u32>,
    pub(super) placeholder: Option<bool>,
    pub(super) note: Option<String>,
}

/// Men of the enemy's a side believes may be ashore at a place beyond those
/// it holds an estimate of (decision 0176, 1, amended 6 and 7): the side that
/// believes it, a name, the place (`at_base`, a base of the enemy's on the
/// side's map, or `q` and `r`, a hex of land on the map), the men, 1 or more,
/// and the standing (`present`, `absent` or `unknown`). `placeholder` is true
/// where the belief rests on no source, and then `note` is required.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BelievedMenSpec {
    pub(super) side: SideSpec,
    pub(super) name: String,
    pub(super) men: u32,
    pub(super) standing: String,
    pub(super) at_base: Option<u32>,
    pub(super) q: Option<i32>,
    pub(super) r: Option<i32>,
    pub(super) placeholder: Option<bool>,
    pub(super) note: Option<String>,
}

/// The note an entry rests on, or why it is wrong: a placeholder needs one,
/// and a note given is not empty (decision 0173, 5).
pub(super) fn entry_note(
    placeholder: bool,
    note: &Option<String>,
    what: &str,
) -> Result<Option<String>, String> {
    match note {
        Some(text) if text.trim().is_empty() => Err(format!("{what} has an empty note")),
        Some(text) => Ok(Some(text.clone())),
        None if placeholder => Err(format!(
            "{what} is a placeholder and has no note of what it rests on"
        )),
        None => Ok(None),
    }
}

/// A believed force as the scenario states it, checked (decision 0173, 5,
/// 6 and 9) against the world as loaded: its base is an enemy base on the
/// believing side's map, its hex is on the map, its day no later than the
/// last.
pub(super) fn believed_force(
    spec: &BelievedForceSpec,
    index: usize,
    days: u32,
    world: &World,
) -> Result<BelievedForce, String> {
    let side: Side = spec.side.into();
    let what = format!("believed force {} ({})", index + 1, spec.name);
    let kind = parse_kind(&spec.kind).ok_or_else(|| {
        format!(
            "{what} has unknown kind \"{}\": carrier, battleship, cruiser, destroyer, transport or auxiliary",
            spec.kind
        )
    })?;
    if spec.ships == 0 {
        return Err(format!(
            "{what} has no ship: it is believed to have 1 or more"
        ));
    }
    let standing = Standing::from_name(&spec.standing).ok_or_else(|| {
        format!(
            "{what} has unknown standing \"{}\": present, absent or unknown",
            spec.standing
        )
    })?;
    let day = spec.day.unwrap_or(0);
    if day > days {
        return Err(format!(
            "{what} was known there on day {day}, after the scenario's last, day {days}"
        ));
    }
    let last_known = match (spec.at_base, spec.q, spec.r) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
            return Err(format!(
                "{what} has both \"at_base\" and a hex; it takes one or neither"
            ))
        }
        (Some(raw), None, None) => {
            let id = BaseId::new(raw);
            let base = world.umpire().bases.get(&id).ok_or_else(|| {
                format!("{what} is at base {raw}, which is not a base of the scenario")
            })?;
            if base.side == side {
                return Err(format!(
                    "{what} is at base {raw} ({}), which is the {} side's own: a believed force is the enemy's",
                    base.name,
                    side.name()
                ));
            }
            if !base.on_map {
                return Err(format!(
                    "{what} is at base {raw} ({}), which is not on the {} side's map",
                    base.name,
                    side.name()
                ));
            }
            LastKnown::AtBase {
                base: id,
                hex: base.hex,
                day,
            }
        }
        (None, Some(q), Some(r)) => {
            let hex = Hex::new(q, r);
            if !world.map().contains(hex) {
                return Err(format!("{what} is at {q},{r}, off the map"));
            }
            LastKnown::AtHex { hex, day }
        }
        (None, Some(_), None) | (None, None, Some(_)) => {
            return Err(format!(
                "{what} gives one of \"q\" and \"r\"; a hex takes both"
            ))
        }
        (None, None, None) => {
            if spec.day.is_some() {
                return Err(format!(
                    "{what} gives a day and no place; the day is the day it was known at a place"
                ));
            }
            LastKnown::NotKnown
        }
    };
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedForce {
        side,
        name: spec.name.clone(),
        kind,
        ships: spec.ships,
        standing,
        last_known,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}

/// Believed men as the scenario states them, checked (decision 0176, 1 and
/// amended 7) as a believed force is, against the world as loaded: a place,
/// a base of the enemy's on the believing side's map or a hex of the map that
/// is not sea; 1 or more men; a known standing.
pub(super) fn believed_men(
    spec: &BelievedMenSpec,
    index: usize,
    world: &World,
) -> Result<BelievedMen, String> {
    let side: Side = spec.side.into();
    let what = format!("believed men {} ({})", index + 1, spec.name);
    if spec.men == 0 {
        return Err(format!("{what} have no man: they are 1 or more"));
    }
    let standing = Standing::from_name(&spec.standing).ok_or_else(|| {
        format!(
            "{what} have unknown standing \"{}\": present, absent or unknown",
            spec.standing
        )
    })?;
    let (hex, base) = match (spec.at_base, spec.q, spec.r) {
        (Some(_), Some(_), _) | (Some(_), _, Some(_)) => {
            return Err(format!(
                "{what} have both \"at_base\" and a hex; they take one"
            ))
        }
        (Some(raw), None, None) => {
            let id = BaseId::new(raw);
            let base = world.umpire().bases.get(&id).ok_or_else(|| {
                format!("{what} are at base {raw}, which is not a base of the scenario")
            })?;
            if base.side == side {
                return Err(format!(
                    "{what} are at base {raw} ({}), which is the {} side's own: believed men are the enemy's",
                    base.name,
                    side.name()
                ));
            }
            if !base.on_map {
                return Err(format!(
                    "{what} are at base {raw} ({}), which is not on the {} side's map",
                    base.name,
                    side.name()
                ));
            }
            (base.hex, Some(id))
        }
        (None, Some(q), Some(r)) => {
            let hex = Hex::new(q, r);
            let Some(data) = world.map().get(hex) else {
                return Err(format!("{what} are at {q},{r}, off the map"));
            };
            if !matches!(data.kind, HexKind::Land | HexKind::Coast) {
                return Err(format!(
                    "{what} are at {q},{r}, which is sea: men are ashore"
                ));
            }
            (hex, None)
        }
        (None, Some(_), None) | (None, None, Some(_)) => {
            return Err(format!(
                "{what} give one of \"q\" and \"r\"; a hex takes both"
            ))
        }
        (None, None, None) => {
            return Err(format!(
                "{what} have no place: they take \"at_base\" or \"q\" and \"r\""
            ))
        }
    };
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedMen {
        side,
        name: spec.name.clone(),
        hex,
        base,
        men: spec.men,
        standing,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}

/// A believed intent as the scenario states it, checked (decision 0173, 7
/// and 9) as a `set_intent` command of its kind would be for the enemy's
/// side, and every base it names on the believing side's map.
pub(super) fn believed_intent(
    spec: &BelievedIntentSpec,
    index: usize,
    world: &World,
) -> Result<BelievedIntent, String> {
    let side: Side = spec.side.into();
    let what = format!("believed intent {} of the {} side", index + 1, side.name());
    let kind = spec.intent_kind(&what)?;
    let enemy = match side {
        Side::Allied => Side::Japanese,
        Side::Japanese => Side::Allied,
    };
    // Every base it names is on the believing side's map, read from that
    // side's own view: one of its own bases, or an enemy base it sees.
    let view = world.side_view(side);
    for raw in [kind.own_base(), kind.target()].into_iter().flatten() {
        let on_map = view.bases().any(|base| base.id == raw)
            || view.enemy_bases().any(|base| base.id == raw);
        if !on_map {
            return Err(format!(
                "{what} names base {}, which is not on the {} side's map",
                raw.raw(),
                side.name()
            ));
        }
    }
    // A kind that names a task force is refused before this (the
    // architect's ruling on reading 10). For the kinds left, the command's
    // check reads, of the enemy, only the side of a base it names, which the
    // believing side's map shows (checked above); of the believing side, the
    // `on_map` of its own base (a hold or target of it); and the map.
    debug_assert!(kind.force().is_none());
    world.check_root_intent(enemy, &kind).map_err(|refusal| {
        format!(
            "{what} could not be given to the {} side: {refusal}",
            enemy.name()
        )
    })?;
    let placeholder = spec.placeholder.unwrap_or(false);
    Ok(BelievedIntent {
        side,
        kind,
        placeholder,
        note: entry_note(placeholder, &spec.note, &what)?,
    })
}
