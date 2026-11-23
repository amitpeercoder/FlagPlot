//! Scenario 130, part A: the tests of its statements (see the doc comment
//! of the file above).

#[allow(unused_imports)]
use super::common;

pub use harness::*;

/// The game's harness, shared with parts B, E and G (they `use` it as
/// `super::part_a::...`): the scenario's text, how a variant is built from
/// it, how it is played, and what the staff handed in.
#[allow(dead_code)]
pub mod harness {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use sim_core::command::{
        CommandKind, IntentKind, NoticeKind, Reason, StaffDecisions, StaffOrder,
    };
    use sim_core::hex::Hex;
    use sim_core::ids::{AirGroupId, IntentId, TaskForceId};
    use sim_core::orders::{AirGroupOrder, OrderKind, TaskForceOrder};
    use sim_core::state::{Side, TruthState};
    use sim_data::{LoadOptions, Scenario};
    use sim_staff::{as_staffs, staffs};

    use super::super::common;

    pub fn path() -> PathBuf {
        common::data_dir()
            .join("scenarios/a-limit-binds-what-it-names/130_a_limit_binds_what_it_names.toml")
    }

    pub fn text() -> String {
        std::fs::read_to_string(path()).expect("the scenario file reads")
    }

    pub fn hex(q: i32, r: i32) -> Hex {
        Hex { q, r }
    }

    /// Carrier force C, task force 1.
    pub fn c() -> TaskForceId {
        TaskForceId::new(1)
    }

    /// Adds `what` at the mark `# MARK: name`; the mark stays.
    pub fn at(text: &str, name: &str, what: &str) -> String {
        let mark = format!("# MARK: {name}\n");
        common::replace_once(text, &mark, &format!("{what}{mark}"))
    }

    /// A `[[command]]` of the Allied side on `day`.
    pub fn command(day: u32, body: &str) -> String {
        format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
    }

    /// `body` added at the commands mark, on day 1.
    pub fn with(text: &str, body: &str) -> String {
        at(text, "commands", &command(1, body))
    }

    /// A forces limit on `intent` with `rest` as its other keys.
    pub fn forces_limit(text: &str, intent: u32, rest: &str) -> String {
        with(
            text,
            &format!("kind = \"set_limit\"\nlimit = \"forces\"\nintent = {intent}\n{rest}"),
        )
    }

    /// A game played with the doctrine's staffs: what the Allied staff handed
    /// in each day, the truth at the end of each day (index 0 is the start).
    pub struct Played {
        pub handed: BTreeMap<u32, StaffDecisions>,
        pub truth: Vec<TruthState>,
        pub scenario: Scenario,
    }

    pub fn play_with(text: &str, options: &LoadOptions, days: u32) -> Played {
        play_at(text, &path(), options, days, Side::Allied)
    }

