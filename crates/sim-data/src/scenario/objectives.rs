use crate::quantity::Quantity;
use crate::reference::parse_kind;
use serde::Deserialize;
use sim_core::fixed::Fixed;
use sim_core::ids::BaseId;
use sim_core::objectives::{Levels, Objective, Objectives, ShipPoints, SideObjectives};
use sim_core::refdata::ShipKind;
use std::collections::BTreeMap;

/// The `[objectives]` table (decision 0054, "As built"): the three levels,
/// an optional note of how the day went in history, and a table for each
/// side.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ObjectivesSpec {
    pub(super) minor: Quantity,
    pub(super) major: Quantity,
    pub(super) decisive: Quantity,
    /// The days the result is judged over (decision 0088): a whole number,
    /// at least the scenario's days. Defaults to the scenario's days.
    pub(super) horizon_days: Option<i64>,
    pub(super) history: Option<String>,
    /// Decision 0187, 22: `true` for a scenario for play, which has a scale
    /// of its own; absent or `false`, the scenario is of record.
    pub(super) for_play: Option<bool>,
    pub(super) allied: SideObjectivesSpec,
    pub(super) japanese: SideObjectivesSpec,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SideObjectivesSpec {
    pub(super) mission: String,
    #[serde(default)]
    pub(super) objective: Vec<ObjectiveSpec>,
}

/// One `[[objectives.<side>.objective]]`, by its `kind`.
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum ObjectiveSpec {
    /// `points` is a table from a ship kind (`carrier`, `battleship`,
    /// `cruiser`, `destroyer`, `transport`, `auxiliary`) to points.
    ShipsSunk {
        points: BTreeMap<String, Quantity>,
    },
    ShipsCrippled {
        points: BTreeMap<String, Quantity>,
    },
    /// Decision 0088: points for a ship of each kind away for the whole
    /// horizon.
    ShipsOutOfAction {
        points: BTreeMap<String, Quantity>,
    },
    AircraftDestroyed {
        points: Quantity,
    },
    CargoLanded {
        base: u32,
        points_per_t: Quantity,
    },
    BaseOpen {
        base: u32,
        points: Quantity,
        closed: Option<bool>,
    },
    /// Decision 0090, 25: points if the base is the side's at the end.
    BaseHeld {
        base: u32,
        points: Quantity,
    },
    /// Decision 0090, 26: points for each 100 enemy men lost.
    MenLost {
        points_per_100_men: Quantity,
    },
    /// Decision 0186: points for each 100 men the side's ships land in the
    /// base's hex and do not take off again.
    MenLanded {
        base: u32,
        points_per_100_men: Quantity,
    },
    /// Decision 0218, 32: points for each enemy submarine sunk.
    SubmarinesSunk {
        points: Quantity,
    },
}

pub(super) fn points_of(quantity: &Quantity, what: &str) -> Result<Fixed, String> {
    let points = quantity.to_fixed().map_err(|e| format!("{what} {e}"))?;
    if points.is_negative() {
        return Err(format!("{what} is negative"));
    }
    Ok(points)
}

pub(super) fn ship_points(
    table: &BTreeMap<String, Quantity>,
    what: &str,
) -> Result<ShipPoints, String> {
    let mut points = BTreeMap::new();
    for (name, quantity) in table {
        let kind: ShipKind =
            parse_kind(name).ok_or_else(|| format!("{what} has unknown ship kind \"{name}\""))?;
        points.insert(kind, points_of(quantity, &format!("{what} {name}"))?);
    }
    Ok(ShipPoints(points))
}

