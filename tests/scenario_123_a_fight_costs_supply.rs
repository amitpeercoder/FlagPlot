//! Scenario 123: a fight costs supply. A day's fight charges each side's
//! bases in the hex for its men at grips, ammunition first and then
//! supply, and a side that cannot be supplied fires short.
//!
//! Proves decision 0209. The statements are the architect's, written
//! before the code. A mismatch goes back to the architect, and nobody else
//! changes a statement or a number here.
//!
//! # The setting
//!
//! Scenario 122's, in a file of its own: a placeholder scenario on a
//! placeholder map, with no real unit. Its ground figures are in a file of
//! its own, the frozen step 1 figures with eight added: the three of
//! decision 0205 (`rations_landed_nights` 3, `hungry_below_share` 0.6,
//! `hunger_nights_before_loss` 14), the two of decision 0208
//! (`hunger_weakness_per_night` 0.01, `unfed_fighting_share` 0.9), and
//! `fight_supply_t_per_man` 0.01, `unsupplied_fire_share` 0.25 and
//! `fight_days_landed` 4. In the frozen figures the daily supply for a
//! man is 0.005 t, `men_per_shot` is 100, the limit on engaged men is out
//! of reach (`engagement_ratio` 1,000,000) and no unit's weight or skill
//! changes its shots. Every land unit is standard, with no fatigue.
//!
//! Side A has a base Q, with no aircraft, and a land unit V of 1,000 men
//! ashore there, on defence. The men's charge at Q is 5 t a night. Q's
//! supply and ammunition at the start are what each statement gives; the
//! ammunition is nothing unless it says. Side B has a land unit E of
//! 1,000 men in Q's hex, set to attack, with no base there.
//!
//! The first night comes before the first fight, as in scenario 122. On
//! it V is fed in full wherever Q starts with 5 t or more, and E is fed
//! nothing: E's count is 1, its condition 0.99, its share from the meal
//! 0.9, and its firing men are 1,000 times 0.891, 891. V's firing men are
//! 1,000. Every statement below is of the first fight, on day 1, unless
//! it says otherwise.
//!
//! "Shots" are a unit's shots as its fire records them, after the limit
//! on engaged men and after this rule.
//!
//! # Statements: the charge
//!
//! 1. **Supplied in full.** Q starts with 20 t of supply. The night takes
//!    5 and leaves 15. V's men at grips are 1,000 and its want is 1,000
//!    times 0.01, 10 t. All of it is charged, from supply: Q has 5 t
//!    after the fight's charge. The supplied share is 1 and the fire share
//!    1, and V's shots are 1,000 over 100, 10.
//! 2. **Supplied by half.** Q starts with 10 t. The night leaves 5. The
//!    want is 10 t and 5 are charged; Q has nothing. The supplied share
//!    is 0.5, the fire share 0.25 plus 0.75 times 0.5, 0.625, and V's
//!    shots are 6.25. V's firing men are still 1,000.
//! 3. **Nothing to fire with.** Q starts with 5 t. The night takes it all
//!    and V is fed in full. The want is 10 t and nothing is charged. The
//!    supplied share is 0, the fire share 0.25, and V's shots are 2.5.
//!    The charge is still recorded for A: 10 t wanted, none charged.
//! 4. **A side with no base.** In each of statements 1 to 3, E's men at
//!    grips are 891 and its want 8.91 t. B has no base in the hex, so
//!    nothing is charged. E's fire share is 0.25 and its shots are 8.91
//!    times 0.25, 2.2275.
//! 5. **Ammunition first.** Q starts with 20 t of supply and 6 t of
//!    ammunition. The night leaves 15 t of supply. The want of 10 t takes
//!    the 6 t of ammunition and 4 t of supply: Q has no ammunition and 11
//!    t of supply. The supplied share is 1 and V's shots are 10. The
//!    record gives 6 t from ammunition and 4 t from supply.
//! 6. **Ammunition alone.** Q starts with 5 t of supply and 4 t of
//!    ammunition. The night takes the supply. The want is 10 t and 4 are
//!    charged, from ammunition. The supplied share is 0.4, the fire share
//!    0.25 plus 0.75 times 0.4, 0.55, and V's shots are 5.5.
//! 7. **No fight, no charge.** With E not in the scenario, Q starting
//!    with 20 t of supply and 6 t of ammunition has 15 t and 6 t after
//!    day 1, and 10 t and 6 t after day 2. Nothing is recorded of a
//!    fight's charge.
//!
//! # Statements: the limit on engaged men
//!
//! The same scenario with V of 500 men and Q starting with 17.5 t of
//! supply, on a ground file whose `engagement_ratio` is 1, the least the
//! game takes. The night takes 2.5 t and leaves 15. V's firing men are
//! 500 and E's 891. (Corrected by the architect on Nov 22, 2026: as first
//! written the ratio was 0.5, which no ground file may give, decision
//! 0105.)
//!
//! 8. **A side inside its limit pays for all who fire.** The limit for A
//!    is E's fighting men, above V's 500 firing men: A's men at grips are
//!    500 and its want 5 t. All is charged and Q has 10 t after the
//!    fight's charge. V's shots are 5.
//! 9. **Only the men at grips are paid for, and the cut is on the shots
//!    after the limit.** The limit for B is V's 500 fighting men, below
//!    E's 891 firing men: B's men at grips are 500 and its want 5 t, of
//!    which nothing is charged. E's shots are 8.91 times 500 over 891, 5,
//!    times 0.25: 1.25.
//!
//! # Statements: what a landing brings
//!
//! Scenario 119's landing: side A's force T with a land unit U of 3,000
//! men aboard, and a hex P where A has no base. No enemy is at P.
//!
//! 10. **Days of fighting, as ammunition.** When T lands its first man at
//!     P the base formed has 45 t of supply, the rations of decision
//!     0205, and 3,000 times 4 times 0.01 t, 120 t, of ammunition. The
//!     record of the ammunition names 3,000 men, 4 days and 120 t, and
//!     cites what the rations' record cites.
//! 11. **Every landing brings them.** A second force with 1,000 men
//!     landing at P on a later day brings no rations and 1,000 times 4
//!     times 0.01 t, 40 t, of ammunition: P has 160 t of it.
//! 12. **Men do not eat it.** With all of U ashore at P and nothing else
//!     landed, the three nights after the landing are fed in full and the
//!     fourth is not; P's ammunition is 120 t throughout.
//!
//! # Statements: a dry run, and where it runs
//!
//! 13. **A side a dry run makes up is taken as supplied.** With the
//!     world's setting that takes B as supplied, in statement 1: E's fire
//!     share is 1 and its shots 8.91, nothing is charged to B and no
//!     charge is recorded for B. A is charged as in statement 1.
//! 14. **Without the three figures,** on this scenario's ground file
//!     with the three taken out, in statement 1's setting (corrected by
//!     the architect on Nov 22, 2026: scenario 122's file, as first
//!     written, has another `unfed_fighting_share`): V's shots are 10 and E's 8.91, Q has 15 t
//!     after the fight, nothing is recorded of a fight's charge, and the
//!     landing of statement 10 brings 45 t of supply and no ammunition.
//! 15. **The loader.** A ground file with one or two of the three figures
//!     and not all is refused. One with the three and without the three
//!     of decision 0205 is refused. One with the three and the three of
//!     0205 and without the two of 0208 is taken. A
//!     `fight_supply_t_per_man` or `fight_days_landed` below 0 is
//!     refused, and an `unsupplied_fire_share` below 0 or above 1.
//! 16. **The files.** `data/reference/ground_hunger.toml` gives
//!     `fight_supply_t_per_man` 0.01, `unsupplied_fire_share` 0.25 and
//!     `fight_days_landed` 4, and no figure of it that the live
//!     `ground.toml` has differs from it. Its frozen counterpart gives
//!     the three with the same values. Scenario 119's and scenario 122's
//!     own files are as they were.
//!
//! # Statements: what is recorded and who knows
//!
//! 17. **The order and the causes.** In statement 5 the record of A's
//!     charge comes after the fight's event and before any unit's fire,
//!     with Q's ammunition spent and Q's supply spent, each citing the
//!     charge; V's fire cites the charge. A's comes before B's.
//! 18. **Each side sees its own.** In statement 2, A is shown its charge
//!     and, in V's fire, a supplied share of 0.5 and a fire share of
//!     0.625. B is shown none of A's charge, A's stocks spent or those
//!     factors, and A none of B's.
//! 19. **The side's own count of firing men is not cut.** In statement 3,
//!     A is told that 1,000 of V would fire.
//!
//! # Properties
//!
//! - **No proof and no recorded game moves.** Every scenario and recorded
//!   game on file but the table's copies gives the events and the daily
//!   hashes it gave before.
//! - **No new state.** The state's layout version is what it was.
//! - **Nothing is drawn by the rule.** The same game played twice gives
//!   the same events, and in statement 14's setting the dice of the fight
//!   fall as they did before the change.
//! - **Stocks never go below nothing,** and what is charged in a fight is
//!   never more than what was wanted.

