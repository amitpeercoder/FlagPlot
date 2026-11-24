//! Scenario 130, part D: the tests of its statements (see the doc comment
//! of the file above).
//!
//! A reason says which limits it speaks of (decision 0222, rule 5). Each
//! test plays a staffed game, as `sim-cli run` plays it (the doctrine's
//! staffs), reads what the Allied staff handed in, and finds the reason
//! the statement is of by the words it says: its binding and what would
//! change it. The list is `Reason::limits`; a limit in it is a kind and
//! the order it is set on (`ReasonLimit { kind, intent, base }`), the
//! order being the nearest on the chain of the order the reason serves
//! that sets it, and, where none does, the root of the chain.
//!
//! Every game is a scenario file on file, read as it is, or one with a
//! command or a line changed here (the same changes the older proofs of
//! these scenarios make). The games and the reasons they are asked of:
//!
//! - 17: scenario 86, a hold and the supply the staff forms under it; and
//!   scenario 54, a deny that names a force in its forces limit.
//! - 18: scenario 40a (the area), scenario 69 with a hold whose forces
//!   limit names no source (the source), scenario 116 with two orders
//!   (the rank), scenario 53 (the risk, set on the order above), scenario
//!   116 with a targets limit (the targets), and scenario 81 in four
//!   forms, with "not before", "wait for", "after" and scenario 82 with a
//!   knowledge limit (a landing's four).
//! - 19: scenario 116, the reasons whose words name no limit.
//! - 20: scenarios 116, 54 and 86; see [`statement_20_changes_no_order_no_command_and_no_hash`].

use super::common;

use std::collections::BTreeMap;
use std::path::PathBuf;

use sim_core::command::{LimitKind, Reason, ReasonLimit, StaffDecisions};
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_data::Scenario;
use sim_staff::{as_staffs, staffs};

// ---------------------------------------------------------------- The harness

/// A game played with the doctrine's staffs: what the Allied staff handed
/// in each day, and the scenario at the end (for its command state).
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    scenario: Scenario,
}

fn scenario_path(relative: &str) -> PathBuf {
    common::data_dir().join("scenarios").join(relative)
}

fn text_of(relative: &str) -> String {
    std::fs::read_to_string(scenario_path(relative)).expect("the scenario file reads")
}

fn play(relative: &str, text: &str, days: u32) -> Played {
    let mut scenario = common::from_text(text, &scenario_path(relative)).expect("it loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
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
                .get(&Side::Allied)
                .cloned()
                .unwrap_or_default(),
        );
    }
    Played { handed, scenario }
}

/// Every reason of what was handed in on `day`: the orders', the
/// commands' and the decisions to give no order.
fn reasons_of(decisions: &StaffDecisions) -> Vec<&Reason> {
    let mut found: Vec<&Reason> = Vec::new();
    found.extend(decisions.orders.iter().map(|o| &o.reason));
    found.extend(decisions.commands.iter().map(|c| &c.reason));
    found.extend(decisions.no_orders.iter());
    found
}

fn says(reason: &Reason, words: &str) -> bool {
    reason
        .binding
        .iter()
        .chain(reason.would_change.iter())
        .any(|line| line.contains(words))
}

impl Played {
    /// The reasons of `day` that serve `intent` and say `words` in their
    /// binding or in what would change them.
    fn reasons(&self, day: u32, intent: u32, words: &str) -> Vec<&Reason> {
        let handed = self.handed.get(&day).expect("the day was played");
        reasons_of(handed)
            .into_iter()
            .filter(|r| r.serves.intent == Some(IntentId::new(intent)) && says(r, words))
            .collect()
    }

    /// The one reason of `day` that serves `intent` and says `words`.
    fn the_reason(&self, day: u32, intent: u32, words: &str) -> &Reason {
        let found = self.reasons(day, intent, words);
        assert!(
            !found.is_empty(),
            "day {day}: no reason of intent:{intent} says {words:?}; the day's reasons: {}",
            self.summary(day)
        );
        found[0]
    }

