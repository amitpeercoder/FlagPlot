//! Scenario 106: what to strike first. A commander sets, on an intent, the
//! kinds of ship to strike in his order; the staff gives that order to the
//! groups it commands, and a carrier force reads it when it chooses what to
//! close on.
//!
//! Proves decision 0190. The statements are the architect's, written
//! before the code. If the code disagrees, the code or the rule is wrong:
//! a mismatch goes back to the architect, and nobody else changes a
//! statement or a number here.
//!
//! # The setting
//!
//! A placeholder scenario made for the test, on the placeholder map, with
//! no real ship or aircraft. Side A has a carrier force, F, with one
//! bomber group, B, of strike radius 200 nm, and one fighter group. Side
//! J has two forces: C, with a carrier, and T, with three transports and
//! a destroyer. A holds contacts of both, each fresh, reported as a
//! carrier and as a transport. F is given "operate a carrier force" at
//! its own hex, at balanced, with both leaves.
//!
//! Distances are by air from F's hex.
//!
//! # Statements
//!
//! 1. **With no limit, as today.** C at 150 nm and T at 100 nm, both in
//!    reach. B's target list is the six kinds in the order of importance.
//!    B launches at C: the kind first on its list, though T is nearer.
//! 2. **With the limit, the transports.** The same, and the commander
//!    sets on F's intent the targets limit transport, carrier,
//!    battleship, cruiser, destroyer, auxiliary. The next morning the
//!    staff orders B's target list to those six in that order, and the
//!    order's reason names the limit and the intent. B launches at T. In
//!    T's force its aircraft attack a transport and not the destroyer.
//! 3. **It does not hold the group.** After statement 2 the side's
//!    command state holds no hold on B, and on later mornings the staff
//!    still orders B (its mission, its rest when tired).
//! 4. **A kind left out is not launched at.** The limit is transport
//!    alone. T is not held as a contact; C is, in reach. B does not
//!    launch, and the strike's own record of that pulse says no contact
//!    of a kind on the group's list is held.
//! 5. **Read up the tree, the nearest.** The limit is set on the
//!    commander's intent and the strike is flown under a task the staff
//!    formed beneath it: A has a base with a bomber group D, and a deny
//!    of the waters within 1 hex of T's hex, under which the staff forms
//!    a task to strike ships from that base. D has the limit's list. A second
//!    limit set on the staff's own task, carrier first, replaces it for
//!    that task alone.
//! 6. **Cleared.** The limit set with no kinds is cleared: the next
//!    morning the staff orders B's list back to the six kinds in the
//!    order of importance.
//! 7. **The waters come first, where the commander says so.** A deny of
//!    the waters within 1 hex of T's hex, all seven of them on the map,
//!    names F in its forces limit and carries a targets limit of the six
//!    kinds in the order of importance with `waters_first` true. T,
//!    reported as a cruiser, is in those waters at 150 nm; C, reported as
//!    a carrier, is outside them at 100 nm. The staff orders B's waters
//!    to strike first to the deny's seven hexes, and the order serves
//!    the deny. B launches at T, though C is the more important kind and
//!    the nearer. With the deny withdrawn the staff orders B's waters
//!    empty the next morning, and B launches at C.
//! 7a. **Not without the switch.** The same deny naming F with no
//!    targets limit, or with one whose `waters_first` is false: the
//!    staff gives no order about waters, B's are empty, and B launches at
//!    C. `waters_first` true on an intent that is not a deny or beneath
//!    one is refused, with the reason.
//! 7b. **The carrier's search covers the waters.** In statement 7 the
//!    waters' hexes are among the threats of F's search arc; in
//!    statement 7a they are not.
//! 8. **Both parts together.** As statement 7, with the limit's kinds
//!    those of statement 2: B's list is transport first though the limit
//!    is on the deny and F serves a task of its own; B launches at T, and
//!    its aircraft attack a transport of T's force and not the
//!    destroyer.
//! 9. **Closing on what is in the waters.** As statement 7 with T at 300
//!    nm, beyond reach, and no carrier contact held. At balanced F closes
//!    on T, to the first hex on its way within 200 nm of T's hex, and the
//!    reason says the contact is in the waters of the deny. With the
//!    deny withdrawn F does not close: nothing of a kind it closes on is
//!    held. At preserve the force F closes on nothing, with the deny or
//!    without.
//! 9a. **A contact outside the waters is ranked as today.** As statement
//!    7 with T moved one hex outside the waters: B launches at C. And a
//!    contact in the waters beyond B's reach does not keep B from C in
//!    reach: B launches at C.
//! 9d. **Closing leaves a carrier in reach for a contact in the
//!    waters.** As statement 7 with T in the waters at 300 nm and C
//!    outside them at 100 nm, in reach. At balanced F closes on T.
//! 9b. **Preserve the force keeps the limit's list.** With the limit of
//!    statement 2 at preserve the force, B's list is the limit's six
//!    kinds and not the risk's three.
//! 9c. **A group launches once in a pulse.** With B of 60 aircraft, T
//!    reported as a cruiser of 4 ships in the waters and C outside: the
//!    aircraft the sizing figures give for T go at T and the rest do not
//!    fly that pulse; none goes at C. A second bomber group aboard F
//!    launches at C. The test takes the number sent at T from the sizing
//!    figures in the reference file and states it.
//! 10. **Refused:** a limit that names no intent of the side's in force;
//!     a kind named twice. Each with the reason, as the other limits are
//!     refused. A limit on a staff-made task with other kinds than its
//!     parent's is taken, not refused as wider.
//! 11. **What a seat reads.** `command_state` shows the limit on the
//!     intent, the kinds in order. The other side is shown nothing of it.
//! 12. **No game without the limit changes.** Scenario 5, the five-day
//!     scenario's file a and the 39 days' first week, each played by its
//!     staffs: every order a staff hands in and every event of the game
//!     is what it was before this rule. (The daily hashes all move, with
//!     the new fields of the state; the test compares orders and events
//!     against a record made before the change.)
//!
//! # Properties
//!
//! - With no targets limit anywhere, every order a staff hands in is the
//!   order it handed in before this rule, on the scenarios above.
//! - A group's list set from a limit holds exactly the limit's kinds, in
//!   its order, each once.
//! - A limit of all six kinds in the default order gives the orders of
//!   no limit, at balanced and at press.
//! - Moving a kind earlier on the limit never lowers the aircraft sent at
//!   contacts of that kind.
//! - A contact moved into the waters never ranks lower than it did.
//!
//! The tests are written by the build under these statements.

// The architect's statement numbers (7a, 9a to 9d) are list items in the doc
// comment above, which is left as written.
#![allow(clippy::doc_lazy_continuation)]

use crate::common;

// ------------------------------------------------- Statement 12: the record

mod record {
    use std::fmt::Write as _;
    use std::path::Path;

    use sim_core::state::Side;
    use sim_staff::{as_staffs, staffs};

    use super::common;

    /// FNV-1a, 64 bits: a digest of a text that does not change with the
    /// compiler, as the standard library's hasher may.
    pub fn fnv(text: &str) -> u64 {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in text.bytes() {
            h ^= u64::from(byte);
            h = h.wrapping_mul(0x0100_0000_01b3);
        }
        h
    }

    /// A scenario file under `data/scenarios`, played by its staffs on the
    /// live figures for `days` days (all of them where `None`): every
    /// decision each side's staff handed in, day by day, then every event of
    /// the game, as text. The daily hashes are not in it.
    pub fn played(file: &str, days: Option<u32>) -> String {
        let path = common::data_dir().join("scenarios").join(file);
        // Live figures: the files are live scenarios, and nothing here
        // asserts a number of a figure.
        let mut scenario = common::load_live(Path::new(&path)).expect("the scenario loads");
        let held = staffs();
        let mut out = String::new();
        loop {
            let day = scenario.world.day() + 1;
            if days.is_some_and(|last| day > last) {
                break;
            }
            let summary = scenario
                .next_day_staffed(as_staffs(&held))
                .expect("a day resolves");
            if summary.is_none() {
                break;
            }
            for side in Side::ALL {
                let handed = scenario
                    .world
                    .staff_handed_in()
                    .get(&side)
                    .cloned()
                    .unwrap_or_default();
                writeln!(out, "day {day} {side:?} {handed:#?}").expect("a string writes");
            }
        }
        for event in scenario.world.events().iter() {
            writeln!(out, "{event:?}").expect("a string writes");
        }
        out
    }
}

