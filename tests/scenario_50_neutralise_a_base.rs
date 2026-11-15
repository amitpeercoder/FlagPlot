//! Scenario 50: the commander states one end, "neutralise base 2", and the
//! staff at the area's level gives base 1 the task of raiding it, and at
//! the base's level decides which mornings the raid goes.
//!
//! Proves decision 0120 against a hand calculation. The data is placeholder
//! data: see `data/scenarios/50_neutralise_a_base.toml`. It runs on the
//! frozen step 1 figures, with the fatigue figures loaded and the wear,
//! upkeep and supply figures not. Base 2 has nothing at it: no patrol
//! rises, no gun fires and no aircraft is lost. The bombs' hits on base 2
//! are rolled, and nothing here depends on them or asserts them.
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
//! Fatigue, as scenario 47: a group that flew in a daylight pulse gains 3
//! and 2 for each 100 nm of its longest sortie, there and back; a group
//! that did not fly in a daylight pulse loses 5; never below 0. A group is
//! able to fly its ready aircraft times 100 less its fatigue, over 100, cut
//! to a whole aircraft. A base strike launches once a day, in the morning
//! where the base is in reach then. A fighter group's escorts are the
//! aircraft it is able to fly less its patrol, and they go with the raid
//! from their base. A hex is 40 nm.
//!
//! The placeholder dive bomber (type 3): a bomb, a strike radius of 250
//! nm. The placeholder fighter (type 4): a reach of 250 nm.
//!
//! Base 2 is 5 hexes from base 1: 200 nm. A raid is a sortie of 400 nm: a
//! group that flies it gains 3 + 8 = 11 in the morning and, not flying in
//! the afternoon, loses 5: 6 in the day. A day on which a group does not
//! fly at all takes 10 off.
//!
//! ## Inputs
//!
//! A placeholder map of open sea. Allied base 1 at (0,0). Japanese base 2
//! at (5,0), on the Allied map, with nothing at it. The game is 12 days.
//!
//! | Group | Type        | Ready aircraft | At the start |
//! | ----- | ----------- | -------------- | ------------ |
//! | 1     | dive bomber | 10             | resting, no held share |
//! | 2     | fighter     | 8              | resting, no setting (a patrol share of 1) |
//! | 3     | dive bomber | 10             | resting, no held share |
//!
//! The Allied side's commands of day 1: intent 1, neutralise base 2; leave
//! to prepare and leave to launch both given. No limit is set. So the risk
//! is balanced: a bomber rests above a fatigue of 45, holds none back, a
//! raid sends at least 6 and goes without an escort if it must, and a group
//! stands down after losing a quarter of what it sent.
//!
//! ## The tree on the first morning
//!
//! Plan 1 is drafted, made ready and released. At the area's level the
//! staff finds one base of the side, base 1, with bombers that reach base
//! 2 on normal reach, 200 nm against 250, and forms the task.
//!
//! | Intent | Kind | Parent | Maker | Addressee | Plan |
//! | ------ | ---- | ------ | ----- | --------- | ---- |
//! | 1      | neutralise base 2 | none | the commander | the area | 1 |
//! | 2      | raid base 2 from base 1 | 1 | the staff | base 1 | 1 |
//!
//! The reason of the forming serves intent 1, was decided at the area's
//! level, and gives the distance and the reach. Every order below serves
//! intent 2, was decided at the base's level, and has a chain of one:
//! intent 1, the commander's.
//!
//! ## Day by day
//!
//! Fatigue is read in the morning. All three groups have the same fatigue
//! on every morning.
//!
//! | Day | Fatigue | The raid | Each bomber group sends | Escorts | The staff's orders |
//! | --- | ------- | -------- | ----------------------- | ------- | ------------------ |
//! | 1   | 0       | goes     | 10                      | 8       | Groups 1 and 3: strike base 2. Group 2: patrol, share 0 |
//! | 2   | 6       | goes     | 9                       | 7       | none |
//! | 3   | 12      | goes     | 8                       | 7       | none |
//! | 4   | 18      | goes     | 8                       | 6       | none |
//! | 5   | 24      | goes     | 7                       | 6       | none |
//! | 6   | 30      | goes     | 7                       | 5       | none |
//! | 7   | 36      | goes     | 6                       | 5       | none |
//! | 8   | 42      | goes     | 5                       | 4       | none |
//! | 9   | 48      | does not |                         |         | Groups 1 and 3: rest |
//! | 10  | 38      | goes     | 6                       | 4       | Groups 1 and 3: strike base 2 |
//! | 11  | 44      | goes     | 5                       | 4       | none |
//! | 12  | 50      | does not |                         |         | Groups 1 and 3: rest |
//!
//! At the end of day 12 every group has fatigue 40.
//!
//! Why, where it is not plain:
//!
//! - **Day 1** has four orders: a mission for each bomber group, and for
//!   group 2 the patrol mission and the share. A mission and a setting are
//!   two orders. No held share is ordered: the groups have none and
//!   balanced asks for none.
//! - **What goes.** 10 x 0.94 = 9.4, cut to 9, and so on down the column.
//!   The fighters: 8 x 0.94 = 7.52, cut to 7; 8 x 0.88 = 7.04; 8 x 0.82 =
//!   6.56; 8 x 0.76 = 6.08; 8 x 0.70 = 5.6; 8 x 0.64 = 5.12; 8 x 0.58 =
//!   4.64; on day 10, 8 x 0.62 = 4.96; on day 11, 8 x 0.56 = 4.48. On
//!   every raid morning the two groups send 10 or more between them,
//!   against the 6 a balanced raid asks.
//! - **Day 9.** Both bomber groups are at 48, above 45: neither goes, the
//!   raid does not go, and both rest. Group 2 is given no order: it keeps
//!   its mission and, with no raid, flies nothing. All three lose 10.
//! - **Day 10.** At 38 the bombers go again.
//! - **Day 12.** At 50 they rest.
//!
//! Ten raids are launched, on days 1 to 8, 10 and 11. The staff gives 4,
//! 0, 0, 0, 0, 0, 0, 0, 2, 2, 0 and 2 orders: 10 in all. No staff order
//! and no staff command is refused, and no notice is handed in.
//!
//! ## The measure
//!
//! At the end of day 12 the measure of intent 1 reads: 12 days in force;
//! 10 raids flown at base 2; no air raid on a base of the side, so it was
//! met on every day. The aircraft claimed on the ground are not asserted.
//!
//! # Properties
//!
//! - **Preserve the force.** With the risk of intent 1 set to preserve on
//!   day 1, day 1 has six orders: the four above and a held share of
//!   0.333333 for groups 1 and 3. On days 1 and 2 each bomber group sends
//!   6 (10 x 0.666667 = 6.66667, and 9 x 0.666667 = 6.000003, each cut to
//!   6): 12 against the 12 preserve asks, and the raid goes. On day 3, at
//!   12, each is able to fly 8 and sends 5: 10 against 12. The raid does
//!   not go, both rest, and the reason gives 10 against 12. On day 4, at
//!   2, each is able to fly 9 and sends 6, and the raid goes.
//! - **Press.** With the risk of intent 1 set to press on day 1, the raid
//!   goes on days 1 to 11, the last at a fatigue of 60, which is not above
//!   the level, with 4 from each bomber group. On day 12, at 66, both
//!   rest. The staff gives 4 orders on day 1 and 2 on day 12.
//! - **No escort.** With no group 2: at balanced the raids go as in the
//!   main case, with no escort. At preserve no raid goes on any day, and
//!   the staff hands in on day 1 that it cannot meet the intent.
//! - **Out of reach.** With base 2 at (7,0), 280 nm: the staff forms no
//!   raid task, hands in on day 1 that it cannot meet intent 1, and its
//!   reason gives 280 nm against a reach of 250.
//! - **What the last raid cost.** In a view built for it, at balanced: a
//!   group that sent 10 on its last raid and lost 3 stands down the next
//!   morning, and its reason gives 3 of 10 against a quarter; one that
//!   lost 2 goes. At press the first goes too.
//! - **A base held and a base to neutralise.** With the commander also
//!   stating, first, intent 1, hold base 1, and neutralise base 2 as
//!   intent 2: the staff forms cover of base 1 as intent 3 and the raid as
//!   intent 4. On day 1 group 2 is given the patrol and a share of 0.5,
//!   serving intent 3, and the raid task gives it no order; groups 1 and 3
//!   are given the strike, serving intent 4. Four orders. The raid of day
//!   1 has 20 bombers and 4 escorts.
//! - **Whose a group is.** In a view built for it, with a cover of base 1
//!   and the raid task both in force and a group at base 1 whose type can
//!   search and can bomb base 2: the raid task orders it and the cover
//!   doctrine does not, whichever intent ranks higher. With the neutralise
//!   intent ranked above the hold intent, group 2 is still the cover's.
//! - **A raid stated alone.** With the commander stating "raid base 2 from
//!   base 1" as intent 1 and nothing else, the orders are the main case's,
//!   each serving intent 1 with an empty chain.
//! - **A held group is left alone:** with group 1 held from day 3, it is
//!   given no order from then, and group 3 raids alone when it can send
//!   the 6 a raid asks. It goes on days 3 to 7, sending 8, 8, 7, 7 and 6.
//!   On day 8, at 42, it can send 5: the raid does not go and it rests. It
//!   goes on days 9 and 10, at 32 and 38, sending 6 each day; rests on day
//!   11, at 44, when it can send 5; and goes on day 12, at 34, sending 6.
//!   It ends day 12 at 40. (Restated by the architect on Nov 16, 2026,
//!   after the build: the first draft had it keep the main case's days.)
//! - **Withdrawing the end** on day 5 withdraws the raid task that day,
//!   and the staff gives no order from then. The groups keep the missions
//!   they had.
//! - **The level is named.** Every reason serving intent 2 says the base
//!   decided, and the reason of the forming says the area decided.
//! - The staff orders no group the raid task may not use, and the same
//!   start gives the same orders.
//! - Two worlds that differ only in what the Allied side cannot see give
//!   the same intents, orders and reasons.
//! - Every staffed scenario built before this one gives the orders it
//!   gave.

