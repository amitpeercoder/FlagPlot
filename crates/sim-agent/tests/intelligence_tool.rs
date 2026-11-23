//! The tool `intelligence` (decision 0226, "The paper, as amended"): what a
//! seat holds of the enemy, gathered on one answer.
//!
//! Written by the architect on Nov 23, 2026, before the code. The statements
//! are numbered; each test below is named for the statements it proves. If
//! the code disagrees with a statement or a number here, the code or the
//! rule is wrong: nobody changes a statement or a number to make a test
//! pass. A mismatch goes back to the architect.
//!
//! The tool makes no rule and rolls nothing: it gathers. So most numbers
//! here are not worked by hand but said as equal to a line the side's own
//! tools already gave (the briefing of each morning, `list_contacts`,
//! `list_bases`, `explain`), which the test reads from those tools in the
//! same session. The numbers worked by hand are the ones a scenario's file
//! fixes and the ages that follow from the frozen search figure; they are
//! under "Hand calculation". Where the dice decide (what a report said, how
//! many aircraft a claim gave) the test reads the value from the side's own
//! briefing and asserts the rest, as `tests/scenario_24_shells_and_estimates.rs`
//! does.
//!
//! # The scenarios
//!
//! Every load is through `support::load` or `support::load_text`, so on the
//! frozen step 1 figures (`data/reference/step1`), as every test of this
//! crate is. None of the five is one of the scenarios that has a frozen copy
//! in this crate's loader (5, 6, 7 and 8), and none is a scenario for play.
//! No scenario is added. Each seat named is a player's, the other the
//! script's with the scenario's own orders, and there is no staff unless a
//! statement says so.
//!
//! | Tag | File | Seed | What it gives |
//! | --- | ---- | ---- | ------------- |
//! | A | `24_shells_and_estimates.toml` | 1942 | Both seats. Allied: a contact made, refreshed and dropped for its age; its base shelled on two nights; an estimate ashore made, replaced and found empty; an enemy base found. Japanese: an opening estimate that a later report replaces; two claims from the sea |
//! | B | `05b_strike.toml` | 11 | Allied seat. A contact made and struck, a claim with what the target was taken for, the contact refreshed, a second strike that finds nothing and drops it |
//! | C | `04_savo_island.toml` | 1942 | Allied seat. A contact given at the start and dropped with no report since; a contact held; claims of ships sunk in a fight at sea; a surface action met |
//! | D | `table-submarines/128_a_submarine_in_the_way.toml` | 6 | Both seats. Allied: suspected waters told before day 1, two reports of a submarine of which one is dropped, a carrier torpedoed twice. Japanese: two claims of a submarine |
//! | E | `62_reports_between_commands.toml` | 1942 | Command levels on, the Allied side alone, a contact given at the start. Seats `area`, `area:easier`, `task_force:1`, `task_force:2` and `base:1`, made as `tests/seat.rs` makes them |
//!
//! Seed 11 of B and seed 6 of D are chosen because on them the thing wanted
//! happens (B: the afternoon strike flies at a report one hex out and finds
//! nothing; D: the boat fires on the first two mornings, as the file's head
//! says). The test first asserts, from the side's own briefings, that it did
//! (statement 0), so that a change of rule that moves the dice fails there
//! and says so, and not in a statement about the tool.
//!
//! # The answer
//!
//! `intelligence` takes no argument and refuses any, as `events` refuses an
//! argument it does not know. Its answer is JSON with `"format"`,
//! `"version"` and `"kind": "intelligence"`, valid against
//! `schemas/intelligence.schema.json`. Beside them it has:
//!
//! - `parts`: the names of the parts that hold something, in the paper's
//!   order: `seen`, `lost_sight_of`, `claimed`, `enemy_has_done`, `ashore`,
//!   `enemy_bases`, `enemy_submarines`, `before_the_game`.
//! - one key for each name in `parts`, and no key for a part that is not in
//!   it.
//! - `not_kept`: the names of the parts this seat is not given (statement
//!   16). Left out where it is empty.
//!
//! A line of a part is of the type the tool that already says that thing
//! uses (`ContactLine`, the briefing's `Claim`, `Raid`, `AttackOnShip`,
//! `BombardmentAgainst`, `Fight`, `GroundEstimateLine`, `FoundEmptyLine`,
//! `EnemyBaseLine`, `SubmarineReportLine`, `SuspectedWatersLine`), with
//! `day` and `pulse` added where that type has none. The answer holds no
//! sentence of the tool's own: its only words are the `words` of suspected
//! waters, as told.
//!
//! "The briefing of morning N" below is `get_briefing` with `text` false,
//! asked by the same seat before day N is played (and, for the last day,
//! after the game is over). "Day d of the briefings" is the briefing of
//! morning d + 1, which says what happened on day d.
//!
//! # Statements
//!
//! 0. **The scenarios do what they were chosen for,** read from the side's
//!    own briefings and `list_contacts`: A, B, C and D each as the table
//!    above says, on the seat named.
//!
//! 1. **The parts are in the paper's order and a part with nothing is left
//!    out.** `parts` is a subsequence of the eight names in the order above,
//!    the answer has a key for each and for no other part, and no part's key
//!    holds an empty list or an object whose lists are all empty. On the
//!    first morning of A the Allied answer has `parts` empty and none of the
//!    eight keys.
//!
//! 2. **SEEN** (`seen`) is, line for line, the `contacts` of `list_contacts`
//!    with no area, asked at the same moment, put in this order: the least
//!    `age_pulses` first, and of two of one age the higher contact number
//!    first. Each line has the contact's handle, its place, `ship_count`,
//!    `taken_for` and `age_pulses`, and nothing else.
//!
//! 3. **LOST SIGHT OF** (`lost_sight_of`) has one line for every contact the
//!    side has dropped, and none for a contact held. A line has:
//!    - `contact`, the handle the contact had;
//!    - `last_report`: `day`, `pulse`, `place`, `ship_count`, `taken_for`
//!      and `event`, which are those of the last report that made or
//!      refreshed it, field for field the `new` or `refreshed` line of the
//!      briefings that names that event. For a contact given at the start
//!      and never refreshed, `day` is 0, `pulse` is null and the rest is as
//!      the scenario gave it;
//!    - `dropped`: `day`, `pulse`, `event`, and `why`, which is one of two
//!      words and no other: `no_report_since` (with `age_pulses`, the age it
//!      was dropped at) for a contact in a briefing's `dropped`, or
//!      `strike_found_nothing` (with `place`, where the strike flew) for one
//!      in a briefing's `not_there`.
//!
//!    The lines are in the order dropped, the latest first, and of two
//!    dropped in one pulse the higher contact number first. A contact
//!    number that was dropped and is not held again is in no line of `seen`.
//!
//! 4. **CLAIMED** (`claimed`) has `days` and `totals`.
//!    - `days` is a list of `{day, claims}`, the latest day first, with no
//!      day that has no claim. The `claims` of a day are, in the order the
//!      side was told them, every claim in the `losses.claims` of that day
//!      of the briefings, field for field, each with its `pulse` and
//!      `event` added. So a claim of ships from the air has `taken_for`, a
//!      claim of a fight at sea has none, a claim of a bombardment has
//!      `from_bombardment` and a claim of a submarine has `from_submarine`.
//!    - Counted over every day, the claims are as many as the seat's picture
//!      holds, no more and no fewer.
//!    - `totals` has exactly four keys, `sunk`, `hits`, `shot_down` and
//!      `on_ground`, each the sum of that field over every claim of `days`.
//!      It has no total by kind of ship, by place or by day, and nothing in
//!      the answer subtracts a claim from a count of ships seen.
//!    - No claim names a contact: no line of `claimed` has a key `contact`
//!      or a handle of a contact in any value.
//!
//! 5. **WHAT THE ENEMY HAS DONE** (`enemy_has_done`) is a list of days, the
//!    latest first, with no day on which nothing was done. A day has `day`
//!    and, each left out where empty and each in the order told:
//!    - `raids`: the `action.raids_on_bases` of that day of the briefings,
//!      field for field, with `recognised` as the briefing has it;
//!    - `bombardments`: its `action.bombardments_against`;
//!    - `attacks_on_ships`: its `action.attacks_on_ships`. A submarine's
//!      salvo is not among them: the briefing does not say it;
//!    - `submarine_attacks`: one line for each salvo a submarine fired at a
//!      ship of the side's, read from the side's own events (the salvo from
//!      a submarine, as the side is given it): `against`, the handle of the
//!      side's ship; `place`; `pulse`; and `event`. It has no count of
//!      torpedoes or of hits and nothing of the boat, none of which the
//!      side is told (amended on Nov 23, 2026, the rulings on the build);
//!    - `fights`: its `action.fights`, each naming the side's own force and
//!      never the enemy's.
//!
//!    A fight ashore is not in this part.
//!
//! 6. **ASHORE** (`ashore`) has `estimates`, which is the `ground_estimates`
//!    of `list_contacts` at the same moment, line for line (place, `men`,
//!    `came_from`, `day`, `age_days`, `event`); and `found_empty`, every
//!    line of `found_empty_yesterday` of every briefing so far, the latest
//!    first, each with the `day` it was found empty. Either is left out
//!    where it is empty.
//!
//! 7. **ENEMY BASES** (`enemy_bases`) lists a base only if it is the
//!    enemy's on the seat's map now (a line of `list_bases`' enemy bases,
//!    whose `base`, `place` and `held_by` the line repeats) and one of
//!    these is held of it: an estimate of `ashore.estimates` in its hex,
//!    given as `estimate` (that line); or claims of aircraft destroyed on
//!    the ground in its hex with `from_air` true, given as
//!    `claimed_on_ground_from_air`, their sum, with `claims`, each one's
//!    `day` and `number`, the aircraft it claims. A claim from the sea puts no base in this part. The
//!    lines are in the order of the bases' numbers.
//!
//! 8. **ENEMY SUBMARINES** (`enemy_submarines`) has `reports`, the
//!    `submarine_reports` of `list_contacts` at the same moment, and
//!    `suspected_waters`, its `suspected_waters`, each line for line and
//!    each left out where empty. A report that was dropped is in neither.
//!
//! 9. **BEFORE THE GAME** (`before_the_game`) has `contacts`: each contact
//!    the side was given at the start, as given (the handle, place,
//!    `ship_count`, `taken_for`, the age in pulses it was given with, and
//!    the number of its event of day 0); and `estimates`: each opening
//!    estimate of men ashore, as given (place, `men`, `came_from` "told",
//!    the number of its event of day 0). Each is left out where empty. It
//!    says the same on every day of the game, whatever was reported since:
//!    a contact since dropped and an estimate since replaced are still in
//!    it as given. It holds no sentence and nothing of a directive.
//!
//! 10. **The words** (rule 3). The key for what a contact or a target was
//!     thought to be is `taken_for` wherever it occurs, and no line of a
//!     part has a key `kind`. No line of `seen`, `lost_sight_of` or
//!     `before_the_game` has a key that contains "force", and the answer
//!     gives no count of contacts or of forces. No key and no value
//!     contains "his" as a word. The answer does not hold the words
//!     "confirmed" or "destroyed" as a key of a claim: what is claimed is
//!     `claimed_sunk`, `claimed_hits`, `claimed_shot_down` and
//!     `claimed_on_ground`, as the briefing has them.
//!
//! 11. **Nothing in it is the staff's estimate.** The answer has no key and
//!     no value that says how many ships or aircraft the enemy has or has
//!     left, that two contacts are one, or that a claim is true or false.
//!     A contact struck and then dropped is in `lost_sight_of` and its
//!     claim in `claimed`, and neither line names the other.
//!
//! 12. **Two worlds** (rule 4, first half). Two games that differ only in
//!     something the seat cannot know give the same `intelligence`, byte
//!     for byte as serialised, on every morning and when the game is over.
//!     The test first asserts that the two scenario texts differ, and that
//!     the umpire's state of the two differs on the last day.
//!     - A, the Japanese seat: Allied base 1 starts with 1900 t of supply
//!       in place of 2000 t.
//!     - A, the Allied seat: Japanese ship 1 starts with 1900 t of fuel in
//!       place of 2000 t.
//!     - B, the Allied seat: Japanese ship 5 starts with 1500 t of fuel in
//!       place of 2000 t.
//!     - D, the Japanese seat: Allied ship 2, the destroyer, starts with
//!       400 t of fuel in place of 500 t.
//!     None of the four changes a draw: the hand calculation of A charges
//!     supply and fuel that never run short, and in B and D the ship lies
//!     stopped.
//!
//! 13. **Every handle and number is the seat's own** (rule 4, second half).
//!     The test walks the whole answer of every seat of A, B, C and D on
//!     every morning and at the end:
//!     - every `event` number is one `explain` knows at that seat (its
//!       answer is not the not-known answer), and the kind of the event
//!       explained is the kind the line is of: a report for a
//!       `last_report` and an opening contact, a contact dropped or a
//!       contact not there for a `dropped`, a claim for a claim, a ground
//!       report for an estimate, and so on;
//!     - every handle of a contact is in `list_contacts` now, or is the
//!       `contact` of a `new` line of an earlier briefing, or is a contact
//!       of `before_the_game`;
//!     - every handle of a base is in `list_bases` now, and every handle of
//!       a unit is one `get_entity` knows at that seat;
//!     - every other number in a line is equal to the same field of the
//!       line statements 2 to 9 name for it. No number in the answer has
//!       another source, but the four totals and
//!       `claimed_on_ground_from_air`, which are sums of numbers in it.
//!
//! 14. **The game over** (rule 6). After the last day the tool still
//!     answers, with the same keys and by the same statements, and the
//!     answer has no key `result`, `points`, `margin`, `score` or `truth`
//!     and no key that `get_briefing` gives only when the game is over. In
//!     A at the Japanese seat the estimate at (8,0) is still the last
//!     report's and the totals are still what was claimed; statement 12
//!     holds of the answer given after the end.
//!
//! 15. **A reading and no more** (as amended, 1). Asking `intelligence` any
//!     number of times on any morning changes no day's state hash: game A
//!     played with the tool asked by both seats every morning ends each
//!     day on the hashes of game A played without it. Two calls at one
//!     moment give the same bytes. The tool is in `TOOLS` and in
//!     `WATCHER_TOOLS`, a watcher's read of it for a seat gives what that
//!     seat is given, and `VERSION` is 52.
//!
//! 16. **Command levels on** (rule 5), over E.
//!     - At `area:easier`, whose picture is the side's own belief, every
//!       part is given: `before_the_game.contacts` holds the one contact
//!       the file gives, 1 ship taken for a carrier at (12,0), given at
//!       age 0, and there is no `not_kept`.
//!     - At `area`, `task_force:1`, `task_force:2` and `base:1`,
//!       `not_kept` is exactly `["lost_sight_of", "enemy_has_done",
//!       "before_the_game"]`, the answer has none of those three keys and
//!       `parts` none of those names, on every morning.
//!     - At each of those four seats `seen`, `ashore` and
//!       `enemy_submarines` are as statements 2, 6 and 8 say of
//!       `list_contacts` asked at that seat, and `claimed` holds the claims
//!       of that seat's picture and no other.
//!     - No line of `enemy_bases` at any seat names a base whose
//!       `held_by` is the seat's own side; base 1, the side's own, is in
//!       none.
//!     - `tests/seat.rs` asks every tool a seat has in its two worlds;
//!       `intelligence` is among them with no exemption.
//!     With command levels off (A to D) no answer has `not_kept`.
//!
//! # Hand calculation
//!
//! Only what a file or a frozen figure fixes. The frozen `search.toml` has
//! `contact_drop_age_pulses` 5. A day's pulses are night, morning,
//! afternoon. A contact ages by one at the end of each pulse in which no
//! report refreshes it, and is dropped in the pulse its age reaches 5.
//!
//! ## A, the Japanese seat
//!
//! The file gives the Japanese side one opening estimate, 300 Allied men at
//! Y, (8,0), and Allied bases 1 at (0,0) and 2 at (8,0) are on its map.
//!
//! - First morning: `parts` is `["ashore", "enemy_bases",
//!   "before_the_game"]`. `ashore.estimates` is one line: (8,0), 300 men,
//!   "told", day 0, age 0 days. `enemy_bases` is one line: base 2, with
//!   that estimate. Base 1 is not listed. `before_the_game.estimates` is
//!   one line: (8,0), 300 men, "told"; it has no `contacts`.
//! - The force fires on the nights of days 1 and 2 and on no other, so the
//!   side has two claims and no more: `claimed.days` is day 2 then day 1,
//!   one claim each, at (0,0), in the night, `from_bombardment` true,
//!   `from_air` false, no `taken_for`. With c1 and c2 the aircraft each
//!   claims on the ground, read from the briefings: `totals` is `sunk` 0,
//!   `hits` 0, `shot_down` 0, `on_ground` c1 + c2.
//! - When the game is over (day 4 resolved): `parts` is `["claimed",
//!   "ashore", "enemy_bases", "before_the_game"]`. Unit 3 fights on days 1
//!   and 2 and is destroyed on day 3 (it has 800 men and loses 20 for each
//!   of at most 30 shots; the run decides the day, and statement 0 reads
//!   that the last Japanese ground report is of day 2). So
//!   `ashore.estimates` is one line at (8,0) of day 2, "men in contact",
//!   age 4 - 2 = 2 days, with the men that report gave.
//!   `enemy_bases` is base 2 alone with that estimate: base 1, at whose
//!   hex c1 + c2 aircraft are claimed, is not listed, the claims being
//!   from the sea. `before_the_game.estimates` is still the one line of
//!   300 men, "told", though the estimate held is another.
//! - No `seen`, `lost_sight_of`, `enemy_has_done` or `enemy_submarines` on
//!   any morning: the side sights no ship, and the fight ashore is not an
//!   attack this part lists.
//!
//! ## A, the Allied seat
//!
//! - After day 1: `seen` is one contact, made in the night of day 1 by the
//!   report the bombardment gave and aged in the morning and the
//!   afternoon: age 2. `enemy_has_done` is one day, day 1, with one
//!   bombardment, of base 1, and no raid, attack on a ship or fight.
//!   `ashore.estimates` is one line at (8,0), day 1, "men in contact", age
//!   0. `enemy_bases` is base 3 at (8,0), found that day, with that
//!   estimate. No `before_the_game`: the side was given nothing.
//! - After day 2: the contact was refreshed in the night of day 2 (the
//!   second bombardment) and aged twice: age 2. `enemy_has_done` is day 2
//!   then day 1, one bombardment each.
//! - After day 3: ages 3 in the night, 4 in the morning, 5 in the afternoon
//!   of day 3, and it is dropped then. `seen` is not in `parts`.
//!   `lost_sight_of` is one line: `last_report` day 2, night, with what the
//!   briefing's refresh of that night said; `dropped` day 3, afternoon,
//!   `why` `no_report_since`, `age_pulses` 5. If the run destroys unit 3
//!   on day 3 (statement 0 reads it), `ashore` has no `estimates` and one
//!   `found_empty` line, (8,0), of day 3, with the men and the day, 2, of
//!   the estimate cleared; and `enemy_bases` is not in `parts`.
//! - Over: as after day 3; `parts` is `["lost_sight_of",
//!   "enemy_has_done", "ashore"]`.
//!
//! ## B, the Allied seat, seed 11
//!
//! One day. The morning search sights and a strike attacks; the afternoon
//! search sights again, the report is a hex out, and the strike sent at it
//! finds nothing. When the game is over `parts` is `["lost_sight_of",
//! "claimed"]`:
//!
//! - `lost_sight_of` is one line: `last_report` day 1, afternoon, as the
//!   briefing's last refresh; `dropped` day 1, afternoon, `why`
//!   `strike_found_nothing`, `place` the place of that last report, and no
//!   `age_pulses`.
//! - `claimed.days` is day 1 with one claim, of the morning, `from_air`
//!   true, with `taken_for`; `totals.hits` is its `claimed_hits`,
//!   `totals.sunk` its `claimed_sunk`, and the other two are 0.
//! - Neither line names the other (statement 11), though the claim was made
//!   on the contact since lost.
//!
//! ## C, the Allied seat
//!
//! The file gives the Allied side one contact: 8 ships taken for cruisers
//! at (-1,-9), 2 pulses old. The game is 1 day.
//!
//! - First morning: `seen` holds it at age 2, and
//!   `before_the_game.contacts` is that one line: 8, "cruiser", (-1,-9),
//!   given at age 2.
//! - No report refreshes it at that place (statement 0 reads that the
//!   reports of the night make another contact). It ages to 3 in the
//!   night, 4 in the morning and 5 in the afternoon of day 1 and is
//!   dropped then. When the game is over `lost_sight_of` has its line:
//!   `last_report` day 0, pulse null, 8, "cruiser", (-1,-9); `dropped` day
//!   1, afternoon, `no_report_since`, `age_pulses` 5.
//!   `before_the_game.contacts` still has it as given.
//! - `claimed`: every claim is of a fight at sea, with no `taken_for`;
//!   `totals.sunk` is the sum of their `claimed_sunk`, and `hits`,
//!   `shot_down` and `on_ground` are 0.
//! - `enemy_has_done` is day 1, with the `fights` and `attacks_on_ships`
//!   of the briefing after it and no raid.
//!
//! ## D
//!
//! The file tells the Allied side, before day 1, of waters 3 hexes round
//! (5,0); the game is 3 days.
//!
//! - Allied, first morning: `parts` is `["enemy_submarines"]`, with
//!   `suspected_waters` one line, (5,0), `distance_hexes` 3, the file's
//!   words, and no `reports`. The waters are not in `before_the_game`:
//!   that part is contacts and estimates of men.
//! - Allied, over: `enemy_submarines.reports` is the reports
//!   `list_contacts` holds then, and the report dropped on day 3 is not
//!   among them or anywhere else in the answer. `enemy_has_done` is day 2
//!   then day 1, each with one line of `submarine_attacks`, against ship
//!   1 at (5,0) in the morning, and no `attacks_on_ships`.
//! - Japanese, over: `claimed.days` is day 2 then day 1, one claim each,
//!   `from_submarine` true, with `taken_for`; `totals.hits` and
//!   `totals.sunk` are the sums of the two and the other two are 0.

