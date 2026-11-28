//! Scenario 140: men, then food. The third rung of the ladder (decision
//! 0237): a game for play with two decisions, the risk of a landing and how
//! the place is fed once it is taken. At the risk "balanced" the landing
//! comes after the mission's last day; at "press" the men land on day 12
//! with three nights of rations, and a supply run must land by day 15.
//!
//! The data is placeholder data: see
//! `data/scenarios/ladder/140_men_then_food.toml` and
//! `data/map/placeholder_ladder.toml`. No rule is proved here that another
//! test does not prove: what is proved is that the game the player is given
//! has the answers the record says it has.
//!
//! **It runs on the live figures,** the ones the file names, through
//! `common::load_live` or `common::from_text_with` with the live options,
//! and asserts numbers that depend on them. That is decision 0237's named
//! exception to the rule that hand calculations run on the frozen figures
//! (question 8, confirmed by the designer on Nov 27, 2026): a game for play
//! that is no battle's is proved on the figures it is played on. A tuning
//! that moves a number below comes back to the architect.
//!
//! **Built.** Each test below asserts its statement, on the file as given
//! with the case's commands added at its mark `# orders`. The last day's
//! hash of each game is pinned in `tests/golden/pinned_hashes.txt`.
//!
//! # Hand calculation
//!
//! Written by the architect before the code. Everything is asserted
//! exactly. If the code disagrees, the code or the scenario is wrong: do not
//! change a number here to make a test pass. A mismatch goes back to the
//! architect.
//!
//! Every stock, load and day below was worked by hand from the rules cited
//! and then checked on the engine as it stood on Nov 28, 2026 (`sim-cli
//! run` on this file with the commands of each case added at its mark).
//!
//! ## The figures read
//!
//! - `ground_hunger.toml: daily_supply_t_per_man` 0.005: the 600 men are
//!   charged 3 t a night.
//! - `ground_hunger.toml: rations_landed_nights` 3: the landing puts
//!   600 x 3 x 0.005 = 9 t of supply into the base it forms.
//! - `ground_hunger.toml: preparation_per_day` 5: a force in port with the
//!   order to prepare a landing gains 5 of loading a day, and each unit of
//!   its roster 5 of rehearsal (the unit's assault skill is the standard, so
//!   its factor is 1). The staff's marks: 50 at press, 100 at balanced.
//! - `ground_hunger.toml: hunger_nights_before_loss` 14: nobody can be lost
//!   in this game.
//! - Class 11 (`ship_classes_placeholder_transports.csv`): 200 men, no
//!   cargo, 16 kn, 500 t of fuel over 4,000 nm, so 0.125 t a mile. Class 12
//!   (`ship_classes_placeholder_fast_transport_cargo.csv`): the same with
//!   30 t of cargo.
//! - `port_sizes.csv`: size 1 (S) has one berth at 50 t an hour; size 0 (E)
//!   works by lighter at 20 t an hour.
//! - The map: S (0,0) to E (3,3) is 6 hex steps. A ship on open water is
//!   charged the straight line (chapter 2: a step is charged the miles it
//!   brings the force closer): 40 x the square root of 27 = 207.8, 208 nm.
//!
//! ## What every case shares
//!
//! - **The order of a day** (chapter 1): the commander's commands and the
//!   staff, then the night pulse, the morning pulse and the afternoon pulse,
//!   then the day's ground phase. The men's supply is in the night pulse,
//!   and a ship in port loads after it in the same pulse. So a night's meal
//!   comes before anything landed that day, and before the men who ate it
//!   go aboard. Men aboard ship do not eat and no `MenFed` is recorded for
//!   them.
//! - **A passage** is 208 nm at 16 kn = 13 hours, 1 day, and burns
//!   208 x 0.125 = 26 t in each ship; a round trip burns 52 t. A force that
//!   sails at the start of a day halts at E 13 hours into it, in the morning
//!   pulse (the engine gives the time as 07:00).
//! - **The men at S** eat 3 t a night from base 1 while they are ashore
//!   there: after the night of day d it holds 200 - 3 x d.
//! - **Leaves.** Every case gives leave to prepare and leave to launch on
//!   day 1.
//! - **The landing order** in every case, on day 1: "take the place at
//!   (3,3)" as intent 1, with a forces limit of task force 1, land unit 1
//!   and base 1 as the source.
//!
//! ## The landing at press (cases A to E)
//!
//! With the landing order, on day 1, the risk of intent 1 is set to press.
//!
//! - Day 1: the staff makes plan 1 ready (preparing), forms "ready a landing
//!   at (3,3) by task force 1" under intent 1 with unit 1 on its roster (it
//!   is intent 2, or intent 3 where a hold was given as intent 2, case E),
//!   and gives the force the order to prepare. After day d the force's
//!   loading and the unit's rehearsal are each 5 x d.
//! - After day 10 they are 50, the mark at press. Day 11: the staff hands
//!   in the notice that the landing is ready, and, leave to launch being
//!   given, releases plan 1 and orders the men aboard. Night of day 11: the
//!   meal at S, 600 of 600 fed, base 1 at 200 - 33 = 167 t; then the 600 men
//!   go aboard, 200 in each ship.
//! - Day 12: the force sails at the start of the day and halts at E in the
//!   morning pulse. The landing forms base 3 there for the Allied side (the
//!   next free number), puts 9 t of rations into it and lands the 600 men.
//!   No `MenFed` is recorded on day 12: the men were aboard at the night's
//!   meal. At the end of the day the Japanese side, with no man in the hex,
//!   is beaten without a fight and base 2 changes side with nothing in it.
//!   So base 2 is Japanese after day 11 and Allied after day 12.
//! - Day 13: the staff marks plan 1 complete and hands in the notice that
//!   intent 1 is met. The transports are back at S, each with
//!   200 - 52 = 148 t of fuel.
//! - **The rations.** Base 3 holds 9 t after day 12. The nights of days 13,
//!   14 and 15 take it to 6, 3 and 0. With nothing more landed the men are
//!   unfed from the night of day 16.
//!
//! ## The supply order, and a run
//!
//! "Keep base 2 supplied" with a floor of 2 days and an interval of 5, the
//! right answer of rung 1, given as intent 3 with a forces limit of ship 4
//! and base 1 as the source. The floor's stock is 2 x 3 = 6 t.
//!
//! - **The reading** of a day is the stock of the side's bases in the hex of
//!   base 2, bases 2 and 3 together, on that morning, before the night's
//!   meal (chapter 8, The doctrine). The men are fed from base 2 first and
//!   then from base 3 (chapter 7: the bases are taken in base order).
//! - **A run ordered on day d** loads on day d (under an hour at the berth,
//!   1 day), sails on day d + 1 and is due on day d + 1. She halts at E in
//!   the morning pulse of day d + 1 and unloads into base 2 (at most 30 t at
//!   20 t an hour, 1.5 hours). She is back at S early on day d + 2. Her ship
//!   is committed from day d to day d + 3 and free on day d + 4.
//! - **When a run is needed:** where, with nothing on its way, the reading
//!   of day d + 2 is below the floor's stock.
//! - **Its load** is the floor's stock less the reading of day d + 1, not
//!   below 0, plus the interval's days x 3 t, cut to the ship's 30 t.
//!
//! In each table "E" is the stock of bases 2 and 3 together and "S" the
//! stock of base 1, after the night's meal and that day's loading and
//! landing, in tons.
//!
//! ## Case A: press, and the supply order on day 13. The mission is met
//!
//! - Day 13, reading 9: the reading of day 15 is 3 t, below 6: run 1. The
//!   reading of day 14 is 6 t; load = max(0, 6 - 6) + 5 x 3 = 15 t. Loading
//!   day 13, due day 14, committed days 13 to 16, free day 17. Night of day
//!   13 at S: 15 t loaded, 152 t.
//! - Day 14: the meal from base 3, 3 t left there; 15 t landed into base 2.
//!   From the night of day 15 base 2 pays, and base 3 keeps its 3 t to the
//!   end.
//! - Readings: day 15, 18; day 16, 15; day 17, 12 (the reading of day 19 is
//!   6 t, not below 6: no run); day 18, 9: the reading of day 20 is 3 t,
//!   below 6: run 2. The reading of day 19 is 6 t; load = 15 t. Loading day
//!   18, due day 19, free day 22. S: 137 t.
//! - Day 19: base 2 pays its last 3 t; 15 t landed. Day 21, reading 15: the
//!   reading of day 23 is 9 t: no run.
//!
//! | Night of day | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | E | 9 | 6 | 18 | 15 | 12 | 9 | 6 | 18 | 15 | 12 |
//! | Base 2 | 0 | 0 | 15 | 12 | 9 | 6 | 3 | 15 | 12 | 9 |
//! | S | 167 | 152 | 152 | 152 | 152 | 152 | 137 | 137 | 137 | 137 |
//!
//! 600 of 600 are fed at E on each night from day 13 to day 21. Ship 4 ends
//! with 300 - 2 x 52 = 196 t of fuel.
//!
//! ## Case B: press, and the supply order on day 14. The mission is met
//!
//! - Day 14, reading 6: the reading of day 16 is 0, below 6: run 1. The
//!   reading of day 15 is 3 t; load = (6 - 3) + 15 = 18 t. Loading day 14,
//!   due day 15, free day 18. S: 167 - 18 = 149 t.
//! - Day 15: the meal takes the last 3 t of the rations; 18 t landed.
//! - Readings: day 16, 18; 17, 15; 18, 12; day 19, 9: the reading of day 21
//!   is 3 t: run 2, 15 t, loading day 19, due day 20. S: 134 t.
//!
//! | Night of day | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | E | 9 | 6 | 3 | 18 | 15 | 12 | 9 | 6 | 18 | 15 |
//!
//! 600 of 600 are fed at E on each night from day 13 to day 21. At the end
//! base 2 holds 15 t, base 3 nothing and S 134 t.
//!
//! ## Case C: press, and the supply order on day 15. A day late
//!
//! - Day 15, reading 3: the reading of day 17 is 0: run 1. The reading of
//!   day 16 is 0; load = (6 - 0) + 15 = 21 t. Loading day 15, due day 16.
//!   S: 167 - 21 = 146 t.
//! - **Night of day 16: 0 of 600 fed at E, 600 unfed.** The 21 t land that
//!   morning, after the meal.
//! - Nights of days 17 to 21 take base 2 to 18, 15, 12, 9 and 6 t. The unit
//!   has 1 night of hunger after day 16 and none after day 17.
//! - Day 21, reading 9: the reading of day 23 is 3 t: run 2, 15 t, loading
//!   day 21; it never sails. S ends at 131 t.
//!
//! ## Case D: press, and the supply order on day 12. Refused
//!
//! The commander's commands are taken before the day is fought, and base 2
//! is Japanese until the end of day 12. "Keep base 2 supplied" on day 12 is
//! refused: "base:2 is not one of the side's bases". The forces limit after
//! it is refused too: "intent:3 is not in force". No order is in force and
//! no run is sent. **600 are unfed at E on each night from day 16 to day
//! 21,** six nights. The unit has 6 nights of hunger at the end. S ends at
//! 167 t. Press with no supply order at all is this case without its
//! refusals.
//!
//! ## Case E: press, and a hold given with the landing. The mission is met
//!
//! On day 1, with the landing order: "hold base 2" as intent 2, with a
//! forces limit of ship 4 and base 1 as the source, and no risk set on it
//! (chapter 8, Holding a place, A place not yet the side's). The staff's
//! "ready a landing" is then intent 3. The landing is as above.
//!
//! - Days 1 to 12: the hold waits for the base to be the side's and forms
//!   nothing.
//! - Day 13: base 2 is the side's with men at it and no supply for it is in
//!   force. The staff forms under the hold intent 4, "keep base 2 supplied
//!   at 5 days" (the margin at balanced), with no interval, so 15 days. The
//!   floor's stock is 15 t. It is served from the next morning.
//! - Day 14, reading 6: the reading of day 16 is 0, below 15: run 1. The
//!   reading of day 15 is 3 t; load = (15 - 3) + 15 x 3 = 57 t, cut to the
//!   ship's 30 t. Loading day 14, due day 15, free day 18. S: 137 t.
//! - Day 15: the meal takes the last of the rations; 30 t landed.
//! - Readings: day 16, 30; 17, 27; 18, 24; day 19, 21 (the reading of day
//!   21 is 15 t, not below 15: no run); day 20, 18: the reading of day 22 is
//!   12 t: run 2. The reading of day 21 is 15 t; load = 0 + 45, cut to 30 t.
//!   Loading day 20, due day 21. S: 107 t.
//! - Day 21: the meal, 12 t; 30 t landed, 42 t.
//!
//! | Night of day | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 |
//! | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
//! | E | 9 | 6 | 3 | 30 | 27 | 24 | 21 | 18 | 15 | 42 |
//!
//! 600 of 600 are fed at E on each night from day 13 to day 21.
//!
//! ## Case F: balanced. The landing comes too late
//!
//! The landing order with no risk set, which is balanced, and nothing else.
//!
//! - The mark is 100. After day 20 the loading and the rehearsal are 100.
//!   Day 21: the notice that the landing is ready, on that day and no
//!   other; plan 1 is released; the men are fed at S that night (base 1 at
//!   200 - 63 = 137 t) and go aboard.
//! - The game ends with the 600 men aboard at S. No base is formed at E, no
//!   man of the side is ever ashore there, and **base 2 is Japanese at the
//!   end.** Nobody is unfed on any night: 600 of 600 at S on each of the 21.
//!
//! ## Case G: press given on day 17. Taken late, and fed by the rations
//!
//! The landing order on day 1 with no risk set; on day 17 the risk of intent
//! 1 is set to press. No supply order and no hold.
//!
//! - After day 16 the loading is 80, above press's mark of 50. Day 17: the
//!   notice that the landing is ready; plan 1 is released; the meal at S
//!   (base 1 at 200 - 51 = 149 t), and the men go aboard.
//! - Day 18: the men land. Base 3 holds 9 t. Base 2 is Allied after day 18.
//! - The nights of days 19, 20 and 21 take the rations to 6, 3 and 0.
//!   **Nobody is unfed on any night,** and no run is ever sent: 600 of 600
//!   at S on the nights of days 1 to 17, no record on day 18, 600 of 600 at
//!   E on the nights of days 19 to 21.
//!
//! By the mission's words as the record has them this game meets it with no
//! food sent. What the rung does about that is the designer's
//! (`docs/decisions/as-built/0237-the-first-three-rungs.md`, rung 3); this
//! case proves what the engine does.
//!
//! ## In every case
//!
//! No man is lost: `hunger_nights_before_loss` is 14 and the unit has at
//! most six hungry nights. The Japanese side has no man, so no fight is
//! rolled and no random number is drawn.
//!
//! # Statements
//!
//! 1. The file loads on the figures it names; it has Allied base 1 and
//!    Japanese base 2, which holds nothing; four Allied ships, of which
//!    ships 1 to 3 are task force 1; one Allied land unit of 600 men ashore
//!    at (0,0); no command; the Japanese side has no ship, unit or aircraft;
//!    S to E is 6 hex steps by sea and 208 nm in a straight line.
//! 2. Press (the commands of case D without its supply order): the staff
//!    forms "ready a landing at (3,3) by task force 1" under intent 1 on day
//!    1; the force's loading and the unit's rehearsal are 5 x d after each
//!    day d from 1 to 10; the notice that the landing is ready is handed in
//!    on day 11 and on no other day; plan 1 is preparing after day 10 and
//!    released after day 11.
//! 3. Press: the 600 men are aboard after day 11, 200 in each of ships 1 to
//!    3, and ashore at (3,3) after day 12; base 3 is formed on day 12 at
//!    (3,3) for the Allied side and holds 9 t after that day; base 2 is
//!    Japanese after day 11 and Allied after day 12 and to the end; the
//!    notice that intent 1 is met is handed in on day 13; `MenFed` shows 600
//!    of 600 at (0,0) on the nights of days 1 to 11 and nothing on day 12;
//!    each transport holds 148 t of fuel at the end.
//! 4. Case A: runs of 15 t are added on days 13 and 18, with the loading,
//!    sailing, due and free days above, and on no other day; the stocks of
//!    bases 1, 2 and 3 after each day from 12 to 21 are the table's; 600 of
//!    600 are fed at (3,3) on each night from day 13 to day 21; ship 4 holds
//!    196 t of fuel at the end.
//! 5. Case B: runs of 18 t on day 14 and 15 t on day 19; the stock at E
//!    after each day is the table's; nobody is unfed on any night; base 2
//!    holds 15 t and base 1 134 t at the end.
//! 6. Case C: runs of 21 t on day 15 and 15 t on day 21; 600 unfed at (3,3)
//!    on the night of day 16 and on no other; the unit's nights of hunger
//!    are 1 after day 16 and 0 after day 17; base 2 holds 6 t and base 1
//!    131 t at the end.
//! 7. Case D: both commands of day 12 are refused, with the reasons above;
//!    no run is added; 600 unfed at (3,3) on each night from day 16 to day
//!    21 and on no other; the unit's nights of hunger are 6 at the end; base
//!    2 is Allied at the end.
//! 8. Case E: the staff forms "keep base 2 supplied at 5 days", with no
//!    interval, under intent 2 on day 13 and forms no supply before; runs of
//!    30 t on days 14 and 20; the stock at E after each day is the table's;
//!    nobody is unfed on any night; base 1 holds 107 t at the end.
//! 9. Case F: the loading and the rehearsal are 100 after day 20; the notice
//!    that the landing is ready is handed in on day 21 and on no other day;
//!    at the end the 600 men are aboard, no base of the side is at (3,3),
//!    base 2 is Japanese and base 1 holds 137 t; 600 of 600 are fed at (0,0)
//!    on each of the 21 nights and no `MenFed` is ever recorded at (3,3).
//! 10. Case G: the notice that the landing is ready is handed in on day 17;
//!     base 3 is formed on day 18 with 9 t; base 2 is Allied after day 18;
//!     no run is added; `MenFed` shows 600 of 600 at (0,0) on the nights of
//!     days 1 to 17, nothing on day 18 and 600 of 600 at (3,3) on the nights
//!     of days 19 to 21; base 3 holds nothing at the end.
//! 11. In every case no `MenLostUnfed` is recorded, the unit has 600 men at
//!     the end, no Japanese man, ship or base is added, and the game played
//!     twice gives the same hash each day.

