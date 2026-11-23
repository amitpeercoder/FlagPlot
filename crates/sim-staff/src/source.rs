//! The staff's choice of a source of supply (decision 0228, as amended): one
//! rule for choosing, for a side whose scenario says its staff chooses its
//! sources, used under a commander's hold and under "control an island".
//!
//! 1. A base may be chosen where it is the side's, not in the hex of the
//!    place supplied, the staff may take it and it has supply to spare.
//! 2. Supply to spare: the base's supply above its reserve, and not more than
//!    its hex can spare (the hex's supply less the greater of the summed
//!    reserves of the side's bases there and fifteen days of the hex's
//!    planned use of supply today).
//! 3. The place's need is fifteen days of the planned use of the place's hex.
//! 4. The order: a base that can spare the place's need before one that
//!    cannot; one with a transport force the staff may take stopped there with
//!    nothing aboard before one with none; then the most to spare; then the
//!    nearer by the engine's sea path; then the lower number.
//!
//! It reads the side's own view and its planned command state, and draws no
//! random number.

use std::cmp::Reverse;
use std::collections::BTreeSet;

use sim_core::command::Intent;
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::orders::Cargo;
use sim_core::state::Base;
use sim_core::view::SideView;

use crate::air::Day;
use crate::planner::DEFAULT_INTERVAL_DAYS;
use crate::reasons::base_words;

/// Fifteen days, as a quantity.
fn interval() -> Fixed {
    Fixed::from_int(i64::from(DEFAULT_INTERVAL_DAYS))
}

/// The planned use of supply in `hex` today, in tons: 0 where the side uses
/// nothing there.
fn use_in(view: &SideView<'_>, hex: Hex) -> Fixed {
    view.days_of_supply(hex)
        .map_or(Fixed::ZERO, |figure| figure.planned_use.total_t())
}

/// What the hex of the side keeps (decision 0228, rule 2 as amended in D):
/// the greater of the summed supply reserves of the side's bases there and
/// fifteen days of the hex's planned use today.
pub(crate) fn hex_keeps(view: &SideView<'_>, hex: Hex) -> Fixed {
    let reserves = view
        .bases()
        .filter(|base| base.hex == hex)
        .fold(Fixed::ZERO, |sum, base| {
            sum + base.reserve_of(Cargo::Supply)
        });
    reserves.max(use_in(view, hex) * interval())
}

/// What the hex of the side can spare: its supply less what it keeps, not
/// below 0.
fn hex_spare(view: &SideView<'_>, hex: Hex) -> Fixed {
    let stock = view
        .days_of_supply(hex)
        .map_or(Fixed::ZERO, |figure| figure.stock_t);
    (stock - hex_keeps(view, hex)).max(Fixed::ZERO)
}

/// What the base can spare: the lesser of its own supply above its own
/// reserve and what its hex can spare.
pub(crate) fn base_spare(view: &SideView<'_>, base: &Base) -> Fixed {
    base.above_reserve(Cargo::Supply)
        .min(hex_spare(view, base.hex))
}

/// The place's need: fifteen days of the planned use of its hex today.
pub(crate) fn place_need(view: &SideView<'_>, hex: Hex) -> Fixed {
    use_in(view, hex) * interval()
}

/// Decision 0228, rule 17: the words that say, of a source the staff chose
/// that has nothing to spare, which other bases of the side may be chosen and
/// what each can spare: those not in the hex of the place and not before their
/// day. (A base that is the source of an order higher in priority is named
/// too: the reason of the day has no way to the other orders' sources.)
pub(crate) fn other_sources_words(view: &SideView<'_>, source: BaseId, place: Hex) -> String {
    let mut others: Vec<(BaseId, Fixed)> = view
        .bases()
        .filter(|base| base.id != source && base.hex != place)
        .filter(|base| {
            crate::available::not_before(view, sim_core::ids::EntityRef::Base(base.id)).is_none()
        })
        .map(|base| (base.id, base_spare(view, base)))
        .filter(|(_, spare)| *spare > Fixed::ZERO)
        .collect();
    others.sort_by_key(|(id, _)| *id);
    if others.is_empty() {
        " The staff chose the source, and no other base of the side can spare supply.".to_string()
    } else {
        let list = others
            .iter()
            .map(|(id, spare)| format!("{}, {spare} t to spare", base_words(view, *id)))
            .collect::<Vec<_>>()
            .join("; ");
        format!(" The staff chose the source; other bases of the side that may be chosen: {list}.")
    }
}