// The tests below are written to the statements above. The architect's doc
// comment is kept as written, list continuations and all.
#![allow(clippy::doc_lazy_continuation)]

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::sync::OnceLock;

use serde_json::{json, Value};
use sim_agent::{Post, Seat, Session, TOOLS, VERSION, WATCHER_TOOLS};
use sim_core::ids::{BaseId, TaskForceId};
use sim_core::pictures::CommandRef;
use sim_core::state::Side;
use support::check::valid;
use support::play::{play_day, session_of};
use support::{load, load_text, scenario_text, schemas};

#[derive(Clone, Copy)]
struct Tag {
    file: &'static str,
    seed: u64,
}

const A: Tag = Tag {
    file: "24_shells_and_estimates.toml",
    seed: 1942,
};
const B: Tag = Tag {
    file: "05b_strike.toml",
    seed: 11,
};
const C: Tag = Tag {
    file: "04_savo_island.toml",
    seed: 1942,
};
const D: Tag = Tag {
    file: "table-submarines/128_a_submarine_in_the_way.toml",
    seed: 6,
};
const E: Tag = Tag {
    file: "62_reports_between_commands.toml",
    seed: 1942,
};

const SIDES: [Side; 2] = [Side::Allied, Side::Japanese];

const ORDER: [&str; 8] = [
    "seen",
    "lost_sight_of",
    "claimed",
    "enemy_has_done",
    "ashore",
    "enemy_bases",
    "enemy_submarines",
    "before_the_game",
];