    /// What a day's reasons say, for a failure's message.
    fn summary(&self, day: u32) -> String {
        reasons_of(self.handed.get(&day).expect("the day was played"))
            .iter()
            .map(|r| {
                format!(
                    "[serves {:?}; binding {:?}; would change {:?}; limits {:?}]",
                    r.serves.intent, r.binding, r.would_change, r.limits
                )
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn intent(&self, id: u32) -> &sim_core::command::Intent {
        self.scenario
            .world
            .command_state(Side::Allied)
            .intents
            .get(&IntentId::new(id))
            .expect("the intent is in the state")
    }
}

/// A limit a reason speaks of: its kind and the order it is set on.
fn on(kind: LimitKind, intent: u32) -> ReasonLimit {
    ReasonLimit {
        kind,
        intent: Some(IntentId::new(intent)),
        base: None,
    }
}

fn changed(text: &str, from: &str, to: &str) -> String {
    common::replace_once(text, from, to)
}

/// Adds `what` at the mark `# MARK: name`; the mark stays (scenario 116's
/// and 106's way).
fn at_mark(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    common::replace_once(text, &mark, &format!("{what}{mark}"))
}

fn command(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

const S116: &str = "carriers-serve-an-end/116_carriers_serve_an_end.toml";

// ------------------------------------------------------------- Statement 17

/// Statement 17: a forces limit set on an order above the one the reason
/// is of. Scenario 86: the staff holds a base with a forces limit on the
/// hold, forms the supply under it (a supply intent with no forces limit
/// of its own, which reads the hold's), and on day 3 the pool of the
/// supply has no ship afloat. The reason says what would change it is "a
/// ship afloat in the forces limit"; the limit it speaks of is the forces
/// limit, on the hold above the supply.
#[test]
fn statement_17_a_forces_limit_set_above_names_the_order_above() {
    let relative = "86_fed_with_the_lift.toml";
    let played = play(relative, &text_of(relative), 3);
    let reason = played.the_reason(3, 7, "a ship afloat in the forces limit");
    assert_eq!(reason.limits, vec![on(LimitKind::Forces, 3)]);
    // The order the reason serves is the supply, formed under the hold,
    // and it sets no forces limit of its own; the hold does.
    let supply = played.intent(7);
    assert_eq!(supply.parent, Some(IntentId::new(3)));
    assert!(supply.limits.forces.is_none());
    assert!(played.intent(3).limits.forces.is_some());
    // The words are what they were.
    assert_eq!(reason.binding, vec!["the pool has no ship afloat"]);
    assert_eq!(
        reason.would_change,
        vec!["a ship afloat in the forces limit"]
    );
}

/// Statement 17: a forces limit set on the order the reason is of.
/// Scenario 54: the deny names task force 1 in its forces limit, and the
/// reason of the task the staff forms says so.
#[test]
fn statement_17_a_forces_limit_set_on_the_order_is_named() {
    let relative = "54_a_force_that_stands_by_night.toml";
    let played = play(relative, &text_of(relative), 1);
    let reason = played.the_reason(1, 1, "is named in the forces limit of intent:1");
    assert_eq!(reason.limits, vec![on(LimitKind::Forces, 1)]);
    assert!(played.intent(1).limits.forces.is_some());
    // What would change it names the same limit.
    assert!(reason
        .would_change
        .contains(&"task_force:1 not named in the forces limit".to_string()));
}

// ------------------------------------------------------------- Statement 18

/// Statement 18, the area: scenario 40a, a supply intent whose reason
/// gives the area as part of what binds it.
#[test]
fn statement_18_an_area() {
    let relative = "40a_one_pool_two_hexes.toml";
    let played = play(relative, &text_of(relative), 1);
    let reason = played.the_reason(1, 1, "area: ");
    assert_eq!(reason.limits, vec![on(LimitKind::Area, 1)]);
}

/// Statement 18, the source of a forces limit: scenario 69 with the hold's
/// forces limit naming no source, and an Allied base 3 at C holding 500 t
/// (scenario 69's own variant of decision 0222, rule 1): the reason says no
/// source is named and that one may be named among the bases that could be
/// one, and speaks of the source limit on the hold. (Where no base could be
/// the source, rule 1's reason offers nothing and speaks of no limit.)
#[test]
fn statement_18_a_source() {
    let relative = "69_taken_and_held.toml";
    let base_3 = "[[base]]\nid = 3\nname = \"Placeholder base C\"\nside = \"allied\"\nq = 0\nr = 6\nsupply_t = 500\nfuel_t = 0\n\n# Japanese base 2 at B";
    let text = changed(
        &changed(
            &text_of(relative),
            "intent = 2\ntask_forces = [1]\nsource = 1",
            "intent = 2\ntask_forces = [1]",
        ),
        "# Japanese base 2 at B",
        base_3,
    );
    let played = play(relative, &text, 8);
    let found: Vec<(u32, &Reason)> = played
        .handed
        .iter()
        .flat_map(|(day, d)| reasons_of(d).into_iter().map(move |r| (*day, r)))
        .filter(|(_, r)| {
            r.serves.intent == Some(IntentId::new(2))
                && r.binding
                    .contains(&"no source in the forces limit".to_string())
        })
        .collect();
    assert!(!found.is_empty(), "{}", played.summary(8));
    for (day, reason) in found {
        assert_eq!(
            reason.limits,
            vec![on(LimitKind::Source, 2)],
            "day {day}: {reason:?}"
        );
        assert_eq!(
            reason.would_change,
            vec!["a source named among base:3 (Placeholder base C)"]
        );
    }
}

/// Statement 18, a rank: scenario 116 with the deny (intent 1) and a
/// neutralise (intent 2) ranked above it. The one carrier force goes to
/// the neutralise, and the deny's reason says what would change it: the
/// deny higher in priority than the neutralise. The limit is the rank,
/// on the order that would have to change, the deny.
#[test]
fn statement_18_a_rank() {
    let t = at_mark(
        &text_of(S116),
        "bases",
        "[[base]]\nid = 2\nname = \"Placeholder base E\"\nside = \"japanese\"\nq = 0\nr = 0\nsupply_t = 0\n\n",
    );
    let t = at_mark(
        &t,
        "commands",
        &command(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"neutralise\"\ntarget = 2",
        ),
    );
    let t = at_mark(
        &t,
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"priority\"\nintent = 2\nrank = 1",
        ),
    );
    let played = play(S116, &t, 1);
    let reason = played.the_reason(1, 1, "higher in priority than intent:2");
    assert_eq!(reason.limits, vec![on(LimitKind::Priority, 1)]);
    assert_eq!(
        reason.binding,
        vec![
            "free carrier forces are handed out in one pass, in priority order and then by number"
        ]
    );
}

/// Statement 18, a risk: scenario 53 gives the press risk on the deny
/// (intent 1); the reasons of the tasks the staff forms under it say "the
/// press risk" in what binds them, and the limit is the risk set on the
/// deny above them.
#[test]
fn statement_18_a_risk() {
    let relative = "53_deny_an_area.toml";
    let played = play(relative, &text_of(relative), 1);
    let reason = played.the_reason(1, 2, "at the press risk");
    assert_eq!(reason.limits, vec![on(LimitKind::Risk, 1)]);
    assert_eq!(
        played.intent(1).limits.risk,
        Some(sim_core::command::Risk::Press)
    );
    assert_eq!(played.intent(2).limits.risk, None);
    // The risk by default: scenario 116 sets none, and the limit is the
    // order the commander would set it on, the root.
    let played = play(S116, &text_of(S116), 1);
    let reason = played.the_reason(1, 1, "another risk level than the balanced risk");
    assert_eq!(reason.limits, vec![on(LimitKind::Risk, 1)]);
}

/// Statement 18, the targets: scenario 116 with a targets limit on the
/// deny. The bombers of the carrier force are given the limit's list, and
/// the reason says the targets limit it comes from, set on intent 1.
#[test]
fn statement_18_the_targets() {
    let t = at_mark(
        &text_of(S116),
        "commands",
        &command(
            1,
            "kind = \"set_limit\"\nlimit = \"targets\"\nintent = 1\nkinds = [\"transport\", \"carrier\", \"battleship\", \"cruiser\", \"destroyer\", \"auxiliary\"]\nwaters_first = false",
        ),
    );
    let played = play(S116, &t, 2);
    let reason = played.the_reason(1, 2, "the targets limit set on intent:1");
    assert!(
        reason.limits.contains(&on(LimitKind::Targets, 1)),
        "{:?}",
        reason.limits
    );
    assert_eq!(
        reason.limits,
        vec![on(LimitKind::Risk, 1), on(LimitKind::Targets, 1)]
    );
}

/// Scenario 81, the take that is told to wait: `variant` gives the game
/// its limit in place of "not before day 6".
fn told_to_wait(variant: &str) -> Played {
    let relative = "81_told_to_wait.toml";
    let not_before = "# The limit that tells the plan to wait (decision 0159, 9).\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"not_before\"\nintent = 1\nfrom_day = 6\n";
    let base = text_of(relative);
    let t = match variant {
        "not_before" => base,
        "wait_for" => {
            let force_2 = "\n# Task force 2: one placeholder cruiser, stopped in port at base 1, usable\n# from day 5.\n[[task_force]]\nid = 2\nname = \"Placeholder cruiser force 2\"\nside = \"allied\"\nat_base = 1\navailable_from = 5\nmission = \"surface_combat\"\n\n[[ship]]\nid = 3\nname = \"Placeholder cruiser 3\"\nclass = 1\nside = \"allied\"\nfuel_t = 2000\ntask_force = 2\n";
            let wait_for_2 = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"wait_for\"\nintent = 1\ntask_force = 2\n";
            let t = changed(&base, not_before, "");
            let t = changed(
                &t,
                "# The land units",
                &format!("{force_2}\n# The land units"),
            );
            format!("{t}\n{wait_for_2}")
        }
        "after" => {
            let place_c = "\n# Japanese base 4 at C: nothing at it.\n[[base]]\nid = 4\nname = \"Placeholder airfield C\"\nside = \"japanese\"\nq = 0\nr = 6\nsupply_t = 0\nfuel_t = 0\n\n# Task force 3: one placeholder troop transport, stopped in port at base 1.\n[[task_force]]\nid = 3\nname = \"Placeholder transport force 3\"\nside = \"allied\"\nat_base = 1\nmission = \"transport\"\n\n[[ship]]\nid = 4\nname = \"Placeholder troop transport 4\"\nclass = 10\nside = \"allied\"\nfuel_t = 1000\ntask_force = 3\n\n[[land_unit]]\nid = 3\nname = \"Placeholder Third\"\nside = \"allied\"\nmen = 600\nq = 0\nr = 0\n";
            let intent_3 = "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_intent\"\nintent = 3\nintent_kind = \"take_place\"\nhex = [0, 6]\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\ntask_forces = [3]\nland_units = [3]\nsource = 1\n\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_limit\"\nlimit = \"after\"\nintent = 1\nother_intent = 3\n";
            let t = changed(&base, not_before, "");
            let t = changed(
                &t,
                "# Task force 1:",
                &format!("{place_c}\n# Task force 1:"),
            );
            changed(
                &t,
                "[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n",
                &format!("{intent_3}\n[[command]]\nday = 1\nside = \"allied\"\nkind = \"set_leave\"\nleave = \"prepare\"\n"),
            )
        }
        other => panic!("no such variant {other}"),
    };
    play(relative, &t, 3)
}

/// Statement 18, a landing's "not before": the landing is ready on day 2
/// and the take is told not to go before day 6.
#[test]
fn statement_18_not_before() {
    let played = told_to_wait("not_before");
    let reason = played.the_reason(2, 1, "the commander's limit not before day 6");
    assert_eq!(reason.limits, vec![on(LimitKind::NotBefore, 1)]);
    assert_eq!(
        reason.would_change,
        vec!["the limit met, or the commander clearing it"]
    );
}

/// Statement 18, a landing's "wait for": the take waits for task force 2,
/// usable from day 5.
#[test]
fn statement_18_wait_for() {
    let played = told_to_wait("wait_for");
    let reason = played.the_reason(2, 1, "the commander's limit wait for task_force:2");
    assert_eq!(reason.limits, vec![on(LimitKind::WaitFor, 1)]);
}

/// Statement 18, a landing's "after": the take is to go after intent 3,
/// another take, is done.
#[test]
fn statement_18_after() {
    let played = told_to_wait("after");
    let reason = played.the_reason(2, 1, "the commander's limit after intent:3");
    assert_eq!(reason.limits, vec![on(LimitKind::After, 1)]);
}

/// Statement 18, a landing's "knowledge": scenario 82 has the commander
/// ask for a count of the enemy no more than a day old; the reason says
/// the knowledge limit set on intent 1.
#[test]
fn statement_18_knowledge() {
    let relative = "82_how_much_must_be_known.toml";
    let played = play(relative, &text_of(relative), 1);
    let reason = played.the_reason(1, 1, "the knowledge limit set on intent:1");
    assert_eq!(reason.limits, vec![on(LimitKind::Knowledge, 1)]);
}

// ------------------------------------------------------------- Statement 19

/// The words that name a limit, for the sweep below: any binding or
/// "would change" line without one of them speaks of none.
const LIMIT_WORDS: [&str; 14] = [
    "limit",
    "risk",
    "priority",
    "ranked",
    "area",
    "protected",
    "reserve",
    "source",
    "targets",
    "knowledge",
    "not before",
    "wait for",
    "after intent",
    "preserve",
];

fn names_a_limit(reason: &Reason) -> bool {
    reason
        .binding
        .iter()
        .chain(reason.would_change.iter())
        .any(|line| LIMIT_WORDS.iter().any(|w| line.contains(w)))
}

/// Statement 19: in scenario 116's first days no reason that says nothing
/// of a limit carries one, whichever reason it is; the ones sampled have
/// the words they had.
#[test]
fn statement_19_a_reason_with_no_limit_words_has_an_empty_list() {
    let played = play(S116, &text_of(S116), 4);
    let mut without = 0;
    let mut with = 0;
    for decisions in played.handed.values() {
        for reason in reasons_of(decisions) {
            if names_a_limit(reason) {
                with += 1;
            } else {
                without += 1;
                assert!(reason.limits.is_empty(), "{reason:?}");
            }
        }
    }
    // The game gives both kinds, so the test is not empty either way.
    assert!(without > 10, "{without}");
    assert!(with > 5, "{with}");

    // Two that were read before this rule, word for word.
    let hex = played.the_reason(1, 2, "the force is kept at the task's hex");
    assert!(hex.limits.is_empty());
    assert_eq!(hex.binding, vec!["the force is kept at the task's hex"]);
    assert_eq!(hex.would_change, vec!["the force in the hex"]);
    let leave = played
        .handed
        .get(&1)
        .into_iter()
        .flat_map(|d| reasons_of(d))
        .find(|r| r.would_change == vec!["leave to launch withheld".to_string()])
        .expect("the staff says the leave to launch is withheld");
    assert!(leave.limits.is_empty());
    assert!(leave.binding.is_empty());
}

// ------------------------------------------------------------- Statement 20

/// A 64-bit FNV-1a digest of a string.
fn fnv(text: &str) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in text.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// The state hash at the end of each of the first `days` days of a staffed
/// game, and a digest of what each side's staff handed in that day: its
/// orders, its commands and its reasons as `Debug` says them (the
/// reason's `Debug` leaves its list of limits out, on purpose, so that
/// this is what the staff handed in before the list was filled).
fn digests(relative: &str, days: u32) -> Vec<(u64, u64)> {
    let mut scenario =
        common::from_text(&text_of(relative), &scenario_path(relative)).expect("it loads");
    let held = staffs();
    let mut out = Vec::new();
    for _ in 0..days {
        scenario
            .next_day_watched(as_staffs(&held), &mut |_, _| {})
            .expect("a day resolves");
        let mut said = String::new();
        for side in [Side::Allied, Side::Japanese] {
            if let Some(d) = scenario.world.staff_handed_in().get(&side) {
                said.push_str(&format!("{side:?}: {d:?}\n"));
            }
        }
        out.push((scenario.world.state_hash(), fnv(&said)));
    }
    out
}

/// Statement 20: recording the list changes no order, no command and no
/// hash. The list is recorded in every game now, so the test cannot play
/// the same game with it left out; it holds the game against what it gave
/// before the list was filled. The pinned numbers were taken by playing
/// these games on the code as it stood before rule 5 filled any list (the
/// type was there, every list empty): per day, the state hash and a
/// digest of the orders, commands and reasons of both staffs, said as
/// `Debug` says them. Here they are the same, so no order, no command and
/// no word of a reason moved, and the state hash, which does not read a
/// reason, is the one it was. What the test cannot show is that the list
/// itself is read by nothing in the day: that is a property of the
/// engine, whose hash and day read no reason, shown by every other game
/// on file giving the hash it gave. The state hashes were remade for the
/// state's layout v58 (decision 0223, stage 0), v59 (decision 0228) and v60 (decision
/// 0220); every digest is as it was.
#[test]
fn statement_20_changes_no_order_no_command_and_no_hash() {
    for (relative, days, expected) in PINNED {
        let got = digests(relative, *days);
        assert_eq!(got.len(), expected.len());
        for (i, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert_eq!(
                (format!("{:016x}", g.0), format!("{:016x}", g.1)),
                (format!("{:016x}", e.0), format!("{:016x}", e.1)),
                "{relative} day {}: the hash and the digest of what was handed in",
                i + 1
            );
        }
    }
}

/// See [`statement_20_changes_no_order_no_command_and_no_hash`].
/// A game, its days, and the pinned (state hash, digest) of each day.
type Pinned = (&'static str, u32, &'static [(u64, u64)]);

const PINNED: &[Pinned] = &[
    (
        S116,
        4,
        &[
            (0x3e9471db3f271f83, 0x661cd42fbafd2b5e),
            (0x95b0d40d764b58df, 0x71194a63e9b46cf1),
            (0x52d252dc0c6e9693, 0x342f60124116d86f),
            (0x638dde120404fd8f, 0x656027b30b72d70a),
        ],
    ),
    (
        "54_a_force_that_stands_by_night.toml",
        3,
        &[
            (0x99f4a6126b9aa39a, 0xa80a9a4a55c850f0),
            (0x812c9363765a1828, 0x09a83d2eebb5527b),
            (0x2574162eedbeeefc, 0x4a1c194224410cb0),
        ],
    ),
    (
        "86_fed_with_the_lift.toml",
        4,
        &[
            (0x1d5c05f6d95ef607, 0xb237741f6139a881),
            (0x14363dbdaf1dc5af, 0x1d6e037b3595d27a),
            (0x0687aed894769677, 0xacf61aef0523d659),
            (0x56e922508a201a33, 0x613984aef5fc075d),
        ],
    ),
    (
        "81_told_to_wait.toml",
        4,
        &[
            (0x5063fcc764be45ad, 0x49959b8201418bcb),
            (0x444859a717810b69, 0x9b2677b0215848cd),
            (0x9a946ae01eac6ba3, 0x1f27cd7ff58a73e1),
            (0x0811e106aea4acc9, 0x58e85cf9e3cd634f),
        ],
    ),
    (
        "82_how_much_must_be_known.toml",
        3,
        &[
            (0x3506da9650853e8a, 0xdc58225c411e3afe),
            (0xe8b00b7c1ac0fddd, 0x86e2df66d204821e),
            (0x97ae5647e98885a6, 0x87c6d5c7f0810bad),
        ],
    ),
];
