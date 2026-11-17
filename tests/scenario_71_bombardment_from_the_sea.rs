//! Scenario 71: bombardment from the sea. The commander gives a
//! battleship the task of bombarding an enemy base. The staff sends her
//! in the night, brings her home at the morning, and stops when her
//! rounds are below the level her risk keeps.
//!
//! Proves decision 0146, step 5 of stage 3, second part, against a hand
//! calculation. The data is placeholder data: see
//! `data/scenarios/71_bombardment_from_the_sea.toml`. It runs on the
//! frozen step 1 figures, which give no fuelling rate, so a ship in port
//! is filled in the pulse, and no ammunition in port. Three bombardment
//! figures are its own: `bombardment_minutes` 5, `bombardment_hit_chance`
//! 0 and `bombardment_false_claim_chance` 0. No shell hits and nothing is
//! claimed, so no die changes anything.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the rule is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! ## Figures
//!
//! A hex is 40 nm. Day d begins at hour 24 x (d - 1): a night pulse of 12
//! hours and two daylight pulses of 6. The placeholder battleship (class
//! 9): 16 kn at cruise, 2,000 t of fuel over 8,000 nm, which is 0.25 t a
//! mile; 8 guns at 3 rounds a minute, 100 rounds a gun, an outfit of 800.
//! In a night's bombardment she fires 3 x 8 x 5 = 120 shells. At balanced
//! a ship is short of rounds below half her outfit, 400; at press below
//! 200; at preserve the force below 600. At balanced a sortie needs one
//! and a fifth times the fuel of the passage out and home.
//!
//! ## Inputs
//!
//! The placeholder map of scenario 27: A at (0,0), port size 5, and B at
//! (4,0), 160 nm away. The game is 6 days.
//!
//! Base 1, Allied, at A, with 5,000 t of fuel. Base 2, Japanese, at B, on
//! the Allied map, with no gun, aircraft, supply or man. No contact is
//! held. There is no Japanese unit.
//!
//! Task force 1, Allied, on the bombardment mission: ship 1, a
//! placeholder battleship, stopped at base 1 at cruising speed, full of
//! fuel and rounds, at normal readiness, with no home port and the stance
//! avoid.
//!
//! The Allied side's commands of day 1: intent 1, bombard base 2 with
//! task force 1. No risk is set, so balanced. Leave to prepare and leave
//! to launch are both given.
//!
//! ## The sortie
//!
//! A to B is 160 nm: 10 hours at 16 kn, inside the night's 12. Out and
//! home is 320 nm, 80 t; at balanced the ship needs 96 t.
//!
//! A sortie ordered on day d: the force sails at the start of the night,
//! reaches B at hour 10 of it and halts to bombard. It fires 120 shells
//! in that night's bombardment phase. At the start of the morning it
//! steams on for A: in the morning's 6 hours it makes 96 nm, and it
//! arrives 4 hours into the afternoon, 22 hours after it sailed. The ship
//! takes 80 t at A in that pulse.
//!
//! ## Day by day
//!
//! | Day | Rounds at the staff's hour | The staff's commands and orders | Rounds after the night |
//! | --- | --- | --- | --- |
//! | 1 | 800 | Approves plan 1 to preparing and to released. Home port, base 1. Rules of engagement: engage, break-off level 0.5. Battle readiness. Route to B and A, B marked to act at | 680 |
//! | 2 | 680 | Route, as day 1 | 560 |
//! | 3 | 560 | Route | 440 |
//! | 4 | 440 | Route | 320 |
//! | 5 | 320, below 400 | Normal readiness. Notice: cannot meet intent 1 | 320 |
//! | 6 | 320 | none. Notice: cannot meet intent 1 | 320 |
//!
//! The staff gives 4, 1, 1, 1, 1 and 0 orders: 8 in all. No speed order
//! is given: the force is at cruise. The force halts at B at hours 10,
//! 34, 58 and 82, each to bombard, and steams on at hours 12, 36, 60 and
//! 84. Base 1 holds 4,680 t of fuel at the end.
//!
//! The reasons of days 1 to 4 give base 2 at 4 hexes, 10 hours to it,
//! 2,000 t aboard against 96 needed, and the ship's rounds against 400.
//! The reasons of days 5 and 6 give 320 rounds against 400 and no day
//! from which the force may be used: base 1 holds no ammunition. Every
//! order serves intent 1 and was decided at the level of task force 1.
//! No staff order is refused.
//!
//! # Properties
//!
//! - **Press.** With intent 1 at press: the level is 200. After the
//!   night of day 5 the ship has 200 rounds, which is not below it: she
//!   sorties on all six nights and ends with 80 rounds. The break-off
//!   level ordered is 0.75. Orders: 4, then 1 on each of days 2 to 6.
//! - **Preserve the force.** With intent 1 at preserve the force: the
//!   level is 600. She sorties on days 1 and 2 and has 560 rounds on the
//!   morning of day 3: no sortie, normal readiness, and the notice from
//!   that morning. The break-off level ordered is 0.25 and the fuel
//!   needed 120 t. Orders: 4, 1, 1.
//! - **Preserve the force, and a contact.** At preserve the force, with
//!   the Allied side holding a contact of 2 ships at (5,0) that does not
//!   age out before day 2: on day 1 there is no sortie, the reason gives
//!   2 ships against 1, and day 1 has two orders, the home port and the
//!   rules of engagement.
//! - **Under neutralise.** With the commander's intent 1 "neutralise base
//!   2" and a forces limit of task force 1: the staff forms intent 2,
//!   bombard base 2 with task force 1, under it on day 1, and the force's
//!   orders are those of the main case, serving intent 2.
//! - **Under take a place.** With the inputs of scenario 69's property
//!   "No hold stated" added (base 1 with 1,000 t of supply, task force 2
//!   of two placeholder troop transports, units 1 and 2 of 600 men, the
//!   commander's intent 1 "take the place at base 2") and task force 1
//!   named in that intent's forces limit: the staff forms the bombard
//!   task on the morning of day 2, when plan 1 is released, and gives the
//!   battleship her home port and her rules of engagement. The landing
//!   force is routed on day 5 and is due that day: on the morning of day
//!   5 the battleship is set to battle readiness and routed, and fires
//!   120 shells in that night. Base 2 changes side at the end of day 5.
//!   On the morning of day 6 the target is the side's: she is set to
//!   normal readiness and given nothing more. The transports' orders are
//!   the eight of that property.
//! - **Held.** With task force 1 held from day 3: no order from day 3,
//!   and the reason names the hold.
//! - The same start gives the same orders; two worlds that differ only in
//!   what the Allied side cannot see give the same orders and reasons; a
//!   save on day 3 and a resume give the same game; every scenario built
//!   before this one gives the events it gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{
    Addressee, CommandKind, IntentKind, NoticeKind, PlanState, Reason, StaffDecisions, StaffOrder,
    Staffs,
};
use sim_core::events::{Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::hex::Hex;
use sim_core::ids::{BaseId, IntentId, ShipId, TaskForceId};
use sim_core::orders::{OrderKind, Readiness, SpeedSetting, Stance, TaskForceOrder};
use sim_core::state::{Side, TruthState};
use sim_core::time::Pulse;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";
const A: Hex = Hex { q: 0, r: 0 };
const B: Hex = Hex { q: 4, r: 0 };

fn path() -> PathBuf {
    common::data_dir().join("scenarios/71_bombardment_from_the_sea.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The frozen step 1 figures. The scenario names its own surface combat
/// figures, `data/reference/surface_combat_placeholder_scenario_71.toml`,
/// frozen beside the others: the three bombardment figures its hand
/// calculation names.
fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), &common::frozen()).expect("the scenario loads")
}

struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    hashes: Vec<u64>,
}

fn play(text: &str) -> Played {
    play_from(load(text), BTreeMap::new())
}

fn play_from(mut scenario: Scenario, mut handed: BTreeMap<u32, StaffDecisions>) -> Played {
    let held = staffs();
    let mut truth = vec![scenario.world.umpire().clone()];
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        if summary.is_none() {
            break;
        }
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
    }
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        hashes: scenario.world.day_hashes().to_vec(),
    }
}

/// An order of the staff to task force 1, plainly.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    HomePort(u32),
    Roe(Stance, Fixed),
    Ready(Readiness),
    Speed(SpeedSetting),
    Route(Vec<Hex>, Vec<Hex>),
}

fn given(o: &StaffOrder) -> Option<Given> {
    let OrderKind::TaskForce { target, order } = &o.kind else {
        return None;
    };
    if target.raw() != 1 {
        return None;
    }
    Some(match order {
        TaskForceOrder::SetHomePort(base) => Given::HomePort(base.raw()),
        TaskForceOrder::SetRulesOfEngagement {
            stance, break_off, ..
        } => Given::Roe(*stance, *break_off),
        TaskForceOrder::SetReadiness(r) => Given::Ready(*r),
        TaskForceOrder::SetSpeed(s) => Given::Speed(*s),
        TaskForceOrder::SetRoute {
            waypoints,
            act_at,
            stand_at,
        } => {
            assert!(stand_at.is_empty(), "no stand mark");
            Given::Route(waypoints.clone(), act_at.iter().copied().collect())
        }
        other => panic!("the doctrine gave task force 1 another order: {other:?}"),
    })
}