/// Statement 12, and the first property: no game without the limit changes.
/// Scenario 5, the five-day scenario's file a and the 39 days' first week
/// (stage 3's proof, arrangement a, both sides with intents), each played by
/// its staffs, and, beside them, the second proof's Eastern Solomons and
/// week (file a of each; the week is days 15 to 21 of its file), which give
/// carrier forces and denies: every decision a staff hands in and every
/// event of the game, as text, has the digest it had on the branch this
/// rule was built from (`station-from-the-sea` at 4aebfa0e, before any of
/// decision 0190). The daily hashes are left out: they all move with the
/// new fields of the state. A later change that moves one of these games
/// remakes its digest, with `FLAGPILOT_106_RECORD` set to a folder to write
/// the texts into for a comparison. Remade for decision 0210 (the types of
/// the aircraft claimed shot down): every claim's record gains its list
/// `air_types`, and a claim whose men made the types out its breakdown's
/// factors for them; with those taken out each text is the text of the
/// commit before, line for line. Remade for decision 0214 (a force asks
/// leave to pursue): every reason gains `answer_of` and every staff notice
/// `pursuit`, each none in these games; with those taken out each text is
/// the text of the commit before, line for line. Remade for decisions
/// 0211 and 0212 (a strike spread over the ships of its kind and fire among
/// a carrier's aircraft, which run in every battle): the four games with
/// carrier strikes move in their play; the 39 days' first week, which has
/// none, keeps its digest. Remade for decision 0217 (a deny a carrier task
/// serves is met; a reason's cautions): every reason gains `cautions`, empty
/// in these games; with those taken out the five-day and the second proof's
/// Eastern Solomons are the text of the commit before, line for line, and
/// scenario 5 did not move. In the 39 days' first week the island's deny,
/// which its carrier task serves, no longer hands in the notice that it
/// cannot be met nor fires the standing interrupt, and its reason for the
/// bases names the carrier task first; in the second proof's week a deny the
/// carrier forces serve gains that reason each day. No order or command
/// moved in either, and no other event. Remade for decision 0218, stage 2
/// (submarines against warships): every claim's record gains
/// `from_submarine`, false in these games; with it taken out each text is
/// the text of the commit before (origin/main at c1cf209a), line for line.
/// Remade for decision 0222, rule 1 (the reason of a hold with no source):
/// in the five-day file the Allied hold of Lunga's reason on each of its
/// five days gains the evidence that Tulagi could be the source and its new
/// words; nothing else moved in it, and the other four games did not move
/// (each text against 37fc8eea, line for line).
#[test]
fn s12_no_game_without_the_limit_changes() {
    let games: [(&str, Option<u32>, u64); 5] = [
        ("05_eastern_solomons.toml", None, 0xf71a_aaa2_934a_ef74),
        (
            "five-days/01a_eastern_solomons.toml",
            None,
            0x5789_7532_39be_59bf,
        ),
        (
            "stage-3-proof/07a_slice.toml",
            Some(7),
            0xba4e_2324_4d22_f95f,
        ),
        (
            "second-proof/05a_eastern_solomons.toml",
            None,
            0x72dc_4916_07fd_653b,
        ),
        ("second-proof/07a_week.toml", None, 0xca38_0b3b_7a22_3459),
    ];
    let write_to = std::env::var("FLAGPILOT_106_RECORD").ok();
    let mut moved = Vec::new();
    for (file, days, digest) in games {
        let text = record::played(file, days);
        let found = record::fnv(&text);
        if let Some(dir) = &write_to {
            let name = file.replace('/', "__");
            std::fs::write(std::path::Path::new(dir).join(format!("{name}.txt")), &text)
                .expect("the record writes");
            println!("{file} {found:#018x}");
        }
        if found != digest {
            moved.push(format!("{file}: {found:#018x}, recorded {digest:#018x}"));
        }
    }
    assert!(moved.is_empty(), "games moved: {moved:#?}");
}

// ---------------------------------------------------------------- The harness

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use sim_core::choices::{Moment, PulsePhase};
use sim_core::command::{CommandRefusal, Reason, StaffDecisions, StaffOrder};
use sim_core::events::{Event, EventKind};
use sim_core::hex::Hex;
use sim_core::ids::{AirGroupId, IntentId};
use sim_core::orders::{AirGroupOrder, AirMission, OrderKind, TaskForceOrder};
use sim_core::refdata::ShipKind;
use sim_core::state::Side;
use sim_core::time::Pulse;
use sim_core::view::strike_review::{ContactFate, StrikeReview};
use sim_data::{LoadOptions, Scenario};
use sim_staff::{as_staffs, staffs};

