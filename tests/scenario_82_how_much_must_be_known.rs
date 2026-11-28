//! Scenario 82: how much must be known. The commander asks for an
//! airfield at balanced risk and says that the count of the enemy there
//! must be no more than a day old when the landing goes. The staff looks,
//! readies one unit on what it sees, finds its count a day too old on the
//! morning the landing is ready, looks again, and goes the day after. The
//! plan says each morning what must be known, what is held and who is
//! looking.
//!
//! Proves decision 0159, rule D, items 12 and 13, step 4 of the opening
//! of stage 4, against a hand calculation. The data is placeholder data:
//! see `data/scenarios/82_how_much_must_be_known.toml`. It is scenario
//! 72's start, and runs on scenario 72's figures: the frozen step 1
//! figures with `preparation_per_day` 100 and the four error figures at
//! 1, so a report gives the true count and no die changes anything. The
//! game ends before the landing.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly, except the supply and the fatigue the flights cost, which
//! are the search rule's. If the code disagrees, the code or the rule is
//! wrong: do not change a number here to make a test pass. A mismatch
//! goes back to the architect.
//!
//! ## The limit
//!
//! "Knowledge" is a limit on a "take a place" or a "control an island"
//! of the commander's, read up the tree as the other limits are, and
//! refused on any other kind. It has three settings:
//!
//! - **none:** the landing may go on any count or on none;
//! - **a count:** on a count seen from the air or by men in contact, of
//!   any day;
//! - **a recent count:** on such a count no more than a number of days
//!   old on the morning it goes, 3 where the commander gives no number.
//!
//! Where it is not set, the risk sets it as before this decision: press
//! none, balanced a count, preserve a recent count of 3 days. Where it is
//! set, the risk no longer says how much must be known, and still says
//! how many men are wanted for each of the enemy's.
//!
//! ## The need, as data
//!
//! With each plan of a take the staff hands in what must be known: the
//! place; what is asked (the setting and its days); what is held (the
//! count, where it came from, its day and its age in days, or nothing);
//! what the count held could be in truth, from its grade's range in the
//! figures, the lowest cut and the highest raised to a whole man (none
//! for a count that was told); which group is looking and since which
//! day, or none; and whether the need is met.
//!
//! ## Inputs
//!
//! As scenario 72's: the map of scenario 27, A at (0,0) and B at (4,0);
//! base 1, Allied, at A, with an airfield and 1,000 t; base 2, Japanese,
//! at B, with 100 t; air group 1, Allied, at base 1, 6 ready aircraft
//! that can search 4 hexes or more, at rest; task force 1, two
//! placeholder troop transports of 600 berths, stopped at base 1; units 1
//! and 2, Allied, 600 men each at A; unit 10, Japanese, 150 men at B. No
//! estimate is held. The game is 5 days.
//!
//! The Allied side's commands of day 1: intent 1, take the place at base
//! 2, with a forces limit of task force 1, units 1 and 2, air group 1
//! and base 1 as the source; no risk, so balanced; and the limit
//! "knowledge: a recent count, 1 day" on intent 1. Leave to prepare and
//! leave to launch are both given.
//!
//! ## Day by day
//!
//! | Day | The need that morning | The staff's commands and orders | What happens |
//! | --- | --- | --- | --- |
//! | 1 | Asked: a count no more than 1 day old. Held: nothing. Looking: air group 1, since day 1. Not met | Approves plan 1 to preparing. Forms intent 2, ready a landing by task force 1 at B, with a forces limit of units 1 and 2. Orders `prepare_landing` at B, roster 1 and 2. Sets air group 1 to reconnaissance of B | In the morning one aircraft reports 150 men, seen from the air. Loading 100; each unit's rehearsal 100 |
//! | 2 | Held: 150, seen from the air, of day 1, 1 day old; it could be 150 to 150. Looking: none. Met | Sets the forces limit of intent 2 to unit 1. Orders `prepare_landing` at B, roster 1. Sets air group 1 to rest | The landing was ordered again this morning and is not ready this morning |
//! | 3 | Held: the same, 2 days old. Looking: air group 1, since day 3. Not met | Notice: intent 2 is ready. The plan is not released: the reason says the count is 2 days old and one no more than 1 day old is asked. Sets air group 1 to reconnaissance of B | In the morning one aircraft reports 150 men |
//! | 4 | Held: 150, seen from the air, of day 3, 1 day old. Looking: none. Met | Approves plan 1 to released. Orders a load of supply to a weight of 45 t. Sets air group 1 to rest | |
//! | 5 | The same, 2 days old: the plan is released, and the need is no longer asked | Unit 1 set to attack. Load unit 1 | |
//!
//! - **Day 1.** With no estimate the staff takes every unit the ships can
//!   carry and says it goes with no count.
//! - **Day 2.** At balanced the men wanted are 150 x 2 = 300. Unit 1 has
//!   600 and is enough; unit 2 is left out.
//! - **Day 3.** The second count is the same, so the choice of units is
//!   not made again.
//! - **Day 4.** The supply is 600 x 0.005 x 15 = 45 t.
//!
//! Nine orders in all: two on each of days 1, 2, 4 and 5 and one on day
//! 3. Air group 1 flies twice, on days 1 and 3. No staff order or command
//! is refused. Beside scenario 72's balanced case, which goes on day 3
//! after one flight, the fresher count costs a day and a flight.
//!
//! # Properties
//!
//! - **Not set.** With no knowledge limit: the game of scenario 72's
//!   "Balanced" case. The plan is released on day 3 and air group 1 flies
//!   once. Days 1 to 3 are asserted.
//! - **A count, at preserve the force.** With the risk preserve and
//!   "knowledge: a count": the men wanted on day 2 are 150 x 3 = 450,
//!   unit 1 alone; the plan is released on day 3 on the count of day 1,
//!   2 days old, and it would be released on that count at any age. Days
//!   1 to 3 are asserted.
//! - **None, at preserve the force.** With the risk preserve and
//!   "knowledge: none": no group is sent on any day. The landing is ready
//!   on the morning of day 2 with units 1 and 2: the plan is released,
//!   the reason says it goes on no count, and the load is to 90 t. Days 1
//!   and 2 are asserted.
//! - **Told is not a count.** With an opening estimate of 100 men at B,
//!   told, and "knowledge: a count": on day 1 the need is not met, what
//!   is held is 100, told, with no range, and air group 1 is sent.
//! - **The range.** In the staff's own tests, on the decision's figures:
//!   a count of 3,000 seen from the air, with the range 0.25 to 2, could
//!   be 1,500 to 12,000; by men in contact, with 0.5 to 1.5, 2,000 to
//!   6,000; a count of 100 with the range 0.25 to 2 could be 50 to 400;
//!   a count of 7 with 0.5 to 1.5 could be 4 to 14.
//! - **Refused at intake.** The limit on a "hold" is refused. A recent
//!   count of 0 days is refused.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see and has not been told give the same
//!   orders, reasons and needs; a save on day 3 and a resume give the
//!   same game; with the limit not set, scenarios 67 to 81 give the
//!   orders they gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::belief::GroundGrade;
use sim_core::choices::Moment;
use sim_core::command::{
    CommandKind, CommandRefusal, CountRange, HeldCount, IntentKind, Knowledge, Limit, Looking,
    NeedToKnow, NoticeKind, PlanId, PlanState, Reason, Staff, StaffDecisions, StaffOrder, Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{IntentId, TaskForceId};
use sim_core::orders::{
    AirGroupOrder, AirMission, Cargo, LandUnitOrder, OrderKind, Posture, TaskForceOrder,
};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs, SupplyStaff};