fn route() -> Given {
    Given::Route(vec![B, A], vec![B])
}

fn half() -> Fixed {
    Fixed::from_ratio(1, 2)
}

fn day_1() -> Vec<Given> {
    vec![
        Given::HomePort(1),
        Given::Roe(Stance::Engage, half()),
        Given::Ready(Readiness::Battle),
        route(),
    ]
}

impl Played {
    fn days(&self) -> u32 {
        self.handed.len() as u32
    }

    /// The staff's orders to task force 1 on `day`.
    fn orders(&self, day: u32) -> Vec<Given> {
        self.handed[&day].orders.iter().filter_map(given).collect()
    }

    fn order_counts(&self) -> Vec<usize> {
        (1..=self.days()).map(|d| self.orders(d).len()).collect()
    }

    fn task_orders(&self, day: u32) -> Vec<&StaffOrder> {
        self.handed[&day]
            .orders
            .iter()
            .filter(|o| given(o).is_some())
            .collect()
    }

    fn notices(&self) -> Vec<(u32, NoticeKind, u32)> {
        (1..=self.days())
            .flat_map(|d| {
                self.handed[&d]
                    .notices
                    .iter()
                    .map(move |nt| (d, nt.kind, nt.plan.raw()))
            })
            .collect()
    }

    fn no_orders(&self, day: u32, of: u32) -> Vec<String> {
        self.handed[&day]
            .no_orders
            .iter()
            .filter(|r| r.serves.intent == Some(intent(of)))
            .map(text_of)
            .collect()
    }

    fn rounds(&self, day: u32) -> Fixed {
        self.truth[day as usize].ships[&ShipId::new(1)].gun_rounds
    }

    fn halts(&self) -> Vec<(Hex, Fixed, bool)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceHalted {
                    task_force,
                    hex,
                    work,
                    elapsed_hours,
                } if task_force.raw() == 1 => Some((*hex, *elapsed_hours, work.bombard)),
                _ => None,
            })
            .collect()
    }

    /// The hour each steaming on came, the start of its pulse.
    fn steamed_on(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceSteamedOn { task_force, .. } if task_force.raw() == 1 => {
                    let start = match e.pulse? {
                        Pulse::Night => 0,
                        Pulse::Morning => 12,
                        Pulse::Afternoon => 18,
                    };
                    Some(n(24 * (i64::from(e.day) - 1) + start))
                }
                _ => None,
            })
            .collect()
    }

    fn shells(&self) -> Vec<(u32, Fixed)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::ShoreBombardment {
                    task_force, shells, ..
                } if task_force.raw() == 1 => Some((e.day, *shells)),
                _ => None,
            })
            .collect()
    }

    fn arrivals_home(&self) -> Vec<Fixed> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::TaskForceArrived {
                    task_force,
                    hex,
                    elapsed_hours,
                } if task_force.raw() == 1 && *hex == A => Some(*elapsed_hours),
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

    fn formed(&self) -> Vec<(u32, u32, u32)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::IntentFormed {
                    side: Side::Allied,
                    intent: i,
                    parent: p,
                } => Some((e.day, i.raw(), p.raw())),
                _ => None,
            })
            .collect()
    }
}