// ---------------------------------------------------------------- The harness

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_agent::{neutralise_measure, NeutraliseMeasure};
use sim_core::command::{
    Addressee, CommandKind, CommandRefusal, CommandState, IntentKind, Maker, NoticeKind, PlanId,
    PlanState, Reason, Risk, StaffDecisions, StaffOrder,
};
use sim_core::events::{Cause, Event, EventKind};
use sim_core::fixed::Fixed;
use sim_core::ids::{AirGroupId, BaseId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind};
use sim_core::state::{Side, TruthState};
use sim_data::Scenario;
use sim_staff::raid::{
    can_send, decide, escorts_at, held_share, least_weight, rest_level, stands_down, whose,
    BomberVerdict, CannotRaid, LastRaid, Owner, RaidGroup, RaidRole,
};
use sim_staff::{as_staffs, staffs};

mod common;

const COMMANDS_MARKER: &str = "# The Allied commander's commands";

fn path() -> PathBuf {
    common::data_dir().join("scenarios/50_neutralise_a_base.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn n(whole: i64) -> Fixed {
    Fixed::from_int(whole)
}

fn fx(text: &str) -> Fixed {
    text.parse().expect("a decimal")
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

fn plan(raw: u32) -> PlanId {
    PlanId::new(raw)
}

fn base(raw: u32) -> BaseId {
    BaseId::new(raw)
}

/// The commands of the file itself: the marker line and what follows it.
fn commands() -> String {
    let text = text();
    let at = text.find(COMMANDS_MARKER).expect("the marker");
    text[at..].to_string()
}

/// A command table of the Allied side as the scenario states it.
fn command_table(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

/// The scenario's text with `extra` after its commands.
fn with(extra: &str) -> String {
    format!("{}{}", text(), extra)
}

fn changed(text: &str, original: &str, replacement: &str) -> String {
    common::replace_once(text, original, replacement)
}

/// The commander's one command of day 1 that states the end.
const NEUTRALISE_COMMAND: &str = "intent = 1\nintent_kind = \"neutralise\"\ntarget = 2\n";

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the truth at the end of each day (index 0 is the
/// start), the log, the side's command state at the end and the measure of
/// intent 1 at the end (if it is a neutralise intent).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    truth: Vec<TruthState>,
    events: Vec<Event>,
    state: CommandState,
    measure: Option<NeutraliseMeasure>,
}

fn play(text: &str) -> Played {
    let mut scenario: Scenario = common::from_text(text, &path()).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
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
    let measure = neutralise_measure(&scenario.world.side_view(Side::Allied), intent(1));
    Played {
        handed,
        truth,
        events: scenario.world.events().iter().cloned().collect(),
        state: scenario.world.command_state(Side::Allied).clone(),
        measure,
    }
}

impl Played {
    /// A group's fatigue on the morning of `day`, before the day's orders.
    fn morning(&self, day: u32, raw: u32) -> Fixed {
        self.truth[(day - 1) as usize].air_groups[&group(raw)].fatigue
    }

    fn at_end(&self, raw: u32) -> Fixed {
        self.truth.last().expect("a state").air_groups[&group(raw)].fatigue
    }

    fn mission_at_end(&self, raw: u32) -> AirMission {
        self.truth.last().expect("a state").air_groups[&group(raw)].mission
    }

    fn decisions(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    /// The staff's air group orders of `day`, as (group, order), in the order
    /// handed in.
    fn air_orders(&self, day: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day)
            .orders
            .iter()
            .map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } => (target.raw(), order.clone()),
                other => panic!("the doctrine gave a non-air order: {other:?}"),
            })
            .collect()
    }

    /// The air group orders of `day` that serve `served`.
    fn air_orders_serving(&self, day: u32, served: u32) -> Vec<(u32, AirGroupOrder)> {
        self.decisions(day)
            .orders
            .iter()
            .filter(|o| o.intent == intent(served))
            .map(|o| match &o.kind {
                OrderKind::AirGroup { target, order } => (target.raw(), order.clone()),
                other => panic!("the doctrine gave a non-air order: {other:?}"),
            })
            .collect()
    }

    fn counts(&self) -> Vec<usize> {
        (1..=12).map(|d| self.decisions(d).orders.len()).collect()
    }

    fn all_orders(&self) -> Vec<(u32, &StaffOrder)> {
        self.handed
            .iter()
            .flat_map(|(day, d)| d.orders.iter().map(move |o| (*day, o)))
            .collect()
    }

    fn all(&self, pick: impl Fn(&EventKind) -> bool) -> Vec<&Event> {
        self.events.iter().filter(|e| pick(&e.kind)).collect()
    }

    /// The commands the commander had refused, with the day and the reason.
    fn refusals(&self) -> Vec<(u32, CommandRefusal)> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused { reason, .. } => Some((e.day, *reason)),
                _ => None,
            })
            .collect()
    }

    /// No staff order and no staff command was refused.
    fn nothing_refused(&self) {
        let refused = self.all(|k| {
            matches!(
                k,
                EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
            )
        });
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }

    /// The aircraft each group launched on a base strike on `day`, as
    /// (group, aircraft), in the order of the log.
    fn launched(&self, day: u32) -> Vec<(u32, u32)> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::BaseStrikeLaunched {
                    air_group,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *aircraft)),
                _ => None,
            })
            .collect()
    }

    /// The aircraft of one group launched on `day`, 0 where it did not.
    fn launched_by(&self, day: u32, raw: u32) -> u32 {
        self.launched(day)
            .iter()
            .filter(|(g, _)| *g == raw)
            .map(|(_, a)| *a)
            .sum()
    }

    /// The bombers of every group launched on `day`.
    fn bombers(&self, day: u32) -> u32 {
        self.launched(day).iter().map(|(_, a)| *a).sum()
    }

    /// The escorts sent with the raids of `day`.
    fn escorts(&self, day: u32) -> u32 {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::EscortsSent { aircraft, .. } => Some(*aircraft),
                _ => None,
            })
            .sum()
    }

    /// The days on which a raid was launched.
    fn raid_days(&self) -> Vec<u32> {
        (1..=12).filter(|d| self.bombers(*d) > 0).collect()
    }

    /// Every reason the staff handed in, with the day.
    fn reasons(&self) -> Vec<(u32, &Reason)> {
        let mut found = Vec::new();
        for (day, d) in &self.handed {
            found.extend(d.orders.iter().map(|o| (*day, &o.reason)));
            found.extend(d.commands.iter().map(|c| (*day, &c.reason)));
            found.extend(d.no_orders.iter().map(|r| (*day, r)));
        }
        found
    }

    /// Every reason's chain, followed to its end, reaches an intent the
    /// commander made.
    fn every_chain_ends_with_the_commander(&self) {
        for (day, reason) in self.reasons() {
            let served = reason.serves.intent.expect("a reason serves an intent");
            match reason.serves.chain.last() {
                Some(link) => {
                    assert_eq!(link.maker, Maker::Commander, "day {day}: {reason:?}");
                    assert!(!link.statement.is_empty(), "day {day}");
                    assert_eq!(
                        Some(reason.serves.chain[0].intent),
                        self.state.intents[&served].parent,
                        "day {day}"
                    );
                    assert_eq!(
                        Some(link.intent),
                        Some(self.state.root_of(served)),
                        "day {day}"
                    );
                }
                None => assert_eq!(
                    self.state.intents[&served].maker,
                    Maker::Commander,
                    "day {day}: a reason with no chain serves a staff-made intent"
                ),
            }
        }
    }
}