use std::path::PathBuf;

use sim_core::command::{CommandKind, IntentKind, NoticeKind, PlanState, RunDays};
use sim_core::events::EventKind;
use sim_core::fixed::Fixed;
use sim_core::ids::IntentId;
use sim_core::state::Side;
use sim_data::LoadOptions;

use crate::common::played::{play, Played};
use crate::common::values::{base, hex, n, ship, unit};
use crate::common::{self, data_dir};

fn path() -> PathBuf {
    data_dir().join("scenarios/ladder/140_men_then_food.toml")
}

fn text() -> String {
    std::fs::read_to_string(path()).expect("the scenario file reads")
}

/// The scenario as given, on the live figures, with `orders` added at its
/// mark `# orders`.
fn load(orders: &str) -> sim_data::Scenario {
    let t = common::replace_once(&text(), "# orders", &format!("# orders\n{orders}"));
    common::from_text_with(&t, &path(), &LoadOptions::default()).expect("the scenario loads")
}

fn command(day: u32, body: &str) -> String {
    format!("\n[[command]]\nday = {day}\nside = \"allied\"\n{body}\n")
}

fn leaves() -> String {
    command(1, "kind = \"set_leave\"\nleave = \"prepare\"\ngiven = true")
        + &command(1, "kind = \"set_leave\"\nleave = \"launch\"\ngiven = true")
}