// The architect's statement lists are list items in the doc comment above,
// which is left as written.
#![allow(clippy::doc_lazy_continuation)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use sim_agent::{Seat, Session};
use sim_core::events::{Cause, Event, EventKind, SupplyUse};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, EventId, LandUnitId};
use sim_core::side_events::SideEventKind;
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_core::Staffs;
use sim_data::{LoadOptions, Scenario};

mod common;

const P: Hex = Hex::new(8, 0);
const Q: Hex = Hex::new(10, 0);
const V: u32 = 1;
const E: u32 = 2;

fn path() -> PathBuf {
    common::data_dir().join("scenarios/a-fight-costs-supply/123_a_fight_costs_supply.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    changed(text, &mark, &format!("{what}{mark}"))
}

fn fx(text: &str) -> Fixed {
    text.parse::<Fixed>().expect("a decimal figure")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn unit(raw: u32) -> LandUnitId {
    LandUnitId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

// ---------------------------------------------------------------- Variants

/// Q's stocks at the start, in tons.
fn with_stocks(text: &str, supply: &str, ammunition: &str) -> String {
    let text = changed(
        text,
        "supply_t = 0\n",
        &format!("supply_t = \"{supply}\"\n"),
    );
    changed(
        &text,
        "ammunition_t = 0\n",
        &format!("ammunition_t = \"{ammunition}\"\n"),
    )
}

fn with_days(text: &str, days: u32) -> String {
    changed(text, "days = 16\n", &format!("days = {days}\n"))
}

/// Side B's unit E of 1,000 men in Q's hex, set to attack, with no base of B
/// there.
fn with_e(text: &str) -> String {
    at(
        text,
        "units",
        &format!(
            "[[land_unit]]\nid = {E}\nname = \"Placeholder E\"\nside = \"japanese\"\nmen = 1000\nq = 10\nr = 0\nposture = \"attack\"\n\n"
        ),
    )
}

/// The setting of statements 1 to 6: Q's stocks, E attacking, two days.
fn fight(supply: &str, ammunition: &str) -> String {
    with_days(&with_e(&with_stocks(&text(), supply, ammunition)), 2)
}

/// A force of `ships` placeholder troop transports (class 10), the first
/// numbered `first_ship`, at (q,0).
fn with_force(text: &str, tf: u32, first_ship: u32, ships: u32, q: i32) -> String {
    let mut block = format!(
        "[[task_force]]\nid = {tf}\nname = \"Placeholder force {tf}\"\nside = \"allied\"\nq = {q}\nr = 0\nmission = \"transport\"\n\n"
    );
    for i in 0..ships {
        let id = first_ship + i;
        block.push_str(&format!(
            "[[ship]]\nid = {id}\nname = \"Placeholder troop transport {id}\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = {tf}\n\n"
        ));
    }
    at(text, "forces", &block)
}

/// A land unit aboard a force.
fn with_unit_aboard(text: &str, id: u32, men: u32, tf: u32) -> String {
    at(
        text,
        "forces",
        &format!(
            "[[land_unit]]\nid = {id}\nname = \"Placeholder unit {id}\"\nside = \"allied\"\nmen = {men}\ntask_force = {tf}\n\n"
        ),
    )
}

/// The orders on `day` for a force: route to (dest,0) at cruise and unload.
fn with_go(text: &str, tf: u32, day: u32, dest: i32) -> String {
    let order = |kind: &str, extra: &str| {
        format!(
            "[[order]]\nday = {day}\nside = \"allied\"\ntask_force = {tf}\nkind = \"{kind}\"\n{extra}\n"
        )
    };
    at(
        text,
        "orders",
        &format!(
            "{}{}{}",
            order("set_route", &format!("waypoints = [[{dest}, 0]]\n")),
            order("set_speed", "speed = \"cruise\"\n"),
            order("unload", "")
        ),
    )
}

/// T and U of the statements of landing: T lands U at P in the night of day 1.
fn with_t_and_u(text: &str) -> String {
    let text = with_force(text, 1, 1, 5, 7);
    let text = with_unit_aboard(&text, 2, 3000, 1);
    with_go(&text, 1, 1, 8)
}

/// The landing of statement 10, `days` days.
fn landing(days: u32) -> String {
    with_days(&with_t_and_u(&text()), days)
}

// ---------------------------------------------------------------- The figures

const THREE_LINES: [&str; 3] = [
    "fight_supply_t_per_man = \"0.01\"\n",
    "unsupplied_fire_share = \"0.25\"\n",
    "fight_days_landed = 4\n",
];

const OWN_GROUND: &str = "ground_placeholder_scenario_123.toml";

/// The scenario's ground figures without the three of decision 0209.
fn without_the_three() -> LoadOptions {
    common::altered_reference("no_fight_supply", |root| {
        common::edit_file_by(
            root,
            OWN_GROUND,
            &[
                (THREE_LINES[0], ""),
                (THREE_LINES[1], ""),
                (THREE_LINES[2], ""),
            ],
        );
    })
}

/// The scenario's ground figures with the limit on engaged men at 1, the
/// least the loader takes.
fn engagement_one() -> LoadOptions {
    common::altered_reference("engagement_one", |root| {
        common::edit_file_by(
            root,
            OWN_GROUND,
            &[("engagement_ratio = 1000000\n", "engagement_ratio = 1\n")],
        );
    })
}

// ---------------------------------------------------------------- Playing

struct Played {
    events: Vec<Event>,
    ends: Vec<TruthState>,
    /// The truth at the start of each pulse.
    pulses: BTreeMap<(u32, Pulse), TruthState>,
    /// `SideView::firing_men` of every land unit at the start of each
    /// day's morning pulse, as its own side sees it.
    firing: BTreeMap<(u32, u32), u32>,
    scenario: Scenario,
}

fn load_with(text: &str, options: &LoadOptions) -> Scenario {
    common::from_text_with(text, &path(), options).expect("the scenario loads")
}

fn play(text: &str) -> Played {
    play_with(text, &common::frozen())
}

fn play_with(text: &str, options: &LoadOptions) -> Played {
    play_scenario(load_with(text, options))
}

fn play_scenario(mut scenario: Scenario) -> Played {
    let mut ends = vec![scenario.world.umpire().clone()];
    let mut pulses = BTreeMap::new();
    let mut firing = BTreeMap::new();
    loop {
        let summary = scenario
            .next_day_watched(Staffs::none(), &mut |world, moment| {
                if let sim_core::choices::Moment::PulseBegins { day, pulse } = moment {
                    pulses.insert((day, pulse), world.umpire().clone());
                    if pulse == Pulse::Morning {
                        for (id, land) in &world.umpire().land_units {
                            if let Some(men) = world.side_view(land.side).firing_men(*id) {
                                firing.insert((day, id.raw()), men);
                            }
                        }
                    }
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        ends.push(scenario.world.umpire().clone());
    }
    Played {
        events: scenario.world.events().iter().cloned().collect(),
        ends,
        pulses,
        firing,
        scenario,
    }
}

/// The ids an event cites as causes.
fn cited(event: &Event) -> Vec<EventId> {
    event
        .causes
        .iter()
        .filter_map(|c| match c {
            Cause::Event(id) => Some(*id),
            Cause::Order(_) => None,
        })
        .collect()
}

fn factor(event: &Event, name: &str) -> Fixed {
    event
        .breakdown
        .as_ref()
        .expect("a breakdown")
        .factors
        .iter()
        .find(|f| f.name == name)
        .unwrap_or_else(|| panic!("a factor {name} in {event:?}"))
        .value
}

fn has_factor(event: &Event, name: &str) -> bool {
    event
        .breakdown
        .as_ref()
        .is_some_and(|b| b.factors.iter().any(|f| f.name == name))
}

/// A `FightSupplyCharged` as its figures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Charge {
    side: Side,
    men_at_grips: Fixed,
    wanted: Fixed,
    ammunition: Fixed,
    supply: Fixed,
    share: Fixed,
}

impl Played {
    fn end(&self, day: usize) -> &TruthState {
        &self.ends[day]
    }

    fn before(&self, day: u32, pulse: Pulse) -> &TruthState {
        &self.pulses[&(day, pulse)]
    }

    /// Q's supply and ammunition at the end of `day`.
    fn q(&self, day: usize) -> (Fixed, Fixed) {
        let q = &self.end(day).bases[&base(1)];
        (q.supply_t, q.ammunition_t)
    }

    /// The `GroundFire` of unit `u` on `day`.
    fn fire(&self, u: u32, day: u32) -> &Event {
        self.events
            .iter()
            .find(|e| {
                e.day == day
                    && matches!(&e.kind, EventKind::GroundFire { unit: who, .. } if *who == unit(u))
            })
            .unwrap_or_else(|| panic!("a fire of unit {u} on day {day}"))
    }

    fn shots(&self, u: u32, day: u32) -> Fixed {
        match &self.fire(u, day).kind {
            EventKind::GroundFire { shots, .. } => *shots,
            _ => unreachable!(),
        }
    }

    fn hits(&self, u: u32, day: u32) -> u32 {
        match &self.fire(u, day).kind {
            EventKind::GroundFire { hits, .. } => *hits,
            _ => unreachable!(),
        }
    }

    fn firing_men_of_fire(&self, u: u32, day: u32) -> u32 {
        match &self.fire(u, day).kind {
            EventKind::GroundFire { firing_men, .. } => *firing_men,
            _ => unreachable!(),
        }
    }

    /// Every `FightSupplyCharged`, as (event, its figures).
    fn charge_events(&self) -> Vec<(&Event, Charge)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::FightSupplyCharged {
                    side,
                    men_at_grips,
                    wanted_t,
                    from_ammunition_t,
                    from_supply_t,
                    supplied_share,
                    ..
                } => Some((
                    e,
                    Charge {
                        side: *side,
                        men_at_grips: *men_at_grips,
                        wanted: *wanted_t,
                        ammunition: *from_ammunition_t,
                        supply: *from_supply_t,
                        share: *supplied_share,
                    },
                )),
                _ => None,
            })
            .collect()
    }

    /// The charge of `side` on `day`.
    fn charge_of(&self, side: Side, day: u32) -> Option<(&Event, Charge)> {
        self.charge_events()
            .into_iter()
            .find(|(e, c)| c.side == side && e.day == day)
    }

    fn charge(&self, side: Side, day: u32) -> Charge {
        self.charge_of(side, day)
            .unwrap_or_else(|| panic!("a charge of {side:?} on day {day}"))
            .1
    }

    /// The `BaseAmmunitionSpent` of a base as (event, asked, paid, left).
    fn ammunition_spent(&self) -> Vec<(&Event, Fixed, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseAmmunitionSpent {
                    asked_t,
                    paid_t,
                    left_t,
                    ..
                } => Some((e, *asked_t, *paid_t, *left_t)),
                _ => None,
            })
            .collect()
    }

    /// The `BaseSupplySpent` for the fight as (event, asked, paid, left).
    fn supply_spent_on_fights(&self) -> Vec<(&Event, Fixed, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseSupplySpent {
                    reason: SupplyUse::Fight,
                    asked_t,
                    paid_t,
                    left_t,
                    ..
                } => Some((e, *asked_t, *paid_t, *left_t)),
                _ => None,
            })
            .collect()
    }

    /// Every `AmmunitionLanded` as (event, base, side, men, days, tons).
    fn landed(&self) -> Vec<(&Event, BaseId, u32, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::AmmunitionLanded {
                    base,
                    men,
                    days,
                    ammunition_t,
                    ..
                } => Some((e, *base, *men, *days, *ammunition_t)),
                _ => None,
            })
            .collect()
    }

    /// Every `RationsLanded` as (event, base, men, nights, tons).
    fn rations(&self) -> Vec<(&Event, BaseId, u32, Fixed, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::RationsLanded {
                    base,
                    men,
                    nights,
                    supply_t,
                    ..
                } => Some((e, *base, *men, *nights, *supply_t)),
                _ => None,
            })
            .collect()
    }

    fn formed(&self) -> Vec<(&Event, BaseId, Hex)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::BaseFormed { base, hex, .. } => Some((e, *base, *hex)),
                _ => None,
            })
            .collect()
    }

    fn event(&self, id: EventId) -> &Event {
        self.events
            .iter()
            .find(|e| e.id == id)
            .expect("the cited event is in the log")
    }

    /// The place in the log of the first event that `wanted` gives true for.
    fn index(&self, wanted: impl Fn(&Event) -> bool) -> usize {
        self.events
            .iter()
            .position(wanted)
            .expect("the event is in the log")
    }

    /// The base at P, found by its hex.
    fn p(&self) -> BaseId {
        let end = self.ends.last().unwrap();
        *end.bases
            .iter()
            .find(|(_, b)| b.hex == P)
            .map(|(id, _)| id)
            .expect("a base at P")
    }

    fn at_p(&self, day: usize) -> (Fixed, Fixed) {
        let p = &self.end(day).bases[&self.p()];
        (p.supply_t, p.ammunition_t)
    }
}