fn ask(session: &mut Session, side: Side, tool: &str, args: Value) -> Value {
    session
        .call(side, tool, &args)
        .unwrap_or_else(|e| panic!("{tool} {args}: {e}"))
}

/// Plays the scenario text with both seats a player's, each giving the
/// scenario's own orders, and calls `at` at the start of every day and once
/// more when the game is over, with the number of days resolved.
fn drive_text(text: &str, tag: Tag, mut at: impl FnMut(&mut Session, u32)) -> Session {
    let scenario = load_text(text, tag.file, tag.seed);
    let (mut session, scripts) = session_of(scenario, &SIDES);
    loop {
        let day = session.day();
        at(&mut session, day);
        if day >= session.days() {
            return session;
        }
        for side in SIDES {
            let orders = scripts[&side].get(&(day + 1)).cloned().unwrap_or_default();
            let (refused, ended) = play_day(&mut session, side, &orders);
            assert!(refused.is_empty(), "{refused:?}");
            assert!(ended.get("resolved").is_some(), "{ended}");
        }
    }
}

fn drive(tag: Tag, at: impl FnMut(&mut Session, u32)) -> Session {
    drive_text(&scenario_text(tag.file), tag, at)
}

/// What each seat was told at one moment: the morning after `day` days.
struct Morning {
    day: u32,
    intel: BTreeMap<Side, Value>,
    contacts: BTreeMap<Side, Value>,
    bases: BTreeMap<Side, Value>,
    briefing: BTreeMap<Side, Value>,
}

struct Run {
    days: u32,
    mornings: Vec<Morning>,
}

fn record(session: &mut Session, day: u32) -> Morning {
    let mut morning = Morning {
        day,
        intel: BTreeMap::new(),
        contacts: BTreeMap::new(),
        bases: BTreeMap::new(),
        briefing: BTreeMap::new(),
    };
    for side in SIDES {
        let intel = ask(session, side, "intelligence", json!({}));
        valid(
            &schemas(),
            "intelligence.schema.json",
            &intel,
            "intelligence",
        );
        morning.intel.insert(side, intel);
        morning
            .contacts
            .insert(side, ask(session, side, "list_contacts", json!({})));
        morning
            .bases
            .insert(side, ask(session, side, "list_bases", json!({})));
        morning.briefing.insert(
            side,
            ask(session, side, "get_briefing", json!({"text": false})),
        );
    }
    morning
}

fn play(tag: Tag) -> Run {
    let mut mornings = Vec::new();
    let session = drive(tag, |session, day| mornings.push(record(session, day)));
    Run {
        days: session.days(),
        mornings,
    }
}

fn run_of(tag: Tag) -> &'static Run {
    static RUNS: [OnceLock<Run>; 4] = [
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
        OnceLock::new(),
    ];
    let index = match tag.file {
        f if f == A.file => 0,
        f if f == B.file => 1,
        f if f == C.file => 2,
        f if f == D.file => 3,
        other => panic!("{other} is not one of the four"),
    };
    RUNS[index].get_or_init(|| play(tag))
}

// ---------------------------------------------------------------------
// Reading what the tools said.
// ---------------------------------------------------------------------

fn list<'a>(v: &'a Value, key: &str) -> &'a [Value] {
    match v.get(key) {
        Some(Value::Array(a)) => a,
        _ => &[],
    }
}

fn has_key(v: &Value, key: &str) -> bool {
    v.get(key).is_some()
}

/// The number of a handle written `word:number`.
fn number_of(handle: &Value) -> u64 {
    handle["handle"]
        .as_str()
        .and_then(|h| h.rsplit(':').next())
        .and_then(|n| n.parse().ok())
        .unwrap_or_else(|| panic!("a handle with a number: {handle}"))
}

fn names_of(answer: &Value) -> Vec<&str> {
    list(answer, "parts")
        .iter()
        .map(|p| p.as_str().expect("a name"))
        .collect()
}

fn pulse_rank(name: &Value) -> u8 {
    match name.as_str() {
        None => 0,
        Some("night") => 1,
        Some("morning") => 2,
        Some("afternoon") => 3,
        Some(other) => panic!("not a pulse: {other}"),
    }
}

/// The briefing of the morning after `k` days says what happened on day `k`.
fn told_on_day(run: &Run, side: Side, k: u32) -> &Value {
    &run.mornings[k as usize].briefing[&side]
}

/// Every (run, tag, side, morning) of the four games.
fn every_morning(mut f: impl FnMut(Tag, &Run, Side, &Morning)) {
    for tag in [A, B, C, D] {
        let run = run_of(tag);
        for morning in &run.mornings {
            for side in SIDES {
                f(tag, run, side, morning);
            }
        }
    }
}

fn the_over(run: &Run) -> &Morning {
    run.mornings.last().expect("a morning")
}

// ---------------------------------------------------------------------
// Statement 0.
// ---------------------------------------------------------------------

#[test]
fn statement_0_the_scenarios_do_what_they_were_chosen_for() {
    // A, Allied: a contact made by the bombardment, refreshed, dropped for
    // its age; its base shelled on two nights; an estimate ashore made; an
    // enemy base found.
    let a = run_of(A);
    assert_eq!(a.days, 4);
    let day = |k: u32, side: Side| told_on_day(a, side, k);
    assert_eq!(list(&day(1, Side::Allied)["contacts"], "new").len(), 1);
    assert_eq!(
        list(&day(2, Side::Allied)["contacts"], "refreshed").len(),
        1
    );
    let dropped = list(&day(3, Side::Allied)["contacts"], "dropped");
    assert_eq!(dropped.len(), 1, "{dropped:?}");
    assert_eq!(dropped[0]["age_pulses"], 5);
    for k in [1, 2] {
        let b = list(&day(k, Side::Allied)["action"], "bombardments_against");
        assert_eq!(b.len(), 1, "day {k}");
        assert_eq!(b[0]["base"]["handle"], "base:1");
    }
    let ashore = &day(1, Side::Allied)["enemy_ashore"];
    assert_eq!(list(ashore, "estimates").len(), 1);
    assert_eq!(list(ashore, "bases_found_yesterday").len(), 1);
    // A, Japanese: two claims from the sea on the nights of days 1 and 2,
    // and none after; the last ground report is of day 2.
    for k in [1, 2, 3, 4] {
        let claims = list(&day(k, Side::Japanese)["losses"], "claims");
        assert_eq!(claims.len(), usize::from(k <= 2), "day {k}");
        for claim in claims {
            assert_eq!(claim["from_bombardment"], true);
            assert_eq!(claim["from_air"], false);
        }
    }
    let last = &the_over(a).briefing[&Side::Japanese]["enemy_ashore"];
    assert_eq!(list(last, "estimates")[0]["day"], 2);
    assert_eq!(list(last, "estimates")[0]["came_from"], "men in contact");

    // B, Allied: a contact made, struck with a claim that says what the
    // target was taken for, refreshed, then a strike that finds nothing.
    let b = run_of(B);
    assert_eq!(b.days, 1);
    let day = told_on_day(b, Side::Allied, 1);
    assert_eq!(list(&day["contacts"], "not_there").len(), 1);
    assert_eq!(list(&day["losses"], "claims").len(), 1);
    assert_eq!(day["losses"]["claims"][0]["from_air"], true);
    assert!(day["losses"]["claims"][0]["taken_for"].is_string());
    assert!(!list(&day["contacts"], "refreshed").is_empty());

    // C, Allied: a contact given at the start, 8 ships taken for cruisers
    // at (-1,-9), dropped with no report since; another made in the night;
    // claims of a fight at sea and a surface action met.
    let c = run_of(C);
    assert_eq!(c.days, 1);
    let start = &c.mornings[0].contacts[&Side::Allied];
    assert_eq!(list(start, "contacts").len(), 1);
    assert_eq!(start["contacts"][0]["ship_count"], 8);
    assert_eq!(start["contacts"][0]["taken_for"], "cruiser");
    assert_eq!(start["contacts"][0]["age_pulses"], 2);
    let day = told_on_day(c, Side::Allied, 1);
    let dropped = list(&day["contacts"], "dropped");
    assert_eq!(dropped.len(), 1);
    assert_eq!(dropped[0]["contact"]["handle"], "contact:1");
    assert_eq!(dropped[0]["age_pulses"], 5);
    assert!(list(&day["contacts"], "new")
        .iter()
        .all(|n| n["contact"]["handle"] != "contact:1"));
    assert!(!list(&day["contacts"], "new").is_empty());
    assert!(!list(&day["losses"], "claims").is_empty());
    assert!(list(&day["losses"], "claims")
        .iter()
        .all(|claim| claim["from_air"] == false));
    assert!(!list(&day["action"], "fights").is_empty());

    // D, Allied: waters told before day 1, two reports of a submarine of
    // which one is dropped on day 3, a carrier torpedoed.
    let d = run_of(D);
    assert_eq!(d.days, 3);
    let first = &told_on_day(d, Side::Allied, 0);
    assert_eq!(list(&first["submarines"], "suspected_waters").len(), 1);
    assert_eq!(
        list(&told_on_day(d, Side::Allied, 1)["submarines"], "reports").len(),
        1
    );
    assert_eq!(
        list(&told_on_day(d, Side::Allied, 2)["submarines"], "reports").len(),
        2
    );
    assert_eq!(
        list(&told_on_day(d, Side::Allied, 3)["submarines"], "reports").len(),
        1
    );
    assert_eq!(
        list(&told_on_day(d, Side::Allied, 1)["losses"], "ships_damaged").len(),
        1
    );
    // D, Japanese: two claims of a submarine, on the first two days.
    for k in [1, 2] {
        let claims = list(&told_on_day(d, Side::Japanese, k)["losses"], "claims");
        assert_eq!(claims.len(), 1, "day {k}");
        assert_eq!(claims[0]["from_submarine"], true);
    }
}

// ---------------------------------------------------------------------
// Statement 1.
// ---------------------------------------------------------------------

fn holds_something(part: &Value) -> bool {
    match part {
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => o
            .values()
            .any(|v| matches!(v, Value::Array(a) if !a.is_empty())),
        _ => true,
    }
}