/// Both leaves, "take the place at (3,3)" as intent 1 with the forces limit
/// of task force 1, land unit 1 and source base 1: the landing order of
/// every case, on day 1.
fn landing() -> String {
    leaves()
        + &command(
            1,
            "kind = \"set_intent\"\nintent = 1\nintent_kind = \"take_place\"\nhex = [3, 3]",
        )
        + &command(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 1\ntask_forces = [1]\nland_units = [1]\nsource = 1",
        )
}

/// The risk of intent 1, given on `day`.
fn risk(day: u32, risk: &str) -> String {
    command(
        day,
        &format!("kind = \"set_limit\"\nlimit = \"risk\"\nintent = 1\nrisk = \"{risk}\""),
    )
}

/// Press from day 1: the landing at press.
fn press() -> String {
    landing() + &risk(1, "press")
}

/// "Keep base 2 supplied", floor 2 and interval 5, as intent 3 with ship 4
/// and source base 1, given on `day`.
fn supply_on(day: u32) -> String {
    press()
        + &command(
            day,
            "kind = \"set_intent\"\nintent = 3\nintent_kind = \"keep_supplied\"\nbase = 2\nfloor_days = 2\ninterval_days = 5",
        )
        + &command(
            day,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 3\nships = [4]\nsource = 1",
        )
}

