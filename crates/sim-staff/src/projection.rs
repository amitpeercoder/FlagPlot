//! The staff's projection of days of supply in a hex (decision 0107, 35 and
//! "As built (step 1, the staff)").
//!
//! Today is the view's day plus one. The reading of today is the view's own
//! stock; the reading of day d + 1 is the reading of day d, less the planned
//! use of day d, plus what lands on day d, and not below 0. The planned use
//! of a coming day is today's, with the men due counted from their due day
//! and the aircraft of a scheduled arrival from its day. What lands: for
//! each run of an approved plan of the side not yet finished, its load while
//! it has not sailed, else the supply aboard its ships; it lands on the
//! run's arrival day plus one, or today if that is past, at the hex it will
//! land (ruling 28): a run loading at the intent's base as it now stands, a
//! run under way at the first hex of its force's route still ahead that is
//! marked to act at, and nowhere once that is passed; with no mark at all,
//! at the intent's base.
//!
//! The side's supply lines (decision 0196, 10) are counted too: a line to a
//! base of the side in the hex adds its supply tons a day to the landings of
//! each day it gives on, before that day's use, as a run's landing is. Fuel
//! is not projected here, so a line's fuel is not counted.
//!
//! Everything is whole numbers of tons and days in `Fixed`.

use sim_core::command::{CommandState, IntentKind, ProjectedReading, SupplyProjection};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::state::AirGroupLocation;
use sim_core::view::SideView;

use crate::fleet::{run_state, Fleet, RunStatus};
use crate::morning::Morning;
use crate::units::{over, whole};

/// The side's supply lines to a base of its own in `hex`.
fn own_lines_in<'a>(
    view: &'a SideView<'a>,
    hex: Hex,
) -> impl Iterator<Item = &'a sim_core::state::SupplyLine> + 'a {
    view.supply_lines()
        .filter(move |line| view.bases().any(|b| b.id == line.base && b.hex == hex))
}

/// The side's supply lines to its own base `base`.
pub(crate) fn own_lines_to<'a>(
    view: &'a SideView<'a>,
    base: sim_core::ids::BaseId,
) -> Vec<sim_core::state::SupplyLine> {
    let own = view.bases().any(|b| b.id == base);
    if !own {
        return Vec::new();
    }
    view.supply_lines()
        .filter(|l| l.base == base)
        .copied()
        .collect()
}

/// The clause that names the lines to `base` beside "holds X and uses Y a
/// day", empty where the side has none.
pub(crate) fn line_clause(view: &SideView<'_>, base: sim_core::ids::BaseId) -> String {
    let mut out = String::new();
    for line in own_lines_to(view, base) {
        let gives = match (line.supply_t > Fixed::ZERO, line.fuel_t > Fixed::ZERO) {
            (true, true) => format!(
                "{} of supply and {} of fuel",
                crate::units::tons(line.supply_t),
                crate::units::tons(line.fuel_t)
            ),
            (false, true) => format!("{} of fuel", crate::units::tons(line.fuel_t)),
            _ => format!("{} of supply", crate::units::tons(line.supply_t)),
        };
        out.push_str(&format!(
            " and is given {gives} a day by a supply line from day {} to day {}",
            line.first_day, line.last_day
        ));
    }
    out
}

/// The stock, the planned use and the landings of each day from today on.
#[derive(Clone, Debug)]
pub(crate) struct Readings {
    pub(crate) today: u32,
    stock0: Fixed,
    use_t: Vec<Fixed>,
    landing_t: Vec<Fixed>,
    stocks: Vec<Fixed>,
}