/// Every fight-supply event of the log, for "nothing recorded".
fn fight_records(played: &Played) -> usize {
    played
        .events
        .iter()
        .filter(|e| {
            matches!(
                &e.kind,
                EventKind::FightSupplyCharged { .. }
                    | EventKind::BaseAmmunitionSpent { .. }
                    | EventKind::BaseSupplySpent {
                        reason: SupplyUse::Fight,
                        ..
                    }
            )
        })
        .count()
}

/// The figures of a charge for A and for B, to compare.
#[track_caller]
fn assert_charge(
    played: &Played,
    side: Side,
    day: u32,
    (men, wanted, ammunition, supply, share): (&str, &str, &str, &str, &str),
) {
    assert_eq!(
        played.charge(side, day),
        Charge {
            side,
            men_at_grips: fx(men),
            wanted: fx(wanted),
            ammunition: fx(ammunition),
            supply: fx(supply),
            share: fx(share),
        },
        "the charge of {side:?} on day {day}"
    );
}

/// V's fire on `day`: its shots, its supplied share and its fire share.
#[track_caller]
fn assert_fire(played: &Played, u: u32, day: u32, shots: &str, supplied: &str, fire_share: &str) {
    assert_eq!(played.shots(u, day), fx(shots), "shots of unit {u}");
    let event = played.fire(u, day);
    assert_eq!(
        factor(event, "supplied_share"),
        fx(supplied),
        "supplied share of unit {u}"
    );
    assert_eq!(
        factor(event, "fire_share"),
        fx(fire_share),
        "fire share of unit {u}"
    );
    assert_eq!(factor(event, "shots"), fx(shots), "the factor shots");
}