/// "Hold base 2" as intent 2, with ship 4 and source base 1, given with the
/// landing at press.
fn hold() -> String {
    press()
        + &command(
            1,
            "kind = \"set_intent\"\nintent = 2\nintent_kind = \"hold\"\nbase = 2",
        )
        + &command(
            1,
            "kind = \"set_limit\"\nlimit = \"forces\"\nintent = 2\nships = [4]\nsource = 1",
        )
}

/// The landing with no risk set, which is balanced.
fn balanced() -> String {
    landing()
}

/// The landing with press given on day 17.
fn late_press() -> String {
    landing() + &risk(17, "press")
}

fn game(orders: &str) -> Played {
    play(load(orders))
}

const S: u32 = 1;
const E: u32 = 3;

/// A run the staff added: the day, its load and its days.
struct RunAdded {
    day: u32,
    load: Fixed,
    days: RunDays,
}

trait Reads {
    fn runs(&self) -> Vec<RunAdded>;
    fn stock_of(&self, day: u32, raw: u32) -> Fixed;
    /// The stock of the side's bases in the hex of base 2, bases 2 and 3.
    fn stock_e(&self, day: u32) -> Fixed;
    /// (day, men, fed, unfed) of each night's `MenFed` at `at`.
    fn fed(&self, at: sim_core::hex::Hex) -> Vec<(u32, u32, u32, u32)>;
    fn unfed_days(&self, at: sim_core::hex::Hex) -> Vec<u32>;
    fn notice_days(&self, kind: NoticeKind) -> Vec<u32>;
    fn plan_day(&self, plan: u32, to: PlanState) -> Option<u32>;
    fn aboard(&self, day: u32) -> u32;
    fn refused(&self) -> Vec<(u32, String)>;
    fn formed(&self) -> Vec<(u32, IntentId, IntentKind)>;
    fn lost(&self) -> usize;
}

