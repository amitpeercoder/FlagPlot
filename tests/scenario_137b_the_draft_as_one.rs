//! Scenario 137b: the draft as one, and the staff's plan beside it. The
//! commander at his seat previews "control the island" with two of its
//! landings written under it, as one draft; signs it or takes it back as
//! one; and tries it beside the same order with the landings left to the
//! staff, in the same worlds.
//!
//! Proves decision 0232, step 2 (rules 20 to 26). **It has no arithmetic
//! of the game:** it is a test of the tools, beside scenario 100's. The
//! game under it is scenario 137's, whose days are proved in
//! `tests/scenario_137_a_plan_the_commander_writes.rs`; nothing of them is
//! worked again here.
//!
//! **Written before the tools are built.** Every test below is empty and
//! marked `#[ignore = "statement written, not built"]`. The build fills
//! each to the statement above it and takes the mark off. Nobody but the
//! architect changes a statement, and never to make a test pass: a
//! mismatch comes back to the architect.
//!
//! # The game
//!
//! No file of its own. The test builds it from the text of
//! `data/scenarios/a-plan-the-commander-writes/137_a_plan_the_commander_writes.toml`:
//!
//! - every `[[command]]` is taken out (the text from the line "# The
//!   Allied commander's commands" to the end): the commander gives his
//!   commands from his seat;
//! - it names a trial's figures, `trial =
//!   "../../reference/trial_placeholder_scenario_100.toml"`, scenario
//!   100's, read from a reference root the test makes
//!   (`common::altered_reference`, on the frozen step 1 figures) in which
//!   two sizes are changed: `small` is 4 worlds for 7 days and `standard`
//!   4 worlds for 12 days. `full`, the seed 1942 and the 200 t of supply
//!   an enemy base is assumed to hold are scenario 100's. Placeholders,
//!   set by the architect for these statements; no file is added to the
//!   repository.
//!
//! It names no file of sides and the Allied side believes nothing, holds
//! no contact and no estimate: in every world of a trial the enemy is
//! bases 2 and 4 as the Allied map has them, with no man, no ship and no
//! aircraft. So the four worlds of a trial play alike, and a count of
//! worlds here is all four or none.
//!
//! The session is built as `sim-cli` builds one (scenario 100's
//! `session_with`), the Allied seat played and the Japanese seat
//! scripted, **with one thing more: the runner handed to the session is
//! wrapped by the test** so that every `TrialAsked` the session hands it
//! is kept, and then passed on unchanged to `sim-cli`'s runner. That is
//! how the test reads the commands of each trial. The test is the
//! umpire's and may read them; the seat is shown nothing more by it.
//!
//! Leave to prepare is the side's from the start. Unless a statement
//! says otherwise, **the evening** is the evening before day 1 with one
//! command queued, `set_leave`: leave to launch, given. It is command 1
//! of the queue.
//!
//! ## The drafts
//!
//! **Draft W, "with the waits"**, scenario 137's main game: `preview` of
//!
//! ```json
//! {"command": "set_intent", "intent_kind": "control_island", "island": 1,
//!  "under": [
//!    {"intent_kind": "take_place", "hex": [0, 6],
//!     "limits": [{"limit": "forces", "task_forces": [3], "land_units": [3]}]},
//!    {"intent_kind": "take_place", "hex": [4, 0],
//!     "limits": [{"limit": "after", "other_intent": 2},
//!                {"limit": "not_before", "from_day": 4}]}
//!  ]}
//! ```
//!
//! No number is given. The order is intent 1, the side's next free
//! number; the take of C is intent 2 and the take of B intent 3. The
//! batch is six commands, in this order:
//!
//! | Place | Command |
//! | ----- | ------- |
//! | 1 | `set_intent` 1, control island 1 |
//! | 2 | `set_intent` 2, take the place at (0,6), parent 1 |
//! | 3 | `set_limit`, forces, on intent 2: task force 3 and land unit 3 |
//! | 4 | `set_intent` 3, take the place at (4,0), parent 1 |
//! | 5 | `set_limit`, "after intent 2", on intent 3 |
//! | 6 | `set_limit`, "not before day 4", on intent 3 |
//!
//! **Draft N, "no waits"**: draft W without the two limits of the second
//! sub-intent. Four commands.
//!
//! **Draft O, "the order alone"**: `preview` of `{"command":
//! "set_intent", "intent_kind": "control_island", "island": 1}`. One
//! command, and nothing written.
//!
//! What scenario 137 proves of the game these make, and all that these
//! statements rest on: with the landings left to the staff, and with
//! draft N, both places are the side's from the end of day 5; with draft
//! W the anchorage is the side's from the end of day 5 and the airfield
//! from the end of day 9, since its plan cannot be released before the
//! morning after the anchorage is taken and the landing then takes three
//! days to embark. The island's end is both places.
//!
//! # Rule 20: a preview takes an order with its written part
//!
//! ## 1. The draft is read as one
//!
//! On the evening, draft W is answered with the kind `preview`, `valid`
//! true, and:
//!
//! - `preview` 1, `day` 1, `intent` 1; `command` is the order's, and
//!   `limits` and `shown_limits` are empty lists;
//! - `under`, a list of two, in the order given. Each entry has `intent`
//!   (2, then 3), `command` (the sub-intent's `set_intent` as taken in,
//!   with `parent` 1), `shown` (its words), and `limits` and
//!   `shown_limits` (one, then two);
//! - `with_change` is the staff's reading of the whole draft, the order
//!   with both landings in force under it and the island's plan read as
//!   released. Its `tree` has first an entry for intent 2 and one for
//!   intent 3, each with `parent` 1, `maker` "commander" and `standing`
//!   true; then the staff's entries, numbered from 4, each with `maker`
//!   "staff". None of the staff's entries is a "take a place". Among
//!   them are a "ready a landing" by task force 3 at (0,6) with `parent`
//!   2 and one by task force 1 at (4,0) with `parent` 3. Its
//!   `no_orders` hold rule 9's line for intent 2 and for intent 3;
//! - `as_things_stand` has no order and no command for intent 1: it is
//!   not there.
//!
//! Nothing is queued: `queued` lists command 1 alone, with the `as_of`
//! it had. The session's hash of the game is what it was.
//!
//! ## 2. The numbers
//!
//! A sub-intent given no number takes one past the highest number the
//! side has ever set, has queued, or that stands before it in the batch.
//! Each alone, on a fresh evening, each answered as a valid preview:
//!
//! | The side has queued | The order's `intent` | The first sub-intent's | The second's | The numbers |
//! | --- | --- | --- | --- | --- |
//! | nothing more | none given | none | none | 1, 2 and 3 (draft W) |
//! | a root hold of base 1, his intent 1 | none | none | none, its "after" naming 3 | 2, 3 and 4 |
//! | nothing more | 4 | 7 | none, its "after" naming 7 | 4, 7 and 8 |
//! | nothing more | none | none | 9, its "after" naming 2 | 1, 2 and 9 |
//!
//! In each, the answer's `intent` is the order's number and `under`
//! gives each sub-intent's.
//!
//! ## 3. One the engine would refuse: that refusal, with its place
//!
//! Each alone, on the evening. The answer is of the kind `command`, with
//! `tool` "preview", `status` "refused", `command` the command refused,
//! `reason` the engine's words, and `of_batch` its place and the size of
//! the batch. The order is place 1; then its limits; then each
//! sub-intent, followed by its own limits, in the order given.
//!
//! | The draft | Refused | `of_batch` | The reason |
//! | --- | --- | --- | --- |
//! | The order; the take of C with the limit "after intent 3"; the take of B | the "after": intent 3 is not yet set when it is taken in | place 3 of 4 | "No such intent": intent 3 is not in force |
//! | The order; the take of B; the take of B again | the second take | place 3 of 3 | "Already planned": intent 2 already plans the place at (4,0) |
//! | The order with the limit "risk press"; the take of C; a take of the place at (0,0) | the take of (0,0), which is no place of the island | place 4 of 4 | "Cannot stand under", naming intent 1 |
//! | Draft N with the limit "risk press" on the take of B | the risk: press is wider than the island's balanced | place 5 of 5 | the limit is wider than what its parent, intent 1, allows |
//! | The order as intent 4; the take of C as intent 2 | the take: its number is not above the order's | place 2 of 2 | "Intent number not usable": intent 2 |
//!
//! After all five, asked one after another on one evening: `queued`
//! gives the entries and the `as_of` it gave before them; and draft W,
//! previewed next, is preview 1. Nothing was numbered and nothing
//! queued.
//!
//! ## 4. What is an error of the call
//!
//! Not a refusal of the engine's and not an answer: an error, as a bad
//! `limits` is today, that names what is wrong.
//!
//! - `under` with `"command": "set_limit"`: it is given with a
//!   `set_intent`;
//! - `under` that is not a list, or an entry of it that is not an object:
//!   the error names `under`, and the entry by its place, `under[0]`;
//! - an entry of `under` that carries `parent`: the error names
//!   `under[0]` and `parent`, and says the sub-intent stands under that
//!   order;
//! - a limit of an entry that carries `intent`, or that is a reserve: as
//!   for the order's own `limits`, the error naming `under[1].limits[0]`.
//!
//! Each numbers nothing and queues nothing.
//!
//! # Rule 21: signed as one, taken back as one
//!
//! ## 5. `approve` queues all of it, in order
//!
//! On the evening, after draft W is shown as preview 1, `approve` of
//! preview 1 is answered with `status` "queued", `preview` 1, `intent`
//! 1, `queued` 7, `limits_queued` 0 and `under_queued` `[{"intent": 2,
//! "limits_queued": 1}, {"intent": 3, "limits_queued": 2}]`.
//!
//! `queued` then lists seven commands: the leave, and the six of the
//! batch in the batch's order, as places 2 to 7 of the queue `commands`.
//!
//! `end_day` resolves day 1. The side's commands taken in on day 1 are
//! those seven and none is refused. In the command state intents 1, 2
//! and 3 are in force, the commander's; intents 2 and 3 stand under
//! intent 1; intent 2 has the forces limit, and intent 3 the two waits.
//! The staff's intents are numbered from 4 and none is a "take a place":
//! it is scenario 137's first morning.
//!
//! ## 6. Or none of it
//!
//! On the evening, after draft W is shown as preview 1, the commander
//! queues one thing more (`set_leave`: leave to prepare, given). Then
//! `approve` of preview 1 is refused as void, saying what was queued
//! after it, and `queued` lists the two leaves and nothing of the batch.
//!
//! With statement 3 this is "whole or not at all": a batch one of whose
//! commands the engine would refuse is never numbered, so it cannot be
//! approved; a batch that is numbered is queued whole or, void, not at
//! all.
//!
//! ## 7. `take_back`
//!
//! Each on a fresh evening with draft W approved, the queue `commands`
//! holding: 1 the leave; 2 the order; 3 the take of C; 4 its forces
//! limit; 5 the take of B; 6 its "after intent 2"; 7 its "not before".
//!
//! - **The order takes its written part with it.** `take_back` of
//!   command 2 takes commands 2 to 7: the answer names all six as taken,
//!   and `queued` lists the leave alone.
//! - **A sub-intent takes its own limits.** `take_back` of command 5
//!   takes commands 5, 6 and 7, and `queued` lists commands 1 to 4 as
//!   they were. Then `take_back` of command 3 takes commands 3 and 4,
//!   and `queued` lists the leave and the order.
//! - **A sub-intent another command rests on is not taken back.** With
//!   all seven queued, `take_back` of command 3, the take of C, is
//!   refused: the "after intent 2" of command 6 rests on it, and the
//!   words name that command and its place and say to take that back
//!   first. The queue and its `as_of` are as they were. It is the rule
//!   as it is today for any queued command that would no longer be taken
//!   in.
//! - **A limit alone** goes alone: `take_back` of command 7 takes it and
//!   nothing else.
//!
//! # Rule 22: a sub-intent under an order that stands
//!
//! ## 8. Previewed as one `set_intent` with `parent`, by the preview as it is
//!
//! On the evening, draft O is previewed (preview 1) and approved: the
//! queue holds the leave and the order, intent 1, queued and not in
//! force.
//!
//! - `preview` of `{"command": "set_intent", "parent": 1, "intent_kind":
//!   "take_place", "hex": [0, 6], "limits": [{"limit": "forces",
//!   "task_forces": [3], "land_units": [3]}]}` is preview 2, valid:
//!   `intent` 2, the next free number; `command` has `parent` 1;
//!   `limits` has the one limit; `under` is an empty list. `approve` of
//!   it queues two commands, with `limits_queued` 1, and the queue holds
//!   four.
//! - On a fresh evening with the order queued so, the same preview with
//!   the limit "risk press" in place of the forces limit is refused as
//!   wider than its parent allows, `of_batch` place 2 of 2: as an
//!   order's limit is today.
//! - With the take of C queued as above, a second take of C under intent
//!   1, previewed, is refused as "Already planned", naming intent 2. One
//!   command: no `of_batch`.
//! - **Under an order in force.** With the order and the leave queued
//!   and nothing written, `end_day` resolves day 1: the staff forms its
//!   own takes of both places. On the evening of day 1 a take of the
//!   place at (4,0) with `parent` 1, previewed, is refused as "Already
//!   planned", naming the staff's take of that place (decision 0232,
//!   Question 3: a place the staff has planned stays the staff's).
//!
//! # Rules 23 to 25: the staff's own plan beside it
//!
//! A trial's size is `small`, 4 worlds for 7 days, unless a statement
//! says `standard`, 4 worlds for 12.
//!
//! ## 9. The second trial's commands are the first's less the written part
//!
//! On the evening, draft W is shown as preview 1 and `trial` of preview
//! 1 is asked with no `compare`. The session hands the runner one
//! `TrialAsked`, which the test's wrapper kept:
//!
//! | | The first trial | The second trial |
//! | --- | --- | --- |
//! | Standing commands | the leave to launch | the leave to launch |
//! | Course, all on day 1 | the six commands of the batch, in the batch's order | the order, command 1 of the batch |
//!
//! - `intent` is intent 1, the order: it is in both trials.
//! - `second.of` is the staff's plan: `BesideOf::LeftToTheStaff`, named
//!   `left_to_the_staff`.
//! - **Less the written part, and nothing else:** the second's standing
//!   commands are the first's, command for command; and the first's
//!   course, with every step taken out that is a `set_intent` with a
//!   parent or a limit on intent 2 or intent 3, is the second's course,
//!   step for step.
//! - No leave is given by the trial in either: the side holds leave to
//!   prepare and the commander has queued leave to launch.
//!
//! **The same leaves and the same settings in both.** On an evening with
//! nothing queued (no leave to launch): the first's course is the batch
//! and then what the trial gives, the leave to launch and whatever
//! setting of an order is carried with it (decision 0214); the second's
//! is the order and then the same, command for command. The rule of the
//! bullet above holds as written.
//!
//! **With limits on the order.** Draft W with the order's own `limits`
//! `[{"limit": "risk", "risk": "press"}]`: the batch is seven, and the
//! second's course is the order and its risk, places 1 and 2 of the
//! batch.
//!
//! ## 10. Both trials play the same worlds
//!
//! How the test sees it:
//!
//! - **One asking.** Both trials are in the one `TrialAsked` of
//!   statement 9: one trial's number, 1; one size; begun by the runner
//!   on the one view, with the side's one set of beliefs and the trial's
//!   one seed. A world of a trial is made from those and its own number
//!   and from nothing of a course (chapter 9, Playing a trial, rule 1).
//! - **The pictures are the same.** The test plays the two sets of
//!   commands of statement 9 itself, through `sim_trial`, on the
//!   scenario's own Allied view of the evening, with the same figures,
//!   trial's number and size: for each world, 1 to 4, the list of what
//!   its picture assumed is the same in the two plays, item for item.
//! - **The second trial is the trial of the order alone.** On the same
//!   evening draft O is shown and `trial` of it asked with `compare`
//!   false. Its `how_it_went`, `last_day`, `could_not_be_played`, `cost`
//!   and `enemy_lost` are, field for field, those of `beside` in the
//!   answer of statement 11 for draft W, and in the answer for draft N.
//!
//! ## 11. The answer: what the second trial is of, and the two world for world
//!
//! `trial` of preview 1 (draft W), no `compare` given, on the evening.
//! The answer is of the kind `trial`:
//!
//! - `asked`: `preview` 1, `intent` null, `tried_intent` 1, `compare`
//!   true, `second_trial` true, `beside` "left_to_the_staff", `in_force`
//!   false, `dates` null; `standing` is the one leave in words and
//!   `course` the six commands of the batch in words, in order.
//! - `how_it_went` is the order's with his plan: not met by day 7 in
//!   worlds 1 to 4, met and kept in none.
//! - `beside.of` is "left_to_the_staff"; `beside.order` is 1, the order
//!   whose end is read in both trials; `beside.how_it_went` is the
//!   order's with the landings left to the staff: met and kept in worlds
//!   1 to 4; `beside.with_your_plan` is the order's in the first trial,
//!   here equal to `how_it_went`.
//! - `beside.world_for_world`: `both` none; `with_your_plan_alone` none;
//!   `with_the_staffs_alone` worlds 1, 2, 3 and 4; `neither` none;
//!   `apart` none.
//! - **In words.** One line says "Beside it, the same order with the
//!   landings left to the staff, in the same worlds", and how the order
//!   went there. One line gives the tally: "your plan against the
//!   staff's", "met and kept in both in none", "with your plan alone in
//!   none", "with the staff's plan alone in 4 worlds (worlds 1, 2, 3 and
//!   4)", "in neither in none".
//!
//! The tally for each draft and size, the order's end read in both
//! trials:
//!
//! | The draft | Size | Both | With his plan alone | With the staff's alone | Neither |
//! | --- | --- | --- | --- | --- | --- |
//! | W, with the waits | `small`, 7 days | none | none | 1, 2, 3 and 4 | none |
//! | N, no waits | `small` | 1, 2, 3 and 4 | none | none | none |
//! | W, with the waits | `standard`, 12 days | 1, 2, 3 and 4 | none | none | none |
//!
//! In the third row the day his plan meets the order's end is later than
//! the day the staff's does, in every world: the middle day of
//! `how_it_went` is greater than that of `beside.how_it_went`. (Scenario
//! 137 has them as day 9 and day 5. The days a trial gives are reported
//! by the build for the architect and not asserted here beyond that
//! relation.)
//!
//! It gives no advice: neither plan is called the better.
//!
//! ## 12. The costs
//!
//! In each answer of statement 11: `cost` and `enemy_lost`, and
//! `beside.cost` and `beside.enemy_lost`, each hold the same five costs
//! (men lost to the enemy, men lost unfed, ships sunk, ships out of
//! action, aircraft lost), each as the lowest, the middle and the
//! highest: the second trial's are given as the first's are.
//!
//! No number of a cost is worked here. What is asserted:
//!
//! - men lost to the enemy, ships sunk and aircraft lost are 0, 0 and 0
//!   for the side and for the trial's enemy, in both trials: no world
//!   holds an enemy man, ship or aircraft;
//! - `beside.cost` and `beside.enemy_lost` are those of the trial of the
//!   order alone (statement 10);
//! - the words give the second trial's costs in two lines that begin
//!   "Beside it,", as the first's are given.
//!
//! ## 13. A new sub-intent under an order that stands
//!
//! On the evening, draft O is previewed and approved: the queue holds
//! the leave and the order. Then `preview` of `{"command": "set_intent",
//! "parent": 1, "intent_kind": "take_place", "hex": [4, 0], "limits":
//! [{"limit": "not_before", "from_day": 8}]}` is preview 2, and its
//! sub-intent is intent 2. `trial` of preview 2, no `compare` given:
//!
//! | | The first trial | The second trial |
//! | --- | --- | --- |
//! | Standing commands | the leave; the order | the leave; the order |
//! | Course | the take of B, parent 1; its "not before day 8" | empty |
//!
//! - `intent` is intent 2, the thing tried. `second.of` is
//!   `LeftToTheStaff`: the order with that place left to the staff.
//! - The answer's `how_it_went` is the landing's: the place at (4,0) is
//!   not the side's by day 7 in worlds 1 to 4 (its plan is not released
//!   before day 8).
//! - `asked.beside` and `beside.of` are "left_to_the_staff";
//!   `beside.order` is 1; `beside.with_your_plan`, the order's end with
//!   his landing, is not met in worlds 1 to 4; `beside.how_it_went`, the
//!   order's end with that place left to the staff, is met and kept in
//!   worlds 1 to 4; `world_for_world` is the staff's alone in worlds 1
//!   to 4.
//! - The words say "Beside it, the same order with the landing at (4,0)
//!   left to the staff, in the same worlds".
//!
//! **Queued, it reads the same.** `approve` of preview 2, then `trial`
//! of intent 2 with no `compare`: the first trial has the four standing
//! commands and an empty course; the second has the standing commands
//! less the two that set intent 2 and its limit, and an empty course.
//! Its `how_it_went`, `cost`, `enemy_lost` and `beside` are those of the
//! trial of preview 2, field for field.
//!
//! ## 14. A sub-intent in force
//!
//! On the evening draft W is approved and `end_day` resolves day 1.
//! Nothing is queued on the evening of day 1. `trial` of intent 3, the
//! take of B, no `compare` given: two trials are played (rule 24), of
//! days 2 to 8.
//!
//! - Both have no standing command. The first's course is empty. The
//!   second's is one command, on the coming morning, before the staff is
//!   asked: `withdraw_intent` 3. `second.of` is
//!   `BesideOf::GivenBackToTheStaff`, named `given_back_to_the_staff`.
//! - `asked.in_force` is true; `asked.beside` and `beside.of` are
//!   "given_back_to_the_staff"; `beside.order` is 1.
//! - `beside.with_your_plan`: the order's end is not met by day 8 in
//!   worlds 1 to 4. `beside.how_it_went`: met and kept in worlds 1 to 4,
//!   the staff forming its own take of B on the morning of day 2 with
//!   what was already ordered standing (rule 15). `world_for_world` is
//!   the staff's alone in worlds 1 to 4.
//! - The words say "Beside it, the same order with the landing at (4,0)
//!   given back to the staff on the coming morning, in the same worlds",
//!   that what was already ordered stands, and that the withdrawal is
//!   the trial's and not his: intent 3 is not withdrawn in his game.
//! - After it intent 3 is in force and the hash is what it was.
//!
//! With `compare` false one trial is played and `beside` is null.
//!
//! **The order itself, in force,** is as it was before this step:
//! `trial` of intent 1 with no `compare` plays one trial, and the words
//! say the order is in force and that `compare` shows what it costs.
//!
//! ## 15. A queued order with its written part
//!
//! On the evening, draft W approved: seven commands queued, none in
//! force.
//!
//! - **The order is read as its preview was.** `trial` of intent 1, no
//!   `compare` given: the first trial has the seven standing commands
//!   and an empty course; the second has the standing commands less the
//!   five of the written part (commands 3 to 7 of the queue): the leave
//!   and the order. `second.of` is `LeftToTheStaff`. The answer's
//!   `how_it_went`, `cost`, `enemy_lost` and `beside` are those of
//!   statement 11's first answer, field for field.
//! - **A sub-intent nothing else names.** `trial` of intent 3, no
//!   `compare` given: the second has the standing commands less commands
//!   5, 6 and 7. `beside.of` is "left_to_the_staff", `beside.order` 1,
//!   and `world_for_world` the staff's alone in worlds 1 to 4.
//! - **A sub-intent another queued command names.** `trial` of intent 2,
//!   with no `compare` given and with `compare` true, is refused, nothing
//!   played: the "after intent 2" on intent 3 stands among his commands
//!   and names intent 2, and the words name that command (chapter 9,
//!   What-ifs from a seat, rule 9, as it is). With `compare` false it is
//!   played, one trial.
//!
//! ## 16. Where `compare` is not given (rule 24)
//!
//! | What is tried | Second trial | Where stated |
//! | --- | --- | --- |
//! | A preview of a new order with sub-intents | played | 11 |
//! | A preview of a new sub-intent; a sub-intent queued | played | 13, 15 |
//! | A sub-intent in force | played | 14 |
//! | A queued order with sub-intents queued under it | played | 15 |
//! | Draft O, a new order with nothing written, the side having no other plan | not played, as before: the words say the side has no other plan to weigh | here |
//! | The order in force | not played, as before | 14 |
//!
//! And with `compare` false, `trial` of preview 1 (draft W) plays one
//! trial: `asked.second_trial` is false and `beside` null, and its
//! `how_it_went` and `cost` are those of statement 11's first answer.
//!
//! ## 17. Asked before it is played
//!
//! `trial` of preview 1 (draft W) with `describe` true: the kind is
//! `trial_described`; `asked.beside` is "left_to_the_staff"; `plays` has
//! two entries, the first and the second trial, each of 4 worlds for 7
//! days; and the words say "Two trials of 4 worlds of 7 days each, in
//! the same worlds: the first with the draft, the second the same order
//! with the landings left to the staff." Nothing is played, the hash is
//! what it was, and `trials` lists nothing more.
//!
//! # Rule 26: no window for a written part
//!
//! ## 18. `dates` is refused
//!
//! Each alone, each an error with nothing played, the hash what it was
//! and nothing more in the list `trials` gives. The words of each hold
//! the sentence: `A written landing is held to a day by its "not
//! before", which can be previewed and compared`.
//!
//! - `trial` of preview 1 (draft W) with `dates` "chosen", and with
//!   `dates` `[4]`: the words begin "preview 1 carries landings you
//!   wrote: a window is not offered for a written part."
//! - `trial` of the preview of statement 13 (a new sub-intent under an
//!   order that stands) with `dates` "chosen": "preview 2 is of a
//!   landing you wrote under intent 1: a window is not offered for a
//!   written part."
//! - With draft W approved: `trial` of intent 3 with `dates` "chosen":
//!   "intent 3 is a landing you wrote under intent 1: a window is not
//!   offered for a written part."
//! - With draft W approved: `trial` of intent 1 with `dates` "chosen":
//!   "intent 1 has landings you wrote under it: a window is not offered
//!   for a written part." (A window would hand the order in later and
//!   leave its landings behind.)
//! - Each of the four with `describe` true is refused the same.
//! - **In force,** on the evening of day 1 of statement 14: `trial` of
//!   intent 3 with `dates` "chosen" is refused as an order in force is
//!   today, in the words chapter 9 has, which name a "not before".
//!
//! Draft O, with nothing written, is given its window as before:
//! `trial` of it with `dates` `[4]` is answered with the kind `window`.
//!
//! # Properties
//!
//! - **Nothing is changed.** The session's hash of the game is the same
//!   before and after every `preview` and every `trial` above, and the
//!   queue is the same. A preview tried is not spent: after statement
//!   11's trials, `approve` of preview 1 queues the six.
//! - **Asked twice, the same.** Statement 11's first question asked
//!   again on the same evening gives the same fields and words, but for
//!   `estimate`.
//! - **One estimate each.** After statement 11's three questions,
//!   `trials` lists three estimates, each of the kind `trial`, and gives
//!   each again unchanged.
//! - **The live enemy is not read.** The same game with a Japanese
//!   fighting unit of 300 men added ashore at B, which the Allied side
//!   is told nothing of: statement 11's first answer is the same, byte
//!   for byte.
//! - **The words** of every answer above hold no per cent sign and none
//!   of "chance", "points", "better", "should", "recommend" or "advise".
//! - **The answers are checked against their schemas** (`preview`,
//!   `command`, `trial`, `trial_described`), and each carries the
//!   version of the agent's interface, 57.
//!
//! # What this assumes of the build
//!
//! Decision 0232's rules 20 to 26, none of them code when this was
//! written, as its section "Found in writing step 2's statements" makes
//! them exact:
//!
//! - `preview` of a `set_intent` takes `under`; its answer gains `under`;
//!   `of_batch` counts the whole batch; `approve`'s answer gains
//!   `under_queued`, and its `limits_queued` counts the order's own
//!   limits;
//! - `take_back` of a `set_intent` that makes a new intent takes with it
//!   the queued `set_intent`s that name it as their parent, and what
//!   goes with each of those;
//! - `BesideOf` gains `LeftToTheStaff` and `GivenBackToTheStaff`; the
//!   second trial is asked as rule 23 has it, with the same leaves and
//!   the same carried settings as the first;
//! - the answer's `beside` gains `order` and `with_your_plan`, and its
//!   `world_for_world` has the keys `both`, `with_your_plan_alone`,
//!   `with_the_staffs_alone`, `neither` and `apart`, each null or absent
//!   for the kinds of second trial that exist today, whose answers do
//!   not change;
//! - `compare` not given, by rule 24; `dates` refused, by rule 26;
//! - the agent's interface is version 57.
//!
//! The words the statements quote are asserted as quoted. Any other word
//! is the build's to choose; the tests read it for what this comment
//! says it names. Intent and preview numbers, places in the queue and in
//! a batch, and counts of worlds are asserted as given.