// ---------------------------------------------------------------- The setting

#[test]
fn the_setting_is_what_the_comment_says() {
    let played = play(&fight("20", "0"));
    let figures = played.scenario.world.ref_data().ground.as_ref().unwrap();
    let hunger = figures.hunger.expect("the figures of 0205");
    assert_eq!(hunger.rations_landed_nights, n(3));
    assert_eq!(hunger.hungry_below_share, fx("0.6"));
    assert_eq!(hunger.hunger_nights_before_loss, 14);
    let mending = hunger.mending.expect("the figures of 0208");
    assert_eq!(mending.hunger_weakness_per_night, fx("0.01"));
    assert_eq!(mending.unfed_fighting_share, fx("0.9"));
    let fight = hunger.fight_supply.expect("the figures of 0209");
    assert_eq!(fight.fight_supply_t_per_man, fx("0.01"));
    assert_eq!(fight.unsupplied_fire_share, fx("0.25"));
    assert_eq!(fight.fight_days_landed, n(4));
    assert_eq!(figures.daily_supply_t_per_man, fx("0.005"));
    assert_eq!(figures.men_per_shot, n(100));
    let start = played.end(0);
    assert_eq!(start.land_units[&unit(V)].men_ashore, 1000);
    assert_eq!(start.land_units[&unit(E)].men_ashore, 1000);
    assert_eq!(start.land_units[&unit(E)].hex, Some(Q));
    assert_eq!(start.bases.len(), 1);
    assert_eq!(start.bases[&base(1)].hex, Q);
}

// ---------------------------------------------------------------- Statements: the charge

/// 1. Supplied in full.
#[test]
fn statement_1_supplied_in_full() {
    let played = play(&fight("20", "0"));
    // The night takes 5 and leaves 15.
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        n(15)
    );
    assert_eq!(played.firing_men_of_fire(V, 1), 1000);
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "0", "10", "1"));
    // Q has 5 t after the fight's charge: by the record and by the state.
    let spent = played.supply_spent_on_fights();
    assert_eq!(spent[0].2, n(10), "paid");
    assert_eq!(spent[0].3, n(5), "left");
    assert_eq!(played.q(1), (n(5), Fixed::ZERO));
    assert_fire(&played, V, 1, "10", "1", "1");
}

/// 2. Supplied by half.
#[test]
fn statement_2_supplied_by_half() {
    let played = play(&fight("10", "0"));
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        n(5)
    );
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "0", "5", "0.5"));
    assert_eq!(played.q(1), (Fixed::ZERO, Fixed::ZERO));
    assert_fire(&played, V, 1, "6.25", "0.5", "0.625");
    // 0.25 plus 0.75 times 0.5.
    assert_eq!(fx("0.25") + fx("0.75") * fx("0.5"), fx("0.625"));
    // V's firing men are still 1,000.
    assert_eq!(played.firing_men_of_fire(V, 1), 1000);
}

/// 3. Nothing to fire with.
#[test]
fn statement_3_nothing_to_fire_with() {
    let played = play(&fight("5", "0"));
    // The night takes it all and V is fed in full.
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        Fixed::ZERO
    );
    assert_eq!(played.firing_men_of_fire(V, 1), 1000);
    // The charge is still recorded for A: 10 t wanted, none charged.
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "0", "0", "0"));
    assert_eq!(played.q(1), (Fixed::ZERO, Fixed::ZERO));
    assert_fire(&played, V, 1, "2.5", "0", "0.25");
}

/// 4. A side with no base, in each of statements 1 to 3.
#[test]
fn statement_4_a_side_with_no_base() {
    for (supply, label) in [
        ("20", "statement 1"),
        ("10", "statement 2"),
        ("5", "statement 3"),
    ] {
        let played = play(&fight(supply, "0"));
        assert_eq!(played.firing_men_of_fire(E, 1), 891, "{label}");
        assert_charge(&played, Side::Japanese, 1, ("891", "8.91", "0", "0", "0"));
        assert_fire(&played, E, 1, "2.2275", "0", "0.25");
        // Nothing is charged to B: B has no base.
        assert_eq!(played.end(1).bases.len(), 1, "{label}");
    }
}

/// 5. Ammunition first.
#[test]
fn statement_5_ammunition_first() {
    let played = play(&fight("20", "6"));
    let morning = &played.before(1, Pulse::Morning).bases[&base(1)];
    assert_eq!((morning.supply_t, morning.ammunition_t), (n(15), n(6)));
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "6", "4", "1"));
    assert_eq!(played.q(1), (n(11), Fixed::ZERO));
    assert_fire(&played, V, 1, "10", "1", "1");
    // The record gives 6 t from ammunition and 4 t from supply.
    let ammunition = played.ammunition_spent();
    assert_eq!(ammunition[0].2, n(6), "paid from ammunition");
    assert_eq!(ammunition[0].3, Fixed::ZERO, "ammunition left");
    let supply = played.supply_spent_on_fights();
    assert_eq!(supply[0].2, n(4), "paid from supply");
    assert_eq!(supply[0].3, n(11), "supply left");
}

/// 6. Ammunition alone.
#[test]
fn statement_6_ammunition_alone() {
    let played = play(&fight("5", "4"));
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        Fixed::ZERO
    );
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "4", "0", "0.4"));
    assert_eq!(played.q(1), (Fixed::ZERO, Fixed::ZERO));
    assert_fire(&played, V, 1, "5.5", "0.4", "0.55");
    // 0.25 plus 0.75 times 0.4.
    assert_eq!(fx("0.25") + fx("0.75") * fx("0.4"), fx("0.55"));
    assert_eq!(played.ammunition_spent()[0].2, n(4));
}

/// 7. No fight, no charge.
#[test]
fn statement_7_no_fight_no_charge() {
    let t = with_days(&with_stocks(&text(), "20", "6"), 2);
    let played = play(&t);
    assert_eq!(played.q(1), (n(15), n(6)));
    assert_eq!(played.q(2), (n(10), n(6)));
    // Nothing is recorded of a fight's charge.
    assert_eq!(fight_records(&played), 0);
    assert!(played.charge_events().is_empty());
    assert!(played.ammunition_spent().is_empty());
    assert!(played.supply_spent_on_fights().is_empty());
}

// ---------------------------------------------------------------- Statements: the limit on engaged men

/// The setting of 8 and 9: the ratio at its least,
/// 1, and V of 500 men (the night takes 2.5 t of Q's 17.5, leaving the 15 t
/// of the statements). The limit for A is E's 891 fighting men, above V's
/// 500; the limit for B is V's 500, below E's 891: the figures are those of
/// the statements.
fn limited() -> Played {
    let t = changed(&text(), "men = 1000\n", "men = 500\n");
    let t = with_days(&with_e(&with_stocks(&t, "17.5", "0")), 2);
    play_with(&t, &engagement_one())
}

#[test]
fn statement_8_only_the_men_at_grips_are_paid_for() {
    let played = limited();
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        n(15)
    );
    assert_eq!(played.firing_men_of_fire(V, 1), 500);
    assert_charge(&played, Side::Allied, 1, ("500", "5", "0", "5", "1"));
    assert_eq!(played.q(1), (n(10), Fixed::ZERO));
    assert_fire(&played, V, 1, "5", "1", "1");
}

#[test]
fn statement_9_the_cut_is_on_the_shots_after_the_limit() {
    let played = limited();
    assert_eq!(played.firing_men_of_fire(E, 1), 891);
    assert_charge(&played, Side::Japanese, 1, ("500", "5", "0", "0", "0"));
    assert_fire(&played, E, 1, "1.25", "0", "0.25");
}