#[test]
fn statement_1_the_parts_are_in_order_and_an_empty_part_is_left_out() {
    let mut seen_a_part = BTreeSet::new();
    every_morning(|tag, _, side, morning| {
        let answer = &morning.intel[&side];
        let parts = names_of(answer);
        let order: Vec<&str> = ORDER
            .iter()
            .copied()
            .filter(|n| parts.contains(n))
            .collect();
        assert_eq!(parts, order, "{} {side:?} {}", tag.file, morning.day);
        for name in ORDER {
            assert_eq!(
                has_key(answer, name),
                parts.contains(&name),
                "{} {side:?} {} {name}",
                tag.file,
                morning.day
            );
            if parts.contains(&name) {
                assert!(holds_something(&answer[name]), "{name} is empty: {answer}");
                seen_a_part.insert(name);
            }
        }
        // Nothing but the answer's own keys.
        for key in answer.as_object().expect("an object").keys() {
            assert!(
                ["format", "version", "kind", "parts"].contains(&key.as_str())
                    || ORDER.contains(&key.as_str()),
                "{key}"
            );
        }
        assert_eq!(answer["kind"], "intelligence");
        assert_eq!(answer["format"], "flagplot-agent");
        assert_eq!(answer["version"], 52);
    });
    // Between them the four games give every part.
    assert_eq!(seen_a_part.len(), ORDER.len(), "{seen_a_part:?}");
    // The first morning of A at the Allied seat: nothing is held.
    let first = &run_of(A).mornings[0].intel[&Side::Allied];
    assert_eq!(names_of(first), Vec::<&str>::new());
    for name in ORDER {
        assert!(!has_key(first, name));
    }
}

// ---------------------------------------------------------------------
// Statement 2.
// ---------------------------------------------------------------------

#[test]
fn statement_2_seen_is_the_contacts_held_newest_first() {
    let mut most = 0;
    every_morning(|tag, _, side, morning| {
        let answer = &morning.intel[&side];
        let mut held: Vec<Value> = list(&morning.contacts[&side], "contacts").to_vec();
        held.sort_by(|a, b| {
            a["age_pulses"]
                .as_u64()
                .cmp(&b["age_pulses"].as_u64())
                .then(number_of(&b["contact"]).cmp(&number_of(&a["contact"])))
        });
        let seen = list(answer, "seen");
        assert_eq!(
            seen,
            held.as_slice(),
            "{} {side:?} {}",
            tag.file,
            morning.day
        );
        assert_eq!(has_key(answer, "seen"), !held.is_empty());
        for line in seen {
            let keys: BTreeSet<&str> = line
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(
                keys,
                BTreeSet::from(["contact", "place", "ship_count", "taken_for", "age_pulses"])
            );
        }
        most = most.max(seen.len());
    });
    assert!(most >= 1);
}

#[test]
fn statement_2_seen_is_in_order_of_age_and_then_number_with_three_contacts_held() {
    // None of the four games holds two contacts at once, so scenario C's
    // text is given two more starting contacts, of ages 0 and 2, far from
    // the first: three contacts, two of one age.
    let text = scenario_text(C.file).replacen(
        "# Japanese: into the hex of Savo Island",
        "[[contact]]\nside = \"allied\"\nq = 6\nr = -4\nship_count = 2\nkind = \"carrier\"\nage_pulses = 0\n\n[[contact]]\nside = \"allied\"\nq = -6\nr = -9\nship_count = 3\nkind = \"destroyer\"\nage_pulses = 2\n\n# Japanese: into the hex of Savo Island",
        1,
    );
    let mut checked = 0;
    drive_text(&text, C, |session, day| {
        if day != 0 {
            return;
        }
        let held = ask(session, Side::Allied, "list_contacts", json!({}));
        let mut expected: Vec<Value> = list(&held, "contacts").to_vec();
        assert_eq!(expected.len(), 3, "{held}");
        expected.sort_by(|a, b| {
            a["age_pulses"]
                .as_u64()
                .cmp(&b["age_pulses"].as_u64())
                .then(number_of(&b["contact"]).cmp(&number_of(&a["contact"])))
        });
        let ages: Vec<u64> = expected
            .iter()
            .map(|c| c["age_pulses"].as_u64().unwrap())
            .collect();
        assert_eq!(ages, vec![0, 2, 2]);
        let answer = ask(session, Side::Allied, "intelligence", json!({}));
        assert_eq!(list(&answer, "seen"), expected.as_slice());
        // Of the two of age 2 the higher number comes first.
        assert!(
            number_of(&answer["seen"][1]["contact"]) > number_of(&answer["seen"][2]["contact"])
        );
        checked += 1;
    });
    assert_eq!(checked, 1);
}

// ---------------------------------------------------------------------
// Statement 3.
// ---------------------------------------------------------------------

/// The lines of `briefing[part][key]` of the briefings of days 1 to `upto`,
/// each with the day it says it of.
fn briefed<'a>(
    run: &'a Run,
    side: Side,
    upto: u32,
    part: &str,
    key: &str,
) -> Vec<(u32, &'a Value)> {
    (1..=upto)
        .flat_map(|k| {
            list(&told_on_day(run, side, k)[part], key)
                .iter()
                .map(move |line| (k, line))
        })
        .collect()
}

#[test]
fn statement_3_lost_sight_of_says_the_last_report_and_which_of_two_reasons() {
    let mut lines_seen = 0;
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let lost = list(answer, "lost_sight_of");
        let upto = morning.day;
        let drops = briefed(run, side, upto, "contacts", "dropped");
        let nothing = briefed(run, side, upto, "contacts", "not_there");
        // One line for each contact dropped and none other.
        assert_eq!(
            lost.len(),
            drops.len() + nothing.len(),
            "{} {side:?} {}",
            tag.file,
            morning.day
        );
        let held: Vec<&Value> = list(&morning.contacts[&side], "contacts")
            .iter()
            .map(|c| &c["contact"])
            .collect();
        let reports: Vec<(u32, &Value)> = briefed(run, side, upto, "contacts", "new")
            .into_iter()
            .chain(briefed(run, side, upto, "contacts", "refreshed"))
            .collect();
        for line in lost {
            lines_seen += 1;
            let contact = &line["contact"];
            assert!(!held.contains(&contact), "a contact held is not lost");
            let keys: BTreeSet<&str> = line
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, BTreeSet::from(["contact", "last_report", "dropped"]));
            // Dropped, with which of the two things the side was told.
            let dropped = &line["dropped"];
            match dropped["why"].as_str().expect("why") {
                "no_report_since" => {
                    let (k, told) = drops
                        .iter()
                        .find(|(_, d)| &d["contact"] == contact)
                        .expect("a dropped line of the briefings");
                    assert_eq!(dropped["day"], *k);
                    assert_eq!(dropped["age_pulses"], told["age_pulses"]);
                    assert_eq!(dropped["event"], told["event"]);
                    assert!(!has_key(dropped, "place"));
                }
                "strike_found_nothing" => {
                    let (k, told) = nothing
                        .iter()
                        .find(|(_, d)| &d["contact"] == contact)
                        .expect("a not_there line of the briefings");
                    assert_eq!(dropped["day"], *k);
                    assert_eq!(dropped["place"], told["place"]);
                    assert_eq!(dropped["event"], told["event"]);
                    assert!(!has_key(dropped, "age_pulses"));
                }
                other => panic!("{other} is not one of the two"),
            }
            assert_eq!(
                dropped
                    .as_object()
                    .unwrap()
                    .keys()
                    .filter(|k| *k == "why")
                    .count(),
                1
            );
            // The last report that made or refreshed it, field for field.
            let last = &line["last_report"];
            let made: Vec<&(u32, &Value)> = reports
                .iter()
                .filter(|(_, r)| &r["contact"] == contact)
                .collect();
            if last["day"] == 0 {
                assert!(last["pulse"].is_null());
                assert!(
                    made.is_empty(),
                    "a contact given at the start and never refreshed"
                );
                // As the scenario gave it.
                let at_start = &run.mornings[0].contacts[&side]["contacts"];
                let given = at_start
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|c| &c["contact"] == contact)
                    .expect("given at the start");
                assert_eq!(last["place"], given["place"]);
                assert_eq!(last["ship_count"], given["ship_count"]);
                assert_eq!(last["taken_for"], given["taken_for"]);
            } else {
                let (k, told) = made
                    .iter()
                    .copied()
                    .max_by_key(|(_, r)| r["event"].as_u64())
                    .expect("a report");
                assert_eq!(last["day"], *k);
                assert_eq!(last["event"], told["event"]);
                assert_eq!(last["place"], told["place"]);
                assert_eq!(last["ship_count"], told["ship_count"]);
                assert_eq!(last["taken_for"], told["taken_for"]);
                assert!(last["pulse"].is_string());
            }
            let keys: BTreeSet<&str> = last
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(
                keys,
                BTreeSet::from(["day", "pulse", "place", "ship_count", "taken_for", "event"])
            );
        }
        // The latest dropped first, and of one pulse the higher number first.
        let order: Vec<(u64, u8, u64)> = lost
            .iter()
            .map(|l| {
                (
                    l["dropped"]["day"].as_u64().unwrap(),
                    pulse_rank(&l["dropped"]["pulse"]),
                    number_of(&l["contact"]),
                )
            })
            .collect();
        assert!(order.windows(2).all(|w| w[0] >= w[1]), "{order:?}");
        // A contact dropped and not held again is in no line of `seen`.
        for line in lost {
            assert!(list(answer, "seen")
                .iter()
                .all(|s| s["contact"] != line["contact"]));
        }
    });
    assert!(lines_seen >= 3, "{lines_seen}");

    // The hand calculation. A, Allied: dropped on day 3, in the afternoon,
    // at age 5 with its last report in the night of day 2.
    let a = run_of(A);
    for morning in &a.mornings[3..] {
        let line = &morning.intel[&Side::Allied]["lost_sight_of"][0];
        assert_eq!(
            list(&morning.intel[&Side::Allied], "lost_sight_of").len(),
            1
        );
        assert_eq!(line["last_report"]["day"], 2);
        assert_eq!(line["last_report"]["pulse"], "night");
        assert_eq!(line["dropped"]["day"], 3);
        assert_eq!(line["dropped"]["pulse"], "afternoon");
        assert_eq!(line["dropped"]["why"], "no_report_since");
        assert_eq!(line["dropped"]["age_pulses"], 5);
    }
    assert!(!has_key(
        &a.mornings[2].intel[&Side::Allied],
        "lost_sight_of"
    ));
    // B: the strike found nothing, in the afternoon of day 1, at the place
    // of the last report.
    let line = &the_over(run_of(B)).intel[&Side::Allied]["lost_sight_of"][0];
    assert_eq!(line["last_report"]["day"], 1);
    assert_eq!(line["last_report"]["pulse"], "afternoon");
    assert_eq!(line["dropped"]["day"], 1);
    assert_eq!(line["dropped"]["pulse"], "afternoon");
    assert_eq!(line["dropped"]["why"], "strike_found_nothing");
    assert_eq!(line["dropped"]["place"], line["last_report"]["place"]);
    // C: given at the start, 8 ships taken for cruisers at (-1,-9), dropped
    // in the afternoon of day 1 at age 5.
    let line = &the_over(run_of(C)).intel[&Side::Allied]["lost_sight_of"][0];
    assert_eq!(line["last_report"]["day"], 0);
    assert!(line["last_report"]["pulse"].is_null());
    assert_eq!(line["last_report"]["ship_count"], 8);
    assert_eq!(line["last_report"]["taken_for"], "cruiser");
    assert_eq!(line["last_report"]["place"]["q"], -1);
    assert_eq!(line["last_report"]["place"]["r"], -9);
    assert_eq!(line["dropped"]["day"], 1);
    assert_eq!(line["dropped"]["pulse"], "afternoon");
    assert_eq!(line["dropped"]["age_pulses"], 5);
}