    /// A game from `text` as if it lay at `file` (so that its relative paths
    /// read), played for `side`.
    pub fn play_at(
        text: &str,
        file: &std::path::Path,
        options: &LoadOptions,
        days: u32,
        side: Side,
    ) -> Played {
        let mut scenario = common::from_text_with(text, file, options).expect("the scenario loads");
        let held = staffs();
        let mut handed = BTreeMap::new();
        let mut truth = vec![scenario.world.umpire().clone()];
        loop {
            let day = scenario.world.day() + 1;
            if day > days {
                break;
            }
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
                    .get(&side)
                    .cloned()
                    .unwrap_or_default(),
            );
            truth.push(scenario.world.umpire().clone());
        }
        Played {
            handed,
            truth,
            scenario,
        }
    }

    /// Played on the frozen step 1 figures.
    pub fn play(text: &str, days: u32) -> Played {
        play_with(text, &common::frozen(), days)
    }

    /// A reason's text: its calculation, its evidence and its bindings (not
    /// its list of limits, which part D proves).
    pub fn words(reason: &Reason) -> String {
        let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
        format!(
            "{} | {} | {}",
            reason.worked,
            evidence.join(" | "),
            reason.binding.join(" | ")
        )
    }

    impl Played {
        pub fn decisions(&self, day: u32) -> &StaffDecisions {
            &self.handed[&day]
        }

        /// Every task the staff formed on `day`: (parent, kind).
        pub fn formed_all(&self, day: u32) -> Vec<(IntentId, IntentKind)> {
            self.decisions(day)
                .commands
                .iter()
                .filter_map(|c| match c.kind {
                    CommandKind::FormIntent { parent, kind } => Some((parent, kind)),
                    _ => None,
                })
                .collect()
        }

        /// The carrier tasks the staff formed on `day`: (parent, force, hex).
        pub fn formed(&self, day: u32) -> Vec<(IntentId, TaskForceId, Hex)> {
            self.formed_all(day)
                .into_iter()
                .filter_map(|(parent, kind)| match kind {
                    IntentKind::OperateCarriers { force, hex } => Some((parent, force, hex)),
                    _ => None,
                })
                .collect()
        }

        /// The reason of the first task formed on `day` for which `pick` is
        /// true.
        pub fn formed_reason(&self, day: u32, pick: impl Fn(&IntentKind) -> bool) -> String {
            self.decisions(day)
                .commands
                .iter()
                .find(|c| matches!(&c.kind, CommandKind::FormIntent { kind, .. } if pick(kind)))
                .map(|c| words(&c.reason))
                .unwrap_or_default()
        }

        /// The reason of the carrier task formed on `day`.
        pub fn station_reason(&self, day: u32) -> String {
            self.formed_reason(day, |k| matches!(k, IntentKind::OperateCarriers { .. }))
        }

        pub fn force_orders(&self, day: u32, f: TaskForceId) -> Vec<TaskForceOrder> {
            self.decisions(day)
                .orders
                .iter()
                .filter_map(|o| match &o.kind {
                    OrderKind::TaskForce { target, order } if *target == f => Some(order.clone()),
                    _ => None,
                })
                .collect()
        }

        pub fn route_of(&self, day: u32, f: TaskForceId) -> Option<Vec<Hex>> {
            self.force_orders(day, f).into_iter().find_map(|o| match o {
                TaskForceOrder::SetRoute { waypoints, .. } => Some(waypoints),
                _ => None,
            })
        }

        pub fn group_orders(&self, day: u32, g: u32) -> Vec<AirGroupOrder> {
            self.decisions(day)
                .orders
                .iter()
                .filter_map(|o: &StaffOrder| match &o.kind {
                    OrderKind::AirGroup { target, order } if *target == AirGroupId::new(g) => {
                        Some(order.clone())
                    }
                    _ => None,
                })
                .collect()
        }

        /// Every reason of a day as one text.
        pub fn said(&self, day: u32) -> String {
            let d = self.decisions(day);
            d.orders
                .iter()
                .map(|o| &o.reason)
                .chain(d.no_orders.iter())
                .chain(d.commands.iter().map(|c| &c.reason))
                .map(words)
                .collect::<Vec<_>>()
                .join("\n")
        }

        pub fn cannot_meet(&self, day: u32) -> bool {
            self.decisions(day)
                .notices
                .iter()
                .any(|n| n.kind == NoticeKind::CannotMeet)
        }
    }
}

// ---------------------------------------------------------------- Statements

use sim_core::command::IntentKind;
use sim_core::ids::{BaseId, IntentId, TaskForceId};
use sim_core::orders::{AirGroupOrder, TaskForceOrder};

/// The game the statements of Part A are on:
/// `data/scenarios/a-limit-binds-what-it-names/130_a_limit_binds_what_it_names.toml`
/// (its header says what it holds). Day 1 only is played: the Allied side
/// holds a deny of the waters within 2 hexes of (0,0) as intent 1; base B
/// (base 2) lies 4 hexes west of them with a bomber group (4) whose normal
/// reach of 210 nm touches them from 160 nm, and a group (5) that searches
/// to 300 nm; C (task force 1) lies at (10,0), 7 hexes from the station (3,0)
/// that decision 0202's rule chooses for it (scenario 116, statement 2); S
/// (task force 2) is a surface force far away that no order uses unless it is
/// named.
fn one_day(text: &str) -> Played {
    play(text, 1)
}

fn operate(force: u32, h: sim_core::hex::Hex) -> IntentKind {
    IntentKind::OperateCarriers {
        force: TaskForceId::new(force),
        hex: h,
    }
}

fn strike_at_b() -> IntentKind {
    IntentKind::StrikeShips {
        base: BaseId::new(2),
    }
}

fn search_from_b() -> IntentKind {
    IntentKind::SearchWaters {
        base: BaseId::new(2),
    }
}