impl Readings {
    /// The projection of the side's stock in `hex` over `horizon_days` days
    /// after today, with the landings of its runs counted. `None` where the
    /// side holds no base in the hex.
    pub(crate) fn build(
        view: &SideView<'_>,
        fleet: &Fleet<'_>,
        state: &CommandState,
        hex: Hex,
        horizon_days: u32,
    ) -> Option<Readings> {
        let figure = view.days_of_supply(hex)?;
        let today = view.day() + 1;
        let len = horizon_days as usize + 1;
        let mut use_t = vec![figure.planned_use.total_t(); len];

        // Men due on a coming day are fed from that day.
        for (day, charge) in view.men_due_by_day(hex) {
            if day > today {
                let first = (day - today) as usize;
                for slot in use_t.iter_mut().skip(first) {
                    *slot += charge;
                }
            }
        }

        // Aircraft the scenario has scheduled to arrive, and replacements,
        // are paid for from the day they come.
        if let Some(supply) = view.air_supply_figures() {
            let upkeep = supply.daily_supply_t_per_aircraft;
            let hex_of_group = |group: &sim_core::state::AirGroup| match group.location {
                AirGroupLocation::Base(base) => view.bases().find(|b| b.id == base).map(|b| b.hex),
                _ => None,
            };
            for arrival in view.arrivals() {
                if arrival.day < today && !arrival.waiting {
                    continue;
                }
                let Some(group) = view.air_groups().find(|g| g.id == arrival.air_group) else {
                    continue;
                };
                if hex_of_group(group) != Some(hex) {
                    continue;
                }
                let first = arrival.day.max(today) - today;
                for slot in use_t.iter_mut().skip(first as usize) {
                    *slot += upkeep * whole(arrival.aircraft);
                }
            }
            for stream in view.replacements() {
                let at_hex = view
                    .bases()
                    .find(|b| b.id == stream.base)
                    .is_some_and(|b| b.hex == hex);
                if !at_hex {
                    continue;
                }
                for (i, slot) in use_t.iter_mut().enumerate() {
                    let days = u32::try_from(i + 1).unwrap_or(u32::MAX);
                    *slot += upkeep * whole(stream.aircraft_per_day) * whole(days);
                }
            }
        }

        let mut landing_t = vec![Fixed::ZERO; len];
        for (day, tons) in run_landings(view, fleet, state, hex) {
            let index = day.saturating_sub(today) as usize;
            if index < len {
                landing_t[index] += tons;
            }
        }
        for line in own_lines_in(view, hex) {
            for (i, slot) in landing_t.iter_mut().enumerate() {
                let day = today + u32::try_from(i).unwrap_or(0);
                if line.gives_on(day) {
                    *slot += line.supply_t;
                }
            }
        }
        let mut readings = Readings {
            today,
            stock0: figure.stock_t,
            use_t,
            landing_t,
            stocks: Vec::new(),
        };
        readings.recompute();
        Some(readings)
    }

    fn recompute(&mut self) {
        let len = self.use_t.len();
        let mut stocks = Vec::with_capacity(len);
        let mut stock = self.stock0;
        for i in 0..len {
            stocks.push(stock);
            stock = (stock - self.use_t[i] + self.landing_t[i]).max(Fixed::ZERO);
        }
        self.stocks = stocks;
    }

    /// Counts `tons` as landing on `day`, as a run's load is counted.
    pub(crate) fn add_landing(&mut self, day: u32, tons: Fixed) {
        let index = day.saturating_sub(self.today) as usize;
        if index < self.landing_t.len() {
            self.landing_t[index] += tons;
            self.recompute();
        }
    }

    /// The last day the projection reaches.
    pub(crate) fn last_day(&self) -> u32 {
        self.today + u32::try_from(self.use_t.len().saturating_sub(1)).unwrap_or(0)
    }

    fn index(&self, day: u32) -> Option<usize> {
        let i = day.checked_sub(self.today)? as usize;
        (i < self.use_t.len()).then_some(i)
    }

    /// The stock at the start of `day`, if the projection reaches it.
    pub(crate) fn stock(&self, day: u32) -> Option<Fixed> {
        self.index(day).map(|i| self.stocks[i])
    }

    /// The planned use of `day`, if the projection reaches it.
    pub(crate) fn use_on(&self, day: u32) -> Option<Fixed> {
        self.index(day).map(|i| self.use_t[i])
    }

    /// The figure of `day`: the stock over the use, none where the use is 0.
    pub(crate) fn days(&self, day: u32) -> Option<Fixed> {
        let i = self.index(day)?;
        (!self.use_t[i].is_zero()).then(|| self.stocks[i] / self.use_t[i])
    }

    /// The planned use of the `count` days from `from` on, the last known
    /// use standing for any day beyond the projection.
    pub(crate) fn use_over(&self, from: u32, count: u32) -> Fixed {
        let last = self.use_t.last().copied().unwrap_or(Fixed::ZERO);
        (0..count).fold(Fixed::ZERO, |sum, k| {
            sum + self.use_on(from + k).unwrap_or(last)
        })
    }