fn mission(m: AirMission) -> AirGroupOrder {
    AirGroupOrder::SetAirMission(m)
}

fn share(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetPatrolShare { share: fx(s) }
}

fn held(s: &str) -> AirGroupOrder {
    AirGroupOrder::SetStrikeHeldShare { share: fx(s) }
}

const CAP: AirMission = AirMission::CombatAirPatrol;
const REST: AirMission = AirMission::Rest;

fn strike() -> AirMission {
    AirMission::BaseStrike(base(2))
}

/// The staff's orders of the main case, by day, as (group, order). Day 1:
/// each bomber group strikes base 2 and group 2 gets the patrol and a share
/// of 0, in group number order, a mission before a setting.
fn table_orders() -> Vec<Vec<(u32, AirGroupOrder)>> {
    vec![
        vec![
            (1, mission(strike())),
            (2, mission(CAP)),
            (2, share("0")),
            (3, mission(strike())),
        ],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![(1, mission(REST)), (3, mission(REST))],
        vec![(1, mission(strike())), (3, mission(strike()))],
        vec![],
        vec![(1, mission(REST)), (3, mission(REST))],
    ]
}

/// What each bomber group sends on the raid of each day, 0 where none goes.
const SENDS: [u32; 12] = [10, 9, 8, 8, 7, 7, 6, 5, 0, 6, 5, 0];