// ---------------------------------------------------------------------
// Statement 4.
// ---------------------------------------------------------------------

fn strip_pulse(claim: &Value) -> Value {
    let mut claim = claim.clone();
    claim.as_object_mut().unwrap().remove("pulse");
    claim
}

#[test]
fn statement_4_claimed_is_every_claim_by_day_with_four_totals() {
    let mut claims_seen = 0;
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        // The claims of each day of the briefings, in the order told.
        let mut expected_days: Vec<(u32, Vec<Value>)> = Vec::new();
        for (k, claim) in briefed(run, side, morning.day, "losses", "claims") {
            match expected_days.last_mut() {
                Some((day, claims)) if *day == k => claims.push(claim.clone()),
                _ => expected_days.push((k, vec![claim.clone()])),
            }
        }
        expected_days.reverse();
        if expected_days.is_empty() {
            assert!(!has_key(answer, "claimed"), "{label}");
            return;
        }
        let claimed = &answer["claimed"];
        let days = list(claimed, "days");
        assert_eq!(days.len(), expected_days.len(), "{label}");
        let (mut sunk, mut hits, mut down, mut ground) = (0, 0, 0, 0);
        for (day, (k, expected)) in days.iter().zip(&expected_days) {
            assert_eq!(day["day"], *k, "{label}");
            let claims = list(day, "claims");
            assert!(!claims.is_empty());
            assert_eq!(claims.len(), expected.len(), "{label}");
            for (mine, theirs) in claims.iter().zip(expected) {
                claims_seen += 1;
                assert!(mine["pulse"].is_string());
                assert_eq!(strip_pulse(mine), *theirs, "{label}: field for field");
                assert!(!has_key(mine, "contact"));
                sunk += mine["claimed_sunk"].as_u64().unwrap();
                hits += mine["claimed_hits"].as_u64().unwrap();
                down += mine["claimed_shot_down"].as_u64().unwrap();
                ground += mine["claimed_on_ground"].as_u64().unwrap();
            }
        }
        let latest_first: Vec<u64> = days.iter().map(|d| d["day"].as_u64().unwrap()).collect();
        assert!(
            latest_first.windows(2).all(|w| w[0] > w[1]),
            "{latest_first:?}"
        );
        let totals = &claimed["totals"];
        let keys: BTreeSet<&str> = totals
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(
            keys,
            BTreeSet::from(["sunk", "hits", "shot_down", "on_ground"])
        );
        assert_eq!(totals["sunk"], sunk, "{label}");
        assert_eq!(totals["hits"], hits, "{label}");
        assert_eq!(totals["shot_down"], down, "{label}");
        assert_eq!(totals["on_ground"], ground, "{label}");
        // No handle of a contact is in any value of the claims.
        assert!(!claimed.to_string().contains("contact"), "{label}");
        let claim_keys: BTreeSet<&str> = ["claimed", "days", "totals"].into();
        assert!(claimed
            .as_object()
            .unwrap()
            .keys()
            .all(|k| claim_keys.contains(k.as_str())));
    });
    assert!(claims_seen >= 6, "{claims_seen}");

    // The hand calculation. A, Japanese: two claims from the sea, at (0,0),
    // in the night, of the days 2 and 1; the aircraft on the ground are the
    // sum of the two.
    let over = &the_over(run_of(A)).intel[&Side::Japanese]["claimed"];
    let days = list(over, "days");
    assert_eq!(days.len(), 2);
    assert_eq!(days[0]["day"], 2);
    assert_eq!(days[1]["day"], 1);
    let (c2, c1) = (&days[0]["claims"][0], &days[1]["claims"][0]);
    for claim in [c1, c2] {
        assert_eq!(claim["place"]["q"], 0);
        assert_eq!(claim["place"]["r"], 0);
        assert_eq!(claim["pulse"], "night");
        assert_eq!(claim["from_bombardment"], true);
        assert_eq!(claim["from_air"], false);
        assert!(claim["taken_for"].is_null());
    }
    assert_eq!(
        over["totals"],
        json!({
            "sunk": 0,
            "hits": 0,
            "shot_down": 0,
            "on_ground": c1["claimed_on_ground"].as_u64().unwrap() + c2["claimed_on_ground"].as_u64().unwrap(),
        })
    );
    // B: one claim of the morning from the air, with what it was taken for.
    let over = &the_over(run_of(B)).intel[&Side::Allied]["claimed"];
    assert_eq!(list(over, "days").len(), 1);
    let claim = &over["days"][0]["claims"][0];
    assert_eq!(over["days"][0]["claims"].as_array().unwrap().len(), 1);
    assert_eq!(claim["pulse"], "morning");
    assert_eq!(claim["from_air"], true);
    assert!(claim["taken_for"].is_string());
    assert_eq!(over["totals"]["hits"], claim["claimed_hits"]);
    assert_eq!(over["totals"]["sunk"], claim["claimed_sunk"]);
    assert_eq!(over["totals"]["shot_down"], 0);
    assert_eq!(over["totals"]["on_ground"], 0);
    // C: claims of a fight at sea, none taken for anything; only ships sunk.
    let over = &the_over(run_of(C)).intel[&Side::Allied]["claimed"];
    let mut sunk = 0;
    for claim in over["days"][0]["claims"].as_array().unwrap() {
        assert!(claim["taken_for"].is_null());
        assert_eq!(claim["from_air"], false);
        sunk += claim["claimed_sunk"].as_u64().unwrap();
    }
    assert_eq!(
        over["totals"],
        json!({"sunk": sunk, "hits": 0, "shot_down": 0, "on_ground": 0})
    );
    // D, Japanese: two claims of a submarine with what she took for her
    // target, days 2 then 1.
    let over = &the_over(run_of(D)).intel[&Side::Japanese]["claimed"];
    let days = list(over, "days");
    assert_eq!(
        (days[0]["day"].as_u64(), days[1]["day"].as_u64()),
        (Some(2), Some(1))
    );
    let (c2, c1) = (&days[0]["claims"][0], &days[1]["claims"][0]);
    for claim in [c1, c2] {
        assert_eq!(claim["from_submarine"], true);
        assert!(claim["taken_for"].is_string());
    }
    let hits = c1["claimed_hits"].as_u64().unwrap() + c2["claimed_hits"].as_u64().unwrap();
    let sunk = c1["claimed_sunk"].as_u64().unwrap() + c2["claimed_sunk"].as_u64().unwrap();
    assert_eq!(
        over["totals"],
        json!({"sunk": sunk, "hits": hits, "shot_down": 0, "on_ground": 0})
    );
}

// ---------------------------------------------------------------------
// Statement 5.
// ---------------------------------------------------------------------

#[test]
fn statement_5_what_the_enemy_has_done_by_day() {
    let mut days_seen = 0;
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        // Each day of the briefings that says something was done, latest
        // first.
        let mut expected: Vec<Value> = Vec::new();
        for k in (1..=morning.day).rev() {
            let action = &told_on_day(run, side, k)["action"];
            let mut day = serde_json::Map::new();
            for (mine, theirs) in [
                ("raids", "raids_on_bases"),
                ("bombardments", "bombardments_against"),
                ("attacks_on_ships", "attacks_on_ships"),
                ("fights", "fights"),
            ] {
                if !list(action, theirs).is_empty() {
                    day.insert(mine.to_string(), action[theirs].clone());
                }
            }
            if !day.is_empty() {
                day.insert("day".to_string(), json!(k));
                expected.push(Value::Object(day));
            }
        }
        // The four lists the briefing gives; a submarine's attacks are the
        // tool's own, read from the side's events (the ruling on the build).
        let mine: Vec<Value> = list(answer, "enemy_has_done")
            .iter()
            .filter_map(|day| {
                let mut day = day.clone();
                day.as_object_mut().unwrap().remove("submarine_attacks");
                (day.as_object().unwrap().len() > 1).then_some(day)
            })
            .collect();
        assert_eq!(mine, expected, "{label}");
        let has_attacks = list(answer, "enemy_has_done")
            .iter()
            .any(|d| has_key(d, "submarine_attacks"));
        assert!(
            has_key(answer, "enemy_has_done") == (!expected.is_empty() || has_attacks),
            "{label}"
        );
        days_seen += expected.len();
        for day in &expected {
            // Four things and a submarine's attacks, and no other; a fight
            // ashore is not among them.
            for key in day.as_object().unwrap().keys() {
                assert!(
                    ["day", "raids", "bombardments", "attacks_on_ships", "fights"]
                        .contains(&key.as_str())
                );
            }
            // A fight names the side's own force and never the enemy's.
            for fight in list(day, "fights") {
                let force = fight["force"]["handle"].as_str().unwrap();
                assert!(force.starts_with("task_force:"), "{force}");
                assert!(!has_key(fight, "enemy") && !has_key(fight, "enemy_force"));
            }
        }
    });
    assert!(days_seen >= 5, "{days_seen}");

    // A, Allied: day 1 and day 2 hold one shelling of base 1 each, and no
    // raid, attack on a ship or fight.
    let over = &the_over(run_of(A)).intel[&Side::Allied];
    let done = list(over, "enemy_has_done");
    assert_eq!(done.len(), 2);
    assert_eq!(
        (done[0]["day"].as_u64(), done[1]["day"].as_u64()),
        (Some(2), Some(1))
    );
    for day in done {
        assert_eq!(day.as_object().unwrap().len(), 2);
        assert_eq!(day["bombardments"].as_array().unwrap().len(), 1);
        assert_eq!(day["bombardments"][0]["base"]["handle"], "base:1");
    }
    // C, Allied: day 1 holds the fights and the attacks on ships, no raid.
    let done = &the_over(run_of(C)).intel[&Side::Allied]["enemy_has_done"];
    assert_eq!(done.as_array().unwrap().len(), 1);
    assert!(!list(&done[0], "fights").is_empty());
    assert!(!list(&done[0], "attacks_on_ships").is_empty());
    assert!(!has_key(&done[0], "raids"));
}

#[test]
fn statement_5_d_a_submarines_attacks_are_in_what_the_enemy_has_done() {
    // D, Allied, over: day 2 then day 1, each with one line of
    // `submarine_attacks`, against ship 1 at (5,0) in the morning, and no
    // `attacks_on_ships`.
    let over = &the_over(run_of(D)).intel[&Side::Allied];
    let done = list(over, "enemy_has_done");
    assert_eq!(done.len(), 2, "{over}");
    assert_eq!(
        (done[0]["day"].as_u64(), done[1]["day"].as_u64()),
        (Some(2), Some(1))
    );
    for day in done {
        assert!(!has_key(day, "attacks_on_ships"));
        let attacks = list(day, "submarine_attacks");
        assert_eq!(attacks.len(), 1);
        let line = &attacks[0];
        assert_eq!(line["against"]["handle"], "ship:1");
        assert_eq!(line["place"]["q"], 5);
        assert_eq!(line["place"]["r"], 0);
        assert_eq!(line["pulse"], "morning");
        let keys: BTreeSet<&str> = line
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, BTreeSet::from(["against", "place", "pulse", "event"]));
    }
}

// ---------------------------------------------------------------------
// Statements 6 to 9.
// ---------------------------------------------------------------------