impl Reads for Played {
    fn runs(&self) -> Vec<RunAdded> {
        let mut out = Vec::new();
        for (day, d) in &self.handed[&Side::Allied] {
            for c in &d.commands {
                if let CommandKind::AddRun { run, .. } = &c.kind {
                    out.push(RunAdded {
                        day: *day,
                        load: run.load_t,
                        days: run.days,
                    });
                }
            }
        }
        out
    }

    fn stock_of(&self, day: u32, raw: u32) -> Fixed {
        self.truth[day as usize].bases[&base(raw)].supply_t
    }

    fn stock_e(&self, day: u32) -> Fixed {
        let truth = &self.truth[day as usize];
        truth
            .bases
            .values()
            .filter(|b| b.hex == hex(3, 3) && b.side == Side::Allied)
            .map(|b| b.supply_t)
            .fold(n(0), |a, b| a + b)
    }

    fn fed(&self, at: sim_core::hex::Hex) -> Vec<(u32, u32, u32, u32)> {
        self.scenario
            .world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::MenFed {
                    hex,
                    men,
                    fed,
                    unfed,
                    ..
                } if *hex == at => Some((e.day, *men, *fed, *unfed)),
                _ => None,
            })
            .collect()
    }

    fn unfed_days(&self, at: sim_core::hex::Hex) -> Vec<u32> {
        self.fed(at)
            .into_iter()
            .filter(|(_, _, _, unfed)| *unfed > 0)
            .map(|(day, _, _, _)| day)
            .collect()
    }

    fn notice_days(&self, kind: NoticeKind) -> Vec<u32> {
        self.handed[&Side::Allied]
            .iter()
            .filter(|(_, d)| d.notices.iter().any(|x| x.kind == kind))
            .map(|(day, _)| *day)
            .collect()
    }

    fn plan_day(&self, plan: u32, to: PlanState) -> Option<u32> {
        self.scenario
            .world
            .events()
            .iter()
            .find_map(|e| match &e.kind {
                EventKind::PlanStateChanged {
                    side,
                    plan: p,
                    to: t,
                    ..
                } if *side == Side::Allied && p.raw() == plan && *t == to => Some(e.day),
                _ => None,
            })
    }

    /// The men aboard ships at the end of `day`.
    fn aboard(&self, day: u32) -> u32 {
        self.truth[day as usize]
            .ships
            .values()
            .filter_map(|s| s.troops)
            .map(|t| t.men)
            .sum()
    }

    fn refused(&self) -> Vec<(u32, String)> {
        self.scenario
            .world
            .events()
            .iter()
            .filter_map(|e| match &e.kind {
                EventKind::CommandRefused { reason, .. } => Some((e.day, reason.to_string())),
                _ => None,
            })
            .collect()
    }

    fn formed(&self) -> Vec<(u32, IntentId, IntentKind)> {
        self.handed[&Side::Allied]
            .iter()
            .flat_map(|(day, d)| d.commands.iter().map(move |c| (*day, &c.kind)))
            .filter_map(|(day, k)| match k {
                CommandKind::FormIntent { parent, kind } => Some((day, *parent, *kind)),
                _ => None,
            })
            .collect()
    }

    fn lost(&self) -> usize {
        self.scenario
            .world
            .events()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::MenLostUnfed { .. }))
            .count()
    }
}