// ---------------------------------------------------------------- Statements: what a landing brings

/// 10. Days of fighting, as ammunition.
#[test]
fn statement_10_days_of_fighting_as_ammunition() {
    let played = play(&landing(2));
    let formed = played.formed();
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].2, P);
    let p = formed[0].1;
    assert_eq!(played.p(), p);
    let landed = played.landed();
    assert_eq!(landed.len(), 1);
    let (event, at, men, days, tons) = &landed[0];
    assert_eq!(*at, p);
    // The record names 3,000 men, 4 days and 120 t.
    assert_eq!(*men, 3000);
    assert_eq!(*days, n(4));
    assert_eq!(*tons, n(120));
    // 3,000 times 4 times 0.01.
    assert_eq!(n(3000) * n(4) * fx("0.01"), n(120));
    assert_eq!(event.day, 1);
    assert_eq!(event.pulse, Some(Pulse::Night));
    // When T lands its first man the base formed has 45 t of supply, the
    // rations of decision 0205, and 120 t of ammunition.
    let after = &played.before(1, Pulse::Morning).bases[&p];
    assert_eq!((after.supply_t, after.ammunition_t), (n(45), n(120)));
    // It cites what the rations' record cites.
    let rations = played.rations();
    assert_eq!(rations.len(), 1);
    assert_eq!(rations[0].4, n(45));
    assert_eq!(event.causes, rations[0].0.causes);
    assert_eq!(cited(event), vec![formed[0].0.id], "the BaseFormed");
}

/// 11. Every landing brings them.
#[test]
fn statement_11_every_landing_brings_them() {
    // T2: two transports at (7,0) (1,200 men aboard at most) with W, 1,000
    // men, going to P on day 3.
    let t = with_t_and_u(&text());
    let t = with_force(&t, 2, 6, 2, 7);
    let t = with_unit_aboard(&t, 3, 1000, 2);
    let t = with_go(&t, 2, 3, 8);
    let played = play(&with_days(&t, 4));
    let p = played.p();
    assert_eq!(played.at_p(1).1, n(120));
    assert_eq!(played.at_p(2).1, n(120));
    assert_eq!(played.end(3).land_units[&unit(3)].men_ashore, 1000);
    let landed = played.landed();
    assert_eq!(landed.len(), 2);
    assert_eq!(landed[0].2, 3000);
    assert_eq!(landed[0].4, n(120));
    let (second, at, men, days, tons) = &landed[1];
    assert_eq!((*at, *men, *days, *tons), (p, 1000, n(4), n(40)));
    assert_eq!(second.day, 3);
    // 1,000 times 4 times 0.01, and no rations: the one record of rations
    // is the first force's.
    assert_eq!(n(1000) * n(4) * fx("0.01"), n(40));
    assert_eq!(played.rations().len(), 1);
    assert_eq!(played.rations()[0].2, 3000);
    assert_eq!(played.formed().len(), 1);
    assert!(cited(second)
        .iter()
        .all(|id| !matches!(played.event(*id).kind, EventKind::BaseFormed { .. })));
    // P has 160 t of ammunition.
    assert_eq!(played.at_p(3).1, n(160));
    assert_eq!(played.at_p(4).1, n(160));
}

/// 12. Men do not eat it.
#[test]
fn statement_12_men_do_not_eat_it() {
    let played = play(&landing(5));
    // All of U is ashore at P and nothing else landed.
    assert_eq!(
        played.before(1, Pulse::Morning).land_units[&unit(2)].men_ashore,
        3000
    );
    assert_eq!(played.landed().len(), 1);
    // The three nights after the landing are fed in full and the fourth is
    // not: the stock of supply is 45, 30, 15, then nothing, and the count of
    // hunger is nothing until the fifth night.
    let supply = |day: usize| played.at_p(day).0;
    assert_eq!(supply(1), n(45));
    assert_eq!(supply(2), n(30));
    assert_eq!(supply(3), n(15));
    assert_eq!(supply(4), Fixed::ZERO);
    for day in 1..=4usize {
        assert_eq!(played.end(day).land_units[&unit(2)].hunger_nights, 0);
    }
    assert_eq!(played.end(5).land_units[&unit(2)].hunger_nights, 1);
    // P's ammunition is 120 t throughout.
    for day in 1..=5usize {
        assert_eq!(
            played.at_p(day).1,
            n(120),
            "ammunition at the end of day {day}"
        );
    }
    for (day, pulse) in [(1, Pulse::Morning), (3, Pulse::Night), (5, Pulse::Night)] {
        assert_eq!(
            played.before(day, pulse).bases[&played.p()].ammunition_t,
            n(120)
        );
    }
}

// ---------------------------------------------------------------- Statements: a dry run, and where it runs

/// 13. A side a dry run makes up is taken as supplied.
#[test]
fn statement_13_a_side_a_dry_run_makes_up_is_taken_as_supplied() {
    let mut scenario = load_with(&fight("20", "0"), &common::frozen());
    scenario.world.set_supplied_in_full(Side::Japanese);
    let played = play_scenario(scenario);
    // E's fire share is 1 and its shots 8.91.
    assert_eq!(played.firing_men_of_fire(E, 1), 891);
    assert_fire(&played, E, 1, "8.91", "1", "1");
    // Nothing is charged to B and no charge is recorded for B.
    assert!(played
        .charge_events()
        .iter()
        .all(|(_, c)| c.side == Side::Allied));
    assert!(played.charge_of(Side::Japanese, 1).is_none());
    assert!(played.charge_of(Side::Japanese, 2).is_none());
    // A is charged as in statement 1.
    assert_charge(&played, Side::Allied, 1, ("1000", "10", "0", "10", "1"));
    assert_eq!(played.q(1), (n(5), Fixed::ZERO));
    assert_fire(&played, V, 1, "10", "1", "1");
}

/// 14. On scenario 123's own ground file with the three figures of
/// 0209 removed. (Its `unfed_fighting_share` is 0.9, so E's 891 firing men
/// are as in statement 1.)
#[test]
fn statement_14_without_the_three_figures() {
    let played = play_with(&fight("20", "0"), &without_the_three());
    assert_eq!(played.shots(V, 1), n(10));
    assert_eq!(played.shots(E, 1), fx("8.91"));
    assert_eq!(played.q(1), (n(15), Fixed::ZERO));
    assert_eq!(
        played.before(1, Pulse::Morning).bases[&base(1)].supply_t,
        n(15)
    );
    assert_eq!(fight_records(&played), 0);
    assert!(played.charge_events().is_empty());
    for u in [V, E] {
        let fire = played.fire(u, 1);
        assert!(!has_factor(fire, "supplied_share"));
        assert!(!has_factor(fire, "fire_share"));
    }
    // The landing of statement 10 brings 45 t of supply and no ammunition.
    let landing = play_with(&landing(2), &without_the_three());
    assert_eq!(landing.rations().len(), 1);
    assert_eq!(landing.rations()[0].4, n(45));
    assert!(landing.landed().is_empty());
    assert_eq!(landing.at_p(1), (n(45), Fixed::ZERO));
}

// ---------------------------------------------------------------- Statements: the loader and the files

fn live_ground_text() -> String {
    std::fs::read_to_string(common::frozen_root().join("ground.toml"))
        .expect("the frozen ground.toml reads")
}

const THREE_OF_0205: [&str; 3] = [
    "rations_landed_nights = 3\n",
    "hungry_below_share = \"0.6\"\n",
    "hunger_nights_before_loss = 14\n",
];
const TWO_OF_0208: [&str; 2] = [
    "hunger_weakness_per_night = \"0.01\"\n",
    "unfed_fighting_share = \"0.9\"\n",
];