fn text_of(reason: &Reason) -> String {
    format!(
        "{} | {} | {}",
        reason.worked,
        reason.binding.join("; "),
        reason
            .evidence
            .iter()
            .map(|e| e.what.clone())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

fn assert_says(all: &str, parts: &[&str]) {
    for part in parts {
        assert!(all.contains(part), "the reason says {part:?}: {all}");
    }
}

fn with_commands(t: &str, extra: &str) -> String {
    format!("{t}\n{extra}")
}

fn risk(level: &str) -> String {
    format!(
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\"\n"
    )
}

const BOMBARD: &str = "intent_kind = \"bombard\"\ntask_force = 1\ntarget = 2\n";

fn main_game() -> &'static Played {
    static GAME: std::sync::OnceLock<Played> = std::sync::OnceLock::new();
    GAME.get_or_init(|| play(&text()))
}

// ---------------------------------------------------------------- The hand calculation

#[test]
fn eight_orders_day_by_day() {
    let g = main_game();
    assert_eq!(g.days(), 6);
    assert_eq!(g.orders(1), day_1());
    for day in 2..=4 {
        assert_eq!(g.orders(day), vec![route()], "day {day}");
    }
    assert_eq!(g.orders(5), vec![Given::Ready(Readiness::Normal)]);
    assert!(g.orders(6).is_empty());
    assert_eq!(g.order_counts(), vec![4, 1, 1, 1, 1, 0]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
    // No speed order: the force is at cruise.
    assert!(!(1..=6).any(|d| g.orders(d).iter().any(|o| matches!(o, Given::Speed(_)))));
}

#[test]
fn the_plan_is_approved_and_released_on_day_1() {
    let g = main_game();
    let approvals: Vec<u32> = g.handed[&1]
        .commands
        .iter()
        .map(|c| match c.kind {
            CommandKind::Approve { plan } => plan.raw(),
            ref other => panic!("another command: {other:?}"),
        })
        .collect();
    assert_eq!(approvals, vec![1, 1]);
    for day in 2..=6 {
        assert!(g.handed[&day].commands.is_empty(), "day {day}");
    }
    let released = g.events.iter().find_map(|e| match &e.kind {
        EventKind::PlanStateChanged { plan, to, .. }
            if plan.raw() == 1 && *to == PlanState::Released =>
        {
            Some(e.day)
        }
        _ => None,
    });
    assert_eq!(released, Some(1));
}

#[test]
fn the_rounds_after_each_night() {
    let g = main_game();
    let after: Vec<Fixed> = (1..=6).map(|d| g.rounds(d)).collect();
    assert_eq!(after, [680, 560, 440, 320, 320, 320].map(n).to_vec());
    assert_eq!(g.shells(), (1..=4).map(|d| (d, n(120))).collect::<Vec<_>>());
}

#[test]
fn the_halts_and_the_steaming_on() {
    let g = main_game();
    assert_eq!(
        g.halts(),
        vec![
            (B, n(10), true),
            (B, n(34), true),
            (B, n(58), true),
            (B, n(82), true)
        ]
    );
    assert_eq!(g.steamed_on(), vec![n(12), n(36), n(60), n(84)]);
    // Home 22 hours after it sailed.
    assert_eq!(g.arrivals_home(), vec![n(22), n(46), n(70), n(94)]);
}

#[test]
fn fuel_at_the_end() {
    let g = main_game();
    assert_eq!(g.truth[6].bases[&BaseId::new(1)].fuel_t, n(4680));
    for day in 1..=6 {
        assert_eq!(g.truth[day].ships[&ShipId::new(1)].fuel_t, n(2000));
    }
}

#[test]
fn the_notices() {
    let g = main_game();
    assert_eq!(
        g.notices(),
        vec![
            (5, NoticeKind::CannotMeet, 1),
            (6, NoticeKind::CannotMeet, 1)
        ]
    );
}

#[test]
fn the_reasons() {
    let g = main_game();
    let rounds = [800, 680, 560, 440];
    for day in 1..=4u32 {
        let route_reason = g
            .task_orders(day)
            .into_iter()
            .find(|o| given(o) == Some(route()))
            .expect("a route");
        assert_says(
            &text_of(&route_reason.reason),
            &[
                "base:2 (Placeholder enemy base B) is at (4,0), 4 hexes from (0,0)",
                "10 hours to the target at cruise",
                "ship:1 has 2000 t aboard against 96 t needed",
                &format!(
                    "ship:1 has {} rounds against the level of 400",
                    rounds[(day - 1) as usize]
                ),
            ],
        );
    }
    for day in [5, 6] {
        let reasons = g.no_orders(day, 1);
        assert_eq!(reasons.len(), 1, "day {day}: {reasons:?}");
        assert_says(
            &reasons[0],
            &[
                "ship:1 has 320 rounds against the 400",
                "no day from which it may be used",
            ],
        );
    }
    for day in 1..=6 {
        for order in g.task_orders(day) {
            assert_eq!(order.intent, intent(1));
            assert_eq!(
                order.reason.serves.level,
                Some(Addressee::TaskForce(TaskForceId::new(1)))
            );
        }
    }
}

// ---------------------------------------------------------------- The properties

#[test]
fn press() {
    let g = play(&with_commands(&text(), &risk("press")));
    let mut first = day_1();
    first[1] = Given::Roe(Stance::Engage, Fixed::from_ratio(3, 4));
    assert_eq!(g.orders(1), first);
    for day in 2..=6 {
        assert_eq!(g.orders(day), vec![route()], "day {day}");
    }
    assert_eq!(g.rounds(5), n(200));
    assert_eq!(g.rounds(6), n(80));
    assert_eq!(g.order_counts(), vec![4, 1, 1, 1, 1, 1]);
}

#[test]
fn preserve_the_force() {
    let g = play(&with_commands(&text(), &risk("preserve")));
    let mut first = day_1();
    first[1] = Given::Roe(Stance::Engage, Fixed::from_ratio(1, 4));
    assert_eq!(g.orders(1), first);
    assert_eq!(g.orders(2), vec![route()]);
    assert_eq!(g.orders(3), vec![Given::Ready(Readiness::Normal)]);
    assert_eq!(g.order_counts(), vec![4, 1, 1, 0, 0, 0]);
    assert_eq!(g.rounds(2), n(560));
    assert_eq!(
        g.notices(),
        (3..=6)
            .map(|d| (d, NoticeKind::CannotMeet, 1))
            .collect::<Vec<_>>()
    );
    let route_reason = g
        .task_orders(1)
        .into_iter()
        .find(|o| given(o) == Some(route()))
        .expect("a route");
    assert_says(&text_of(&route_reason.reason), &["against 120 t needed"]);
}

#[test]
fn preserve_the_force_and_a_contact() {
    let contact = "[[contact]]\nside = \"allied\"\nq = 5\nr = 0\nship_count = 2\nkind = \"cruiser\"\nage_pulses = 0\n";
    let t = changed(
        &with_commands(&text(), &risk("preserve")),
        COMMANDS_MARKER,
        &format!("{contact}\n{COMMANDS_MARKER}"),
    );
    let g = play(&t);
    assert_eq!(
        g.orders(1),
        vec![
            Given::HomePort(1),
            Given::Roe(Stance::Engage, Fixed::from_ratio(1, 4))
        ]
    );
    let reasons = g.no_orders(1, 1);
    assert_eq!(reasons.len(), 1, "{reasons:?}");
    assert_says(&reasons[0], &["2 ship(s) against the 1"]);
    assert!(g.halts().iter().all(|(_, hour, _)| *hour >= n(24)));
}

#[test]
fn under_neutralise() {
    let t = changed(
        &text(),
        BOMBARD,
        "intent_kind = \"neutralise\"\ntarget = 2\n",
    );
    let t = with_commands(
        &t,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]\n",
    );
    let g = play(&t);
    assert_eq!(g.formed(), vec![(1, 2, 1)]);
    let formed: Vec<IntentKind> = g.handed[&1]
        .commands
        .iter()
        .filter_map(|c| match c.kind {
            CommandKind::FormIntent { kind, .. } => Some(kind),
            _ => None,
        })
        .collect();
    assert_eq!(
        formed,
        vec![IntentKind::Bombard {
            force: TaskForceId::new(1),
            target: BaseId::new(2),
        }]
    );
    let main = main_game();
    for day in 1..=6 {
        assert_eq!(g.orders(day), main.orders(day), "day {day}");
        for order in g.task_orders(day) {
            assert_eq!(order.intent, intent(2));
        }
    }
}

/// The inputs of scenario 69's property "No hold stated" beside the
/// battleship: base 1 with 1,000 t of supply, task force 2 of two
/// placeholder troop transports, units 1 and 2 of 600 men, and the
/// commander's intent 1, take the place at base 2, naming task force 1 too.
fn under_take_a_place() -> String {
    let t = text();
    let t = changed(
        &t,
        "    \"../reference/ship_classes_placeholder_battleship.csv\",\n",
        "    \"../reference/ship_classes_placeholder_battleship.csv\",\n    \"../reference/ship_classes_placeholder_transports.csv\",\n",
    );
    let t = changed(
        &t,
        "air_combat = \"../reference/air_combat.toml\"\n",
        "air_combat = \"../reference/air_combat.toml\"\nground = \"../reference/ground_placeholder_scenario_69.toml\"\n",
    );
    let t = changed(
        &t,
        "supply_t = 0\nfuel_t = 5000\n",
        "supply_t = 1000\nfuel_t = 5000\n",
    );
    let more = "[[task_force]]\nid = 2\nname = \"Placeholder transport force 2\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n\
[[ship]]\nid = 2\nname = \"Placeholder troop transport 1\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 2\n\n\
[[ship]]\nid = 3\nname = \"Placeholder troop transport 2\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 2\n\n\
[[land_unit]]\nid = 1\nname = \"Placeholder First\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n\n\
[[land_unit]]\nid = 2\nname = \"Placeholder Second\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n\n";
    let t = changed(&t, COMMANDS_MARKER, &format!("{more}{COMMANDS_MARKER}"));
    let t = changed(&t, BOMBARD, "intent_kind = \"take_place\"\nhex = [4, 0]\n");
    with_commands(
        &t,
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1, 2]\nland_units = [1, 2]\nsource = 1\n",
    )
}