fn runs_of(g: &Played) -> Vec<(u32, Fixed)> {
    g.runs().iter().map(|r| (r.day, r.load)).collect()
}

fn days_of(r: &RunAdded) -> (u32, u32, u32, u32) {
    (r.days.load, r.days.sail, r.days.arrive, r.days.free)
}

fn tons(list: &[i64]) -> Vec<Fixed> {
    list.iter().map(|t| n(*t)).collect()
}

fn s_hex() -> sim_core::hex::Hex {
    hex(0, 0)
}

fn e_hex() -> sim_core::hex::Hex {
    hex(3, 3)
}

/// `(day, 600, 600, 0)` for each day of the range.
fn all_fed(days: std::ops::RangeInclusive<u32>) -> Vec<(u32, u32, u32, u32)> {
    days.map(|d| (d, 600, 600, 0)).collect()
}

/// Every game of statement 11, by case.
fn every_case() -> Vec<(&'static str, String)> {
    vec![
        ("A", supply_on(13)),
        ("B", supply_on(14)),
        ("C", supply_on(15)),
        ("D", supply_on(12)),
        ("E", hold()),
        ("F", balanced()),
        ("G", late_press()),
    ]
}

/// Statement 1: the file as the player is given it.
#[test]
fn the_file_as_given() {
    let t = text();
    assert!(!t.contains("[[command]]"), "no command in the file");
    assert!(t.contains("../../reference/ground_hunger.toml"));
    assert!(!t.contains("step1"), "the live figures");
    let scenario = load("");
    let truth = scenario.world.umpire();
    assert_eq!(truth.bases.len(), 2);
    assert_eq!(truth.bases[&base(1)].side, Side::Allied);
    assert_eq!(truth.bases[&base(2)].side, Side::Japanese);
    assert_eq!(truth.bases[&base(2)].supply_t, n(0));
    assert_eq!(truth.bases[&base(2)].fuel_t, n(0));
    assert_eq!(truth.ships.len(), 4);
    assert!(truth.ships.values().all(|s| s.side == Side::Allied));
    let force: Vec<u32> = truth.task_forces[&sim_core::ids::TaskForceId::new(1)]
        .ships
        .iter()
        .map(|s| s.raw())
        .collect();
    assert_eq!(force, vec![1, 2, 3]);
    assert!(matches!(
        truth.ships[&ship(4)].location,
        sim_core::state::ShipLocation::Port(_)
    ));
    assert_eq!(truth.land_units.len(), 1);
    let u = &truth.land_units[&unit(1)];
    assert_eq!(
        (u.side, u.men_ashore, u.hex),
        (Side::Allied, 600, Some(hex(0, 0)))
    );
    assert!(truth.air_groups.is_empty());
    assert!(scenario
        .world
        .command_state(Side::Allied)
        .intents
        .is_empty());
    assert!(scenario
        .world
        .command_state(Side::Japanese)
        .intents
        .is_empty());
    // The Japanese side has nothing but base 2.
    assert!(truth.ships.values().all(|s| s.side != Side::Japanese));
    assert!(truth.land_units.values().all(|u| u.side != Side::Japanese));
    // S to E is 6 hex steps by sea and 208 nm in a straight line.
    let (s, e) = (hex(0, 0), hex(3, 3));
    assert_eq!(truth.bases[&base(1)].hex, s);
    assert_eq!(truth.bases[&base(2)].hex, e);
    assert_eq!(scenario.world.map().sea_distance(s, e), Some(6));
    assert_eq!(s.air_distance_nm(e), 208);
}

/// Statement 2: at press the landing is ready on day 11.
#[test]
fn at_press_ready_on_day_eleven() {
    let g = game(&press());
    let formed = g.formed();
    assert_eq!(
        formed,
        vec![(
            1,
            IntentId::new(1),
            IntentKind::ReadyLanding {
                force: sim_core::ids::TaskForceId::new(1),
                hex: e_hex(),
                by_day: None,
            }
        )]
    );
    for d in 1..=10u32 {
        let truth = &g.truth[d as usize];
        let force = &truth.task_forces[&sim_core::ids::TaskForceId::new(1)];
        let prep = force.preparation.as_ref().expect("preparing");
        assert_eq!(prep.loading, n(5 * d as i64), "loading after day {d}");
        assert_eq!(
            truth.land_units[&unit(1)].rehearsal,
            n(5 * d as i64),
            "rehearsal after day {d}"
        );
    }
    assert_eq!(g.notice_days(NoticeKind::Ready), vec![11]);
    assert_eq!(g.plan_day(1, PlanState::Preparing), Some(1));
    assert_eq!(g.plan_day(1, PlanState::Released), Some(11));
}