/// What the staff formed on day 1 under the deny, as (kind) in the order it
/// forms them.
fn kinds(played: &Played) -> Vec<IntentKind> {
    played
        .formed_all(1)
        .into_iter()
        .inspect(|(parent, _)| assert_eq!(*parent, IntentId::new(1)))
        .map(|(_, kind)| kind)
        .collect()
}

/// Statement 1's base tasks: what the staff forms with no limit.
fn no_limit_kinds() -> Vec<IntentKind> {
    vec![operate(1, hex(3, 0)), strike_at_b(), search_from_b()]
}

/// The orders the staff gave the groups at B on day 1 (groups 4 and 5).
fn base_group_orders(played: &Played) -> (Vec<AirGroupOrder>, Vec<AirGroupOrder>) {
    (played.group_orders(1, 4), played.group_orders(1, 5))
}

/// Statement 1: with no forces limit, the staff forms the tasks at B for the
/// waters that it forms today, the base tasks (strike ships from B, search
/// the waters from B), and the carrier pass stations C at (3,0). Nothing is
/// formed for S, which no order names. The orders are exact: group 4 is set
/// to the naval strike mission with its strikes sized, group 5 to search,
/// once a day, in the arc 040 to 140 out to 300 nm (the waters within 2
/// hexes of (0,0), widened 30 degrees at each end), and C is routed to
/// (3,0) at full speed (scenario 116's statements 2 and 3).
#[test]
fn statement_1_with_no_limit_the_staff_forms_the_base_tasks() {
    let played = one_day(&text());
    assert_eq!(kinds(&played), no_limit_kinds());
    let (bombers, searchers) = base_group_orders(&played);
    assert_eq!(
        bombers,
        vec![
            AirGroupOrder::SetAirMission(sim_core::orders::AirMission::NavalStrike),
            AirGroupOrder::SetStrikeSizing { on: true },
        ]
    );
    assert_eq!(
        searchers,
        vec![
            AirGroupOrder::SetAirMission(sim_core::orders::AirMission::Search),
            AirGroupOrder::SetSearchOnce { on: true },
            AirGroupOrder::SetSearchArc {
                from_deg: 40,
                to_deg: 140,
                range_nm: sim_core::fixed::Fixed::from_int(300)
            },
        ]
    );
    assert_eq!(played.route_of(1, c()), Some(vec![hex(3, 0)]));
    assert!(played
        .station_reason(1)
        .contains("task_force:1 is free: on the carrier mission"));
    assert!(played
        .formed_reason(1, |k| *k == strike_at_b())
        .contains("air_group:4 (Placeholder dive bomber, 210 nm, a reach of 210 nm)"));
    assert!(played
        .formed_reason(1, |k| *k == search_from_b())
        .contains("air_group:5 (Placeholder carrier scout, a search range of 300 nm)"));
    // Nothing is formed for S, and no order is given to it.
    assert!(played.force_orders(1, TaskForceId::new(2)).is_empty());
}

/// Statement 2: a forces limit on the deny that names C and nothing else
/// still leaves the base tasks, with the same groups: the same tasks, the
/// same orders to groups 4 and 5 and the same reasons for the two base
/// tasks as statement 1's game. C serves the order, stationed at (3,0)
/// (statement 4 has more). Before this rule the order had no base at all.
#[test]
fn statement_2_a_limit_naming_a_carrier_force_leaves_the_base_tasks() {
    let plain = one_day(&text());
    let limited = one_day(&forces_limit(&text(), 1, "task_forces = [1]"));
    assert_eq!(kinds(&limited), no_limit_kinds());
    assert_eq!(base_group_orders(&limited), base_group_orders(&plain));
    for pick in [strike_at_b(), search_from_b()] {
        assert_eq!(
            limited.formed_reason(1, |k| *k == pick),
            plain.formed_reason(1, |k| *k == pick),
            "{pick:?}"
        );
    }
    // C serves the order.
    assert_eq!(limited.formed(1), vec![(IntentId::new(1), c(), hex(3, 0))]);
    // The two groups at B were allowed by no named group: the groups are the
    // staff's.
    assert!(limited
        .formed_reason(1, |k| *k == strike_at_b())
        .contains("air_group:4"));
}