#[test]
fn statement_6_ashore_is_the_estimates_held_and_those_found_empty() {
    let mut empties = 0;
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        let held = list(&morning.contacts[&side], "ground_estimates");
        let mut expected_empty: Vec<Value> = Vec::new();
        for k in (1..=morning.day).rev() {
            for line in list(
                &told_on_day(run, side, k)["enemy_ashore"],
                "found_empty_yesterday",
            ) {
                let mut line = line.clone();
                line.as_object_mut().unwrap().insert("day".into(), json!(k));
                expected_empty.push(line);
            }
        }
        if held.is_empty() && expected_empty.is_empty() {
            assert!(!has_key(answer, "ashore"), "{label}");
            return;
        }
        let ashore = &answer["ashore"];
        assert_eq!(list(ashore, "estimates"), held, "{label}");
        assert_eq!(has_key(ashore, "estimates"), !held.is_empty());
        assert_eq!(
            list(ashore, "found_empty"),
            expected_empty.as_slice(),
            "{label}"
        );
        assert_eq!(has_key(ashore, "found_empty"), !expected_empty.is_empty());
        empties += expected_empty.len();
    });
    assert!(empties >= 1);

    // A, Allied, over: nothing held ashore, the hex found empty on day 3
    // with the men and the day of the estimate cleared.
    let ashore = &the_over(run_of(A)).intel[&Side::Allied]["ashore"];
    assert!(!has_key(ashore, "estimates"));
    assert_eq!(ashore["found_empty"].as_array().unwrap().len(), 1);
    let line = &ashore["found_empty"][0];
    assert_eq!(line["day"], 3);
    assert_eq!(line["reported_day"], 2);
    assert_eq!(line["place"]["q"], 8);
    assert_eq!(line["place"]["r"], 0);
    // A, Japanese: first morning, the one opening estimate; over, the last
    // report's, of day 2, two days old.
    let first = &run_of(A).mornings[0].intel[&Side::Japanese]["ashore"]["estimates"];
    assert_eq!(first.as_array().unwrap().len(), 1);
    assert_eq!(first[0]["men"], 300);
    assert_eq!(first[0]["came_from"], "told");
    assert_eq!(first[0]["day"], 0);
    assert_eq!(first[0]["age_days"], 0);
    assert_eq!(first[0]["place"]["q"], 8);
    let over = &the_over(run_of(A)).intel[&Side::Japanese]["ashore"]["estimates"];
    assert_eq!(over.as_array().unwrap().len(), 1);
    assert_eq!(over[0]["day"], 2);
    assert_eq!(over[0]["came_from"], "men in contact");
    assert_eq!(over[0]["age_days"], 2);
}

#[test]
fn statement_7_enemy_bases_are_only_those_of_which_something_is_held() {
    let mut listed = 0;
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        let enemy = list(&morning.bases[&side], "enemy_bases");
        let estimates = list(&morning.contacts[&side], "ground_estimates");
        // The ground claims from the air the briefings gave, with the day.
        let mut ground: Vec<(u32, &Value)> = Vec::new();
        for (k, claim) in briefed(run, side, morning.day, "losses", "claims") {
            if claim["from_air"] == true && claim["claimed_on_ground"].as_u64().unwrap() > 0 {
                ground.push((k, claim));
            }
        }
        let mut expected: Vec<Value> = Vec::new();
        for base in enemy {
            let at = |p: &Value| {
                p["place"]["q"] == base["place"]["q"] && p["place"]["r"] == base["place"]["r"]
            };
            let estimate = estimates.iter().find(|e| at(e));
            let claims: Vec<&(u32, &Value)> = ground.iter().filter(|(_, c)| at(c)).collect();
            if estimate.is_none() && claims.is_empty() {
                continue;
            }
            let mut line = serde_json::Map::new();
            for key in ["base", "place", "held_by"] {
                line.insert(key.into(), base[key].clone());
            }
            if let Some(estimate) = estimate {
                line.insert("estimate".into(), estimate.clone());
            }
            if !claims.is_empty() {
                let sum: u64 = claims
                    .iter()
                    .map(|(_, c)| c["claimed_on_ground"].as_u64().unwrap())
                    .sum();
                line.insert("claimed_on_ground_from_air".into(), json!(sum));
                line.insert(
                    "claims".into(),
                    json!(claims
                        .iter()
                        .map(|(k, c)| json!({"day": k, "number": c["claimed_on_ground"]}))
                        .collect::<Vec<_>>()),
                );
            }
            expected.push(Value::Object(line));
        }
        assert_eq!(list(answer, "enemy_bases"), expected.as_slice(), "{label}");
        assert_eq!(
            has_key(answer, "enemy_bases"),
            !expected.is_empty(),
            "{label}"
        );
        listed += expected.len();
        // Each is the enemy's on the seat's map now.
        for line in &expected {
            assert_ne!(
                line["held_by"],
                json!(sim_agent::seat_name(side)),
                "{label}"
            );
        }
    });
    assert!(listed >= 6, "{listed}");

    // A, Japanese: base 2 only, with the estimate; base 1, at whose hex
    // aircraft are claimed destroyed on the ground from the sea, is not
    // listed. A, Allied after day 1: base 3, found that day.
    for morning in &run_of(A).mornings {
        let lines = list(&morning.intel[&Side::Japanese], "enemy_bases");
        assert!(lines.iter().all(|l| l["base"]["handle"] == "base:2"));
        assert!(lines
            .iter()
            .all(|l| !has_key(l, "claimed_on_ground_from_air")));
    }
    let over = &the_over(run_of(A)).intel[&Side::Japanese];
    assert_eq!(list(over, "enemy_bases").len(), 1);
    assert!(over["claimed"]["totals"]["on_ground"].as_u64().unwrap() > 0);
    let after_one = &run_of(A).mornings[1].intel[&Side::Allied];
    assert_eq!(after_one["enemy_bases"][0]["base"]["handle"], "base:3");
    assert_eq!(after_one["enemy_bases"][0]["place"]["q"], 8);
    assert_eq!(
        after_one["enemy_bases"][0]["estimate"],
        after_one["ashore"]["estimates"][0]
    );
    // A base found and then found empty holds nothing and is not listed.
    assert!(!has_key(
        &the_over(run_of(A)).intel[&Side::Allied],
        "enemy_bases"
    ));
    // The first morning: base 2 with the estimate told, and no base 1.
    let first = &run_of(A).mornings[0].intel[&Side::Japanese];
    assert_eq!(first["enemy_bases"].as_array().unwrap().len(), 1);
    assert_eq!(first["enemy_bases"][0]["base"]["handle"], "base:2");
    assert_eq!(first["enemy_bases"][0]["estimate"]["men"], 300);
}

#[test]
fn statement_8_enemy_submarines_are_the_reports_held_and_the_waters_as_told() {
    every_morning(|tag, _, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        let held = &morning.contacts[&side];
        let part = &answer["enemy_submarines"];
        let reports = list(held, "submarine_reports");
        let waters = list(held, "suspected_waters");
        assert_eq!(list(part, "reports"), reports, "{label}");
        assert_eq!(list(part, "suspected_waters"), waters, "{label}");
        assert_eq!(
            has_key(answer, "enemy_submarines"),
            !reports.is_empty() || !waters.is_empty()
        );
        assert_eq!(has_key(part, "reports"), !reports.is_empty());
        assert_eq!(has_key(part, "suspected_waters"), !waters.is_empty());
    });
    // D, Allied: the waters before day 1 and no report; over, the report
    // dropped on day 3 (the one of day 1, at (5,-1)) is nowhere.
    let first = &run_of(D).mornings[0].intel[&Side::Allied];
    assert_eq!(names_of(first), vec!["enemy_submarines"]);
    let waters = &first["enemy_submarines"]["suspected_waters"];
    assert_eq!(waters.as_array().unwrap().len(), 1);
    assert_eq!(waters[0]["place"]["q"], 5);
    assert_eq!(waters[0]["place"]["r"], 0);
    assert_eq!(waters[0]["distance_hexes"], 3);
    assert_eq!(
        waters[0]["words"],
        "Placeholder: the staff believes enemy submarines work in these waters."
    );
    assert!(!has_key(&first["enemy_submarines"], "reports"));
    assert!(!has_key(first, "before_the_game"));
    let over = &the_over(run_of(D)).intel[&Side::Allied];
    let reports = list(&over["enemy_submarines"], "reports");
    assert_eq!(reports.len(), 1);
    assert_eq!(reports[0]["day"], 2);
    assert!(!over.to_string().contains("\"q\":5,\"r\":-1"));
}

#[test]
fn statement_9_before_the_game_is_as_given_on_every_day() {
    every_morning(|tag, run, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        // The same on every day of the game, whatever was reported since.
        let first = &run.mornings[0].intel[&side];
        assert_eq!(
            answer.get("before_the_game"),
            first.get("before_the_game"),
            "{label}"
        );
        if let Some(part) = answer.get("before_the_game") {
            let text = part.to_string().to_lowercase();
            assert!(
                !text.contains("directive") && !text.contains("order"),
                "{label}"
            );
            for line in list(part, "contacts") {
                let keys: BTreeSet<&str> = line
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(String::as_str)
                    .collect();
                assert_eq!(
                    keys,
                    BTreeSet::from([
                        "contact",
                        "place",
                        "ship_count",
                        "taken_for",
                        "age_pulses",
                        "event"
                    ])
                );
            }
            for line in list(part, "estimates") {
                let keys: BTreeSet<&str> = line
                    .as_object()
                    .unwrap()
                    .keys()
                    .map(String::as_str)
                    .collect();
                assert_eq!(keys, BTreeSet::from(["place", "men", "came_from", "event"]));
                assert_eq!(line["came_from"], "told");
            }
        }
    });
    // C: the contact the file gives, as given, at the first morning and when
    // the game is over, though it has been dropped.
    let c = run_of(C);
    let given = &c.mornings[0].contacts[&Side::Allied]["contacts"][0];
    for morning in &c.mornings {
        let part = &morning.intel[&Side::Allied]["before_the_game"];
        assert_eq!(part["contacts"].as_array().unwrap().len(), 1);
        assert!(!has_key(part, "estimates"));
        let line = &part["contacts"][0];
        assert_eq!(line["contact"], given["contact"]);
        assert_eq!(line["place"], given["place"]);
        assert_eq!(line["ship_count"], 8);
        assert_eq!(line["taken_for"], "cruiser");
        assert_eq!(line["age_pulses"], 2);
        assert_eq!(line["event"], 1);
    }
    // A, Japanese: the opening estimate as told, though the one held is
    // another; the side was given no contact. A, Allied: nothing.
    for morning in &run_of(A).mornings {
        let part = &morning.intel[&Side::Japanese]["before_the_game"];
        assert!(!has_key(part, "contacts"));
        assert_eq!(part["estimates"].as_array().unwrap().len(), 1);
        assert_eq!(part["estimates"][0]["men"], 300);
        assert_eq!(part["estimates"][0]["came_from"], "told");
        assert_eq!(part["estimates"][0]["place"]["q"], 8);
        assert!(!has_key(&morning.intel[&Side::Allied], "before_the_game"));
    }
    let over = &the_over(run_of(A)).intel[&Side::Japanese];
    assert_ne!(over["ashore"]["estimates"][0]["men"], 300);
    // D: the waters told before day 1 are not here.
    for morning in &run_of(D).mornings {
        assert!(!has_key(&morning.intel[&Side::Allied], "before_the_game"));
    }
}

