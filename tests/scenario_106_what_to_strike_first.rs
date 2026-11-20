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

mod common;

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
/// the texts into for a comparison.
#[test]
fn s12_no_game_without_the_limit_changes() {
    let games: [(&str, Option<u32>, u64); 5] = [
        ("05_eastern_solomons.toml", None, 0x6ed6_d981_3862_0cae),
        (
            "five-days/01a_eastern_solomons.toml",
            None,
            0x0986_e16d_1c5c_11bb,
        ),
        (
            "stage-3-proof/07a_slice.toml",
            Some(7),
            0x2a39_9fd1_1603_6253,
        ),
        (
            "second-proof/05a_eastern_solomons.toml",
            None,
            0x0e25_c601_cbf3_eb7c,
        ),
        ("second-proof/07a_week.toml", None, 0xcfbf_084c_b2f4_3ec5),
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