fn path() -> PathBuf {
    common::data_dir().join("scenarios/what-to-strike-first/106_what_to_strike_first.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

fn hex(q: i32, r: i32) -> Hex {
    Hex { q, r }
}

fn group(raw: u32) -> AirGroupId {
    AirGroupId::new(raw)
}

fn intent(raw: u32) -> IntentId {
    IntentId::new(raw)
}

/// Adds `what` at the mark `# MARK: name` of the scenario's text; the mark
/// stays, so that more can be added.
fn at(text: &str, name: &str, what: &str) -> String {
    let mark = format!("# MARK: {name}\n");
    common::replace_once(text, &mark, &format!("{what}{mark}"))
}

fn command(body: &str) -> String {
    format!("[[command]]\nday = 1\nside = \"allied\"\n{body}\n\n")
}

/// The figures: the frozen step 1 figures with the strike radius of the
/// dive bomber (type 3) made 200 nm and the strike contact age 4 pulses, so
/// that a contact held from the start is fresh enough in the morning; with
/// `per_ship`, scenario 53's table of aircraft for each ship added too.
fn figures(per_ship: bool) -> LoadOptions {
    figures_with(per_ship, 36)
}

/// [`figures`], with the placeholder carrier (class 4) carrying `deck`
/// aircraft instead of 36 (statement 9c puts 84 aboard).
fn figures_with(per_ship: bool, deck: u32) -> LoadOptions {
    common::altered_reference("s106", |dir| {
        common::edit_file(dir, "ship_classes.csv", |text| {
            common::replace_once(
                &text,
                "4,Placeholder carrier,carrier,16,30,3000,9000,0,0,0,0,0,0,0,0,600,0,36,0",
                &format!(
                    "4,Placeholder carrier,carrier,16,30,3000,9000,0,0,0,0,0,0,0,0,600,0,{deck},0"
                ),
            )
        });
        common::edit_file(dir, "aircraft_types.csv", |text| {
            common::replace_once(
                &text,
                "3,Placeholder dive bomber,bomber,0,0,250,bomb,60,0,0",
                "3,Placeholder dive bomber,bomber,0,0,200,bomb,60,0,0",
            )
        });
        common::edit_file(dir, "air_combat.toml", |text| {
            let text = common::replace_once(
                &text,
                "strike_contact_max_age_pulses = 0",
                "strike_contact_max_age_pulses = 4",
            );
            if per_ship {
                format!("{text}\ncruiser = 12\ndestroyer = 6\ntransport = 6\nauxiliary = 4\n")
            } else {
                text
            }
        });
    })
}

/// What one contact of a scene is held as.
#[derive(Clone, Copy)]
struct Held {
    hex: (i32, i32),
    kind: &'static str,
    ships: u32,
}

/// A scene: where C and T lie and what the Allied side holds of each (`None`:
/// nothing). The units are moved to the hex; the contact is where they are.
#[derive(Clone, Copy)]
struct Scene {
    c: (i32, i32),
    c_held: Option<Held>,
    t: (i32, i32),
    t_held: Option<Held>,
}

impl Scene {
    /// Statement 1: C, a carrier, 160 nm from F at (0,4), and T, three
    /// transports, 120 nm from F at (3,0). Both held, fresh.
    fn statement_1() -> Scene {
        Scene {
            c: (0, 4),
            c_held: Some(Held {
                hex: (0, 4),
                kind: "carrier",
                ships: 1,
            }),
            t: (3, 0),
            t_held: Some(Held {
                hex: (3, 0),
                kind: "transport",
                ships: 3,
            }),
        }
    }

    /// Statement 7: T, reported a cruiser of 4 ships, at (4,0), 160 nm from F;
    /// C, a carrier, at (0,3), 120 nm from F.
    fn statement_7() -> Scene {
        Scene {
            c: (0, 3),
            c_held: Some(Held {
                hex: (0, 3),
                kind: "carrier",
                ships: 1,
            }),
            t: (4, 0),
            t_held: Some(Held {
                hex: (4, 0),
                kind: "cruiser",
                ships: 4,
            }),
        }
    }

    /// T and its contact moved to `(q, r)`, reported as it was.
    fn t_at(mut self, q: i32, r: i32) -> Scene {
        self.t = (q, r);
        if let Some(held) = &mut self.t_held {
            held.hex = (q, r);
        }
        self
    }

    fn c_at(mut self, q: i32, r: i32) -> Scene {
        self.c = (q, r);
        if let Some(held) = &mut self.c_held {
            held.hex = (q, r);
        }
        self
    }

    fn t_kind(mut self, kind: &'static str, ships: u32) -> Scene {
        if let Some(held) = &mut self.t_held {
            held.kind = kind;
            held.ships = ships;
        }
        self
    }

    fn c_kind(mut self, kind: &'static str) -> Scene {
        if let Some(held) = &mut self.c_held {
            held.kind = kind;
        }
        self
    }

    fn no_c_held(mut self) -> Scene {
        self.c_held = None;
        self
    }

    fn no_t_held(mut self) -> Scene {
        self.t_held = None;
        self
    }

    /// The scenario's text with this scene.
    fn text(&self) -> String {
        let mut t = common::replace_once(
            &text(),
            "q = 0\nr = 4\nmission = \"carrier\"",
            &format!("q = {}\nr = {}\nmission = \"carrier\"", self.c.0, self.c.1),
        );
        t = common::replace_once(
            &t,
            "q = 3\nr = 0\nmission = \"transport\"",
            &format!(
                "q = {}\nr = {}\nmission = \"transport\"",
                self.t.0, self.t.1
            ),
        );
        for held in [self.c_held, self.t_held].into_iter().flatten() {
            t = at(
                &t,
                "contacts",
                &format!(
                    "[[contact]]\nside = \"allied\"\nq = {}\nr = {}\nship_count = {}\nkind = \"{}\"\nage_pulses = 0\n\n",
                    held.hex.0, held.hex.1, held.ships, held.kind
                ),
            );
        }
        t
    }
}

/// The six kinds in the order of importance, as the default list has them.
fn default_kinds() -> Vec<&'static str> {
    vec![
        "carrier",
        "battleship",
        "cruiser",
        "destroyer",
        "transport",
        "auxiliary",
    ]
}

/// The kinds of statement 2: the transports first.
fn transport_first() -> Vec<&'static str> {
    vec![
        "transport",
        "carrier",
        "battleship",
        "cruiser",
        "destroyer",
        "auxiliary",
    ]
}

fn kinds_toml(kinds: &[&str]) -> String {
    let quoted: Vec<String> = kinds.iter().map(|k| format!("\"{k}\"")).collect();
    format!("[{}]", quoted.join(", "))
}

/// A targets limit of the Allied commander on `intent_id`, from day 1.
fn targets_limit(intent_id: u32, kinds: &[&str], waters_first: bool) -> String {
    command(&format!(
        "kind = \"set_limit\"\nlimit = \"targets\"\nintent = {intent_id}\nkinds = {}\nwaters_first = {waters_first}",
        kinds_toml(kinds)
    ))
}

/// The commander's "deny the waters within `distance` hexes of `(q, r)`" as
/// intent 2, naming F (task force 1) in its forces limit.
fn deny_naming_f(q: i32, r: i32, distance: u32) -> String {
    let mut out = command(&format!(
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"deny\"\nhex = [{q}, {r}]\ndistance = {distance}"
    ));
    out.push_str(&command(
        "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\ntask_forces = [1]",
    ));
    out
}

fn with_limit(scene: &Scene, kinds: &[&str]) -> String {
    at(&scene.text(), "commands", &targets_limit(1, kinds, false))
}

/// The seven hexes of the waters within 1 hex of (4,0).
fn waters_of_t() -> BTreeSet<Hex> {
    [
        hex(4, 0),
        hex(5, 0),
        hex(3, 0),
        hex(4, 1),
        hex(4, -1),
        hex(5, -1),
        hex(3, 1),
    ]
    .into_iter()
    .collect()
}

/// An order of the staff.
#[derive(Clone, PartialEq, Eq, Debug)]
enum Given {
    Force(u32, TaskForceOrder),
    Air(u32, AirGroupOrder),
}

fn given(o: &StaffOrder) -> Given {
    match &o.kind {
        OrderKind::TaskForce { target, order } => Given::Force(target.raw(), order.clone()),
        OrderKind::AirGroup { target, order } => Given::Air(target.raw(), order.clone()),
        other => panic!("the doctrine gave an order of another kind: {other:?}"),
    }
}

/// A game played to its end with the doctrine's staffs: what the Allied staff
/// handed in each day, the log, the Allied side's strike reviews at each
/// strike decision, and the Allied command state at the end.
struct Played {
    handed: BTreeMap<u32, StaffDecisions>,
    events: Vec<Event>,
    reviews: BTreeMap<(u32, Pulse, u32), StrikeReview>,
    state: sim_core::command::CommandState,
}