    /// Whether the figure of `day` is below `floor_days`. `None` where there
    /// is no figure or the day is out of reach.
    pub(crate) fn below_floor(&self, day: u32, floor_days: u32) -> Option<bool> {
        self.days(day).map(|days| days < whole(floor_days))
    }

    /// The readings in the form the proposal carries.
    pub(crate) fn projection(&self, hex: Hex, floor_days: u32) -> SupplyProjection {
        let readings: Vec<ProjectedReading> = (0..self.use_t.len())
            .map(|i| {
                let day = self.today + u32::try_from(i).unwrap_or(0);
                ProjectedReading {
                    day,
                    stock_t: self.stocks[i],
                    use_t: self.use_t[i],
                    days: (!self.use_t[i].is_zero()).then(|| over(self.stocks[i], self.use_t[i])),
                }
            })
            .collect();
        let first_day_below = readings
            .iter()
            .find(|reading| reading.days.is_some_and(|days| days < whole(floor_days)))
            .map(|reading| reading.day);
        SupplyProjection {
            hex,
            floor_days,
            readings,
            first_day_below,
        }
    }
}

/// What the side's runs will land in `hex`: for each run of an approved plan
/// not finished, its load while it has not sailed, else the supply aboard
/// its ships, on the run's arrival day plus one or today if that is past.
fn run_landings(
    view: &SideView<'_>,
    fleet: &Fleet<'_>,
    state: &CommandState,
    hex: Hex,
) -> Vec<(u32, Fixed)> {
    let today = view.day() + 1;
    let mut landings = Vec::new();
    for plan in state.plans.values().filter(|plan| plan.state.approved()) {
        // The runs of a plan are its supply intent's: the plan's own intent,
        // or the "keep supplied" the staff formed in the plan under a hold
        // (decision 0144, 7).
        let supply_base = |intent: &sim_core::command::Intent| match intent.kind {
            IntentKind::KeepSupplied { base, .. } => Some(base),
            _ => None,
        };
        let Some(base) = state
            .intents
            .get(&plan.intent)
            .and_then(supply_base)
            .or_else(|| {
                state
                    .intents_in_force()
                    .filter(|i| i.plan == plan.id)
                    .find_map(supply_base)
            })
        else {
            continue;
        };
        let Some(destination) = view.bases().find(|b| b.id == base) else {
            continue;
        };
        for run in plan.runs.values() {
            let Some(source) = view.bases().find(|b| b.id == run.source) else {
                continue;
            };
            let found = run_state(fleet, state, plan.id, run, source.hex, today);
            let (tons, lands_at) = match found.status {
                RunStatus::Finished => continue,
                RunStatus::Loading => (run.load_t, Some(destination.hex)),
                RunStatus::Underway => (
                    found.aboard_t,
                    underway_landing(fleet, &found.afloat, destination.hex),
                ),
            };
            if lands_at == Some(hex) {
                landings.push(((run.days.arrive + 1).max(today), tons));
            }
        }
    }
    landings
}

/// The hex a run under way will land: the first hex of its force's route
/// still ahead that is marked to act at, and none once that is passed; with
/// no mark at all, the intent's base (`base_hex`).
fn underway_landing(
    fleet: &Fleet<'_>,
    afloat: &[sim_core::ids::ShipId],
    base_hex: Hex,
) -> Option<Hex> {
    let Some(force) = afloat.iter().find_map(|ship| fleet.force_of(*ship)) else {
        return Some(base_hex);
    };
    let route = &force.route;
    if route.act_at.is_empty() {
        return Some(base_hex);
    }
    route
        .waypoints
        .get(route.next_index.min(route.waypoints.len())..)
        .unwrap_or(&[])
        .iter()
        .find(|hex| route.act_at.contains(hex))
        .copied()
}

/// The staff's projection of the stock in `hex` over the next `horizon_days`
/// days, from the view alone: the readings from today, each with its stock,
/// planned use and figure, and the first day the figure falls below
/// `floor_days`. It counts the side's own runs to the hex as the staff does.
/// `None` where the side holds no base in the hex.
pub fn project(
    view: &SideView<'_>,
    hex: Hex,
    floor_days: u32,
    horizon_days: u32,
) -> Option<SupplyProjection> {
    let fleet = Fleet::of(view);
    let morning = Morning::begin(view, &fleet);
    let readings = Readings::build(view, &fleet, morning.state(), hex, horizon_days)?;
    Some(readings.projection(hex, floor_days))
}