pub(super) fn build_objectives(
    spec: &ObjectivesSpec,
    days: u32,
    has_surface_combat: bool,
) -> Result<Objectives, String> {
    let horizon_days = match spec.horizon_days {
        None => days,
        Some(stated) => {
            let stated = u32::try_from(stated)
                .ok()
                .filter(|stated| *stated >= 1)
                .ok_or_else(|| {
                    format!("objectives horizon_days {stated} is not a whole number of at least 1")
                })?;
            if stated < days {
                return Err(format!(
                    "objectives horizon_days {stated} is shorter than the scenario's {days} days"
                ));
            }
            stated
        }
    };
    let uses = |wanted: &str| {
        [&spec.allied, &spec.japanese]
            .into_iter()
            .flat_map(|side| side.objective.iter())
            .any(|objective| {
                matches!(
                    (wanted, objective),
                    (
                        "ships_out_of_action",
                        ObjectiveSpec::ShipsOutOfAction { .. }
                    ) | ("ships_crippled", ObjectiveSpec::ShipsCrippled { .. })
                )
            })
    };
    if uses("ships_out_of_action") {
        if uses("ships_crippled") {
            return Err(
                "a scenario counts damaged ships one way: it may not have both ships_out_of_action and ships_crippled (decision 0088)"
                    .to_string(),
            );
        }
        if !has_surface_combat {
            return Err(
                "ships_out_of_action needs the surface combat figures, which hold the yard figures (decision 0088)"
                    .to_string(),
            );
        }
    }
    let level = |quantity: &Quantity, name: &str| {
        quantity
            .to_fixed()
            .map_err(|e| format!("objectives level {name} {e}"))
    };
    let (minor, major, decisive) = (
        level(&spec.minor, "minor")?,
        level(&spec.major, "major")?,
        level(&spec.decisive, "decisive")?,
    );
    let levels = Levels::new(minor, major, decisive).ok_or_else(|| {
        format!(
            "objectives levels must be above zero and rising, but minor is {minor}, major {major} and decisive {decisive}"
        )
    })?;
    let side = |name: &str, spec: &SideObjectivesSpec| -> Result<SideObjectives, String> {
        if spec.mission.trim().is_empty() {
            return Err(format!("the {name} mission is empty"));
        }
        let mut objectives = Vec::new();
        for (index, objective) in spec.objective.iter().enumerate() {
            let what = format!("{name} objective {}", index + 1);
            objectives.push(match objective {
                ObjectiveSpec::ShipsSunk { points } => {
                    Objective::ShipsSunk(ship_points(points, &what)?)
                }
                ObjectiveSpec::ShipsCrippled { points } => {
                    Objective::ShipsCrippled(ship_points(points, &what)?)
                }
                ObjectiveSpec::ShipsOutOfAction { points } => {
                    Objective::ShipsOutOfAction(ship_points(points, &what)?)
                }
                ObjectiveSpec::AircraftDestroyed { points } => Objective::AircraftDestroyed {
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::CargoLanded { base, points_per_t } => Objective::CargoLanded {
                    base: BaseId::new(*base),
                    points_per_t: points_of(points_per_t, &format!("{what} points_per_t"))?,
                },
                ObjectiveSpec::BaseOpen {
                    base,
                    points,
                    closed,
                } => Objective::BaseOpen {
                    base: BaseId::new(*base),
                    closed: closed.unwrap_or(false),
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::BaseHeld { base, points } => Objective::BaseHeld {
                    base: BaseId::new(*base),
                    points: points_of(points, &format!("{what} points"))?,
                },
                ObjectiveSpec::MenLost { points_per_100_men } => Objective::MenLost {
                    points_per_100_men: points_of(
                        points_per_100_men,
                        &format!("{what} points_per_100_men"),
                    )?,
                },
                ObjectiveSpec::MenLanded {
                    base,
                    points_per_100_men,
                } => Objective::MenLanded {
                    base: BaseId::new(*base),
                    points_per_100_men: points_of(
                        points_per_100_men,
                        &format!("{what} points_per_100_men"),
                    )?,
                },
                ObjectiveSpec::SubmarinesSunk { points } => Objective::SubmarinesSunk {
                    points: points_of(points, &format!("{what} points"))?,
                },
            });
        }
        Ok(SideObjectives {
            mission: spec.mission.clone(),
            objectives,
        })
    };
    Ok(Objectives {
        allied: side("allied", &spec.allied)?,
        japanese: side("japanese", &spec.japanese)?,
        levels,
        horizon_days,
        history: spec.history.clone(),
        for_play: spec.for_play.unwrap_or(false),
        // Filled by `World::set_objectives` from the scenario's bases.
        base_hexes: BTreeMap::new(),
    })
}