#[test]
fn under_take_a_place_it_fires_on_the_night_the_landing_is_due() {
    let g = play(&under_take_a_place());
    // The bombard task is formed on day 2, when plan 1 is released.
    let formed: Vec<(u32, IntentKind)> = (1..=g.days())
        .flat_map(|d| {
            g.handed[&d]
                .commands
                .iter()
                .filter_map(move |c| match c.kind {
                    CommandKind::FormIntent {
                        kind: kind @ IntentKind::Bombard { .. },
                        ..
                    } => Some((d, kind)),
                    _ => None,
                })
        })
        .collect();
    assert_eq!(
        formed,
        vec![(
            2,
            IntentKind::Bombard {
                force: TaskForceId::new(1),
                target: BaseId::new(2),
            }
        )]
    );
    assert!(g.orders(1).is_empty());
    assert_eq!(
        g.orders(2),
        vec![Given::HomePort(1), Given::Roe(Stance::Engage, half())]
    );
    assert!(g.orders(3).is_empty());
    assert!(g.orders(4).is_empty());
    assert_eq!(g.orders(5), vec![Given::Ready(Readiness::Battle), route()]);
    assert_eq!(g.orders(6), vec![Given::Ready(Readiness::Normal)]);
    assert_eq!(g.shells(), vec![(5, n(120))]);
    let changed_side = g.events.iter().find_map(|e| match &e.kind {
        EventKind::BaseChangedSide { base, .. } if base.raw() == 2 => Some(e.day),
        _ => None,
    });
    assert_eq!(changed_side, Some(5));
    // The transports' orders are the eight of scenario 69's property.
    let transports: Vec<u32> = (1..=g.days())
        .map(|d| {
            g.handed[&d]
                .orders
                .iter()
                .filter(|o| given(o).is_none())
                .count() as u32
        })
        .collect();
    assert_eq!(transports.iter().sum::<u32>(), 8, "{transports:?}");
    assert_eq!(transports, vec![1, 1, 2, 2, 2, 0]);
    assert!(g.refused().is_empty(), "{:?}", g.refused());
}