/// The escorts of each day's raid.
const ESCORTS: [u32; 12] = [8, 7, 7, 6, 6, 5, 5, 4, 0, 4, 4, 0];

/// Fatigue on the morning of each day, the same for all three groups.
const MORNINGS: [i64; 12] = [0, 6, 12, 18, 24, 30, 36, 42, 48, 38, 44, 50];

fn mentions(reason: &Reason, words: &[&str]) {
    for word in words {
        assert!(
            reason.worked.contains(word),
            "the reason's worked part has no \"{word}\": {}",
            reason.worked
        );
    }
}

/// A reason of the raid doctrine under the neutralise intent: it serves
/// intent 2 and plan 1, was decided at the level of base 1, and its chain is
/// one link long: intent 1, the commander's.
fn serves_the_raid_under_the_end(reason: &Reason) {
    assert_eq!(reason.serves.intent, Some(intent(2)));
    assert_eq!(reason.serves.plan, Some(plan(1)));
    assert_eq!(reason.serves.amendment, None);
    assert_eq!(reason.serves.level, Some(Addressee::Base(base(1))));
    assert!(!reason.serves.statement.is_empty());
    assert_eq!(reason.serves.chain.len(), 1, "{:?}", reason.serves);
    let link = &reason.serves.chain[0];
    assert_eq!(link.intent, intent(1));
    assert_eq!(link.maker, Maker::Commander);
    assert!(
        link.statement.contains("neutralise") && link.statement.contains("base:2"),
        "{}",
        link.statement
    );
    assert!(!reason.worked.is_empty());
}

// ------------------------------------------------------- The hand calculation

#[test]
fn the_commands_of_the_file_are_the_ones_these_tests_vary() {
    let c = commands();
    assert_eq!(c.matches("[[command]]").count(), 3);
    assert!(c.contains("intent_kind = \"neutralise\""));
    assert!(c.contains(NEUTRALISE_COMMAND));
    assert!(!c.contains("set_limit"));
}

#[test]
fn the_tree_on_the_first_morning() {
    let played = play(&text());
    let state = &played.state;
    assert_eq!(state.intents.len(), 2);
    assert_eq!(state.plans.len(), 1);
    let one = &state.intents[&intent(1)];
    assert_eq!(one.kind, IntentKind::Neutralise { target: base(2) });
    assert_eq!(one.parent, None);
    assert_eq!(one.maker, Maker::Commander);
    assert_eq!(one.addressee, Addressee::Area);
    assert_eq!(one.plan, plan(1));
    assert_eq!(one.set_day, 1);
    let two = &state.intents[&intent(2)];
    assert_eq!(
        two.kind,
        IntentKind::Raid {
            base: base(1),
            target: base(2)
        }
    );
    assert_eq!(two.parent, Some(intent(1)));
    assert_eq!(two.maker, Maker::Staff);
    assert_eq!(two.addressee, Addressee::Base(base(1)));
    assert_eq!(two.plan, plan(1));
    assert_eq!(two.set_day, 1);
    assert_eq!(two.withdrawn_day, None);
    assert_eq!(state.plans[&plan(1)].state, PlanState::Released);
}

#[test]
fn on_the_first_morning_the_staff_makes_the_plan_ready_releases_it_and_forms_intent_2() {
    let played = play(&text());
    let commands = &played.decisions(1).commands;
    assert_eq!(commands.len(), 3);
    assert_eq!(commands[0].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(commands[1].kind, CommandKind::Approve { plan: plan(1) });
    assert_eq!(
        commands[2].kind,
        CommandKind::FormIntent {
            parent: intent(1),
            kind: IntentKind::Raid {
                base: base(1),
                target: base(2)
            },
        }
    );
    // The reason of the forming serves intent 1, was decided at the area's
    // level, has no chain (intent 1 is the commander's own) and gives the
    // distance and the reach.
    let forming = &commands[2].reason;
    assert_eq!(forming.serves.intent, Some(intent(1)));
    assert_eq!(forming.serves.plan, Some(plan(1)));
    assert_eq!(forming.serves.level, Some(Addressee::Area));
    assert!(forming.serves.chain.is_empty());
    mentions(forming, &["200 nm", "250"]);
    for day in 2..=12 {
        assert!(played.decisions(day).commands.is_empty(), "day {day}");
    }
    let formed = played.all(|k| matches!(k, EventKind::IntentFormed { .. }));
    assert_eq!(formed.len(), 1);
    assert_eq!(formed[0].day, 1);
    assert!(matches!(
        formed[0].kind,
        EventKind::IntentFormed { intent: i, parent: p, .. } if i == intent(2) && p == intent(1)
    ));
}

#[test]
fn the_staffs_orders_of_every_morning_follow_the_table() {
    let played = play(&text());
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "the orders of day {day}");
    }
}

#[test]
fn the_staff_gives_four_none_none_none_none_none_none_none_two_two_none_two_orders() {
    let played = play(&text());
    assert_eq!(played.counts(), vec![4, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 2]);
    assert_eq!(played.counts().iter().sum::<usize>(), 10);
}