fn play(text: &str, figures: &LoadOptions) -> Played {
    let mut scenario: Scenario =
        common::from_text_with(text, &path(), figures).expect("the scenario loads");
    let held = staffs();
    let mut handed = BTreeMap::new();
    let mut reviews = BTreeMap::new();
    loop {
        let day = scenario.world.day() + 1;
        let summary = scenario
            .next_day_watched(as_staffs(&held), &mut |world, moment| {
                let Moment::Phase {
                    day,
                    pulse,
                    phase: PulsePhase::StrikeDecisions,
                } = moment
                else {
                    return;
                };
                let view = world.side_view(Side::Allied);
                let groups: Vec<AirGroupId> = view
                    .air_groups()
                    .filter(|g| g.mission == AirMission::NavalStrike)
                    .map(|g| g.id)
                    .collect();
                for g in groups {
                    if let Some(review) = view.strike_review(g, pulse) {
                        reviews.insert((day, pulse, g.raw()), review);
                    }
                }
            })
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
    Played {
        handed,
        events: scenario.world.events().iter().cloned().collect(),
        reviews,
        state: scenario.world.command_state(Side::Allied).clone(),
    }
}

/// A reason's text: its calculation, its evidence and its bindings.
fn words(reason: &Reason) -> String {
    let evidence: Vec<&str> = reason.evidence.iter().map(|e| e.what.as_str()).collect();
    format!(
        "{} | {} | {}",
        reason.worked,
        evidence.join(" | "),
        reason.binding.join(" | ")
    )
}

impl Played {
    fn day(&self, day: u32) -> &StaffDecisions {
        &self.handed[&day]
    }

    fn orders(&self, day: u32) -> Vec<(Given, &StaffOrder)> {
        self.day(day).orders.iter().map(|o| (given(o), o)).collect()
    }

    /// The orders of a day to air group `g` setting its target list.
    fn list_orders(&self, day: u32, g: u32) -> Vec<(&StaffOrder, Vec<ShipKind>)> {
        self.orders(day)
            .into_iter()
            .filter_map(|(o, raw)| match o {
                Given::Air(t, AirGroupOrder::SetTargetPriority { kinds }) if t == g => {
                    Some((raw, kinds))
                }
                _ => None,
            })
            .collect()
    }

    /// The orders of a day to air group `g` setting its waters.
    fn waters_orders(&self, day: u32, g: u32) -> Vec<(&StaffOrder, BTreeSet<Hex>)> {
        self.orders(day)
            .into_iter()
            .filter_map(|(o, raw)| match o {
                Given::Air(t, AirGroupOrder::SetStrikeFirstWaters { hexes }) if t == g => {
                    Some((raw, hexes.into_iter().collect()))
                }
                _ => None,
            })
            .collect()
    }

    /// Every order of the day to air group `g`.
    fn orders_to_group(&self, day: u32, g: u32) -> Vec<AirGroupOrder> {
        self.orders(day)
            .into_iter()
            .filter_map(|(o, _)| match o {
                Given::Air(t, order) if t == g => Some(order),
                _ => None,
            })
            .collect()
    }

    fn routes_of_force(&self, day: u32, f: u32) -> Vec<Vec<Hex>> {
        self.orders(day)
            .into_iter()
            .filter_map(|(o, _)| match o {
                Given::Force(t, TaskForceOrder::SetRoute { waypoints, .. }) if t == f => {
                    Some(waypoints)
                }
                _ => None,
            })
            .collect()
    }

    /// Every reason of a day, the orders' and the decisions to give none.
    fn said(&self, day: u32) -> String {
        let d = self.day(day);
        d.orders
            .iter()
            .map(|o| &o.reason)
            .chain(d.no_orders.iter())
            .map(words)
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// The `StrikeLaunched` events of a day: (group, hex flown to, aircraft).
    fn launches(&self, day: u32) -> Vec<(u32, Hex, u32)> {
        self.events
            .iter()
            .filter(|e| e.day == day)
            .filter_map(|e| match &e.kind {
                EventKind::StrikeLaunched {
                    air_group,
                    hex,
                    aircraft,
                    ..
                } => Some((air_group.raw(), *hex, *aircraft)),
                _ => None,
            })
            .collect()
    }

    fn refusals(&self) -> Vec<CommandRefusal> {
        self.events
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused { reason, .. } => Some(*reason),
                _ => None,
            })
            .collect()
    }

    fn nothing_refused(&self) {
        let refused: Vec<&Event> = self
            .events
            .iter()
            .filter(|e| {
                matches!(
                    e.kind,
                    EventKind::OrderRejected { .. } | EventKind::CommandRefused { .. }
                )
            })
            .collect();
        assert!(refused.is_empty(), "something was refused: {refused:?}");
    }
}

// ------------------------------------------------------- More of the harness

/// A command of the Allied commander on `day`.
fn command_on(day: u32, body: &str) -> String {
    format!("[[command]]\nday = {day}\nside = \"allied\"\n{body}\n\n")
}

fn risk_of_f(level: &str) -> String {
    command(&format!(
        "kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{level}\""
    ))
}

/// A scout group (3) aboard F, so that F's search has an arc to give.
fn scout_group() -> String {
    "[[air_group]]\nid = 3\nname = \"Placeholder carrier scouts 3\"\nside = \"allied\"\nship = 1\naircraft_type = 2\nready = 6\n\n".to_string()
}

/// A second bomber group (4) aboard F with `ready` dive bombers.
fn second_bomber_group(ready: u32) -> String {
    format!("[[air_group]]\nid = 4\nname = \"Placeholder dive bombers 4\"\nside = \"allied\"\nship = 1\naircraft_type = 3\nready = {ready}\n\n")
}

/// B's ready aircraft made `ready` instead of 12.
fn b_ready(text: &str, ready: u32) -> String {
    common::replace_once(
        text,
        "name = \"Placeholder dive bombers B\"\nside = \"allied\"\nship = 1\naircraft_type = 3\nready = 12",
        &format!("name = \"Placeholder dive bombers B\"\nside = \"allied\"\nship = 1\naircraft_type = 3\nready = {ready}"),
    )
}

fn days(text: &str, n: u32) -> String {
    common::replace_once(text, "days = 2\n", &format!("days = {n}\n"))
}

/// The deny of statement 7: the waters within 1 hex of `(q, r)`, naming F,
/// with a targets limit of `kinds` and `waters_first`.
fn deny_with_limit(q: i32, r: i32, kinds: &[&str], waters_first: bool) -> String {
    let mut out = deny_naming_f(q, r, 1);
    out.push_str(&targets_limit(2, kinds, waters_first));
    out
}

fn scene7_with_deny(kinds: &[&str]) -> String {
    at(
        &Scene::statement_7().text(),
        "commands",
        &deny_with_limit(4, 0, kinds, true),
    )
}

fn pulse_launches(p: &Played, day: u32, pulse: Pulse) -> Vec<(u32, Hex, u32)> {
    p.events
        .iter()
        .filter(|e| e.day == day && e.pulse == Some(pulse))
        .filter_map(|e| match &e.kind {
            EventKind::StrikeLaunched {
                air_group,
                hex,
                aircraft,
                ..
            } => Some((air_group.raw(), *hex, *aircraft)),
            _ => None,
        })
        .collect()
}

fn morning(p: &Played, day: u32) -> Vec<(u32, Hex, u32)> {
    pulse_launches(p, day, Pulse::Morning)
}

/// The ships a group's aircraft attacked in the morning of `day`.
fn attacked_in_morning(p: &Played, day: u32, g: u32) -> Vec<u32> {
    p.events
        .iter()
        .filter(|e| e.day == day && e.pulse == Some(Pulse::Morning))
        .filter_map(|e| match &e.kind {
            EventKind::AirAttack {
                air_group, ship, ..
            } if air_group.raw() == g => Some(ship.raw()),
            _ => None,
        })
        .collect()
}

fn review(p: &Played, day: u32, pulse: Pulse, g: u32) -> &StrikeReview {
    p.reviews
        .get(&(day, pulse, g))
        .expect("the group had a review at that strike decision")
}

fn kinds_of(names: &[&str]) -> Vec<ShipKind> {
    names
        .iter()
        .map(|n| match *n {
            "carrier" => ShipKind::Carrier,
            "battleship" => ShipKind::Battleship,
            "cruiser" => ShipKind::Cruiser,
            "destroyer" => ShipKind::Destroyer,
            "transport" => ShipKind::Transport,
            "auxiliary" => ShipKind::Auxiliary,
            other => panic!("no kind {other}"),
        })
        .collect()
}

// ---------------------------------------------------------------- Statements

/// The scenario's file is the setting the doc comment describes.
#[test]
fn the_file_is_the_setting_the_doc_comment_describes() {
    let scenario = common::from_text_with(&text(), &path(), &figures(false)).expect("loads");
    let truth = scenario.world.umpire();
    assert_eq!(truth.task_forces.len(), 3);
    let here = hex(0, 0);
    assert_eq!(
        truth.air_group_hex(&truth.air_groups[&group(1)]),
        Some(here)
    );
    // Statement 1's distances: C 160 nm from F, T 120 nm.
    assert_eq!(here.air_distance_nm(hex(0, 4)), 160);
    assert_eq!(here.air_distance_nm(hex(3, 0)), 120);
    // Statement 7's: T 160 nm, C 120 nm; the waters' seven hexes are on the
    // map and none is C's, F's or the base's.
    assert_eq!(here.air_distance_nm(hex(4, 0)), 160);
    assert_eq!(here.air_distance_nm(hex(0, 3)), 120);
    assert_eq!(waters_of_t().len(), 7);
    for h in waters_of_t() {
        assert!(h.distance(hex(4, 0)) <= 1);
    }
}

/// Statement 1. With no limit, as today. C at 160 nm and T at 120 nm (the
/// nearest the hexes give to the statement's 150 and 100: C farther, both in
/// reach of 200), both held, fresh. B's list is the six kinds in the order of
/// importance, and B launches at C though T is nearer.
#[test]
fn s01_with_no_limit_as_today() {
    let p = play(&Scene::statement_1().text(), &figures(false));
    p.nothing_refused();
    let r = review(&p, 1, Pulse::Morning, 1);
    assert_eq!(r.target_list, kinds_of(&default_kinds()));
    assert!(r.waters_first.is_empty());
    // The staff gives B no list: it has the default already.
    assert!(p.list_orders(1, 1).is_empty());
    assert!(p.waters_orders(1, 1).is_empty());
    assert_eq!(morning(&p, 1), vec![(1, hex(0, 4), 12)]);
}