/// Statement 3: a limit that names the bomber group of B (group 4) and
/// nothing else binds the air groups: group 5 is not used by the order, so
/// no search task is formed at B and group 5 is given no order; the strike
/// task is formed with group 4. A task force the staff would have used
/// with no limit, C, is still used: stationed at (3,0), as free. And a limit
/// that names a task force (S, task force 2) and no group leaves the groups
/// to the staff: both base tasks are formed with statement 1's groups, and S
/// is given the task to stand at the centre of the waters, (0,0); the task
/// forces are bound by that limit, so C, not named, is not stationed.
#[test]
fn statement_3_a_limit_naming_a_group_binds_the_groups_and_not_the_task_forces() {
    let limited = one_day(&forces_limit(&text(), 1, "air_groups = [4]"));
    assert_eq!(kinds(&limited), vec![operate(1, hex(3, 0)), strike_at_b()]);
    assert!(limited.group_orders(1, 5).is_empty());
    assert!(!limited.group_orders(1, 4).is_empty());
    assert_eq!(limited.route_of(1, c()), Some(vec![hex(3, 0)]));
    assert!(limited.station_reason(1).contains("task_force:1 is free"));

    // The converse: a task force named and no group. The groups are the
    // staff's, so both base tasks are formed with the groups statement 1 has;
    // ships are bound, so S is the one force used (stood at the centre of the
    // waters) and C, not named, is not stationed.
    let named_s = one_day(&forces_limit(&text(), 1, "task_forces = [2]"));
    assert_eq!(
        kinds(&named_s),
        vec![
            strike_at_b(),
            search_from_b(),
            IntentKind::StandAt {
                force: TaskForceId::new(2),
                hex: hex(0, 0)
            },
        ]
    );
    assert_eq!(
        base_group_orders(&named_s),
        base_group_orders(&one_day(&text()))
    );
    assert_eq!(named_s.route_of(1, c()), None);
}

/// Statement 4, the first half: with a forces limit that names C, the staff
/// stations C as it stations a free carrier force for the order, at the hex
/// its rule chooses, (3,0), 7 hexes of passage, and not at (10,0), the hex C
/// lies in that morning; its reason says C is named in the order's forces
/// limit, not that it is free; C is routed to (3,0).
#[test]
fn statement_4_a_named_carrier_force_is_stationed_by_the_staff() {
    let played = one_day(&forces_limit(&text(), 1, "task_forces = [1]"));
    assert_eq!(played.formed(1), vec![(IntentId::new(1), c(), hex(3, 0))]);
    assert_eq!(played.route_of(1, c()), Some(vec![hex(3, 0)]));
    let said = played.station_reason(1);
    assert!(
        said.contains("The station is (3,0), 120 nm (3 hexes) east of the centre of the waters"),
        "{said}"
    );
    assert!(
        said.contains("is named in your order's forces limit"),
        "{said}"
    );
    assert!(!said.contains("is free"), "{said}");
    assert!(said.contains("7 hexes of passage"), "{said}");
    assert!(!played
        .force_orders(1, c())
        .iter()
        .any(|o| matches!(o, TaskForceOrder::SetRoute { waypoints, .. } if waypoints == &vec![hex(10, 0)])));
}

/// Statement 4, the second half: where the commander gives C the task
/// himself, an order (intent 2) to operate C from the hex (5,0), with the
/// forces limit naming C on the deny, C works from that hex: the staff forms
/// no carrier task, the commander's stands as it was stated, and C is
/// routed to (5,0), not to the staff's (3,0).
#[test]
fn statement_4_a_carrier_force_the_commander_tasked_works_from_his_hex() {
    let t = forces_limit(&text(), 1, "task_forces = [1]");
    let t = with(
        &t,
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"operate_carriers\"\ntask_force = 1\nhex = [5, 0]",
    );
    let played = one_day(&t);
    assert_eq!(played.formed(1), vec![]);
    assert_eq!(played.route_of(1, c()), Some(vec![hex(5, 0)]));
    let state = played
        .scenario
        .world
        .side_view(sim_core::state::Side::Allied)
        .command_state()
        .clone();
    assert_eq!(
        state.intents.get(&IntentId::new(2)).map(|i| i.kind),
        Some(operate(1, hex(5, 0)))
    );
}

// ------------------------------------------------------------- Statement 5