/// One base the staff may choose, as it is weighed.
#[derive(Clone, Debug)]
pub(crate) struct Weighed {
    pub(crate) base: BaseId,
    pub(crate) spare: Fixed,
    pub(crate) covers: bool,
    /// The transport forces the staff may take stopped there with nothing
    /// aboard.
    pub(crate) forces: BTreeSet<TaskForceId>,
    pub(crate) nm: Option<u32>,
    pub(crate) words: String,
}

/// The bases weighed for one place.
pub(crate) struct Weighing {
    /// The bases that may be chosen, in the order of rule 4: the first is
    /// the choice.
    pub(crate) chosen: Vec<Weighed>,
    /// Each base the staff passed over, in base order, with the words why.
    pub(crate) passed: Vec<String>,
}

impl Weighing {
    /// The words of every base that may be chosen, in base order, then of
    /// every base passed over.
    pub(crate) fn lines(&self) -> Vec<String> {
        let mut weighed: Vec<&Weighed> = self.chosen.iter().collect();
        weighed.sort_by_key(|w| w.base);
        weighed
            .into_iter()
            .map(|w| w.words.clone())
            .chain(self.passed.iter().cloned())
            .collect()
    }
}

fn forces_words(forces: &BTreeSet<TaskForceId>) -> String {
    if forces.is_empty() {
        "no transport force stopped there with nothing aboard".to_string()
    } else {
        format!(
            "{} stopped there with nothing aboard",
            forces
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Day<'_, '_> {
    /// Rules 1 to 4: the bases of the side that may be chosen as the source
    /// of the supply for the place at `place`, in the order the staff takes
    /// them. `end` is the intent whose chain the staff may not take a base
    /// from; `excluded` the hexes of the places supplied, where no source is;
    /// `chain` the hold whose commander's limits on ships bind the pool.
    pub(crate) fn weigh_sources(
        &self,
        end: &Intent,
        excluded: &BTreeSet<Hex>,
        place: Hex,
        chain: Option<IntentId>,
    ) -> Weighing {
        let need = place_need(self.view, place);
        let others = self.others_above(end);
        let mut bases: Vec<Base> = self
            .view
            .bases()
            .filter(|base| !excluded.contains(&base.hex))
            .cloned()
            .collect();
        bases.sort_by_key(|base| base.id);
        let mut chosen = Vec::new();
        let mut passed = Vec::new();
        for base in bases {
            let text = base_words(self.view, base.id);
            if let Err(why) = self.may_take_base(base.id, &others) {
                passed.push(format!("{text}: {why}"));
                continue;
            }
            let spare = base_spare(self.view, &base);
            if spare <= Fixed::ZERO {
                if base.supply_t > Fixed::ZERO {
                    passed.push(format!(
                        "{text} holds {} t and keeps {} t, so it has nothing to spare",
                        base.supply_t,
                        base.supply_t - spare
                    ));
                }
                continue;
            }
            let mut forces = self.pool_at(base.id, &others);
            if let Some(hold) = chain {
                forces.retain(|force| self.chain_allows_force(hold, *force));
            }
            let nm = self.view.sea_charged_nm(base.hex, place);
            let covers = spare >= need;
            let against = if covers {
                format!("enough for the place's need of {need} t")
            } else {
                format!("less than the place's need of {need} t")
            };
            let distance = nm.map_or_else(
                || "no sea path to the place".to_string(),
                |nm| format!("{nm} nm from the place"),
            );
            let words = format!(
                "{text}: {spare} t to spare, {against}, {}, {distance}",
                forces_words(&forces)
            );
            chosen.push(Weighed {
                base: base.id,
                spare,
                covers,
                forces,
                nm,
                words,
            });
        }
        chosen.sort_by_key(|w| {
            (
                Reverse(w.covers),
                Reverse(!w.forces.is_empty()),
                Reverse(w.spare),
                w.nm.unwrap_or(u32::MAX),
                w.base,
            )
        });
        Weighing { chosen, passed }
    }

    /// Whether the forces limits of the commander's on `hold`'s chain that
    /// bind ships allow `force`: with none binding, any force.
    pub(crate) fn chain_allows_force(&self, hold: IntentId, force: TaskForceId) -> bool {
        self.state
            .ship_limits_on_chain(hold)
            .iter()
            .all(|limit| limit.task_forces.contains(&force))
    }
}