use crate::common;
use crate::common::replace_once as changed;
use crate::common::values::{assert_says, group, intent, n, text_of};

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const KNOWLEDGE_MARKER: &str = "# The knowledge limit\n";
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/82_how_much_must_be_known.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

/// The Allied side's estimate of the enemy at B: men, grade, day.
type Estimate = Option<(u32, GroundGrade, u32)>;

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    /// The need to know of each morning, worked from the view the staff is
    /// asked on, as a seat's `plans` works it.
    needs: BTreeMap<u32, Vec<NeedToKnow>>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
    /// The Allied estimate at B after each day: index 0 before day 1.
    estimates: Vec<Estimate>,
}

fn estimate_of(scenario: &Scenario) -> Estimate {
    scenario
        .world
        .side_view(Side::Allied)
        .ground_estimate(B)
        .map(|e| (e.men, e.grade, e.day))
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new(), BTreeMap::new())
}

fn play_from(
    mut scenario: Scenario,
    mut handed: BTreeMap<u32, StaffDecisions>,
    mut needs: BTreeMap<u32, Vec<NeedToKnow>>,
) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    let mut estimates = vec![estimate_of(&scenario)];
    loop {
        let day = scenario.world.day() + 1;
        let mut morning: Option<Vec<NeedToKnow>> = None;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                if let Moment::Orders { .. } = moment {
                    morning = Some(SupplyStaff.needs_to_know(world.side_view(Side::Allied)));
                }
            })
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
        needs.insert(day, morning.expect("the staff's moment was passed"));
        handed.insert(
            day,
            scenario
                .world
                .staff_handed_in()
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
        truth.push(scenario.world.umpire().clone());
        estimates.push(estimate_of(&scenario));
    }
    Played {
        handed,
        needs,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
        estimates,
    }
}

