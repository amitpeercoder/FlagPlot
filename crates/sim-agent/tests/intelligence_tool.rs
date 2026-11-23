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
//!    - `attacks_on_ships`: its `action.attacks_on_ships`, a submarine's
//!      salvoes among them;
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
//!    `day` and number. A claim from the sea puts no base in this part. The
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
//!   then day 1, each with the attack on ship 1 by torpedo as the briefing
//!   has it.
//! - Japanese, over: `claimed.days` is day 2 then day 1, one claim each,
//!   `from_submarine` true, with `taken_for`; `totals.hits` and
//!   `totals.sunk` are the sums of the two and the other two are 0.

// Each test below is named for the statements it proves and is to be
// written to them. None is built yet.

#[test]
#[ignore = "statement written, not built"]
fn statement_0_the_scenarios_do_what_they_were_chosen_for() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_1_the_parts_are_in_order_and_an_empty_part_is_left_out() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_2_seen_is_the_contacts_held_newest_first() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_3_lost_sight_of_says_the_last_report_and_which_of_two_reasons() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_4_claimed_is_every_claim_by_day_with_four_totals() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_5_what_the_enemy_has_done_by_day() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_6_ashore_is_the_estimates_held_and_those_found_empty() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_7_enemy_bases_are_only_those_of_which_something_is_held() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_8_enemy_submarines_are_the_reports_held_and_the_waters_as_told() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_9_before_the_game_is_as_given_on_every_day() {}

#[test]
#[ignore = "statement written, not built"]
fn statements_10_and_11_the_words_and_no_estimate_of_the_staffs() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_12_two_worlds_give_the_same_answer_byte_for_byte() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_13_every_handle_and_number_is_the_seats_own() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_14_when_the_game_is_over_it_still_shows_belief_only() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_15_a_reading_that_changes_no_hash_and_a_watcher_has_it() {}

#[test]
#[ignore = "statement written, not built"]
fn statement_16_with_command_levels_on_three_parts_are_not_kept_at_a_command() {}