/// Statement 3: at press the men land on day 12 with three nights of
/// rations, and the place is the side's that day.
#[test]
fn landed_on_day_twelve_with_three_nights() {
    let g = game(&press());
    assert_eq!(g.aboard(10), 0);
    assert_eq!(g.aboard(11), 600);
    for s in 1..=3 {
        let t = g.truth[11].ships[&ship(s)].troops.expect("men aboard");
        assert_eq!((t.unit, t.men), (unit(1), 200), "ship {s}");
    }
    assert_eq!(g.aboard(12), 0);
    let u = &g.truth[12].land_units[&unit(1)];
    assert_eq!((u.men_ashore, u.hex), (600, Some(e_hex())));
    let formed: Vec<(u32, u32, sim_core::hex::Hex, Side)> = g
        .scenario
        .world
        .events()
        .iter()
        .filter_map(|e| match &e.kind {
            EventKind::BaseFormed {
                base, hex, side, ..
            } => Some((e.day, base.raw(), *hex, *side)),
            _ => None,
        })
        .collect();
    assert_eq!(formed, vec![(12, E, e_hex(), Side::Allied)]);
    assert_eq!(g.stock_of(11, 2), n(0));
    assert!(!g.truth[11].bases.contains_key(&base(E)));
    assert_eq!(g.stock_of(12, E), n(9));
    assert_eq!(g.truth[11].bases[&base(2)].side, Side::Japanese);
    for d in 12..=21usize {
        assert_eq!(g.truth[d].bases[&base(2)].side, Side::Allied, "day {d}");
    }
    assert_eq!(g.notice_days(NoticeKind::Met), vec![13]);
    let nights = all_fed(1..=11);
    let at_s = g.fed(s_hex());
    assert_eq!(
        at_s.iter()
            .filter(|(d, ..)| *d <= 12)
            .cloned()
            .collect::<Vec<_>>(),
        nights
    );
    for s in 1..=3 {
        assert_eq!(g.truth[21].ships[&ship(s)].fuel_t, n(148), "ship {s}");
    }
}

/// Statement 4: case A, the supply order on day 13, and every man fed.
#[test]
fn supply_ordered_on_day_thirteen() {
    let g = game(&supply_on(13));
    assert_eq!(runs_of(&g), vec![(13, n(15)), (18, n(15))]);
    let days: Vec<_> = g.runs().iter().map(days_of).collect();
    assert_eq!(days, vec![(13, 14, 14, 17), (18, 19, 19, 22)]);
    let e: Vec<Fixed> = (12..=21).map(|d| g.stock_e(d)).collect();
    assert_eq!(e, tons(&[9, 6, 18, 15, 12, 9, 6, 18, 15, 12]));
    let b2: Vec<Fixed> = (12..=21).map(|d| g.stock_of(d, 2)).collect();
    assert_eq!(b2, tons(&[0, 0, 15, 12, 9, 6, 3, 15, 12, 9]));
    let s: Vec<Fixed> = (12..=21).map(|d| g.stock_of(d, S)).collect();
    assert_eq!(s, tons(&[167, 152, 152, 152, 152, 152, 137, 137, 137, 137]));
    let at_e = g.fed(e_hex());
    assert_eq!(at_e, all_fed(13..=21));
    assert_eq!(g.truth[21].ships[&ship(4)].fuel_t, n(196));
}

/// Statement 5: case B, the supply order on day 14, and every man fed.
#[test]
fn supply_ordered_on_day_fourteen() {
    let g = game(&supply_on(14));
    assert_eq!(runs_of(&g), vec![(14, n(18)), (19, n(15))]);
    let days: Vec<_> = g.runs().iter().map(days_of).collect();
    assert_eq!(days[0], (14, 15, 15, 18));
    assert_eq!((days[1].0, days[1].2), (19, 20));
    let e: Vec<Fixed> = (12..=21).map(|d| g.stock_e(d)).collect();
    assert_eq!(e, tons(&[9, 6, 3, 18, 15, 12, 9, 6, 18, 15]));
    assert_eq!(g.stock_of(14, S), n(149));
    assert_eq!(g.stock_of(19, S), n(134));
    assert!(g.unfed_days(e_hex()).is_empty());
    assert_eq!(g.fed(e_hex()), all_fed(13..=21));
    assert_eq!(g.stock_of(21, 2), n(15));
    assert_eq!(g.stock_of(21, E), n(0));
    assert_eq!(g.stock_of(21, S), n(134));
}

/// Statement 6: case C, the supply order on day 15, a day late.
#[test]
fn supply_ordered_on_day_fifteen_is_a_day_late() {
    let g = game(&supply_on(15));
    assert_eq!(runs_of(&g), vec![(15, n(21)), (21, n(15))]);
    let days: Vec<_> = g.runs().iter().map(days_of).collect();
    assert_eq!((days[0].0, days[0].2), (15, 16));
    assert_eq!(g.stock_of(15, S), n(146));
    assert_eq!(g.unfed_days(e_hex()), vec![16]);
    assert_eq!(
        g.fed(e_hex())
            .into_iter()
            .filter(|(d, ..)| *d == 16)
            .collect::<Vec<_>>(),
        vec![(16, 600, 0, 600)]
    );
    let b2: Vec<Fixed> = (17..=21).map(|d| g.stock_of(d, 2)).collect();
    assert_eq!(b2, tons(&[18, 15, 12, 9, 6]));
    let hungry = |d: usize| g.truth[d].land_units[&unit(1)].hunger_nights;
    assert_eq!((hungry(16), hungry(17)), (1, 0));
    assert_eq!(g.stock_of(21, 2), n(6));
    assert_eq!(g.stock_of(21, S), n(131));
}