#![allow(clippy::doc_lazy_continuation, clippy::doc_markdown)]

/// Statement 1: draft W previewed on the evening is preview 1, of intent
/// 1, with `under` giving intents 2 and 3, each with its command, its
/// parent and its limits; the staff's reading is of the whole draft (the
/// two standing entries, no take of the staff's, a landing under each);
/// nothing is queued and the hash is what it was.
#[test]
#[ignore = "statement written, not built"]
fn s01_the_draft_is_read_as_one() {
    todo!()
}

/// Statement 2: the four rows of the table of numbers, each a valid
/// preview with the numbers given.
#[test]
#[ignore = "statement written, not built"]
fn s02_the_numbers() {
    todo!()
}

/// Statement 3: the five drafts the engine would refuse, each answered
/// as that refusal with its place in the batch and the batch's size;
/// after all five nothing is queued and draft W is preview 1.
#[test]
#[ignore = "statement written, not built"]
fn s03_a_refusal_with_its_place_in_the_batch() {
    todo!()
}

/// Statement 4: the errors of the call, each naming what is wrong, each
/// numbering and queueing nothing.
#[test]
#[ignore = "statement written, not built"]
fn s04_an_error_of_the_call() {
    todo!()
}

/// Statement 5: `approve` of preview 1 queues the six commands in the
/// batch's order, with `limits_queued` 0 and `under_queued` as given;
/// day 1 takes in the seven, none refused, and the command state is
/// scenario 137's first morning.
#[test]
#[ignore = "statement written, not built"]
fn s05_approve_queues_all_of_it_in_order() {
    todo!()
}