/// Loads a ground file made of the frozen `ground.toml` and `lines`, under a
/// folder of its own in the temporary folder.
fn load_lines(
    tag: &str,
    lines: &[&str],
) -> Result<sim_core::refdata::GroundFigures, sim_data::DataError> {
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = std::env::temp_dir().join(format!(
        "flagplot_scenario_123_loader_{}_{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).expect("a folder");
    let file = dir.join(format!("{tag}.toml"));
    let mut text = live_ground_text();
    if !text.ends_with('\n') {
        text.push('\n');
    }
    for line in lines {
        text.push_str(line);
    }
    std::fs::write(&file, text).expect("the file writes");
    let loaded = sim_data::load_ground_figures(&file);
    let _ = std::fs::remove_dir_all(&dir);
    loaded
}

/// 15. The loader.
#[test]
fn statement_15_the_loader() {
    let with = |extra: &[&'static str]| -> Vec<&'static str> {
        THREE_OF_0205.iter().chain(extra.iter()).copied().collect()
    };
    // One or two of the three and not all are refused (with the three of
    // 0205 given).
    for (i, subset) in [
        vec![0],
        vec![1],
        vec![2],
        vec![0, 1],
        vec![0, 2],
        vec![1, 2],
    ]
    .iter()
    .enumerate()
    {
        let some: Vec<&str> = subset.iter().map(|k| THREE_LINES[*k]).collect();
        assert!(
            load_lines(&format!("partial_{i}"), &with(&some)).is_err(),
            "{some:?}"
        );
    }
    // The three, and the three of 0205, load; without the two of 0208.
    let all = with(&THREE_LINES);
    let figures = load_lines("with_the_three", &all).expect("the three and the three of 0205");
    let hunger = figures.hunger.unwrap();
    let fight = hunger.fight_supply.unwrap();
    assert_eq!(fight.fight_supply_t_per_man, fx("0.01"));
    assert_eq!(fight.unsupplied_fire_share, fx("0.25"));
    assert_eq!(fight.fight_days_landed, n(4));
    assert!(hunger.mending.is_none());
    // The three without the three of 0205, or with only some of them, are
    // refused.
    assert!(load_lines("without_0205", &THREE_LINES).is_err());
    for (i, left_out) in THREE_OF_0205.iter().enumerate() {
        let some: Vec<&str> = THREE_OF_0205
            .iter()
            .filter(|l| *l != left_out)
            .chain(THREE_LINES.iter())
            .copied()
            .collect();
        assert!(load_lines(&format!("without_0205_{i}"), &some).is_err());
    }
    // With the two of 0208 too, taken.
    let with_two: Vec<&str> = all.iter().chain(TWO_OF_0208.iter()).copied().collect();
    let figures = load_lines("with_0208", &with_two).expect("with the two of 0208");
    let hunger = figures.hunger.unwrap();
    assert!(hunger.mending.is_some());
    assert!(hunger.fight_supply.is_some());
    // The figures below 0 are refused, and the share above 1.
    let bad = [
        (
            "t_below",
            with(&[
                "fight_supply_t_per_man = \"-0.01\"\n",
                THREE_LINES[1],
                THREE_LINES[2],
            ]),
        ),
        (
            "days_below",
            with(&[THREE_LINES[0], THREE_LINES[1], "fight_days_landed = -1\n"]),
        ),
        (
            "share_below",
            with(&[
                THREE_LINES[0],
                "unsupplied_fire_share = \"-0.01\"\n",
                THREE_LINES[2],
            ]),
        ),
        (
            "share_above",
            with(&[
                THREE_LINES[0],
                "unsupplied_fire_share = \"1.01\"\n",
                THREE_LINES[2],
            ]),
        ),
    ];
    for (tag, lines) in &bad {
        assert!(load_lines(tag, lines).is_err(), "{tag}");
    }
    // 0 and 1 are taken where they are inside the range.
    for (tag, lines, share, days) in [
        (
            "zeros",
            with(&[
                "fight_supply_t_per_man = \"0\"\n",
                "unsupplied_fire_share = \"0\"\n",
                "fight_days_landed = 0\n",
            ]),
            0,
            0,
        ),
        (
            "share_one",
            with(&[
                THREE_LINES[0],
                "unsupplied_fire_share = \"1\"\n",
                THREE_LINES[2],
            ]),
            1,
            4,
        ),
    ] {
        let figures = load_lines(tag, &lines).unwrap_or_else(|e| panic!("{tag}: {}", e.message));
        let fight = figures.hunger.unwrap().fight_supply.unwrap();
        assert_eq!(fight.unsupplied_fire_share, n(share), "{tag}");
        assert_eq!(fight.fight_days_landed, n(days), "{tag}");
    }
    // The frozen ground.toml as it stands loads, with none of the figures.
    let plain = load_lines("plain", &[]).expect("the frozen file loads");
    assert!(plain.hunger.is_none());
}

fn toml_of(relative: &str) -> toml::Table {
    let file = common::data_dir().join(relative);
    std::fs::read_to_string(&file)
        .unwrap_or_else(|e| panic!("{}: {e}", file.display()))
        .parse::<toml::Table>()
        .expect("the file is TOML")
}

const FIGHT_SUPPLY: [&str; 3] = [
    "fight_supply_t_per_man",
    "unsupplied_fire_share",
    "fight_days_landed",
];

fn gives_the_three(table: &toml::Table, file: &str) {
    assert_eq!(
        table["fight_supply_t_per_man"].as_str(),
        Some("0.01"),
        "{file}"
    );
    assert_eq!(
        table["unsupplied_fire_share"].as_str(),
        Some("0.25"),
        "{file}"
    );
    assert_eq!(table["fight_days_landed"].as_integer(), Some(4), "{file}");
}

/// 16. The files.
#[test]
fn statement_16_the_files() {
    let hunger = toml_of("reference/ground_hunger.toml");
    let live = toml_of("reference/ground.toml");
    gives_the_three(&hunger, "ground_hunger.toml");
    // No figure of it that the live ground.toml has differs from it.
    for (key, value) in &live {
        assert_eq!(hunger.get(key), Some(value), "{key} is the live file's");
    }
    for key in FIGHT_SUPPLY {
        assert!(!live.contains_key(key), "{key} in the live ground.toml");
    }
    // Its frozen counterpart gives the three with the same values.
    let frozen = toml_of("reference/step1/ground_hunger.toml");
    gives_the_three(&frozen, "step1/ground_hunger.toml");
    for key in FIGHT_SUPPLY {
        assert_eq!(frozen.get(key), hunger.get(key), "{key} in the frozen copy");
    }
    // Scenario 119's and scenario 122's own files are as they were: they
    // give none of the three.
    for number in ["119", "122"] {
        let live = format!("reference/ground_placeholder_scenario_{number}.toml");
        let own = toml_of(&live);
        let own_frozen = toml_of(&format!(
            "reference/step1/ground_placeholder_scenario_{number}.toml"
        ));
        for key in FIGHT_SUPPLY {
            assert!(!own.contains_key(key), "{key} in {live}");
            assert!(!own_frozen.contains_key(key), "{key} in the frozen {live}");
        }
        assert_eq!(own, own_frozen, "scenario {number}'s two files agree");
    }
    // Scenario 123's own file gives the three, and so does its frozen copy.
    let own = toml_of("reference/ground_placeholder_scenario_123.toml");
    gives_the_three(&own, "scenario 123's own file");
    assert_eq!(
        own,
        toml_of("reference/step1/ground_placeholder_scenario_123.toml")
    );
}

// ---------------------------------------------------------------- Statements: what is recorded and who knows

/// 17. The order and the causes, in statement 5.
#[test]
fn statement_17_the_order_and_the_causes() {
    let played = play(&changed(&fight("20", "6"), "days = 2\n", "days = 1\n"));
    let pos = |id: EventId| played.events.iter().position(|e| e.id == id).unwrap();
    let fight_event =
        played.index(|e| e.day == 1 && matches!(e.kind, EventKind::GroundFight { hex: Q, .. }));
    let (a, _) = played.charge_of(Side::Allied, 1).expect("A's charge");
    let (b, _) = played.charge_of(Side::Japanese, 1).expect("B's charge");
    let ammunition = played.ammunition_spent();
    let supply = played.supply_spent_on_fights();
    assert_eq!((ammunition.len(), supply.len()), (1, 1));
    let (ammunition, supply) = (ammunition[0].0, supply[0].0);
    let first_fire = pos(played.fire(V, 1).id).min(pos(played.fire(E, 1).id));
    // After the fight's event and before any unit's fire.
    for (what, id) in [
        ("A's charge", a.id),
        ("B's charge", b.id),
        ("Q's ammunition spent", ammunition.id),
        ("Q's supply spent", supply.id),
    ] {
        assert!(fight_event < pos(id), "{what} comes after the fight");
        assert!(pos(id) < first_fire, "{what} comes before any fire");
    }
    // A's comes before B's, and A's stocks before B's charge.
    assert!(pos(a.id) < pos(b.id));
    assert!(pos(ammunition.id) < pos(b.id));
    assert!(pos(supply.id) < pos(b.id));
    // Ammunition before supply.
    assert!(pos(ammunition.id) < pos(supply.id));
    // The causes.
    let fight_id = played.events[fight_event].id;
    assert_eq!(cited(a), vec![fight_id], "A's charge cites the fight");
    assert_eq!(cited(b), vec![fight_id], "B's charge cites the fight");
    assert_eq!(
        cited(ammunition),
        vec![a.id],
        "the ammunition cites the charge"
    );
    assert_eq!(cited(supply), vec![a.id], "the supply cites the charge");
    assert!(
        cited(played.fire(V, 1)).contains(&a.id),
        "V's fire cites A's charge"
    );
    assert!(
        cited(played.fire(E, 1)).contains(&b.id),
        "E's fire cites B's charge"
    );
}

/// What a side's view shows, whole, of the fight's records.
struct Shown {
    charges: Vec<(Side, Fixed)>,
    ammunition_spent: usize,
    supply_spent_on_fights: usize,
    /// (day, supplied share, fire share) of every fire the side sees with
    /// them.
    fire: Vec<(u32, Fixed, Fixed)>,
}

fn shown_to(played: &Played, side: Side) -> Shown {
    let mut shown = Shown {
        charges: Vec::new(),
        ammunition_spent: 0,
        supply_spent_on_fights: 0,
        fire: Vec::new(),
    };
    for e in played.scenario.world.side_view(side).events() {
        match &e.kind {
            SideEventKind::Whole(EventKind::FightSupplyCharged { side, wanted_t, .. }) => {
                shown.charges.push((*side, *wanted_t))
            }
            SideEventKind::Whole(EventKind::BaseAmmunitionSpent { .. }) => {
                shown.ammunition_spent += 1;
            }
            SideEventKind::Whole(EventKind::BaseSupplySpent {
                reason: SupplyUse::Fight,
                ..
            }) => shown.supply_spent_on_fights += 1,
            _ => {}
        }
        if let Some(b) = &e.breakdown {
            let find = |name: &str| b.factors.iter().find(|f| f.name == name).map(|f| f.value);
            match (find("supplied_share"), find("fire_share")) {
                (Some(s), Some(f)) => shown.fire.push((e.day, s, f)),
                (None, None) => {}
                other => panic!("one of the two factors alone: {other:?} in {e:?}"),
            }
        }
    }
    shown
}

fn session_after(text: &str, days: u32) -> Session {
    let scenario = load_with(text, &common::frozen());
    let name = scenario.name.clone();
    let mut session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Player,
    );
    for _ in 0..days {
        for side in [Side::Allied, Side::Japanese] {
            session
                .call(side, "end_day", &json!({}))
                .expect("end_day answers");
        }
    }
    session
}

fn briefing(session: &mut Session, side: Side) -> Value {
    session
        .call(side, "get_briefing", &json!({ "text": false }))
        .expect("the briefing")
}

/// 18. Each side sees its own, in statement 2.
#[test]
fn statement_18_each_side_sees_its_own() {
    let t = changed(&fight("10", "0"), "days = 2\n", "days = 1\n");
    let played = play(&t);
    // A is shown its charge and, in V's fire, a supplied share of 0.5 and a
    // fire share of 0.625; of B's it is shown nothing (E's are 0 and 0.25).
    let a = shown_to(&played, Side::Allied);
    assert!(!a.charges.is_empty());
    assert!(a.charges.iter().all(|(side, _)| *side == Side::Allied));
    assert!(a.charges.iter().all(|(_, wanted)| *wanted == n(10)));
    assert!(a.supply_spent_on_fights >= 1);
    assert_eq!(
        a.fire
            .iter()
            .filter(|f| f.0 == 1)
            .map(|f| (f.1, f.2))
            .collect::<Vec<_>>(),
        vec![(fx("0.5"), fx("0.625"))]
    );
    assert!(a
        .fire
        .iter()
        .all(|f| (f.1, f.2) != (Fixed::ZERO, fx("0.25")) || f.0 > 1));
    // B is shown none of A's charge, A's stocks spent or those factors, and
    // its own charge (E's 8.91 t wanted) and its own factors.
    let b = shown_to(&played, Side::Japanese);
    assert!(!b.charges.is_empty());
    assert!(b.charges.iter().all(|(side, _)| *side == Side::Japanese));
    assert!(b.charges.iter().all(|(_, wanted)| *wanted == fx("8.91")));
    assert_eq!(b.ammunition_spent, 0);
    assert_eq!(b.supply_spent_on_fights, 0);
    assert_eq!(
        b.fire
            .iter()
            .filter(|f| f.0 == 1)
            .map(|f| (f.1, f.2))
            .collect::<Vec<_>>(),
        vec![(Fixed::ZERO, fx("0.25"))]
    );
    // The tools of a session say the same: B's briefing is told nothing of
    // the figures of A's fire.
    let mut session = session_after(&t, 1);
    let briefing_b = briefing(&mut session, Side::Japanese).to_string();
    for word in ["0.625", "BaseAmmunitionSpent", "FightSupplyCharged"] {
        assert!(!briefing_b.contains(word), "B's briefing gives {word}");
    }
}

/// 19. The side's own count of firing men is not cut, in statement 3.
#[test]
fn statement_19_the_sides_own_count_of_firing_men_is_not_cut() {
    let t = fight("5", "0");
    let played = play(&t);
    // A is told, on the morning of the fight, that 1,000 of V would fire.
    assert_eq!(played.firing[&(1, V)], 1000);
    // The fight's own record has 1,000 firing and 2.5 shots.
    assert_eq!(played.firing_men_of_fire(V, 1), 1000);
    assert_eq!(played.shots(V, 1), fx("2.5"));
    // The briefing's line for V says 1,000 would fire, at the start.
    let mut session = session_after(&t, 0);
    let b = briefing(&mut session, Side::Allied);
    let line = b["land_units"]["units"]
        .as_array()
        .expect("units")
        .iter()
        .find(|l| l["land_unit"]["handle"] == "land_unit:1")
        .cloned()
        .expect("V's line");
    assert_eq!(line["firing_men_now"].as_u64(), Some(1000));
}

// ---------------------------------------------------------------- Properties

fn scenarios_dir() -> PathBuf {
    common::data_dir().join("scenarios")
}

/// Every `.toml` under `dir`, recursively, in order.
fn tomls_under(dir: &Path, into: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .expect("the folder reads")
        .map(|e| e.expect("an entry reads").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            tomls_under(&entry, into);
        } else if entry.extension().is_some_and(|x| x == "toml") {
            into.push(entry);
        }
    }
}

/// The value of a top-level `ground = "..."` line of a scenario file.
fn ground_line(text: &str) -> Option<String> {
    text.lines()
        .find(|l| l.starts_with("ground = \""))
        .map(|l| {
            l.trim_start_matches("ground = \"")
                .trim_end_matches('"')
                .to_string()
        })
}

fn gives_fight_supply(figures: &sim_core::refdata::GroundFigures) -> bool {
    figures.hunger.and_then(|h| h.fight_supply).is_some()
}

/// Properties: no proof and no recorded game moves. No scenario on file
/// names a ground file that gives the figures of 0209 but the table's copies
/// and the folders of scenarios 119, 122 and 123, and no figures file in
/// `data/reference` (live or frozen) gives them but `ground_hunger.toml` and
/// scenario 123's own. (That every scenario and recorded game gives the
/// events and the daily hashes it gave before is checked by running them
/// before and after.)
#[test]
fn property_no_game_on_file_moves() {
    let mut files = Vec::new();
    tomls_under(&scenarios_dir(), &mut files);
    let mut named = 0;
    let mut own_gives = 0;
    for file in &files {
        let rel = file
            .strip_prefix(scenarios_dir())
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let own = rel.starts_with("hunger-takes-time/")
            || rel.starts_with("hungry-men-are-slow-to-mend/")
            || rel.starts_with("a-fight-costs-supply/")
            || rel.starts_with("table-hunger/");
        let text = std::fs::read_to_string(file).expect("the scenario reads");
        let Some(ground) = ground_line(&text) else {
            continue;
        };
        let ground_path = file.parent().unwrap().join(&ground);
        let figures = sim_data::load_ground_figures(&ground_path)
            .unwrap_or_else(|e| panic!("{rel} names {ground}: {}", e.message));
        named += 1;
        if !own {
            assert!(
                !gives_fight_supply(&figures),
                "{rel} names {ground}, which gives the figures of 0209"
            );
        } else if gives_fight_supply(&figures) {
            own_gives += 1;
        }
    }
    assert!(named > 50, "{named} scenarios name a ground file");
    assert!(own_gives >= 1, "scenario 123 names a file that gives them");
    // The figures files themselves.
    for dir in [
        common::data_dir().join("reference"),
        common::data_dir().join("reference/step1"),
    ] {
        for entry in std::fs::read_dir(&dir).expect("the folder reads") {
            let path = entry.expect("an entry reads").path();
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if !(name.starts_with("ground") && name.ends_with(".toml")) {
                continue;
            }
            let figures = sim_data::load_ground_figures(&path).expect("a ground file loads");
            let may = name == "ground_hunger.toml" || name == OWN_GROUND;
            assert_eq!(
                gives_fight_supply(&figures),
                may,
                "{} gives the figures: {}",
                path.display(),
                !may
            );
        }
    }
}

/// Properties: no new state. The layout version of the state, and the rules
/// label, are what they were, and a recorded game saved before the change
/// loads and replays to its hashes.
#[test]
fn property_no_new_state() {
    assert_eq!(sim_core::hash::STATE_VERSION, b"flagplot-state-v53");
    assert_eq!(sim_core::world::RULES_VERSION, 26);
    let root = common::data_dir().join("..");
    let file = root.join("crates/sim-cli/tests/recorded_games/eastern_solomons_haiku/save.json");
    let saved = sim_cli::save::read(file.to_str().expect("a path")).expect("the save reads");
    let mut scenario = Scenario::load(&root.join(&saved.scenario_file))
        .expect("the scenario loads")
        .with_seed(saved.seed)
        .expect("seeded");
    scenario.replace_script(saved.script.clone());
    scenario.replace_script_inside_day(saved.script_inside_day.clone());
    scenario.replace_commands(saved.commands.clone());
    assert!(!saved.hashes.is_empty());
    for (day, hash) in &saved.hashes {
        for (side, decisions) in saved.staff.get(day).into_iter().flatten() {
            scenario.world.submit_staff(*side, decisions.clone());
        }
        let summary = scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("the day resolves")
            .expect("the day exists");
        assert_eq!(hash, &format!("{:016x}", summary.state_hash), "day {day}");
    }
}

/// Properties: nothing is drawn by the rule. The same game played twice
/// gives the same events; the charge, which is made before any die is
/// rolled, is the same under another seed; and in statement 14's setting the
/// dice of the fight fall as they did before the change was made.
#[test]
fn property_nothing_is_drawn() {
    // The same game twice.
    let t = fight("10", "6");
    let (first, second) = (play(&t), play(&t));
    assert_eq!(first.events, second.events);
    assert!(first.events.len() > 20);
    // Another seed: the charges and the stocks are the same.
    let seeded = play_scenario(
        load_with(&t, &common::frozen())
            .with_seed(7)
            .expect("a seed can be set"),
    );
    let first_charges: Vec<Charge> = first.charge_events().iter().map(|c| c.1).collect();
    assert!(!first_charges.is_empty());
    let day_one = |p: &Played| -> Vec<Charge> {
        p.charge_events()
            .iter()
            .filter(|(e, _)| e.day == 1)
            .map(|c| c.1)
            .collect()
    };
    assert_eq!(day_one(&first), day_one(&seeded));
    assert_eq!(first.shots(V, 1), seeded.shots(V, 1));
    assert_eq!(first.shots(E, 1), seeded.shots(E, 1));
    // Statement 14's setting (statement 1's stocks, E attacking, the three
    // figures removed): the hits as they were before the change.
    let before = play_with(&fight("20", "0"), &without_the_three());
    assert_eq!(before.hits(V, 1), 5, "V's hits on day 1");
    assert_eq!(before.hits(E, 1), 2, "E's hits on day 1");
    assert_eq!(before.hits(V, 2), 4, "V's hits on day 2");
    assert_eq!(before.hits(E, 2), 4, "E's hits on day 2");
}

/// Properties: stocks never go below nothing, and what is charged in a fight
/// is never more than what was wanted, in several settings.
#[test]
fn property_stocks_never_go_below_nothing() {
    let settings: Vec<(String, LoadOptions)> = vec![
        (fight("20", "0"), common::frozen()),
        (fight("10", "0"), common::frozen()),
        (fight("5", "0"), common::frozen()),
        (fight("0", "0"), common::frozen()),
        (fight("20", "6"), common::frozen()),
        (fight("5", "4"), common::frozen()),
        (fight("0", "3"), common::frozen()),
        (fight("100", "100"), common::frozen()),
        (fight("20", "0"), engagement_one()),
        (landing(6), common::frozen()),
        (
            with_days(&with_e(&with_stocks(&text(), "12", "7")), 10),
            common::frozen(),
        ),
    ];
    let mut fights = 0;
    for (i, (t, options)) in settings.iter().enumerate() {
        let played = play_with(t, options);
        for state in &played.ends {
            for base in state.bases.values() {
                assert!(base.supply_t >= Fixed::ZERO, "setting {i}: supply");
                assert!(base.ammunition_t >= Fixed::ZERO, "setting {i}: ammunition");
            }
        }
        for (_, charge) in played.charge_events() {
            fights += 1;
            assert!(
                charge.ammunition >= Fixed::ZERO && charge.supply >= Fixed::ZERO,
                "setting {i}"
            );
            assert!(
                charge.ammunition + charge.supply <= charge.wanted,
                "setting {i}: charged more than wanted: {charge:?}"
            );
            assert!(charge.share >= Fixed::ZERO && charge.share <= Fixed::ONE);
        }
        for (_, asked, paid, left) in played
            .ammunition_spent()
            .into_iter()
            .chain(played.supply_spent_on_fights())
        {
            assert!(paid <= asked, "setting {i}: paid more than asked");
            assert!(left >= Fixed::ZERO, "setting {i}: left below nothing");
        }
    }
    assert!(fights >= 20, "{fights} charges were checked");
}