/// The holds' game of `130_two_holds.toml` made one place held by two men:
/// both units at T (base 2) on the attack, and a group of fighters (group 1)
/// at base 2 for its cover; with `limit` as the forces limit of the hold
/// (intent 1) in place of the file's. Played two days.
pub fn hold_with_men(limit: &str) -> Played {
    use super::part_e::{holds_path, holds_text};
    let t = holds_text();
    let t = common::replace_once(
        &t,
        "q = 0\nr = 5\nposture = \"defend\"",
        "q = 5\nr = 0\nposture = \"attack\"",
    );
    let t = common::replace_once(
        &t,
        "q = 5\nr = 0\nposture = \"defend\"",
        "q = 5\nr = 0\nposture = \"attack\"",
    );
    let t = at(
        &t,
        "units",
        "[[air_group]]\nid = 1\nname = \"Placeholder fighters 1\"\nside = \"allied\"\nbase = 2\naircraft_type = 4\nready = 8\n\n",
    );
    let t = common::replace_once(
        &t,
        "intent = 1\ntask_forces = [1]\nsource = 1",
        &format!("intent = 1\n{limit}"),
    );
    play_at(&t, &holds_path(), &common::frozen(), 2, Side::Allied)
}

/// The posture orders of the hold on day 1: the land units it set to defend.
pub fn units_set_to_defend(played: &Played) -> Vec<u32> {
    played
        .decisions(1)
        .orders
        .iter()
        .filter_map(|o| match &o.kind {
            sim_core::orders::OrderKind::LandUnit {
                target,
                order:
                    sim_core::orders::LandUnitOrder::SetPosture {
                        posture: sim_core::orders::Posture::Defend,
                        ..
                    },
            } => Some(target.raw()),
            _ => None,
        })
        .collect()
}

/// The air missions given to the fighters (group 1) at base 2 by the cover
/// the staff formed under the hold, on day 1.
pub fn cover_orders(played: &Played) -> Vec<AirGroupOrder> {
    played.group_orders(1, 1)
}

use sim_core::state::Side;

/// Statement 5, land units. A hold of base 2 (T) with two fighting land
/// units, 1 and 2, ashore at its base on the attack, and a group of
/// fighters, group 1, there for its cover. With no limit the hold sets both
/// units to defend. With a forces limit that names land unit 1 and nothing
/// else, land units are bound: unit 1 is set to defend and unit 2 is given
/// no order; and nothing else is bound: the cover is still formed and still
/// orders the base's fighters to the patrol at the share of the balanced
/// risk.
#[test]
fn statement_5_a_limit_naming_a_land_unit_binds_land_units_and_nothing_else() {
    let plain = hold_with_men("task_forces = [1]\nsource = 1");
    assert_eq!(units_set_to_defend(&plain), vec![1, 2]);
    let limited = hold_with_men("land_units = [1]");
    assert_eq!(units_set_to_defend(&limited), vec![1]);
    // The cover is the staff's: the same tasks formed and the same orders
    // to the base's fighters as with no land unit named.
    assert!(limited
        .formed_all(1)
        .iter()
        .any(|(parent, kind)| *parent == IntentId::new(1)
            && *kind
                == IntentKind::Cover {
                    base: BaseId::new(2)
                }));
    assert_eq!(cover_orders(&limited), cover_orders(&plain));
    assert_eq!(
        cover_orders(&limited),
        vec![
            AirGroupOrder::SetAirMission(sim_core::orders::AirMission::CombatAirPatrol),
            AirGroupOrder::SetPatrolShare {
                share: sim_core::fixed::Fixed::from_ratio(1, 2)
            },
        ]
    );
}

/// Statement 5, a source. The same hold with a forces limit that names only
/// a source (base 1) binds the source and nothing else: both land units are
/// set to defend, and the cover is formed and orders the base's fighters,
/// as when no kind is named; the supply the staff forms under the hold is
/// formed with that source. (What the supply may use for ships is the
/// forces limit's own ships and task forces, and this limit names none: the
/// staff's reason on day 2 says no ship of the pool is afloat. That is not
/// asserted here; it is reported with the statements.)
#[test]
fn statement_5_a_limit_naming_a_source_binds_the_source_and_nothing_else() {
    let plain = hold_with_men("task_forces = [1]\nsource = 1");
    let limited = hold_with_men("source = 1");
    assert_eq!(units_set_to_defend(&limited), vec![1, 2]);
    assert_eq!(cover_orders(&limited), cover_orders(&plain));
    assert!(limited
        .formed_all(1)
        .iter()
        .any(|(parent, kind)| *parent == IntentId::new(1)
            && *kind
                == IntentKind::Cover {
                    base: BaseId::new(2)
                }));
    // The source binds: the hold forms its supply, which a limit with no
    // source names does not (the land-unit game above forms none and says
    // no source is named on it or above it, decision 0222, rule 1).
    assert!(limited
        .formed_all(1)
        .iter()
        .any(|(parent, kind)| *parent == IntentId::new(1)
            && matches!(kind, IntentKind::KeepSupplied { base, .. } if *base == BaseId::new(2))));
    let land_only = hold_with_men("land_units = [1]");
    assert!(!land_only
        .formed_all(1)
        .iter()
        .any(|(_, kind)| matches!(kind, IntentKind::KeepSupplied { .. })));
    assert!(land_only
        .said(1)
        .contains("no source is named on it or above it"));
}