#[test]
fn every_groups_fatigue_on_every_morning_and_at_the_end_follows_the_table() {
    let played = play(&text());
    for (i, f) in MORNINGS.iter().enumerate() {
        let day = i as u32 + 1;
        for raw in 1..=3 {
            assert_eq!(played.morning(day, raw), n(*f), "group {raw}, day {day}");
        }
    }
    for raw in 1..=3 {
        assert_eq!(played.at_end(raw), n(40), "group {raw}");
    }
}

#[test]
fn the_aircraft_of_every_raid_follow_the_table() {
    let played = play(&text());
    for day in 1..=12u32 {
        let sends = SENDS[(day - 1) as usize];
        assert_eq!(played.launched_by(day, 1), sends, "group 1, day {day}");
        assert_eq!(played.launched_by(day, 3), sends, "group 3, day {day}");
        assert_eq!(played.launched_by(day, 2), 0, "group 2, day {day}");
        assert_eq!(
            played.escorts(day),
            ESCORTS[(day - 1) as usize],
            "the escorts of day {day}"
        );
    }
}

#[test]
fn ten_raids_are_launched_on_days_1_to_8_10_and_11() {
    let played = play(&text());
    assert_eq!(played.raid_days(), vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11]);
    // On every raid morning the two groups send 10 or more between them,
    // against the 6 a balanced raid asks.
    for day in played.raid_days() {
        assert!(played.bombers(day) >= 10, "day {day}");
    }
}

#[test]
fn every_staff_order_serves_intent_2_and_plan_1_through_a_chain_one_intent_long() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        assert_eq!(order.intent, intent(2), "day {day}");
        serves_the_raid_under_the_end(&order.reason);
    }
}

#[test]
fn no_staff_order_and_no_staff_command_is_refused_and_no_notice_is_handed_in() {
    let played = play(&text());
    played.nothing_refused();
    assert!(played.refusals().is_empty());
    for day in 1..=12 {
        assert!(played.decisions(day).notices.is_empty(), "day {day}");
    }
}

#[test]
fn the_measure_of_intent_1_at_the_end_of_day_12() {
    let played = play(&text());
    let m = played.measure.expect("intent 1 is a neutralise intent");
    assert_eq!(m.days_in_force, 12);
    assert_eq!(m.raids_flown, 10);
    assert_eq!(m.raids_on_own_bases, 0);
    assert_eq!(m.days_met, 12);
    // The aircraft claimed on the ground are not asserted: the bombs' hits
    // are rolled.
}

#[test]
fn the_level_is_named_by_every_reason() {
    let played = play(&text());
    for (day, order) in played.all_orders() {
        assert_eq!(
            order.reason.serves.level,
            Some(Addressee::Base(base(1))),
            "day {day}"
        );
        assert!(
            order
                .reason
                .serves
                .to_string()
                .contains("decided at the level of base:1"),
            "{}",
            order.reason.serves
        );
    }
    let forming = &played.decisions(1).commands[2].reason;
    assert!(
        forming
            .serves
            .to_string()
            .contains("decided at the level of the area"),
        "{}",
        forming.serves
    );
}

#[test]
fn the_top_of_every_chain_is_the_commanders_and_why_prints_it() {
    let played = play(&text());
    played.every_chain_ends_with_the_commander();

    let mut scenario: Scenario = common::from_text(&text(), &path()).expect("loads");
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("day 1");
    let view = scenario.world.side_view(Side::Allied);
    let events = view.events();
    let mut seen = 0;
    for (i, e) in events.iter().enumerate() {
        if matches!(
            &e.kind,
            sim_core::side_events::SideEventKind::Whole(EventKind::StaffReason {
                order: Some(_),
                ..
            })
        ) {
            let said = sim_agent::why(&view, i as u32 + 1).to_string();
            assert!(said.contains("under intent 1 (the commander's)"), "{said}");
            assert!(said.contains("neutralise"), "{said}");
            seen += 1;
        }
    }
    assert_eq!(seen, 4);
}

// Every staffed scenario built before this one gives the orders it gave. That
// is checked by the compare script (scripts/compare.sh), not by a test.

// --------------------------------------------------------------- Properties

fn risk_text(risk: &str) -> String {
    with(&command_table(
        1,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{risk}\""),
    ))
}

#[test]
fn preserve_the_force_holds_a_third_back_and_asks_for_12() {
    let played = play(&risk_text("preserve"));
    played.nothing_refused();
    // Day 1: the four orders and a held share of 0.333333 for groups 1 and 3.
    let one = played.air_orders(1);
    assert_eq!(one.len(), 6, "{one:?}");
    assert_eq!(
        one,
        vec![
            (1, mission(strike())),
            (1, held("0.333333")),
            (2, mission(CAP)),
            (2, share("0")),
            (3, mission(strike())),
            (3, held("0.333333")),
        ]
    );
    // Days 1 and 2: each bomber group sends 6, 12 against the 12 asked.
    for day in [1, 2] {
        assert_eq!(played.launched_by(day, 1), 6, "day {day}");
        assert_eq!(played.launched_by(day, 3), 6, "day {day}");
    }
    // Day 3, at 12: each is able to fly 8 and sends 5, 10 against 12. The
    // raid does not go, both rest and the reason gives 10 against 12.
    assert_eq!(played.morning(3, 1), n(12));
    assert_eq!(played.bombers(3), 0);
    assert!(played.air_orders(3).contains(&(1, mission(REST))));
    assert!(played.air_orders(3).contains(&(3, mission(REST))));
    let rests: Vec<&StaffOrder> = played
        .all_orders()
        .into_iter()
        .filter(|(d, _)| *d == 3)
        .map(|(_, o)| o)
        .filter(|o| matches!(&o.kind, OrderKind::AirGroup { order, .. } if *order == mission(REST)))
        .collect();
    assert_eq!(rests.len(), 2);
    for order in rests {
        mentions(&order.reason, &["send 10 between them", "against the 12"]);
    }
    // Day 4, at 2: each is able to fly 9 and sends 6, and the raid goes.
    assert_eq!(played.morning(4, 1), n(2));
    assert_eq!(played.launched_by(4, 1), 6);
    assert_eq!(played.launched_by(4, 3), 6);
}