#[test]
fn held() {
    let hold = "[[command]]\nday = 3\nside = \"allied\"\nkind = \"hold\"\ntask_force = 1\n";
    let g = play(&with_commands(&text(), hold));
    assert_eq!(g.orders(1), day_1());
    assert_eq!(g.orders(2), vec![route()]);
    for day in 3..=6 {
        assert!(g.orders(day).is_empty(), "day {day}");
        let reasons = g.no_orders(day, 1);
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("is held by the commander")),
            "day {day}: {reasons:?}"
        );
    }
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
    let resumed = play_from(at_3, first);
    assert_eq!(resumed.hashes, through.hashes);
    assert_eq!(resumed.handed, through.handed);
}

#[test]
fn the_same_start_gives_the_same_orders() {
    let a = play(&text());
    assert_eq!(&a.handed, &main_game().handed);
    assert_eq!(a.hashes, main_game().hashes);
}

#[test]
fn what_the_side_cannot_see_changes_nothing() {
    let enemy = |men: u32| {
        format!(
            "[[land_unit]]\nid = 9\nname = \"Placeholder enemy\"\nside = \"japanese\"\nmen = {men}\nq = 0\nr = 6\n"
        )
    };
    let with = |men: u32| {
        let t = changed(
            &text(),
            "air_combat = \"../reference/air_combat.toml\"\n",
            "air_combat = \"../reference/air_combat.toml\"\nground = \"../reference/ground.toml\"\n",
        );
        changed(
            &t,
            COMMANDS_MARKER,
            &format!("{}\n{COMMANDS_MARKER}", enemy(men)),
        )
    };
    let a = play(&with(100));
    let b = play(&with(300));
    assert_eq!(a.handed, b.handed);
    for day in 1..=6 {
        assert_eq!(a.orders(day), main_game().orders(day), "day {day}");
    }
}