/// An order of the staff, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Landing(u32, Hex, Vec<u32>),
    Look(u32, Hex),
    Rest(u32),
    LoadTo(Fixed),
    Attack(u32),
    LoadUnit(u32),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce {
            target,
            order: TaskForceOrder::PrepareLanding { hex, roster },
        } => Given::Landing(target.raw(), *hex, roster.iter().map(|u| u.raw()).collect()),
        OrderKind::AirGroup {
            target,
            order: AirGroupOrder::SetAirMission(AirMission::Reconnaissance(hex)),
        } => Given::Look(target.raw(), *hex),
        OrderKind::AirGroup {
            target,
            order: AirGroupOrder::SetAirMission(AirMission::Rest),
        } => Given::Rest(target.raw()),
        OrderKind::TaskForce {
            order:
                TaskForceOrder::LoadTo {
                    cargo: Cargo::Supply,
                    weight_t,
                    ..
                },
            ..
        } => Given::LoadTo(*weight_t),
        OrderKind::LandUnit {
            target,
            order:
                LandUnitOrder::SetPosture {
                    posture: Posture::Attack,
                    when_spent: None,
                },
        } => Given::Attack(target.raw()),
        OrderKind::TaskForce {
            order: TaskForceOrder::LoadUnit { unit, cargo: None },
            ..
        } => Given::LoadUnit(unit.raw()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

/// A staff command, plainly, by kind.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Cmd {
    Approve(u32),
    Form(u32, IntentKind),
    Limit(u32, Vec<u32>),
}

fn cmd(c: &CommandKind) -> Cmd {
    match c {
        CommandKind::Approve { plan } => Cmd::Approve(plan.raw()),
        CommandKind::FormIntent { parent, kind } => Cmd::Form(parent.raw(), *kind),
        CommandKind::SetLimit(Limit::Forces { intent, limit }) => {
            assert!(limit.ships.is_empty() && limit.task_forces.is_empty());
            assert!(limit.air_groups.is_empty() && limit.source.is_none());
            Cmd::Limit(
                intent.raw(),
                limit.land_units.iter().map(|u| u.raw()).collect(),
            )
        }
        other => panic!("the staff handed in another command: {other:?}"),
    }
}