/// Statement 6: something queued after the preview voids it, and
/// `approve` queues nothing of the batch.
#[test]
#[ignore = "statement written, not built"]
fn s06_or_none_of_it() {
    todo!()
}

/// Statement 7, the first bullet: `take_back` of the order takes
/// commands 2 to 7.
#[test]
#[ignore = "statement written, not built"]
fn s07_take_back_of_the_order_takes_its_written_part() {
    todo!()
}

/// Statement 7, the second and fourth bullets: `take_back` of the take
/// of B takes its two limits; then of the take of C its forces limit;
/// and a limit alone goes alone.
#[test]
#[ignore = "statement written, not built"]
fn s07_take_back_of_a_sub_intent_takes_its_limits() {
    todo!()
}

/// Statement 7, the third bullet: `take_back` of the take of C while the
/// "after intent 2" is queued is refused, naming command 6, and the
/// queue is as it was.
#[test]
#[ignore = "statement written, not built"]
fn s07_a_sub_intent_another_rests_on_is_not_taken_back() {
    todo!()
}

/// Statement 8: under a queued order, a sub-intent previewed as one
/// `set_intent` with `parent` and its limits, approved; a limit of it
/// refused with its place; a second take of the place refused as
/// "Already planned"; and under an order in force, a take of a place
/// the staff has planned refused as "Already planned".
#[test]
#[ignore = "statement written, not built"]
fn s08_a_sub_intent_under_an_order_that_stands() {
    todo!()
}