// ---------------------------------------------------------------------
// Statements 10 and 11.
// ---------------------------------------------------------------------

/// Every key and every scalar of a value, with the keys above it.
type Visit<'f> = dyn FnMut(&[String], Option<&str>, &Value) + 'f;

fn walk(value: &Value, path: &mut Vec<String>, f: &mut Visit<'_>) {
    match value {
        Value::Object(map) => {
            for (key, inner) in map {
                path.push(key.clone());
                f(path, Some(key), inner);
                walk(inner, path, f);
                path.pop();
            }
        }
        Value::Array(items) => {
            for inner in items {
                walk(inner, path, f);
            }
        }
        _ => {}
    }
}

fn words_of(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_lowercase)
        .collect()
}

#[test]
fn statements_10_and_11_the_words_and_no_estimate_of_the_staffs() {
    const NOT_ESTIMATES: [&str; 9] = [
        "remaining",
        "left",
        "same",
        "probably",
        "likely",
        "merged",
        "true",
        "false",
        "real",
    ];
    every_morning(|tag, _, side, morning| {
        let answer = &morning.intel[&side];
        let label = format!("{} {side:?} {}", tag.file, morning.day);
        for name in names_of(answer) {
            let part = &answer[name];
            walk(
                part,
                &mut vec![name.to_string()],
                &mut |path, key, inner| {
                    let key = key.unwrap();
                    // What a contact or a target was thought to be is `taken_for`.
                    assert_ne!(key, "kind", "{label} {path:?}");
                    if ["seen", "lost_sight_of", "before_the_game"].contains(&path[0].as_str()) {
                        assert!(!key.contains("force"), "{label} {path:?}");
                    }
                    // No count of contacts or of forces: the only count is the
                    // ships one report gave.
                    if key.contains("count") {
                        assert_eq!(key, "ship_count", "{label} {path:?}");
                    }
                    if path[0] == "claimed" {
                        assert!(
                            !key.contains("confirmed") && !key.contains("destroyed"),
                            "{label} {path:?}"
                        );
                    }
                    assert!(
                        !words_of(key).contains(&"his".to_string()),
                        "{label} {path:?}"
                    );
                    if let Value::String(text) = inner {
                        assert!(
                            !words_of(text).contains(&"his".to_string()),
                            "{label} {path:?}: {text}"
                        );
                        // The only words in the answer are those of suspected
                        // waters, as told; a name is a unit's own.
                        if key == "words" {
                            for word in NOT_ESTIMATES {
                                assert!(
                                    !words_of(text).contains(&word.to_string()),
                                    "{label} {path:?}: {text}"
                                );
                            }
                        }
                    }
                    for word in NOT_ESTIMATES {
                        assert!(
                            !words_of(key).contains(&word.to_string()),
                            "{label} {path:?}"
                        );
                    }
                    // No claim is said to be true or false, and none names a
                    // contact.
                    if path[0] == "claimed" {
                        assert!(
                            !["contact", "same_force", "true_count", "false_claims"].contains(&key)
                        );
                    }
                },
            );
        }
    });
    // A contact struck and then dropped is in `lost_sight_of` and its claim
    // in `claimed`, and neither line names the other.
    let over = &the_over(run_of(B)).intel[&Side::Allied];
    assert!(!over["claimed"].to_string().contains("contact"));
    assert!(!over["lost_sight_of"].to_string().contains("claim"));
    let claim_event = over["claimed"]["days"][0]["claims"][0]["event"].clone();
    assert_ne!(
        claim_event,
        over["lost_sight_of"][0]["last_report"]["event"]
    );
    assert_ne!(claim_event, over["lost_sight_of"][0]["dropped"]["event"]);
}

// ---------------------------------------------------------------------
// Statement 12.
// ---------------------------------------------------------------------

fn changed(text: &str, from: &str, to: &str) -> String {
    assert!(text.contains(from), "the text has no {from:?}");
    text.replacen(from, to, 1)
}

/// The answers a seat is given, at every morning and when the game is over,
/// as serialised, and the last day's hash of the game.
fn answers_of(text: &str, tag: Tag, side: Side) -> (Vec<String>, u64) {
    let mut answers = Vec::new();
    let session = drive_text(text, tag, |session, _| {
        answers.push(ask(session, side, "intelligence", json!({})).to_string());
    });
    (
        answers,
        *session.day_hashes().last().expect("a day was played"),
    )
}

#[test]
fn statement_12_two_worlds_give_the_same_answer_byte_for_byte() {
    let b_text = scenario_text(B.file);
    let b_ship_5 = b_text.find("id = 5").expect("ship 5");
    let (head, tail) = b_text.split_at(b_ship_5);
    let b_other = format!("{head}{}", changed(tail, "fuel_t = 2000", "fuel_t = 1500"));
    let worlds: [(Tag, Side, String, String); 4] = [
        (
            A,
            Side::Japanese,
            scenario_text(A.file),
            changed(&scenario_text(A.file), "supply_t = 2000", "supply_t = 1900"),
        ),
        (
            A,
            Side::Allied,
            scenario_text(A.file),
            changed(&scenario_text(A.file), "fuel_t = 2000", "fuel_t = 1900"),
        ),
        (B, Side::Allied, b_text.clone(), b_other),
        (
            D,
            Side::Japanese,
            scenario_text(D.file),
            changed(&scenario_text(D.file), "fuel_t = 500", "fuel_t = 400"),
        ),
    ];
    for (tag, side, one, other) in worlds {
        assert_ne!(one, other, "the two scenario texts differ");
        let (a, hash_a) = answers_of(&one, tag, side);
        let (b, hash_b) = answers_of(&other, tag, side);
        // The umpire's state of the two differs on the last day.
        assert_ne!(hash_a, hash_b, "{} {side:?}: the worlds differ", tag.file);
        assert_eq!(a.len(), b.len());
        for (day, (x, y)) in a.iter().zip(&b).enumerate() {
            assert_eq!(x, y, "{} {side:?}: morning {day}", tag.file);
        }
        // The answers are answers, and the last is the game's end.
        assert!(a.iter().all(|x| x.contains("\"kind\":\"intelligence\"")));
    }
}

// ---------------------------------------------------------------------
// Statement 13.
// ---------------------------------------------------------------------

fn event_numbers(answer: &Value) -> Vec<(String, u64)> {
    let mut found = Vec::new();
    walk(answer, &mut Vec::new(), &mut |path, key, inner| {
        let key = key.unwrap();
        // The context: the keys above, without a day or a list.
        let context = path.join(".");
        match (key, inner) {
            ("event", Value::Number(n)) => found.push((context, n.as_u64().expect("a number"))),
            ("events", Value::Array(numbers)) => {
                for n in numbers {
                    found.push((context.clone(), n.as_u64().expect("a number")));
                }
            }
            _ => {}
        }
    });
    found
}