/// The patrol game of statement 5: scenario 129's seven boats of the
/// Japanese side at (12,0), and "patrol the waters within 2 hexes of (5,0)",
/// from `data/scenarios/table-submarines/129_the_patrol.toml`, with a forces
/// limit `limit` on the order (or none). The orders of day 1 as (boat,
/// station).
pub fn patrol(limit: Option<&str>) -> Vec<(u32, sim_core::hex::Hex)> {
    let file = common::data_dir().join("scenarios/table-submarines/129_the_patrol.toml");
    let mut t = std::fs::read_to_string(&file).expect("the file reads");
    t.push_str(
        "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_intent\"\nintent = 1\nintent_kind = \"patrol\"\nhex = [5, 0]\ndistance = 2\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_leave\"\nleave = \"prepare\"\ngiven = true\n\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_leave\"\nleave = \"launch\"\ngiven = true\n",
    );
    if let Some(limit) = limit {
        t.push_str(&format!(
            "\n[[command]]\nday = 1\nside = \"japanese\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\n{limit}\n"
        ));
    }
    let played = play_at(&t, &file, &common::frozen(), 1, Side::Japanese);
    played
        .decisions(1)
        .orders
        .iter()
        .filter_map(|o| match &o.kind {
            sim_core::orders::OrderKind::Submarine {
                target,
                order: sim_core::orders::SubmarineOrder::Station(h),
            } => Some((target.raw(), *h)),
            _ => None,
        })
        .collect()
}

/// Statement 5, boats, on an order that reads boats, the patrol: with a
/// limit naming boat 2 only boat 2 is ordered, and to the first station,
/// (5,0); with a limit that names only the source (base 1), boats are not
/// bound and the staff orders all seven to the seven stations of scenario
/// 129's table.
#[test]
fn statement_5_a_limit_naming_a_boat_binds_boats_on_a_patrol() {
    assert_eq!(patrol(Some("submarines = [2]")), vec![(2, hex(5, 0))]);
    let table = [(5, 0), (3, 0), (3, 2), (5, -2), (5, 2), (7, -2), (7, 0)];
    let all: Vec<_> = table
        .iter()
        .enumerate()
        .map(|(i, (q, r))| (i as u32 + 1, hex(*q, *r)))
        .collect();
    assert_eq!(patrol(Some("source = 1")), all);
    assert_eq!(patrol(None), all);
}

/// Statement 5, boats, on an order that does not read them, the deny: with
/// an Allied boat (boat 1, at base H) in statement 1's game and a limit
/// naming only that boat, nothing but boats is bound: the staff still
/// forms statement 1's tasks (the base tasks at B with the same groups, and
/// C stationed at (3,0)) and gives the same orders.
#[test]
fn statement_5_a_limit_naming_a_boat_leaves_the_bases_and_carriers_to_the_staff() {
    let with_boat = |t: &str| {
        let t = common::replace_once(
            t,
            "air_combat = \"../../reference/air_combat.toml\"\n",
            "air_combat = \"../../reference/air_combat.toml\"\nsubmarine_classes = \"../../reference/submarine_classes_placeholder_scenario_128.csv\"\nsubmarine_torpedoes = \"../../reference/submarine_torpedoes_placeholder_scenario_128.csv\"\nsubmarines = \"../../reference/submarines_placeholder_scenario_128.toml\"\n",
        );
        at(
            &t,
            "forces",
            "[[submarine]]\nid = 1\nname = \"Placeholder submarine 1\"\nside = \"allied\"\nclass = 6900\nq = 10\nr = 2\nhome = 1\n\n",
        )
    };
    let plain = one_day(&with_boat(&text()));
    assert_eq!(kinds(&plain), no_limit_kinds());
    let limited = one_day(&forces_limit(&with_boat(&text()), 1, "submarines = [1]"));
    assert_eq!(kinds(&limited), no_limit_kinds());
    assert_eq!(base_group_orders(&limited), base_group_orders(&plain));
    assert_eq!(limited.route_of(1, c()), Some(vec![hex(3, 0)]));
}