fn landing_kind() -> IntentKind {
    IntentKind::ReadyLanding {
        force: TaskForceId::new(1),
        hex: B,
        by_day: None,
    }
}

impl Played {
    fn days(&self) -> u32 {
        u32::try_from(self.handed.len()).expect("few days")
    }

    /// The orders of `day`, in the order the staff gave them.
    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().map(given).collect()
    }

    /// The orders of `day`, sorted, for a day whose two orders come from two
    /// doctrines.
    fn orders_sorted(&self, day: u32) -> Vec<Given> {
        let mut all = self.orders(day);
        all.sort_by_key(|o| format!("{o:?}"));
        all
    }

    fn all_orders(&self) -> Vec<(u32, Given)> {
        (1..=self.days())
            .flat_map(|d| self.orders(d).into_iter().map(move |g| (d, g)))
            .collect()
    }

    fn commands(&self, day: u32) -> Vec<Cmd> {
        self.handed[&day]
            .commands
            .iter()
            .map(|c| cmd(&c.kind))
            .collect()
    }

    fn all_commands(&self) -> Vec<(u32, Cmd)> {
        (1..=self.days())
            .flat_map(|d| self.commands(d).into_iter().map(move |c| (d, c)))
            .collect()
    }

    fn command_reason(&self, day: u32, wanted: &Cmd) -> &Reason {
        &self.handed[&day]
            .commands
            .iter()
            .find(|c| cmd(&c.kind) == *wanted)
            .unwrap_or_else(|| panic!("day {day} has the command {wanted:?}"))
            .reason
    }

    fn all_notices(&self) -> Vec<(u32, NoticeKind, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .map(move |nt| (d, nt.kind, nt.plan.raw()))
            })
            .collect()
    }

    /// The no-order reasons of `day` serving `of`.
    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect()
    }

    fn plan_day(&self, plan: u32, to: PlanState) -> Option<u32> {
        self.events.iter().find_map(|e| match &e.kind {
            EventKind::PlanStateChanged {
                side,
                plan: p,
                to: t,
                ..
            } if *side == Side::Allied && p.raw() == plan && *t == to => Some(e.day),
            _ => None,
        })
    }

    /// The days and pulses of air group 1's flights.
    fn flights(&self) -> Vec<(u32, Option<Pulse>)> {
        self.events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::ReconnaissanceFlown { .. }))
            .map(|e| (e.day, e.pulse))
            .collect()
    }

    /// The Allied ground reports of B: day, pulse, men, grade, replaced.
    fn reports(&self) -> Vec<(u32, Option<Pulse>, u32, GroundGrade, bool)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::GroundReportReceived {
                    side: Side::Allied,
                    hex,
                    men,
                    replaced,
                    grade,
                } if *hex == B => Some((e.day, e.pulse, *men, *grade, *replaced)),
                _ => None,
            })
            .collect()
    }

    fn refused(&self) -> Vec<&Event> {
        self.events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect()
    }

    fn command_refusals(&self) -> Vec<(u32, CommandRefusal)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused { reason, .. } => Some((e.day, *reason)),
                _ => None,
            })
            .collect()
    }

    /// The one need of a morning: plan 1's.
    fn need(&self, day: u32) -> &NeedToKnow {
        let all = &self.needs[&day];
        assert_eq!(
            all.len(),
            1,
            "one need on the morning of day {day}: {all:?}"
        );
        &all[0]
    }
}

/// The scenario with the knowledge limit's setting replaced.
fn knowledge(t: &str, lines: &str) -> String {
    changed(t, "knowledge = \"recent_count\"\nwithin_days = 1\n", lines)
}

/// The scenario with no knowledge limit.
fn without_knowledge(t: &str) -> String {
    let start = t.find(KNOWLEDGE_MARKER).expect("the marker");
    let end = start + t[start..].find("\n\n").expect("the end of the table") + 2;
    format!("{}{}", &t[..start], &t[end..])
}