/// Statement 7: case D, a supply order before the place is the side's is
/// refused.
#[test]
fn a_supply_order_before_the_place_is_ours_is_refused() {
    let g = game(&supply_on(12));
    assert_eq!(
        g.refused(),
        vec![
            (12, "base:2 is not one of the side's bases".to_string()),
            (12, "intent:3 is not in force".to_string()),
        ]
    );
    assert!(g.runs().is_empty());
    assert_eq!(g.unfed_days(e_hex()), (16..=21).collect::<Vec<_>>());
    for (day, men, fed, unfed) in g.fed(e_hex()) {
        assert_eq!(
            (men, fed, unfed),
            if day >= 16 {
                (600, 0, 600)
            } else {
                (600, 600, 0)
            },
            "day {day}"
        );
    }
    assert_eq!(g.truth[21].land_units[&unit(1)].hunger_nights, 6);
    assert_eq!(g.truth[21].bases[&base(2)].side, Side::Allied);
    assert_eq!(g.stock_of(21, S), n(167));
}

/// Statement 8: case E, a hold given with the landing feeds the place.
#[test]
fn a_hold_given_with_the_landing() {
    let g = game(&hold());
    let supply: Vec<_> = g
        .formed()
        .into_iter()
        .filter(|(_, _, k)| matches!(k, IntentKind::KeepSupplied { .. }))
        .collect();
    assert_eq!(
        supply,
        vec![(
            13,
            IntentId::new(2),
            IntentKind::KeepSupplied {
                base: base(2),
                floor_days: 5,
                interval_days: None
            }
        )]
    );
    assert_eq!(runs_of(&g), vec![(14, n(30)), (20, n(30))]);
    let e: Vec<Fixed> = (12..=21).map(|d| g.stock_e(d)).collect();
    assert_eq!(e, tons(&[9, 6, 3, 30, 27, 24, 21, 18, 15, 42]));
    assert_eq!(g.stock_of(14, S), n(137));
    assert_eq!(g.stock_of(20, S), n(107));
    assert!(g.unfed_days(e_hex()).is_empty());
    assert_eq!(g.fed(e_hex()), all_fed(13..=21));
    assert_eq!(g.stock_of(21, S), n(107));
}

/// Statement 9: case F, at balanced the landing comes after the last day.
#[test]
fn at_balanced_too_late() {
    let g = game(&balanced());
    let loading = |d: usize| {
        g.truth[d].task_forces[&sim_core::ids::TaskForceId::new(1)]
            .preparation
            .as_ref()
            .expect("preparing")
            .loading
    };
    assert_eq!(loading(20), n(100));
    assert_eq!(g.truth[20].land_units[&unit(1)].rehearsal, n(100));
    assert_eq!(g.notice_days(NoticeKind::Ready), vec![21]);
    assert_eq!(g.aboard(21), 600);
    let truth = &g.truth[21];
    assert!(truth
        .bases
        .values()
        .all(|b| !(b.side == Side::Allied && b.hex == e_hex())));
    assert_eq!(truth.bases[&base(2)].side, Side::Japanese);
    assert_eq!(g.stock_of(21, S), n(137));
    assert_eq!(g.fed(s_hex()), all_fed(1..=21));
    assert!(g.fed(e_hex()).is_empty());
}

/// Statement 10: case G, press given on day 17, taken late and fed by the
/// rations alone.
#[test]
fn taken_late_and_fed_by_the_rations() {
    let g = game(&late_press());
    assert_eq!(g.notice_days(NoticeKind::Ready), vec![17]);
    assert!(!g.truth[17].bases.contains_key(&base(E)));
    assert_eq!(g.stock_of(18, E), n(9));
    assert_eq!(g.truth[17].bases[&base(2)].side, Side::Japanese);
    assert_eq!(g.truth[18].bases[&base(2)].side, Side::Allied);
    assert!(g.runs().is_empty());
    assert_eq!(g.fed(s_hex()), all_fed(1..=17));
    assert_eq!(g.fed(e_hex()), all_fed(19..=21));
    assert_eq!(g.stock_of(21, E), n(0));
}

/// Statement 11: nobody is lost, and the game is the same played twice.
#[test]
fn nobody_lost_and_the_same_twice() {
    for (name, orders) in every_case() {
        let g = game(&orders);
        assert_eq!(g.lost(), 0, "case {name}");
        let end = &g.truth[21];
        let u = &end.land_units[&unit(1)];
        let aboard: u32 = end
            .ships
            .values()
            .filter_map(|s| s.troops)
            .filter(|t| t.unit == unit(1))
            .map(|t| t.men)
            .sum();
        assert_eq!(u.men_ashore + aboard, 600, "case {name}");
        assert!(end.ships.values().all(|s| s.side != Side::Japanese));
        assert!(end.land_units.values().all(|u| u.side != Side::Japanese));
        assert!(
            end.bases
                .iter()
                .filter(|(_, b)| b.side == Side::Japanese)
                .all(|(id, _)| id.raw() == 2),
            "case {name}"
        );
        let again = game(&orders);
        assert_eq!(g.scenario.world.day_hashes().len(), 21, "case {name}");
        let last = *g.scenario.world.day_hashes().last().expect("a day hash");
        let pin = format!("scenario_140_{}", name.to_lowercase());
        assert_eq!(common::pinned_hash(&pin, last), last, "case {name}");
        assert_eq!(
            g.scenario.world.day_hashes(),
            again.scenario.world.day_hashes(),
            "case {name}"
        );
    }
}