// ------------------------------------------------------------- Statement 6

/// Statement 6, two limits on one chain. The deny (intent 1) has a forces
/// limit L1 that names task force 1 (C) and the air groups 4 and 5; on day 2
/// the commander sets a limit L2 on the strike task the staff formed under
/// it on day 1 (intent 3, strike ships from B) that names air group 5 only.
/// Read up the chain from intent 3:
/// - air groups, a kind both name: only group 5, named by both, is used by
///   the strike task; group 4, named by L1 alone, is not. The staff's day-2
///   reason for the strike task speaks of group 5 and not of group 4. With
///   L2 naming group 4 instead it speaks of group 4 and not of group 5.
/// - task forces, a kind L1 alone names: bound by L1 alone: the one limit on
///   the chain that binds ships is L1, which names C, and C's station stands.
///
/// And for the search task (intent 4), which L2 is not on, only L1 binds the
/// groups: 4 and 5 both.
#[test]
fn statement_6_two_limits_on_the_chain_each_binds_what_it_names() {
    let l1 = "task_forces = [1]\nair_groups = [4, 5]";
    let game = |l2: &str| {
        let t = forces_limit(&text(), 1, l1);
        let t = at(
            &t,
            "commands",
            &command(
                2,
                &format!("kind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\n{l2}"),
            ),
        );
        play(&t, 2)
    };
    let strike_reason = |played: &Played| {
        played
            .decisions(2)
            .no_orders
            .iter()
            .map(words)
            .find(|w| w.starts_with("No order is given for intent:3,"))
            .expect("the strike task's reason of day 2")
    };
    let group = |raw: u32| sim_core::ids::AirGroupId::new(raw);

    let played = game("air_groups = [5]");
    let state = played
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    // Air groups: named by both, only 5; named by L1 alone (intent 4), both.
    assert_eq!(
        state.groups_on_chain(IntentId::new(3)),
        Some([group(5)].into_iter().collect())
    );
    assert_eq!(
        state.groups_on_chain(IntentId::new(4)),
        Some([group(4), group(5)].into_iter().collect())
    );
    // Task forces: L2 names none, so L1 alone binds them.
    let ships = state.ship_limits_on_chain(IntentId::new(3));
    assert_eq!(ships.len(), 1);
    assert_eq!(
        ships[0].task_forces,
        [TaskForceId::new(1)].into_iter().collect()
    );
    let said = strike_reason(&played);
    assert!(said.contains("air_group:5,"), "{said}");
    assert!(!said.contains("air_group:4"), "{said}");
    // C's station is as it was.
    assert!(played
        .decisions(2)
        .no_orders
        .iter()
        .map(words)
        .any(|w| w.contains("stands under intent:1: its station was chosen")));

    let other = game("air_groups = [4]");
    let said = strike_reason(&other);
    assert!(said.contains("air_group:4,"), "{said}");
    assert!(!said.contains("air_group:5"), "{said}");
    let state = other
        .scenario
        .world
        .side_view(Side::Allied)
        .command_state()
        .clone();
    assert_eq!(
        state.groups_on_chain(IntentId::new(3)),
        Some([group(4)].into_iter().collect())
    );
}

// ------------------------------------------------------------ Event helpers

impl Played {
    /// The refusals of commands in the game, in order.
    pub fn refusals(&self) -> Vec<(u32, sim_core::command::CommandRefusal)> {
        self.scenario
            .world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                sim_core::events::EventKind::CommandRefused { reason, .. } => {
                    Some((e.day, *reason))
                }
                _ => None,
            })
            .collect()
    }

    /// The days of the plan amendments in the game, with their plans.
    pub fn amendments(&self) -> Vec<(u32, sim_core::command::PlanId)> {
        self.scenario
            .world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                sim_core::events::EventKind::PlanAmended { plan, .. } => Some((e.day, *plan)),
                _ => None,
            })
            .collect()
    }
}