/// Statement 2. The commander sets on F's intent the targets limit transport,
/// carrier, battleship, cruiser, destroyer, auxiliary. The staff orders B's
/// target list to those six in that order, the reason names the limit and the
/// intent; B launches at T; its aircraft attack a transport and not the
/// destroyer.
#[test]
fn s02_with_the_limit_the_transports() {
    let p = play(
        &with_limit(&Scene::statement_1(), &transport_first()),
        &figures(false),
    );
    p.nothing_refused();
    let lists = p.list_orders(1, 1);
    assert_eq!(lists.len(), 1);
    let (order, kinds) = &lists[0];
    assert_eq!(*kinds, kinds_of(&transport_first()));
    let said = words(&order.reason);
    assert!(
        said.contains("the targets limit set on intent:1"),
        "the reason names the limit and the intent: {said}"
    );
    assert!(said.contains("transport, carrier, battleship, cruiser, destroyer, auxiliary"));
    let r = review(&p, 1, Pulse::Morning, 1);
    assert_eq!(r.target_list, kinds_of(&transport_first()));
    assert_eq!(morning(&p, 1), vec![(1, hex(3, 0), 12)]);
    // T's ships are 20, 21, 22 (transports) and 23 (the destroyer).
    let hit = attacked_in_morning(&p, 1, 1);
    assert!(!hit.is_empty(), "B's aircraft attacked");
    assert!(hit.iter().all(|s| (20..=22).contains(s)), "{hit:?}");
}

/// Statement 3. It does not hold the group: after statement 2 the command
/// state holds no hold on B, and on later mornings the staff still orders B.
#[test]
fn s03_it_does_not_hold_the_group() {
    // B begins tired (fatigue 40, where the rest threshold at balanced is 45)
    // under the live fatigue figures, so that it needs a rest on a later day.
    let t = days(&with_limit(&Scene::statement_1(), &transport_first()), 4);
    let t = common::replace_once(
        &t,
        "air_combat = \"../../reference/air_combat.toml\"\n",
        "air_combat = \"../../reference/air_combat.toml\"\nfatigue = \"../../reference/fatigue.toml\"\n",
    );
    let t = common::replace_once(
        &t,
        "aircraft_type = 3\nready = 12",
        "aircraft_type = 3\nready = 12\nfatigue = 40",
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert!(p.state.hold_on_group(group(1)).is_none());
    assert!(p.state.holds.is_empty(), "{:?}", p.state.holds);
    // Later mornings: the staff goes on ordering B (its mission or its rest).
    let later: Vec<(u32, Vec<AirGroupOrder>)> = (2..=4)
        .map(|d| (d, p.orders_to_group(d, 1)))
        .filter(|(_, o)| !o.is_empty())
        .collect();
    assert!(
        !later.is_empty(),
        "the staff ordered B on a later morning: {:?}",
        (2..=4).map(|d| p.orders_to_group(d, 1)).collect::<Vec<_>>()
    );
}

/// Statement 4. A kind left out is not launched at. The limit is transport
/// alone, T not held, C held in reach: B does not launch, and the strike's own
/// record says no contact of a kind on the list is held.
#[test]
fn s04_a_kind_left_out_is_not_launched_at() {
    let scene = Scene::statement_1().no_t_held();
    let p = play(&with_limit(&scene, &["transport"]), &figures(false));
    p.nothing_refused();
    assert_eq!(p.list_orders(1, 1)[0].1, vec![ShipKind::Transport]);
    assert!(morning(&p, 1).is_empty(), "{:?}", morning(&p, 1));
    let r = review(&p, 1, Pulse::Morning, 1);
    assert_eq!(r.target_list, vec![ShipKind::Transport]);
    assert!(r.launches().is_none());
    assert!(r.answer.is_none());
    assert_eq!(r.contacts.len(), 1);
    assert_eq!(r.contacts[0].kind, ShipKind::Carrier);
    assert_eq!(r.contacts[0].fate, ContactFate::KindNotOnList);
}

/// A bomber group D (group 5, 12 dive bombers) at Allied base 1.
fn group_d() -> String {
    "[[air_group]]\nid = 5\nname = \"Placeholder dive bombers D\"\nside = \"allied\"\nbase = 1\naircraft_type = 3\nready = 12\n\n".to_string()
}

/// The commander's deny of the waters within 1 hex of (4,0) as intent 2, no
/// forces limit, with the limit `kinds` on it, and D (group 5) at base 1: the
/// staff forms under the deny a task, intent 3, to strike ships from base 1.
fn tree(kinds: &[&str]) -> String {
    let mut cmds = command(
        "kind = \"set_intent\"\nintent = 2\nintent_kind = \"deny\"\nhex = [4, 0]\ndistance = 1",
    );
    cmds.push_str(&targets_limit(2, kinds, false));
    let t = at(&Scene::statement_7().text(), "groups", &group_d());
    at(&t, "commands", &cmds)
}

/// Statement 5. Read up the tree, the nearest. The limit is set on the
/// commander's intent (the deny, intent 2) and the strike is flown under the
/// task the staff formed beneath it (intent 3, to strike ships from base 1):
/// D has the limit's list and launches at T. A second limit set on the staff's
/// own task, carrier first, replaces it for that task alone; it is taken, not
/// refused as wider.
#[test]
fn s05_read_up_the_tree_the_nearest() {
    let p = play(&tree(&transport_first()), &figures(false));
    p.nothing_refused();
    let task = p
        .state
        .intents
        .values()
        .find(|i| i.maker == sim_core::command::Maker::Staff)
        .expect("the staff formed a task");
    assert_eq!(task.id, intent(3));
    assert_eq!(task.parent, Some(intent(2)));
    let lists = p.list_orders(1, 5);
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].1, kinds_of(&transport_first()));
    assert_eq!(lists[0].0.intent, intent(3));
    assert!(words(&lists[0].0.reason).contains("the targets limit set on intent:2"));
    // D launches at T in the morning; B is untouched by the limit.
    assert!(morning(&p, 1).contains(&(5, hex(4, 0), 12)));
    assert!(p.list_orders(1, 1).is_empty());
    assert_eq!(
        review(&p, 1, Pulse::Morning, 1).target_list,
        kinds_of(&default_kinds())
    );

    // The commander sets a second limit on the staff's own task on day 2.
    let second = ["carrier", "transport"];
    let t = at(
        &tree(&transport_first()),
        "commands",
        &targets_limit_on(2, 3, &second, false),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    let lists = p.list_orders(2, 5);
    assert_eq!(lists.len(), 1, "{:?}", p.orders_to_group(2, 5));
    assert_eq!(lists[0].1, kinds_of(&second));
    assert!(words(&lists[0].0.reason).contains("the targets limit set on intent:3"));
    // For that task alone: the deny's own limit stands.
    let (on_task, at_task) = p.state.targets_read_up(intent(3)).expect("a limit");
    assert_eq!(at_task, intent(3));
    assert_eq!(on_task.kinds, kinds_of(&second));
    let (on_deny, at_deny) = p.state.targets_read_up(intent(2)).expect("a limit");
    assert_eq!(at_deny, intent(2));
    assert_eq!(on_deny.kinds, kinds_of(&transport_first()));
}

/// A targets limit of the Allied commander on `intent_id`, given on `day`.
fn targets_limit_on(day: u32, intent_id: u32, kinds: &[&str], waters_first: bool) -> String {
    command_on(
        day,
        &format!(
            "kind = \"set_limit\"\nlimit = \"targets\"\nintent = {intent_id}\nkinds = {}\nwaters_first = {waters_first}",
            kinds_toml(kinds)
        ),
    )
}