fn kind_expected(context: &str, answer: &Value) -> &'static str {
    match context {
        c if c.ends_with("last_report.event") => "report_received",
        c if c.ends_with("dropped.event") => {
            let why = answer["lost_sight_of"]
                .as_array()
                .map(|l| {
                    l.iter()
                        .map(|x| x["dropped"]["why"].as_str().unwrap_or(""))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            if why.iter().all(|w| *w == "no_report_since") {
                "contact_dropped"
            } else if why.iter().all(|w| *w == "strike_found_nothing") {
                "contact_not_there"
            } else {
                "either"
            }
        }
        c if c.starts_with("claimed.") => "claim_received",
        c if c.starts_with("before_the_game.contacts") => "report_received",
        c if c.starts_with("before_the_game.estimates") => "ground_report_received",
        c if c.starts_with("ashore.estimates") || c.starts_with("enemy_bases.estimate") => {
            "ground_report_received"
        }
        c if c.starts_with("ashore.found_empty") => "found_empty",
        c if c.starts_with("enemy_has_done") && c.contains("raids") => "base_attacked_against",
        c if c.starts_with("enemy_has_done") && c.contains("bombardments") => {
            "shore_bombardment_against"
        }
        c if c.starts_with("enemy_has_done") && c.contains("fights") => "engagement_ended",
        c if c.starts_with("enemy_has_done") && c.contains("attacks_on_ships") => {
            "air_attack_against"
        }
        c if c.starts_with("enemy_has_done") && c.contains("submarine_attacks") => {
            "salvo_from_submarine"
        }
        c if c.starts_with("enemy_submarines.reports") => "submarine_reported",
        c if c.starts_with("enemy_submarines.suspected_waters") => "suspected_waters_told",
        other => panic!("an event number at {other}"),
    }
}

#[test]
fn statement_13_every_handle_and_number_is_the_seats_own() {
    for tag in [A, B, C, D] {
        // The numbers each side was ever told, by side, gathered as the game
        // is played: the sources a number of the answer may come from.
        let mut told: BTreeMap<Side, Vec<Value>> = BTreeMap::new();
        let mut earlier_new: BTreeMap<Side, BTreeSet<String>> = BTreeMap::new();
        let mut checked = 0;
        drive(tag, |session, day| {
            for side in SIDES {
                let answer = ask(session, side, "intelligence", json!({}));
                let contacts = ask(session, side, "list_contacts", json!({}));
                let bases = ask(session, side, "list_bases", json!({}));
                let briefing = ask(session, side, "get_briefing", json!({"text": false}));
                for line in list(&briefing["contacts"], "new") {
                    earlier_new
                        .entry(side)
                        .or_default()
                        .insert(line["contact"]["handle"].as_str().unwrap().to_string());
                }
                told.entry(side)
                    .or_default()
                    .extend([contacts.clone(), bases.clone(), briefing]);

                // Every event is one explain knows, of the kind the line is of.
                for (context, event) in event_numbers(&answer) {
                    let explained = ask(session, side, "explain", json!({"event": event}));
                    assert_eq!(
                        explained["kind"], "explain",
                        "{} {side:?} {day}: event {event} at {context}",
                        tag.file
                    );
                    let kind = explained["nodes"][0]["event_kind"].as_str().unwrap();
                    let expected = kind_expected(&context, &answer);
                    if expected == "either" {
                        assert!(
                            ["contact_dropped", "contact_not_there"].contains(&kind),
                            "{kind}"
                        );
                    } else if expected == "found_empty" {
                        assert!(kind.starts_with("ground_"), "{context}: {kind}");
                    } else {
                        assert_eq!(kind, expected, "{} {side:?} {day}: {context}", tag.file);
                    }
                    checked += 1;
                }

                // Every handle of a contact is held now, was made by an
                // earlier briefing, or was given before the game.
                let held: BTreeSet<String> = list(&contacts, "contacts")
                    .iter()
                    .map(|c| c["contact"]["handle"].as_str().unwrap().to_string())
                    .collect();
                let given: BTreeSet<String> = list(&answer["before_the_game"], "contacts")
                    .iter()
                    .map(|c| c["contact"]["handle"].as_str().unwrap().to_string())
                    .collect();
                let base_handles: BTreeSet<String> = list(&bases, "bases")
                    .iter()
                    .chain(list(&bases, "enemy_bases"))
                    .map(|b| b["base"]["handle"].as_str().unwrap().to_string())
                    .collect();
                let mut handles: Vec<(String, String)> = Vec::new();
                walk(&answer, &mut Vec::new(), &mut |_, key, inner| {
                    if let (Some(key), Some(handle)) =
                        (key, inner.get("handle").and_then(Value::as_str))
                    {
                        handles.push((key.to_string(), handle.to_string()));
                    }
                });
                for (key, handle) in handles {
                    match key.as_str() {
                        "contact" => assert!(
                            held.contains(&handle)
                                || given.contains(&handle)
                                || earlier_new.get(&side).is_some_and(|s| s.contains(&handle)),
                            "{} {side:?} {day}: {handle}",
                            tag.file
                        ),
                        "base" => assert!(base_handles.contains(&handle), "{handle}"),
                        "against" | "force" | "unit" => {
                            let entity =
                                ask(session, side, "get_entity", json!({"handle": handle}));
                            assert_eq!(
                                entity["kind"], "entity",
                                "{} {side:?} {day}: {handle}",
                                tag.file
                            );
                        }
                        other => panic!("a handle at {other}"),
                    }
                }

                // Every other number is one of the seat's own tools gave.
                let mut sources: BTreeSet<String> = BTreeSet::new();
                for source in &told[&side] {
                    collect_scalars(source, &mut sources);
                }
                let mut numbers: Vec<(String, String)> = Vec::new();
                walk(&answer, &mut Vec::new(), &mut |path, key, inner| {
                    let key = key.unwrap();
                    if key == "version"
                        || key == "claimed_on_ground_from_air"
                        || path.contains(&"totals".to_string())
                        // The one line no other tool has: its event is
                        // explained above, and is in no briefing.
                        || path.contains(&"submarine_attacks".to_string())
                    {
                        return;
                    }
                    match inner {
                        Value::Number(n) => numbers.push((path.join("."), n.to_string())),
                        Value::String(s) if s.chars().all(|c| c.is_ascii_digit() || c == '.') => {
                            numbers.push((path.join("."), s.clone()));
                        }
                        _ => {}
                    }
                });
                // Array members of numbers (`events`) are in the walk's blind
                // spot, and each was explained above.
                for (path, number) in numbers {
                    assert!(
                        sources.contains(&number),
                        "{} {side:?} {day}: {number} at {path} is in no answer of the seat's own tools",
                        tag.file
                    );
                }
            }
        });
        assert!(checked >= 3, "{} {checked}", tag.file);
    }
}

fn collect_scalars(value: &Value, into: &mut BTreeSet<String>) {
    match value {
        Value::Number(n) => {
            into.insert(n.to_string());
        }
        Value::String(s) => {
            into.insert(s.clone());
        }
        Value::Array(items) => items.iter().for_each(|v| collect_scalars(v, into)),
        Value::Object(map) => map.values().for_each(|v| collect_scalars(v, into)),
        _ => {}
    }
}

// ---------------------------------------------------------------------
// Statement 14.
// ---------------------------------------------------------------------

#[test]
fn statement_14_when_the_game_is_over_it_still_shows_belief_only() {
    const ONLY_AT_THE_END: [&str; 8] = [
        "result", "points", "margin", "score", "truth", "history", "outcome", "level",
    ];
    for tag in [A, B, C, D] {
        let run = run_of(tag);
        for side in SIDES {
            let over = &the_over(run).intel[&side];
            assert_eq!(over["kind"], "intelligence");
            walk(over, &mut Vec::new(), &mut |path, key, _| {
                let key = key.unwrap();
                assert!(
                    !ONLY_AT_THE_END.contains(&key),
                    "{} {side:?}: {path:?}",
                    tag.file
                );
                assert!(!key.contains("points"), "{key}");
            });
        }
    }
    // A, Japanese: the estimate at (8,0) is still the last report's, and the
    // totals are still what was claimed.
    let a = run_of(A);
    let over = &the_over(a).intel[&Side::Japanese];
    let after_two = &a.mornings[2].intel[&Side::Japanese];
    assert_eq!(
        over["ashore"]["estimates"][0]["event"],
        after_two["ashore"]["estimates"][0]["event"]
    );
    assert_eq!(
        over["ashore"]["estimates"][0]["men"],
        after_two["ashore"]["estimates"][0]["men"]
    );
    assert_eq!(over["ashore"]["estimates"][0]["day"], 2);
    assert_eq!(over["claimed"]["totals"], after_two["claimed"]["totals"]);
    // The parts as hand-calculated.
    assert_eq!(
        names_of(over),
        vec!["claimed", "ashore", "enemy_bases", "before_the_game"]
    );
    let allied = &the_over(a).intel[&Side::Allied];
    assert_eq!(
        names_of(allied),
        vec!["lost_sight_of", "enemy_has_done", "ashore"]
    );
}

// ---------------------------------------------------------------------
// Statement 15.
// ---------------------------------------------------------------------

#[test]
fn statement_15_a_reading_that_changes_no_hash_and_a_watcher_has_it() {
    // Game A played with the tool asked by both seats, any number of times
    // every morning, ends each day on the hashes of game A played without.
    let mut asked = 0;
    let with = drive(A, |session, _| {
        for side in SIDES {
            let first = ask(session, side, "intelligence", json!({}));
            let second = ask(session, side, "intelligence", json!({}));
            let third = ask(session, side, "intelligence", json!({}));
            // Two calls at one moment give the same bytes.
            assert_eq!(first.to_string(), second.to_string());
            assert_eq!(first.to_string(), third.to_string());
            asked += 1;
        }
    });
    let without = drive(A, |_, _| {});
    assert_eq!(asked, 10);
    assert_eq!(with.day_hashes(), without.day_hashes());
    assert_eq!(with.day_hashes().len(), 4);

    // The tool is in TOOLS and in WATCHER_TOOLS, and the interface is 52.
    assert!(TOOLS.contains(&"intelligence"));
    assert!(WATCHER_TOOLS.contains(&"intelligence"));
    assert_eq!(VERSION, 52);
    let session = drive(A, |_, _| {});
    for side in SIDES {
        assert!(session.tools(side).contains(&"intelligence"));
    }

    // A watcher's read of it for a seat gives what that seat is given, at the
    // first morning (the Japanese seat holds something then) and after a day
    // is played.
    let scenario = load(A.file, A.seed);
    let (session, _) = session_of(scenario, &SIDES);
    let mut session = session.watched();
    for side in SIDES {
        assert!(session.watcher_tools(side).contains(&"intelligence"));
        let seat = ask(&mut session, side, "intelligence", json!({}));
        let watched = session
            .watch(side, "intelligence", &json!({}))
            .expect("the watcher reads it");
        assert_eq!(seat.to_string(), watched.to_string());
    }
    assert!(!names_of(&ask(
        &mut session,
        Side::Japanese,
        "intelligence",
        json!({})
    ))
    .is_empty());

    // It takes no argument and refuses any.
    let refused = session
        .call(Side::Allied, "intelligence", &json!({"day": 1}))
        .expect_err("an argument is refused");
    assert!(refused.message.contains("day"), "{refused}");
}

// ---------------------------------------------------------------------
// Statement 16.
// ---------------------------------------------------------------------

fn seat_session(post: Post) -> Session {
    let text = scenario_text(E.file);
    let scenario = load_text(&text, E.file, E.seed);
    let name = scenario.name.clone();
    // The staff is in every seat, and the scenario's own commands are given
    // as the commander's, as `sim-cli` opens a game.
    let commands = scenario.scripted_commands().clone();
    Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(sim_staff::staffs())
    .with_commands(commands)
    .with_post(Side::Allied, post)
    .expect("the seat is placed")
}

fn force(number: u32) -> Post {
    Post::At(CommandRef::TaskForce(TaskForceId::new(number)))
}

fn base(number: u32) -> Post {
    Post::At(CommandRef::Base(BaseId::new(number)))
}

#[test]
fn statement_16_with_command_levels_on_three_parts_are_not_kept_at_a_command() {
    const NOT_KEPT: [&str; 3] = ["lost_sight_of", "enemy_has_done", "before_the_game"];
    let seats = [
        ("area", Post::Area),
        ("area:easier", Post::AreaEasier),
        ("task_force:1", force(1)),
        ("task_force:2", force(2)),
        ("base:1", base(1)),
    ];
    for (name, post) in seats {
        let mut session = seat_session(post);
        let mut mornings = 0;
        loop {
            let day = session.day();
            let answer = ask(&mut session, Side::Allied, "intelligence", json!({}));
            valid(&schemas(), "intelligence.schema.json", &answer, name);
            let contacts = ask(&mut session, Side::Allied, "list_contacts", json!({}));
            let label = format!("{name} morning {day}");
            let parts = names_of(&answer);
            if name == "area:easier" {
                // Every part is given: the picture is the side's own belief.
                assert!(!has_key(&answer, "not_kept"), "{label}");
                if day == 0 {
                    let given = &answer["before_the_game"]["contacts"];
                    assert_eq!(given.as_array().unwrap().len(), 1, "{label}");
                    assert_eq!(given[0]["ship_count"], 1);
                    assert_eq!(given[0]["taken_for"], "carrier");
                    assert_eq!(given[0]["place"]["q"], 12);
                    assert_eq!(given[0]["place"]["r"], 0);
                    assert_eq!(given[0]["age_pulses"], 0);
                }
            } else {
                assert_eq!(answer["not_kept"], json!(NOT_KEPT), "{label}");
                for part in NOT_KEPT {
                    assert!(!has_key(&answer, part), "{label} {part}");
                    assert!(!parts.contains(&part), "{label} {part}");
                }
            }
            // `seen`, `ashore` and `enemy_submarines` are as statements 2, 6
            // and 8 say of list_contacts asked at that seat.
            let mut held: Vec<Value> = list(&contacts, "contacts").to_vec();
            held.sort_by(|a, b| {
                a["age_pulses"]
                    .as_u64()
                    .cmp(&b["age_pulses"].as_u64())
                    .then(number_of(&b["contact"]).cmp(&number_of(&a["contact"])))
            });
            assert_eq!(list(&answer, "seen"), held.as_slice(), "{label}");
            assert_eq!(
                list(&answer["ashore"], "estimates"),
                list(&contacts, "ground_estimates"),
                "{label}"
            );
            assert_eq!(
                list(&answer["enemy_submarines"], "reports"),
                list(&contacts, "submarine_reports"),
                "{label}"
            );
            assert_eq!(
                list(&answer["enemy_submarines"], "suspected_waters"),
                list(&contacts, "suspected_waters"),
                "{label}"
            );
            // The claims of the seat's picture and no other: those its own
            // briefings told it.
            let briefing = ask(
                &mut session,
                Side::Allied,
                "get_briefing",
                json!({"text": false}),
            );
            let told = list(&briefing["losses"], "claims").len();
            let mine: usize = list(&answer["claimed"], "days")
                .iter()
                .map(|d| list(d, "claims").len())
                .sum();
            assert!(mine >= told, "{label}");
            // No line of `enemy_bases` names a base the seat's side holds.
            for line in list(&answer, "enemy_bases") {
                assert_eq!(line["held_by"], "japanese", "{label}");
                assert_ne!(line["base"]["handle"], "base:1", "{label}");
            }
            mornings += 1;
            if day >= session.days() {
                break;
            }
            let ended = ask(&mut session, Side::Allied, "end_day", json!({}));
            assert_eq!(ended["resolved"], true, "{ended}");
        }
        assert_eq!(mornings, 4, "{name}");
    }
    // With command levels off no answer has `not_kept`.
    every_morning(|tag, _, side, morning| {
        assert!(
            !has_key(&morning.intel[&side], "not_kept"),
            "{} {side:?}",
            tag.file
        );
    });
}