/// The scenario at a risk level, set on intent 1.
fn at_risk(t: &str, level: &str) -> String {
    changed(
        t,
        KNOWLEDGE_MARKER,
        &format!(
            "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n\n{KNOWLEDGE_MARKER}"
        ),
    )
}

/// The scenario ended after `day`: a property asserts the days to it.
fn through_day(t: &str, day: u32) -> String {
    changed(t, "days = 5\n", &format!("days = {day}\n"))
}

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

/// What is asked in the main case: a count no more than 1 day old, set on
/// intent 1.
fn asked_main() -> (Knowledge, Option<IntentId>) {
    (Knowledge::RecentCount(1), Some(intent(1)))
}

fn seen_150(day: u32, age_days: u32) -> HeldCount {
    HeldCount {
        men: 150,
        grade: GroundGrade::SeenFromAir,
        day,
        age_days,
        // Scenario 72's figures: the error range is 1 to 1.
        could_be: Some(CountRange {
            lowest: 150,
            highest: Some(150),
        }),
    }
}

fn group_1_since(day: u32) -> Option<Looking> {
    Some(Looking {
        air_group: group(1),
        since_day: Some(day),
    })
}

/// The orders of days 1 to 3 of scenario 72's main case.
fn scenario_72_orders_to_day_3() -> Vec<(u32, Given)> {
    vec![
        (1, Given::Landing(1, B, vec![1, 2])),
        (1, Given::Look(1, B)),
        (2, Given::Landing(1, B, vec![1])),
        (2, Given::Rest(1)),
        (3, Given::LoadTo(n(45))),
    ]
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn nine_orders_and_none_refused() {
    let g = main_game();
    assert_eq!(g.days(), 5);
    assert_eq!(
        g.orders(1),
        vec![Given::Landing(1, B, vec![1, 2]), Given::Look(1, B)]
    );
    assert_eq!(
        g.orders(2),
        vec![Given::Landing(1, B, vec![1]), Given::Rest(1)]
    );
    assert_eq!(g.orders(3), vec![Given::Look(1, B)]);
    assert_eq!(g.orders_sorted(4), {
        let mut v = vec![Given::LoadTo(n(45)), Given::Rest(1)];
        v.sort_by_key(|o| format!("{o:?}"));
        v
    });
    assert_eq!(g.orders_sorted(5), {
        let mut v = vec![Given::Attack(1), Given::LoadUnit(1)];
        v.sort_by_key(|o| format!("{o:?}"));
        v
    });
    assert_eq!(g.all_orders().len(), 9);
    assert_eq!(
        g.all_commands(),
        vec![
            (1, Cmd::Approve(1)),
            (1, Cmd::Form(1, landing_kind())),
            (1, Cmd::Limit(2, vec![1, 2])),
            (2, Cmd::Limit(2, vec![1])),
            (4, Cmd::Approve(1)),
        ]
    );
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn the_need_each_morning() {
    let g = main_game();
    let (asked, asked_from) = asked_main();
    let need = |held, looking, met| NeedToKnow {
        plan: PlanId::new(1),
        intent: intent(1),
        place: B,
        asked,
        asked_from,
        held,
        looking,
        moved_to_look: None,
        met,
    };
    // Day 1: nothing held; air group 1 looks from day 1; not met.
    assert_eq!(*g.need(1), need(None, group_1_since(1), false));
    // Day 2: 150, seen from the air, of day 1, 1 day old, 150 to 150; none
    // looking; met.
    assert_eq!(*g.need(2), need(Some(seen_150(1, 1)), None, true));
    // Day 3: the same, 2 days old; air group 1 looks from day 3; not met.
    assert_eq!(
        *g.need(3),
        need(Some(seen_150(1, 2)), group_1_since(3), false)
    );
    // Day 4: 150, seen from the air, of day 3, 1 day old; none looking; met.
    assert_eq!(*g.need(4), need(Some(seen_150(3, 1)), None, true));
    // Day 5: the plan is released, and the need is no longer asked.
    assert!(g.needs[&5].is_empty(), "{:?}", g.needs[&5]);
}

#[test]
fn the_flights_and_the_reports() {
    let g = main_game();
    // Air group 1 flies twice, on days 1 and 3, in the morning.
    assert_eq!(
        g.flights(),
        vec![(1, Some(Pulse::Morning)), (3, Some(Pulse::Morning))]
    );
    assert_eq!(
        g.reports(),
        vec![
            (
                1,
                Some(Pulse::Morning),
                150,
                GroundGrade::SeenFromAir,
                false
            ),
            (3, Some(Pulse::Morning), 150, GroundGrade::SeenFromAir, true),
        ]
    );
    assert_eq!(g.estimates[0], None);
    assert_eq!(g.estimates[1], Some((150, GroundGrade::SeenFromAir, 1)));
    assert_eq!(g.estimates[2], Some((150, GroundGrade::SeenFromAir, 1)));
    assert_eq!(g.estimates[3], Some((150, GroundGrade::SeenFromAir, 3)));
}

#[test]
fn day_1_no_count_and_the_group_sent() {
    let g = main_game();
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    let form = text_of(g.command_reason(1, &Cmd::Form(1, landing_kind())));
    assert_says(
        &form,
        &[
            "holds no estimate of the enemy's men",
            "every unit the ships can carry",
            "no count of the enemy",
        ],
    );
    let look = text_of(
        g.handed[&1]
            .orders
            .iter()
            .find(|o| given(o) == Given::Look(1, B))
            .map(|o| &o.reason)
            .expect("the order to look"),
    );
    assert_says(
        &look,
        &[
            "the knowledge limit set on intent:1",
            "no more than 1 day old on the morning it goes",
            "reconnaissance of (4,0)",
        ],
    );
    // Loading 100; each unit's rehearsal 100.
    let force = &g.truth[1].task_forces[&TaskForceId::new(1)];
    assert_eq!(force.preparation.as_ref().map(|p| p.loading), Some(n(100)));
}

#[test]
fn day_2_unit_1_is_enough_and_the_group_rests() {
    let g = main_game();
    // At balanced the men wanted are 150 x 2 = 300: unit 1 has 600.
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(
        &limit,
        &[
            "150 men, seen from the air, of day 1",
            "wants 2 men for each, 300 men",
        ],
    );
    // The landing was ordered again this morning and is not ready.
    let not_ready = g.no_orders(2, 1).join(" ");
    assert_says(&not_ready, &["The landing at (4,0) is not ready"]);
    assert_eq!(g.truth[2].air_groups[&group(1)].mission, AirMission::Rest);
}

#[test]
fn day_3_ready_but_the_count_is_a_day_too_old() {
    let g = main_game();
    // The notice that intent 2 is ready; the plan is not released.
    assert_eq!(
        g.all_notices()
            .into_iter()
            .filter(|(d, _, _)| *d == 3)
            .collect::<Vec<_>>(),
        vec![(3, NoticeKind::Ready, 1)]
    );
    assert!(g.commands(3).is_empty());
    let reasons = g.no_orders(3, 1).join(" ");
    assert_says(
        &reasons,
        &[
            "The landing at (4,0) is ready",
            "150 men, seen from the air, of day 1, 2 days old",
            "no more than 1 day old on the morning it goes",
            "does not release plan:1",
        ],
    );
    // The choice of units is not made again: nothing new is held.
    assert!(!g.commands(3).iter().any(|c| matches!(c, Cmd::Limit(..))));
    assert_eq!(
        g.truth[3].air_groups[&group(1)].mission,
        AirMission::Reconnaissance(B)
    );
}

#[test]
fn day_4_released_with_45_t_and_the_second_count_changes_nothing() {
    let g = main_game();
    assert_eq!(g.plan_day(1, PlanState::Released), Some(4));
    let release = text_of(g.command_reason(4, &Cmd::Approve(1)));
    assert_says(
        &release,
        &[
            "It goes on the count held",
            "150 men, seen from the air, of day 3",
        ],
    );
    // The second count is the same: the units are not chosen again.
    assert!(!g.commands(4).iter().any(|c| matches!(c, Cmd::Limit(..))));
    // The supply is 600 x 0.005 x 15 = 45 t.
    let ships: Fixed = g.truth[4]
        .ships
        .values()
        .fold(Fixed::ZERO, |sum, s| sum + s.supply_t);
    assert_eq!(ships, n(45));
}

#[test]
fn day_5_unit_1_is_set_to_attack_and_loaded() {
    let g = main_game();
    assert_eq!(
        g.truth[5].land_units[&sim_core::ids::LandUnitId::new(1)].posture,
        Posture::Attack
    );
    // The game ends before the landing.
    assert_eq!(
        g.truth[5].land_units[&sim_core::ids::LandUnitId::new(10)].men_ashore,
        150
    );
}

// ---------------------------------------------------------------- Properties

#[test]
fn not_set() {
    // Scenario 72's balanced case: released on day 3, one flight. Days 1 to
    // 3 are asserted.
    let g = play(&through_day(&without_knowledge(&text()), 3));
    let to_day_3: Vec<(u32, Given)> = g
        .all_orders()
        .into_iter()
        .filter(|(d, _)| *d <= 3)
        .collect();
    assert_eq!(to_day_3, scenario_72_orders_to_day_3());
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
    assert_eq!(g.flights(), vec![(1, Some(Pulse::Morning))]);
    // The need is asked by the risk: at balanced, a count of any day.
    assert_eq!(g.need(1).asked, Knowledge::Count);
    assert_eq!(g.need(1).asked_from, None);
    assert!(g.need(3).met);
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(&limit, &["wants 2 men for each, 300 men"]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn a_count_at_preserve_the_force() {
    let g = play(&through_day(
        &at_risk(&knowledge(&text(), "knowledge = \"count\"\n"), "preserve"),
        3,
    ));
    // The men wanted on day 2 are 150 x 3 = 450: unit 1 alone.
    let limit = text_of(g.command_reason(2, &Cmd::Limit(2, vec![1])));
    assert_says(&limit, &["wants 3 men for each, 450 men"]);
    let to_day_3: Vec<(u32, Given)> = g
        .all_orders()
        .into_iter()
        .filter(|(d, _)| *d <= 3)
        .collect();
    assert_eq!(to_day_3, scenario_72_orders_to_day_3());
    // Released on day 3 on the count of day 1, 2 days old.
    assert_eq!(g.plan_day(1, PlanState::Released), Some(3));
    let day_3 = g.need(3);
    assert_eq!(day_3.asked, Knowledge::Count);
    assert_eq!(day_3.asked_from, Some(intent(1)));
    assert_eq!(day_3.held, Some(seen_150(1, 2)));
    assert!(day_3.met);
    assert_eq!(g.flights(), vec![(1, Some(Pulse::Morning))]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn none_at_preserve_the_force() {
    let g = play(&through_day(
        &at_risk(&knowledge(&text(), "knowledge = \"none\"\n"), "preserve"),
        2,
    ));
    // No group is sent on any day.
    assert!(!g
        .all_orders()
        .iter()
        .any(|(_, o)| matches!(o, Given::Look(..) | Given::Rest(..))));
    assert!(g.flights().is_empty());
    assert_eq!(g.orders(1), vec![Given::Landing(1, B, vec![1, 2])]);
    // Ready on the morning of day 2 with units 1 and 2: released, on no
    // count, with the load to 90 t.
    assert_eq!(g.plan_day(1, PlanState::Released), Some(2));
    let release = text_of(g.command_reason(2, &Cmd::Approve(1)));
    assert_says(&release, &["It goes on no count of the enemy at (4,0)"]);
    assert_eq!(g.orders(2), vec![Given::LoadTo(n(90))]);
    for day in 1..=2 {
        let need = g.need(day);
        assert_eq!(need.asked, Knowledge::NoCount);
        assert_eq!(need.held, None);
        assert_eq!(need.looking, None);
        assert!(need.met);
    }
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn told_is_not_a_count() {
    let t = changed(
        &knowledge(&text(), "knowledge = \"count\"\n"),
        COMMANDS_MARKER,
        "[[estimate]]\nside = \"allied\"\nq = 4\nr = 0\nmen = 100\n\n# The Allied commander's commands",
    );
    let g = play(&through_day(&t, 1));
    let day_1 = g.need(1);
    assert_eq!(day_1.asked, Knowledge::Count);
    assert!(!day_1.met);
    assert_eq!(
        day_1.held,
        Some(HeldCount {
            men: 100,
            grade: GroundGrade::Told,
            day: 0,
            age_days: 1,
            could_be: None,
        })
    );
    assert_eq!(day_1.looking, group_1_since(1));
    assert!(g.orders(1).contains(&Given::Look(1, B)));
}

#[test]
fn refused_at_intake() {
    let set = |body: &str| {
        format!("[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\n{body}\n")
    };
    let extra = [
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 5\nintent_kind = \"hold\"\nbase = 1\n".to_string(),
        set("limit = \"knowledge\"\nintent = 5\nknowledge = \"count\""),
        set("limit = \"knowledge\"\nintent = 1\nknowledge = \"recent_count\"\nwithin_days = 0"),
    ]
    .join("\n");
    let g = play(&format!("{}\n{extra}", through_day(&text(), 1)));
    assert_eq!(
        g.command_refusals(),
        vec![
            (1, CommandRefusal::NoKnowledgeLimitOn(intent(5))),
            (1, CommandRefusal::RecentCountOfNoDays(intent(1))),
        ]
    );
}

// ---------------------------------------------------------------- Determinism

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(&a.handed, &main_game().handed);
    assert_eq!(a.needs, main_game().needs);
    assert_eq!(a.hashes, main_game().hashes);
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    // An enemy unit far from B, unseen and untold: two worlds that differ
    // only in its size give the same orders, reasons and needs.
    let with = |men: u32| {
        changed(
            &text(),
            COMMANDS_MARKER,
            &format!(
                "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 0\nr = 6\n\n{COMMANDS_MARKER}"
            ),
        )
    };
    let a = play(&with(100));
    let b = play(&with(300));
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.needs, b.needs);
    assert_eq!(a.all_orders(), main_game().all_orders());
    assert_eq!(a.needs, main_game().needs);
}

/// A game rebuilt from what was handed in to day `upto`, as a saved game
/// replays.
fn rebuilt(handed: &BTreeMap<u32, StaffDecisions>, upto: u32) -> Scenario {
    let mut scenario = load(&text());
    for day in 1..=upto {
        scenario
            .world
            .submit_staff(Side::Allied, handed[&day].clone());
        scenario
            .next_day_watched(Staffs::none(), &mut |_, _| {})
            .expect("a day resolves")
            .expect("a day to resolve");
    }
    scenario
}

#[test]
fn saved_on_day_3_and_resumed() {
    let through = main_game();
    let at_3 = rebuilt(&through.handed, 3);
    assert_eq!(at_3.world.day_hashes(), &through.hashes[..3]);
    let first: BTreeMap<u32, StaffDecisions> =
        (1..=3).map(|d| (d, through.handed[&d].clone())).collect();
    let needs: BTreeMap<u32, Vec<NeedToKnow>> =
        (1..=3).map(|d| (d, through.needs[&d].clone())).collect();
    let resumed = play_from(at_3, first, needs);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
    assert_eq!(resumed.needs, through.needs);
}