/// Statement 6. Cleared. The limit set with no kinds is cleared: the next
/// morning the staff orders B's list back to the six kinds in the order of
/// importance.
#[test]
fn s06_cleared() {
    let t = with_limit(&Scene::statement_1(), &transport_first());
    let t = at(
        &t,
        "commands",
        &command_on(2, "kind = \"set_limit\"\nlimit = \"targets\"\nintent = 1"),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert_eq!(p.list_orders(1, 1)[0].1, kinds_of(&transport_first()));
    let day2 = p.list_orders(2, 1);
    assert_eq!(day2.len(), 1, "{:?}", p.orders_to_group(2, 1));
    assert_eq!(day2[0].1, kinds_of(&default_kinds()));
    assert!(p.state.targets_read_up(intent(1)).is_none());
}

/// Statement 7. The waters come first, where the commander says so. A deny of
/// the waters within 1 hex of T's hex, seven hexes, names F in its forces limit
/// and carries a targets limit of the six kinds in the order of importance
/// with `waters_first` true. T, reported as a cruiser, is in those waters at
/// 160 nm; C, a carrier, is outside them at 120 nm. The staff orders B's waters
/// to the deny's seven hexes, and the order serves the deny (the architect's
/// ruling: it is handed in under the task F serves, intent 1, and its reason's
/// `serves.intent` is the deny). B launches at T. With the deny withdrawn the
/// staff orders B's waters empty the next morning, and B launches at C.
#[test]
fn s07_the_waters_come_first_where_the_commander_says_so() {
    let t = at(
        &scene7_with_deny(&default_kinds()),
        "commands",
        &command_on(2, "kind = \"withdraw_intent\"\nintent = 2"),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    let waters = p.waters_orders(1, 1);
    assert_eq!(waters.len(), 1);
    let (order, hexes) = &waters[0];
    assert_eq!(*hexes, waters_of_t());
    assert_eq!(order.intent, intent(1), "handed in under F's task");
    assert_eq!(
        order.reason.serves.intent,
        Some(intent(2)),
        "serves the deny"
    );
    assert!(order.reason.serves.plan.is_some());
    let r = review(&p, 1, Pulse::Morning, 1);
    assert_eq!(
        r.waters_first,
        waters_of_t().into_iter().collect::<Vec<_>>()
    );
    // T (reported a cruiser) is chosen; C, a carrier and nearer, came first
    // by every rule but the waters.
    assert_eq!(morning(&p, 1), vec![(1, hex(4, 0), 12)]);
    let c = r.contacts.iter().find(|c| c.hex == hex(0, 3)).expect("C");
    assert_eq!(c.kind, ShipKind::Carrier);
    assert!(matches!(c.fate, ContactFate::CameFirst { .. }));
    // The deny withdrawn: the next morning B's waters are ordered empty, and
    // the order serves the task; B launches at C.
    let waters = p.waters_orders(2, 1);
    assert_eq!(waters.len(), 1);
    assert!(waters[0].1.is_empty());
    assert_eq!(waters[0].0.reason.serves.intent, Some(intent(1)));
    assert!(review(&p, 2, Pulse::Morning, 1).waters_first.is_empty());
    assert!(
        morning(&p, 2).contains(&(1, hex(0, 3), 12)),
        "{:?}",
        morning(&p, 2)
    );
}

/// Statement 7a. Not without the switch. The same deny naming F with no
/// targets limit, or with one whose `waters_first` is false: the staff gives
/// no order about waters, B's are empty, and B launches at C. `waters_first`
/// true on an intent that is not a deny or beneath one is refused.
#[test]
fn s07a_not_without_the_switch() {
    let scene = Scene::statement_7();
    let plain = at(&scene.text(), "commands", &deny_naming_f(4, 0, 1));
    let off = at(
        &scene.text(),
        "commands",
        &deny_with_limit(4, 0, &default_kinds(), false),
    );
    for t in [plain, off] {
        let p = play(&t, &figures(false));
        p.nothing_refused();
        assert!(p.waters_orders(1, 1).is_empty());
        assert!(review(&p, 1, Pulse::Morning, 1).waters_first.is_empty());
        assert_eq!(morning(&p, 1), vec![(1, hex(0, 3), 12)]);
    }
    // Refused on an intent that is neither: F's own intent 1.
    let t = at(
        &scene.text(),
        "commands",
        &targets_limit(1, &default_kinds(), true),
    );
    let p = play(&t, &figures(false));
    let refused = p.refusals();
    assert_eq!(refused, vec![CommandRefusal::NoWatersFirstOn(intent(1))]);
    assert_eq!(
        refused[0].to_string(),
        "intent:1 is not a deny and does not stand beneath one: only there may the waters come first"
    );
    assert!(p.state.targets_read_up(intent(1)).is_none());
}

/// Statement 7b. The carrier's search covers the waters. In statement 7 the
/// waters' hexes are among the threats of F's search arc; in statement 7a they
/// are not.
#[test]
fn s07b_the_carriers_search_covers_the_waters() {
    let arc_words = |text: &str| {
        let t = at(text, "groups", &scout_group());
        let p = play(&t, &figures(false));
        p.nothing_refused();
        let arc: Vec<String> = p
            .orders(1)
            .into_iter()
            .filter(|(g, _)| matches!(g, Given::Air(3, AirGroupOrder::SetSearchArc { .. })))
            .map(|(_, o)| words(&o.reason))
            .collect();
        assert_eq!(arc.len(), 1);
        arc[0].clone()
    };
    let on = arc_words(&scene7_with_deny(&default_kinds()));
    for h in waters_of_t() {
        let threat = format!(
            "the waters to strike first of intent:2 at ({},{})",
            h.q, h.r
        );
        assert!(on.contains(&threat), "{threat} is a threat: {on}");
    }
    let plain = at(
        &Scene::statement_7().text(),
        "commands",
        &deny_naming_f(4, 0, 1),
    );
    let off = at(
        &Scene::statement_7().text(),
        "commands",
        &deny_with_limit(4, 0, &default_kinds(), false),
    );
    for t in [plain, off] {
        let said = arc_words(&t);
        assert!(!said.contains("the waters to strike first"), "{said}");
    }
}

/// Statement 8. Both parts together. As statement 7, with the limit's kinds
/// those of statement 2: B's list is transport first though the limit is on
/// the deny and F serves a task of its own; B launches at T, and its aircraft
/// attack a transport of T's force and not the destroyer. (T is reported as
/// the cruiser of statement 7; it is a force of three transports and a
/// destroyer.)
#[test]
fn s08_both_parts_together() {
    let p = play(&scene7_with_deny(&transport_first()), &figures(false));
    p.nothing_refused();
    let lists = p.list_orders(1, 1);
    assert_eq!(lists.len(), 1);
    assert_eq!(lists[0].1, kinds_of(&transport_first()));
    assert!(words(&lists[0].0.reason).contains("the targets limit set on intent:2"));
    assert_eq!(lists[0].0.intent, intent(1));
    assert_eq!(p.waters_orders(1, 1)[0].1, waters_of_t());
    assert_eq!(morning(&p, 1), vec![(1, hex(4, 0), 12)]);
    let hit = attacked_in_morning(&p, 1, 1);
    assert!(!hit.is_empty());
    assert!(hit.iter().all(|s| (20..=22).contains(s)), "{hit:?}");
}

/// Statement 9's scene: T, reported a cruiser of 4 ships, at (7,1), 302 nm
/// from F (the nearest the hexes give to 300), beyond B's reach of 200 nm, and
/// no carrier contact held.
fn scene9() -> Scene {
    Scene::statement_7().t_at(7, 1).no_c_held()
}

fn scene9_with_deny() -> String {
    at(
        &scene9().text(),
        "commands",
        &deny_with_limit(7, 1, &default_kinds(), true),
    )
}

/// The first hex on the way from (0,0) to `to`, along the straight path of
/// open sea, within 200 nm of `to`.
fn first_hex_within_reach(to: Hex) -> Hex {
    let mut here = hex(0, 0);
    while here.air_distance_nm(to) > 200 {
        here = here.step_toward(to);
    }
    here
}

/// Statement 9. Closing on what is in the waters. As statement 7 with T at
/// 302 nm (beyond reach) and no carrier contact held. At balanced F closes on
/// T, to the first hex on its way within 200 nm of T's hex, and the reason says
/// the contact is in the waters of the deny. With the deny withdrawn F does not
/// close: nothing of a kind it closes on is held. At preserve the force F
/// closes on nothing, with the deny or without.
#[test]
fn s09_closing_on_what_is_in_the_waters() {
    assert_eq!(hex(0, 0).air_distance_nm(hex(7, 1)), 302);
    let p = play(&scene9_with_deny(), &figures(false));
    p.nothing_refused();
    let target = first_hex_within_reach(hex(7, 1));
    assert!(target.air_distance_nm(hex(7, 1)) <= 200);
    assert_eq!(p.routes_of_force(1, 1), vec![vec![target]]);
    let said = p.said(1);
    assert!(
        said.contains(
            "in the waters of intent:2 that the targets limit set on intent:2 puts first"
        ),
        "{said}"
    );
    // With the deny withdrawn (the same day it is set): no route, and F
    // closes on nothing for want of a kind it closes on.
    let t = at(
        &scene9_with_deny(),
        "commands",
        &command("kind = \"withdraw_intent\"\nintent = 2"),
    );
    let p = play(&t, &figures(false));
    assert!(p.routes_of_force(1, 1).is_empty());
    let said = p.said(1);
    assert!(said.contains("does not close"), "{said}");
    assert!(
        said.contains("no contact of ships 3 pulses old or less is of that kind"),
        "{said}"
    );
    // At preserve the force, with the deny or without: no route, and the
    // reason says it closes on nothing.
    for t in [scene9_with_deny(), scene9().text()] {
        let t = at(&t, "commands", &risk_of_f("preserve"));
        let p = play(&t, &figures(false));
        assert!(p.routes_of_force(1, 1).is_empty());
        let said = p.said(1);
        assert!(
            said.contains("at the preserve risk it closes on nothing"),
            "{said}"
        );
    }
}

/// Statement 9a. A contact outside the waters is ranked as today. As statement
/// 7 with T moved one hex outside the waters, to (3,2): B launches at C. And a
/// contact in the waters beyond B's reach does not keep B from C in reach: T at
/// (7,1) in the waters of a deny there, C at (0,3): B launches at C.
#[test]
fn s09a_a_contact_outside_the_waters_is_ranked_as_today() {
    assert!(!waters_of_t().contains(&hex(3, 2)));
    assert_eq!(hex(4, 0).distance(hex(3, 2)), 2);
    let t = at(
        &Scene::statement_7().t_at(3, 2).text(),
        "commands",
        &deny_with_limit(4, 0, &default_kinds(), true),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert_eq!(p.waters_orders(1, 1)[0].1, waters_of_t());
    assert_eq!(morning(&p, 1), vec![(1, hex(0, 3), 12)]);

    // At preserve the force, F closes on nothing and stays in its hex, so
    // that T is beyond B's reach at the morning's strike decision (at
    // balanced F closes on T in the night, statement 9d, and T is then in
    // reach).
    let t = at(
        &Scene::statement_7().t_at(7, 1).text(),
        "commands",
        &deny_with_limit(7, 1, &default_kinds(), true),
    );
    let t = at(&t, "commands", &risk_of_f("preserve"));
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert!(p.routes_of_force(1, 1).is_empty());
    assert_eq!(p.waters_orders(1, 1).len(), 1);
    let r = review(&p, 1, Pulse::Morning, 1);
    let t_contact = r.contacts.iter().find(|c| c.hex == hex(7, 1)).expect("T");
    assert!(
        matches!(t_contact.fate, ContactFate::OutOfReach { .. }),
        "{:?}",
        t_contact.fate
    );
    // (At preserve the force a share of the aircraft is held back.)
    let launched = morning(&p, 1);
    assert_eq!(launched.len(), 1, "{launched:?}");
    assert_eq!((launched[0].0, launched[0].1), (1, hex(0, 3)));
}

/// Statement 9d. Closing leaves a carrier in reach for a contact in the
/// waters. As statement 7 with T in the waters at 302 nm and C outside them at
/// 120 nm, in reach. At balanced F closes on T.
#[test]
fn s09d_closing_leaves_a_carrier_in_reach_for_a_contact_in_the_waters() {
    let t = at(
        &Scene::statement_7().t_at(7, 1).text(),
        "commands",
        &deny_with_limit(7, 1, &default_kinds(), true),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    let target = first_hex_within_reach(hex(7, 1));
    assert_eq!(p.routes_of_force(1, 1), vec![vec![target]]);
    assert!(p
        .said(1)
        .contains("in the waters of intent:2 that the targets limit set on intent:2 puts first"));
}

/// Statement 9b. Preserve the force keeps the limit's list. With the limit of
/// statement 2 at preserve the force, B's list is the limit's six kinds and not
/// the risk's three.
#[test]
fn s09b_preserve_the_force_keeps_the_limits_list() {
    let t = at(
        &with_limit(&Scene::statement_1(), &transport_first()),
        "commands",
        &risk_of_f("preserve"),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert_eq!(p.list_orders(1, 1)[0].1, kinds_of(&transport_first()));
    assert_eq!(
        review(&p, 1, Pulse::Morning, 1).target_list,
        kinds_of(&transport_first())
    );
    // Without the limit the risk gives three kinds.
    let none = play(
        &at(
            &Scene::statement_1().text(),
            "commands",
            &risk_of_f("preserve"),
        ),
        &figures(false),
    );
    assert_eq!(
        review(&none, 1, Pulse::Morning, 1).target_list,
        kinds_of(&["carrier", "battleship", "cruiser"])
    );
}

/// Statement 9c. A group launches once in a pulse. With B of 60 aircraft, T
/// reported as a cruiser of 4 ships in the waters and C outside: the aircraft
/// the sizing figures give for T go at T and the rest do not fly that pulse;
/// none goes at C. A second bomber group aboard F launches at C. The figures
/// (the table of aircraft for each ship, in this test's copy of the frozen air
/// combat file) give a cruiser 12: 4 x 12 = 48 go at T.
#[test]
fn s09c_a_group_launches_once_in_a_pulse() {
    let t = at(
        &b_ready(&scene7_with_deny(&default_kinds()), 60),
        "groups",
        &second_bomber_group(12),
    );
    let p = play(&t, &figures_with(true, 100));
    p.nothing_refused();
    let per_ship = 12;
    let ships = 4;
    let sent_at_t = per_ship * ships;
    assert_eq!(sent_at_t, 48);
    assert_eq!(
        morning(&p, 1),
        vec![(1, hex(4, 0), sent_at_t), (4, hex(0, 3), 12)],
        "B sends 48 at T and the other 12 do not fly; group 4 goes at C"
    );
}

/// Statement 10. Refused: a limit that names no intent of the side's in force;
/// a kind named twice. Each with the reason, as the other limits are refused.
/// A limit on a staff-made task with other kinds than its parent's is taken,
/// not refused as wider.
#[test]
fn s10_refused() {
    let p = play(
        &at(
            &Scene::statement_1().text(),
            "commands",
            &targets_limit(9, &transport_first(), false),
        ),
        &figures(false),
    );
    let refused = p.refusals();
    assert_eq!(refused, vec![CommandRefusal::NoSuchIntent(intent(9))]);
    assert_eq!(refused[0].to_string(), "intent:9 is not in force");

    let p = play(
        &at(
            &Scene::statement_1().text(),
            "commands",
            &targets_limit(1, &["carrier", "carrier"], false),
        ),
        &figures(false),
    );
    let refused = p.refusals();
    assert_eq!(refused, vec![CommandRefusal::BadTargetKinds(intent(1))]);
    assert_eq!(
        refused[0].to_string(),
        "the targets limit on intent:1 must name one to six kinds of ship, each once"
    );
    assert!(p.state.targets_read_up(intent(1)).is_none());

    // A limit on a staff-made task with other kinds than its parent's.
    let t = at(
        &tree(&transport_first()),
        "commands",
        &targets_limit_on(2, 3, &["carrier"], false),
    );
    let p = play(&t, &figures(false));
    p.nothing_refused();
    assert_eq!(
        p.state.targets_read_up(intent(3)).expect("a limit").0.kinds,
        vec![ShipKind::Carrier]
    );
}

/// Statement 11. What a seat reads: `command_state` shows the limit on the
/// intent, the kinds in order. The other side is shown nothing of it.
#[test]
fn s11_what_a_seat_reads() {
    let mut scenario = common::from_text_with(
        &with_limit(&Scene::statement_1(), &transport_first()),
        &path(),
        &figures(false),
    )
    .expect("loads");
    let held = staffs();
    scenario
        .next_day_watched(as_staffs(&held), &mut |_, _| {})
        .expect("a day resolves");
    let allied = sim_agent::command_state(
        &scenario.world.side_view(Side::Allied),
        held[&Side::Allied].as_ref(),
        &[],
    );
    let found = find_targets(&allied).expect("the limit is shown on the intent");
    assert_eq!(
        found,
        serde_json::json!({"kinds": transport_first(), "waters_first": false})
    );
    let japanese = sim_agent::command_state(
        &scenario.world.side_view(Side::Japanese),
        held[&Side::Japanese].as_ref(),
        &[],
    );
    let said = japanese.to_string();
    assert!(find_targets(&japanese).is_none(), "{said}");
    assert!(!said.contains("waters_first"), "{said}");
}

/// The first `targets` object found in a JSON value.
fn find_targets(value: &serde_json::Value) -> Option<serde_json::Value> {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(found) = map.get("targets").filter(|v| v.is_object()) {
                return Some(found.clone());
            }
            map.values().find_map(find_targets)
        }
        serde_json::Value::Array(items) => items.iter().find_map(find_targets),
        _ => None,
    }
}

// ---------------------------------------------------------------- Properties

/// The three risks, as the commands set them.
const RISKS: [&str; 3] = ["preserve", "balanced", "press"];

/// Property: a group's list set from a limit holds exactly the limit's kinds,
/// in its order, each once, at every risk.
#[test]
fn p_a_list_set_from_a_limit_holds_exactly_the_limits_kinds_in_order() {
    let limits: Vec<Vec<&str>> = vec![
        vec!["transport"],
        vec!["carrier", "transport"],
        vec!["auxiliary", "destroyer", "cruiser"],
        transport_first(),
        default_kinds().into_iter().rev().collect(),
        vec!["destroyer", "transport", "battleship", "carrier"],
    ];
    for kinds in &limits {
        for risk in RISKS {
            let t = at(
                &with_limit(&Scene::statement_1(), kinds),
                "commands",
                &risk_of_f(risk),
            );
            let p = play(&t, &figures(false));
            p.nothing_refused();
            let want = kinds_of(kinds);
            let got = &review(&p, 1, Pulse::Morning, 1).target_list;
            assert_eq!(got, &want, "{kinds:?} at {risk}");
            let unique: BTreeSet<ShipKind> = got.iter().copied().collect();
            assert_eq!(unique.len(), got.len(), "each kind once");
            // Where the staff ordered the list, it ordered exactly these.
            for (_, ordered) in p.list_orders(1, 1) {
                assert_eq!(ordered, want, "{kinds:?} at {risk}");
            }
        }
    }
}

/// Property: a limit of all six kinds in the default order gives the orders of
/// no limit, at balanced and at press (and the same launches).
#[test]
fn p_a_limit_of_all_six_in_the_default_order_gives_the_orders_of_no_limit() {
    for scene in [Scene::statement_1(), Scene::statement_7()] {
        for risk in ["balanced", "press"] {
            let base = at(&scene.text(), "commands", &risk_of_f(risk));
            let limited = at(
                &base,
                "commands",
                &targets_limit(1, &default_kinds(), false),
            );
            let none = play(&base, &figures(false));
            let with = play(&limited, &figures(false));
            with.nothing_refused();
            for day in [1, 2] {
                let a: Vec<Given> = none.orders(day).into_iter().map(|(g, _)| g).collect();
                let b: Vec<Given> = with.orders(day).into_iter().map(|(g, _)| g).collect();
                assert_eq!(a, b, "day {day} at {risk}");
            }
            assert_eq!(none.launches(1), with.launches(1), "at {risk}");
            assert_eq!(none.launches(2), with.launches(2), "at {risk}");
        }
    }
}

/// Property: moving a kind earlier on the limit never lowers the aircraft sent
/// at contacts of that kind. Two bomber groups of 12 (B and group 4) aboard F,
/// the table of aircraft for each ship (a transport needs 6 a ship); C, a
/// carrier, at (0,4), and T, three transports, at (3,0). For each kind with a
/// contact, the list of the default order with that kind moved one place
/// earlier at a time: the aircraft launched in the morning at its contact
/// does not fall.
#[test]
fn p_moving_a_kind_earlier_never_lowers_the_aircraft_sent_at_it() {
    let sent_at = |kinds: &[&str], target: Hex| -> u32 {
        let t = at(
            &with_limit(&Scene::statement_1(), kinds),
            "groups",
            &second_bomber_group(12),
        );
        let p = play(&t, &figures(true));
        p.nothing_refused();
        morning(&p, 1)
            .into_iter()
            .filter(|(_, h, _)| *h == target)
            .map(|(_, _, n)| n)
            .sum()
    };
    for (kind, target) in [("carrier", hex(0, 4)), ("transport", hex(3, 0))] {
        let others: Vec<&str> = default_kinds().into_iter().filter(|k| *k != kind).collect();
        // Position 5 (index 5 of 6 places) is the last; move it to the first.
        let mut sent = Vec::new();
        for place in (0..=others.len()).rev() {
            let mut kinds = others.clone();
            kinds.insert(place, kind);
            sent.push((place, sent_at(&kinds, target)));
        }
        for pair in sent.windows(2) {
            assert!(
                pair[1].1 >= pair[0].1,
                "{kind} moved from place {} to {} lowered the aircraft sent: {sent:?}",
                pair[0].0,
                pair[1].0
            );
        }
        // It is not a vacuous test: first of all it gets some.
        assert!(sent.last().expect("a place").1 > 0, "{kind}: {sent:?}");
    }
}

/// Property: a contact moved into the waters never ranks lower than it did.
/// The deny of statement 7 names F; its targets limit has `waters_first` off,
/// then on. For each kind the contact in the waters (at (3,1)) is reported as,
/// and a carrier or a transport as the other contact, outside them at (0,3),
/// each of the two contacts in turn being the one in the waters: where B
/// launches at the contact in the waters with the switch off it launches at it
/// with the switch on, and with the switch on it launches at it always.
#[test]
fn p_a_contact_moved_into_the_waters_never_ranks_lower() {
    let kinds = ["carrier", "battleship", "cruiser", "destroyer", "transport"];
    let chosen = |scene: &Scene, waters_first: bool| -> Option<Hex> {
        let t = at(
            &scene.text(),
            "commands",
            &deny_with_limit(4, 0, &default_kinds(), waters_first),
        );
        let p = play(&t, &figures(false));
        p.nothing_refused();
        let r = review(&p, 1, Pulse::Morning, 1);
        r.chosen_contact().map(|c| c.hex)
    };
    let mut checked = 0;
    for inside_kind in kinds {
        for outside_kind in ["carrier", "transport"] {
            // T in the waters, C outside; then C in the waters, T outside.
            let t_in = Scene::statement_7()
                .t_at(3, 1)
                .t_kind(inside_kind, 1)
                .c_at(0, 3)
                .c_kind(outside_kind);
            let c_in = Scene::statement_7()
                .c_at(3, 1)
                .c_kind(inside_kind)
                .t_at(0, 3)
                .t_kind(outside_kind, 1);
            for (scene, inside) in [(t_in, hex(3, 1)), (c_in, hex(3, 1))] {
                assert!(waters_of_t().contains(&inside));
                let off = chosen(&scene, false);
                let on = chosen(&scene, true);
                if off == Some(inside) {
                    assert_eq!(on, Some(inside), "{inside_kind} in, {outside_kind} out");
                }
                // With the waters on, the contact in them is the one taken.
                assert_eq!(on, Some(inside), "{inside_kind} in, {outside_kind} out");
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 20);
}