#[test]
fn press_goes_to_a_fatigue_of_60_and_rests_at_66() {
    let played = play(&risk_text("press"));
    played.nothing_refused();
    assert_eq!(played.raid_days(), (1..=11).collect::<Vec<u32>>());
    // The last raid, on day 11, is at a fatigue of 60 with 4 from each bomber
    // group.
    assert_eq!(played.morning(11, 1), n(60));
    assert_eq!(played.launched_by(11, 1), 4);
    assert_eq!(played.launched_by(11, 3), 4);
    // On day 12, at 66, both rest.
    assert_eq!(played.morning(12, 1), n(66));
    assert!(played.air_orders(12).contains(&(1, mission(REST))));
    assert!(played.air_orders(12).contains(&(3, mission(REST))));
    assert_eq!(played.bombers(12), 0);
    assert_eq!(played.counts()[0], 4);
    assert_eq!(played.counts()[11], 2);
}

fn without_group_2() -> String {
    changed(
        &text(),
        "[[air_group]]\nid = 2\nname = \"Placeholder fighters 2\"\nside = \"allied\"\nbase = 1\naircraft_type = 4\nready = 8\n",
        "",
    )
}

#[test]
fn with_no_group_2_at_balanced_the_raids_go_as_in_the_main_case_with_no_escort() {
    let played = play(&without_group_2());
    played.nothing_refused();
    assert_eq!(played.raid_days(), vec![1, 2, 3, 4, 5, 6, 7, 8, 10, 11]);
    for day in 1..=12u32 {
        let sends = SENDS[(day - 1) as usize];
        assert_eq!(played.launched_by(day, 1), sends, "day {day}");
        assert_eq!(played.launched_by(day, 3), sends, "day {day}");
        assert_eq!(played.escorts(day), 0, "day {day}");
    }
}

#[test]
fn with_no_group_2_at_preserve_no_raid_goes_and_the_staff_cannot_meet_the_intent() {
    let text = changed(
        &without_group_2(),
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"",
        "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"preserve\"\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"",
    );
    let played = play(&text);
    played.nothing_refused();
    assert!(played.raid_days().is_empty(), "{:?}", played.raid_days());
    assert!(
        played
            .decisions(1)
            .notices
            .iter()
            .any(|n| n.kind == NoticeKind::CannotMeet),
        "no cannot-meet notice on day 1"
    );
}

#[test]
fn out_of_reach_the_staff_forms_no_raid_and_cannot_meet_the_intent() {
    let text = changed(
        &text(),
        "side = \"japanese\"\nq = 5\nr = 0",
        "side = \"japanese\"\nq = 7\nr = 0",
    );
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    for day in 1..=12 {
        assert!(played.decisions(day).orders.is_empty(), "day {day}");
        assert!(
            played
                .decisions(day)
                .commands
                .iter()
                .all(|c| !matches!(c.kind, CommandKind::FormIntent { .. })),
            "day {day}"
        );
    }
    let d1 = played.decisions(1);
    assert!(
        d1.notices.iter().any(|n| n.kind == NoticeKind::CannotMeet),
        "no cannot-meet notice on day 1"
    );
    let reason = d1
        .no_orders
        .iter()
        .find(|r| r.serves.intent == Some(intent(1)))
        .expect("a reason serving intent 1");
    mentions(reason, &["280 nm", "250"]);
    assert!(played.raid_days().is_empty());
}

// WHAT THE LAST RAID COST. The hand calculation says this "in a view built
// for it". A full world in which a raid loses a quarter of its aircraft would
// need a defence at the target, and base 2 has none. The decision is a pure
// function of plain numbers, `sim_staff::raid::decide`, which the staff reads
// the side's view into, so the property is tested on that function with
// groups built by hand. It returns the verdict ("3 of 10", with the share a
// quarter from `stand_down_share`); the words of the reason built from it are
// not read here.
fn bomber(id: u32, fatigue: i64, last: Option<LastRaid>) -> RaidGroup {
    RaidGroup {
        id: group(id),
        role: RaidRole::Bomber,
        fatigue: n(fatigue),
        able: 10,
        mission: AirMission::BaseStrike(base(2)),
        patrol_share: Fixed::ONE,
        held_share: Fixed::ZERO,
        last_raid: last,
    }
}

fn fighter(id: u32) -> RaidGroup {
    RaidGroup {
        id: group(id),
        role: RaidRole::Fighter,
        fatigue: n(0),
        able: 8,
        mission: CAP,
        patrol_share: Fixed::ZERO,
        held_share: Fixed::ZERO,
        last_raid: None,
    }
}

fn last(sent: u32, lost: u32) -> Option<LastRaid> {
    Some(LastRaid { day: 4, sent, lost })
}

#[test]
fn a_group_that_lost_3_of_10_stands_down_at_balanced_and_one_that_lost_2_goes() {
    let groups = [
        bomber(1, 0, last(10, 3)),
        bomber(3, 0, last(10, 2)),
        fighter(2),
    ];
    let morning = decide(5, Risk::Balanced, base(2), &groups);
    assert_eq!(
        morning.bombers,
        vec![
            (group(1), BomberVerdict::StandsDown { lost: 3, sent: 10 }),
            (group(3), BomberVerdict::Goes { sends: 10 }),
        ]
    );
    // The share is a quarter: 3 of 10 is over it and 2 of 10 is under it.
    assert!(stands_down(Risk::Balanced, last(10, 3), 5));
    assert!(!stands_down(Risk::Balanced, last(10, 2), 5));
    // Group 3 alone sends 10, over the 6 a balanced raid asks: the raid goes,
    // and group 1 is set to rest.
    assert!(morning.goes);
    assert_eq!(morning.weight, 10);
    assert_eq!(morning.least, least_weight(Risk::Balanced));
    assert_eq!(
        morning.orders,
        vec![
            (group(1), mission(REST)),
            // Group 3 keeps the strike it has.
        ]
    );
    // Only the day after: a raid two days ago costs nothing.
    assert!(!stands_down(Risk::Balanced, last(10, 3), 6));
}