/// Statement 9: the `TrialAsked` of draft W's trial: the standing
/// commands of both trials, the course of each, and that the first's
/// course less the written part is the second's, step for step.
#[test]
#[ignore = "statement written, not built"]
fn s09_the_second_trials_commands_are_the_firsts_less_the_written_part() {
    todo!()
}

/// Statement 9, "The same leaves and the same settings in both" and
/// "With limits on the order".
#[test]
#[ignore = "statement written, not built"]
fn s09_the_same_leaves_and_the_orders_limits_in_both() {
    todo!()
}

/// Statement 10: one asking, one trial's number and one size; the
/// pictures of worlds 1 to 4 assume the same things in the two plays;
/// and `beside` is the trial of the order alone, field for field, for
/// draft W and for draft N.
#[test]
#[ignore = "statement written, not built"]
fn s10_both_trials_play_the_same_worlds() {
    todo!()
}

/// Statement 11: the answer for draft W at `small`: what was asked, how
/// the order went with his plan and with the staff's, the tally, and the
/// words quoted.
#[test]
#[ignore = "statement written, not built"]
fn s11_the_answer_names_the_staffs_plan_and_gives_the_tally() {
    todo!()
}

/// Statement 11's table: the tally for draft N at `small` and for draft
/// W at `standard`, and in the last the staff's day earlier than his.
#[test]
#[ignore = "statement written, not built"]
fn s11_the_tally_for_each_draft_and_size() {
    todo!()
}