#[test]
fn at_press_the_group_that_lost_3_of_10_goes_too() {
    let groups = [
        bomber(1, 0, last(10, 3)),
        bomber(3, 0, last(10, 2)),
        fighter(2),
    ];
    let morning = decide(5, Risk::Press, base(2), &groups);
    assert_eq!(
        morning.bombers,
        vec![
            (group(1), BomberVerdict::Goes { sends: 10 }),
            (group(3), BomberVerdict::Goes { sends: 10 }),
        ]
    );
    assert!(!stands_down(Risk::Press, last(10, 3), 5));
    assert!(morning.goes);
    assert_eq!(morning.weight, 20);
    assert!(morning.orders.is_empty());
}

#[test]
fn the_figures_of_the_risk_table() {
    assert_eq!(rest_level(Risk::Preserve), n(30));
    assert_eq!(rest_level(Risk::Balanced), n(45));
    assert_eq!(rest_level(Risk::Press), n(60));
    assert_eq!(held_share(Risk::Preserve), fx("0.333333"));
    assert_eq!(held_share(Risk::Balanced), n(0));
    assert_eq!(least_weight(Risk::Preserve), 12);
    assert_eq!(least_weight(Risk::Balanced), 6);
    assert_eq!(least_weight(Risk::Press), 1);
    assert_eq!(can_send(10, held_share(Risk::Preserve)), 6);
    assert_eq!(can_send(9, held_share(Risk::Preserve)), 6);
    assert_eq!(can_send(8, held_share(Risk::Preserve)), 5);
    assert_eq!(escorts_at(8, fx("0.5")), 4);
    // At preserve with no fighter the raid cannot be met.
    let morning = decide(1, Risk::Preserve, base(2), &[bomber(1, 0, None)]);
    assert_eq!(morning.cannot, Some(CannotRaid::NoFighter));
    assert!(!morning.goes);
}

/// The commander states, first, hold base 1 as intent 1 and then neutralise
/// base 2 as intent 2.
fn held_and_neutralise() -> String {
    changed(
        &text(),
        NEUTRALISE_COMMAND,
        "intent = 1\nintent_kind = \"hold\"\nbase = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 2\nintent_kind = \"neutralise\"\ntarget = 2\n",
    )
}

#[test]
fn a_base_held_and_a_base_to_neutralise() {
    let played = play(&held_and_neutralise());
    played.nothing_refused();
    let cover = &played.state.intents[&intent(3)];
    assert_eq!(cover.kind, IntentKind::Cover { base: base(1) });
    assert_eq!(cover.maker, Maker::Staff);
    assert_eq!(cover.parent, Some(intent(1)));
    assert_eq!(cover.set_day, 1);
    let raid = &played.state.intents[&intent(4)];
    assert_eq!(
        raid.kind,
        IntentKind::Raid {
            base: base(1),
            target: base(2)
        }
    );
    assert_eq!(raid.maker, Maker::Staff);
    assert_eq!(raid.parent, Some(intent(2)));
    assert_eq!(raid.set_day, 1);
    // Day 1 has four orders: group 2 the patrol and a share of 0.5, serving
    // intent 3, and the strike for groups 1 and 3, serving intent 4.
    assert_eq!(played.air_orders(1).len(), 4, "{:?}", played.air_orders(1));
    assert_eq!(
        played.air_orders_serving(1, 3),
        vec![(2, mission(CAP)), (2, share("0.5"))]
    );
    assert_eq!(
        played.air_orders_serving(1, 4),
        vec![(1, mission(strike())), (3, mission(strike()))]
    );
    // The raid of day 1 has 20 bombers and 4 escorts.
    assert_eq!(played.bombers(1), 20);
    assert_eq!(played.escorts(1), 4);
    played.every_chain_ends_with_the_commander();
}

#[test]
fn with_the_neutralise_intent_ranked_above_the_hold_group_2_is_still_the_covers() {
    let text = format!(
        "{}{}{}",
        held_and_neutralise(),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 2\nrank = 1"
        ),
        command_table(
            1,
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 1\nrank = 2"
        ),
    );
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(
        played.air_orders_serving(1, 3),
        vec![(2, mission(CAP)), (2, share("0.5"))]
    );
    assert_eq!(
        played.air_orders_serving(1, 4),
        vec![(1, mission(strike())), (3, mission(strike()))]
    );
    // The raid task never orders group 2.
    for (day, order) in played.all_orders() {
        if order.intent == intent(4) {
            let OrderKind::AirGroup { target, .. } = &order.kind else {
                unreachable!()
            };
            assert_ne!(*target, group(2), "day {day}");
        }
    }
}

// WHOSE A GROUP IS. No placeholder aircraft type can both search and bomb, so
// the property is tested on `sim_staff::raid::whose`, the rule itself. Its
// arguments are what the group is and which tasks may use it: the rank of the
// intents is not an input, so no rank can change the answer.
#[test]
fn a_group_that_can_search_and_bomb_is_the_raids_and_a_fighter_is_the_covers() {
    // fighter, can bomb the target, a cover may use it, a raid may use it.
    assert_eq!(whose(false, true, true, true), Owner::Raid);
    assert_eq!(whose(true, false, true, true), Owner::Cover);
    // A fighter with only the raid to use it is the raid's.
    assert_eq!(whose(true, false, false, true), Owner::Raid);
    // A group that cannot bomb is the cover's where there is a cover.
    assert_eq!(whose(false, false, true, true), Owner::Cover);
    assert_eq!(whose(false, false, false, true), Owner::Neither);
}

#[test]
fn a_raid_stated_alone_gives_the_main_cases_orders_each_serving_intent_1_with_an_empty_chain() {
    let text = changed(
        &text(),
        NEUTRALISE_COMMAND,
        "intent = 1\nintent_kind = \"raid\"\nbase = 1\ntarget = 2\n",
    );
    let played = play(&text);
    played.nothing_refused();
    assert_eq!(played.state.intents.len(), 1);
    for (i, expected) in table_orders().iter().enumerate() {
        let day = i as u32 + 1;
        assert_eq!(&played.air_orders(day), expected, "day {day}");
    }
    for (day, order) in played.all_orders() {
        assert_eq!(order.intent, intent(1), "day {day}");
        assert_eq!(order.reason.serves.intent, Some(intent(1)), "day {day}");
        assert!(order.reason.serves.chain.is_empty(), "day {day}");
        assert_eq!(
            order.reason.serves.level,
            Some(Addressee::Base(base(1))),
            "day {day}"
        );
    }
    played.every_chain_ends_with_the_commander();
}

#[test]
fn a_held_group_is_left_alone() {
    let text = with(&command_table(3, "kind = \"hold\"\nair_group = 1"));
    let played = play(&text);
    played.nothing_refused();
    for day in 3..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 1),
            "group 1 was ordered on day {day}"
        );
    }
    // Group 3 sends 8 on day 3, against the 6 a raid asks.
    assert_eq!(played.launched_by(3, 3), 8);
}

/// The property as the architect restated it after the build: group 3
/// raids alone when it can send the 6 a raid asks.
#[test]
fn a_held_group_3_raids_alone_when_it_can_send_6() {
    let text = with(&command_table(3, "kind = \"hold\"\nair_group = 1"));
    let played = play(&text);
    played.nothing_refused();
    // Group 1 gets no staff order from day 3.
    for day in 3..=12 {
        assert!(
            played.air_orders(day).iter().all(|(g, _)| *g != 1),
            "group 1 was ordered on day {day}"
        );
    }
    // Group 3's morning fatigue on days 3 to 12.
    let mornings = [12, 18, 24, 30, 36, 42, 32, 38, 44, 34];
    for (k, fatigue) in mornings.iter().enumerate() {
        let day = k as u32 + 3;
        assert_eq!(played.morning(day, 3), n(*fatigue), "day {day}");
    }
    // What group 3 sends: 0 on a day it rests.
    let sends = [8, 8, 7, 7, 6, 0, 6, 6, 0, 6];
    for (k, sent) in sends.iter().enumerate() {
        let day = k as u32 + 3;
        assert_eq!(played.launched_by(day, 3), *sent, "day {day}");
    }
    assert_eq!(played.at_end(3), n(40));
}

#[test]
fn withdrawing_the_end_on_day_5_withdraws_the_raid_and_the_staff_gives_no_order_after() {
    let text = with(&command_table(5, "kind = \"withdraw_intent\"\nintent = 1"));
    let played = play(&text);
    played.nothing_refused();
    let gone = played.all(|k| matches!(k, EventKind::IntentWithdrawn { .. }));
    let two: Vec<&&Event> = gone
        .iter()
        .filter(
            |e| matches!(e.kind, EventKind::IntentWithdrawn { intent: i, .. } if i == intent(2)),
        )
        .collect();
    assert_eq!(two.len(), 1);
    assert_eq!(two[0].day, 5);
    assert!(matches!(
        two[0].kind,
        EventKind::IntentWithdrawn { parent: p, .. } if p == intent(1)
    ));
    let taken = played
        .events
        .iter()
        .find(|e| {
            matches!(
                &e.kind,
                EventKind::CommandTaken {
                    command: CommandKind::WithdrawIntent { intent: i },
                    ..
                } if *i == intent(1)
            )
        })
        .expect("the withdrawal was taken");
    assert!(two[0].causes.contains(&Cause::Event(taken.id)));
    assert_eq!(played.state.intents[&intent(2)].withdrawn_day, Some(5));
    for day in 1..=4 {
        assert_eq!(played.air_orders(day), table_orders()[(day - 1) as usize]);
    }
    for day in 5..=12 {
        let d = played.decisions(day);
        assert!(d.orders.is_empty(), "day {day}");
        assert!(d.commands.is_empty(), "day {day}");
    }
    // The groups keep the missions they had on day 4.
    assert_eq!(played.mission_at_end(1), strike());
    assert_eq!(played.mission_at_end(2), CAP);
    assert_eq!(played.mission_at_end(3), strike());
}

#[test]
fn the_staff_orders_no_group_the_raid_may_not_use() {
    // A forces limit on intent 1 that names groups 1 and 2: the raid under it
    // reads it up the tree and never orders group 3.
    let text = with(&command_table(
        1,
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\nair_groups = [1, 2]",
    ));
    let played = play(&text);
    played.nothing_refused();
    for (day, order) in played.all_orders() {
        let OrderKind::AirGroup { target, .. } = &order.kind else {
            unreachable!()
        };
        assert_ne!(*target, group(3), "group 3 was ordered on day {day}");
    }
    assert_eq!(played.at_end(3), n(0));
    assert_eq!(played.launched_by(1, 3), 0);
    assert_eq!(played.launched_by(1, 1), 10);
}

#[test]
fn the_same_start_gives_the_same_intents_orders_and_reasons() {
    let a = play(&text());
    let b = play(&text());
    assert_eq!(a.handed, b.handed);
    assert_eq!(a.state, b.state);
    assert_eq!(a.events.len(), b.events.len());
}

#[test]
fn two_worlds_that_differ_only_in_what_the_allied_side_cannot_see_give_the_same_decisions() {
    let hidden = changed(
        &text(),
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 0",
        "side = \"japanese\"\nq = 5\nr = 0\nsupply_t = 777",
    );
    let hidden = format!(
        "{hidden}\n[[base]]\nid = 3\nname = \"Placeholder base 3\"\nside = \"japanese\"\nq = 7\nr = 7\nsupply_t = 50\non_map = false\n\n[[air_group]]\nid = 5\nname = \"Placeholder Japanese fighters 5\"\nside = \"japanese\"\nbase = 3\naircraft_type = 4\nready = 20\n"
    );
    let plain = play(&text());
    let other = play(&hidden);
    assert_ne!(
        plain.truth.last().map(|t| t.bases.len()),
        other.truth.last().map(|t| t.bases.len()),
        "the two worlds do not differ"
    );
    assert_eq!(plain.handed, other.handed);
    assert_eq!(plain.state, other.state);
}