/// Statement 12: the five costs of each side in both trials, given
/// alike; the three that are 0; the second's equal to the trial of the
/// order alone.
#[test]
#[ignore = "statement written, not built"]
fn s12_the_costs() {
    todo!()
}

/// Statement 13: a new sub-intent under a queued order, previewed and
/// tried: the commands of both trials, the landing's own end, the
/// order's end in each and the tally; and, queued, the same answer.
#[test]
#[ignore = "statement written, not built"]
fn s13_a_new_sub_intent_under_an_order_that_stands() {
    todo!()
}

/// Statement 14: a sub-intent in force tried with no `compare`: two
/// trials, the second handed its withdrawal; the tally; the words; and
/// nothing withdrawn in the game. With `compare` false, one. The order
/// in force is as it was.
#[test]
#[ignore = "statement written, not built"]
fn s14_a_sub_intent_in_force() {
    todo!()
}

/// Statement 15: with draft W queued, the order tried reads as its
/// preview did; a sub-intent nothing names is tried beside the staff's
/// plan of its place; a sub-intent an "after" names is refused, naming
/// the command, and played with `compare` false.
#[test]
#[ignore = "statement written, not built"]
fn s15_a_queued_order_with_its_written_part() {
    todo!()
}

/// Statement 16: where `compare` is not given, the rows stated here
/// (draft O, and draft W with `compare` false).
#[test]
#[ignore = "statement written, not built"]
fn s16_where_compare_is_not_given() {
    todo!()
}

/// Statement 17: draft W described: two trials of 4 worlds for 7 days,
/// the words quoted, nothing played and nothing kept.
#[test]
#[ignore = "statement written, not built"]
fn s17_asked_before_it_is_played() {
    todo!()
}

/// Statement 18: `dates` refused for a written part, each case with its
/// words, nothing played; and draft O given its window as before.
#[test]
#[ignore = "statement written, not built"]
fn s18_dates_is_refused_for_a_written_part() {
    todo!()
}

/// Properties: nothing is changed, a preview tried is not spent, asked
/// twice the same, one estimate each.
#[test]
#[ignore = "statement written, not built"]
fn nothing_is_changed() {
    todo!()
}

/// Property: the live enemy is not read.
#[test]
#[ignore = "statement written, not built"]
fn the_live_enemy_is_not_read() {
    todo!()
}

/// Properties: the words hold no advice, the answers pass their schemas
/// and carry version 57 of the agent's interface.
#[test]
#[ignore = "statement written, not built"]
fn the_words_the_schemas_and_the_version() {
    todo!()
}
