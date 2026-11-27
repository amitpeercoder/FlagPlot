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
//! In the seat's own numbers the game's task force 3 is task force 2 (a
//! seat numbers its forces 1, 2, ...): where a draft below writes
//! "task_forces": [3] and a statement says "task force 3", the seat
//! gives and is given 2. Land unit 3 is 3.
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
//! - **A change to a queued written part.** With draft W approved:
//!   `preview` of `{"command": "set_limit", "limit": "not_before",
//!   "intent": 3, "from_day": 6}` is preview 2, and `trial` of it with
//!   `dates` "chosen" is refused: "preview 2 is of a landing you wrote
//!   under intent 1: a window is not offered for a written part." Then
//!   `preview` of `{"command": "set_limit", "limit": "not_before",
//!   "intent": 1, "from_day": 2}` is preview 3, and `trial` of it with
//!   `dates` "chosen" is refused: "preview 3 is of intent 1, which has
//!   landings you wrote under it: a window is not offered for a written
//!   part." Each holds the sentence above; nothing is played.
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

use crate::common;

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};

use serde_json::{json, Value};
use sim_agent::{
    BesideOf, CallError, Described, Seat, Session, TrialAsked, TrialRefused, TrialRunner,
    TrialUnderWay,
};
use sim_core::command::CommandKind;
use sim_core::state::Side;
use sim_data::{LoadOptions, Scenario};
use sim_staff::staffs;

#[allow(dead_code)]
#[path = "../crates/sim-agent/tests/support/validator.rs"]
mod validator;

const ALLIED: Side = Side::Allied;

/// The task force the statements name for the landing at C, "task force 3".
const FORCE_OF_C: u64 = 2;

fn path() -> PathBuf {
    common::data_dir()
        .join("scenarios/a-plan-the-commander-writes/137_a_plan_the_commander_writes.toml")
}

/// Scenario 137's text with every `[[command]]` taken out and the trial's
/// figures named.
fn game_text() -> String {
    let full = std::fs::read_to_string(path()).expect("the scenario file reads");
    let cut = full
        .find("# The Allied commander's commands")
        .expect("the commands begin");
    let head = &full[..cut];
    let ground = "ground = \"../../reference/ground_placeholder_scenario_76.toml\"\n";
    common::replace_once(
        head,
        ground,
        &format!("{ground}trial = \"../../reference/trial_placeholder_scenario_100.toml\"\n"),
    )
}

/// The frozen step 1 figures with scenario 100's trial figures, two sizes
/// changed: `small` is 4 worlds for 7 days and `standard` 4 worlds for 12.
fn options() -> &'static LoadOptions {
    static OPTIONS: OnceLock<LoadOptions> = OnceLock::new();
    OPTIONS.get_or_init(|| {
        common::altered_reference("s137b", |dir| {
            common::edit_file(dir, "trial_placeholder_scenario_100.toml", |text| {
                let text = common::replace_once(
                    &text,
                    "[small]\nworlds = 4\ndays = 5",
                    "[small]\nworlds = 4\ndays = 7",
                );
                common::replace_once(
                    &text,
                    "[standard]\nworlds = 8\ndays = 5",
                    "[standard]\nworlds = 4\ndays = 12",
                )
            })
        })
    })
}

fn load(text: &str) -> Scenario {
    common::from_text_with(text, &path(), options()).expect("the scenario loads")
}

/// What a session hands its runner, kept; the trial itself is played by
/// `sim-cli`'s runner, unchanged.
struct Keeping {
    inner: Box<dyn TrialRunner>,
    asked: Arc<Mutex<Vec<TrialAsked>>>,
}

impl TrialRunner for Keeping {
    fn begin(
        &self,
        view: &sim_core::view::SideView<'_>,
        asked: &TrialAsked,
    ) -> Result<Box<dyn TrialUnderWay>, TrialRefused> {
        self.asked.lock().expect("kept").push(asked.clone());
        self.inner.begin(view, asked)
    }

    fn describe(
        &self,
        view: &sim_core::view::SideView<'_>,
        asked: &TrialAsked,
    ) -> Result<Described, TrialRefused> {
        self.inner.describe(view, asked)
    }
}

/// A session built as `sim-cli` builds one, the Allied seat played and the
/// Japanese scripted, with the runner wrapped.
struct Game {
    session: Session,
    asked: Arc<Mutex<Vec<TrialAsked>>>,
}

fn game_of(text: &str) -> Game {
    let mut scenario = load(text);
    let inner = sim_cli::seat_trial::SeatTrials::runner_of(&scenario).expect("it has a trial");
    let asked = Arc::new(Mutex::new(Vec::new()));
    let runner = Keeping {
        inner,
        asked: asked.clone(),
    };
    let commands = scenario.scripted_commands().clone();
    scenario.replace_commands(BTreeMap::new());
    let inside = scenario.orders_inside_a_day();
    let script_inside = scenario.scripted_orders_inside_day().clone();
    let name = scenario.name.clone();
    let session = Session::new(
        scenario.world,
        &name,
        scenario.days,
        Seat::Player,
        Seat::Script(BTreeMap::new()),
    )
    .with_staffs(staffs())
    .with_commands(commands)
    .with_orders_inside_a_day(inside, script_inside)
    .with_trial(Box::new(runner));
    Game { session, asked }
}

/// The evening before day 1 with `set_leave`, leave to launch, queued: command
/// 1 of the queue.
fn evening() -> Game {
    let mut g = game_of(&game_text());
    queue_leave(&mut g);
    g
}

/// The evening with nothing queued.
fn bare_evening() -> Game {
    game_of(&game_text())
}

fn queue_leave(g: &mut Game) {
    let a = g.ask("set_leave", json!({"leave": "launch", "given": true}));
    assert_eq!(a["status"], "queued", "{a}");
}

impl Game {
    fn call(&mut self, tool: &str, args: Value) -> Result<Value, CallError> {
        self.session.call(ALLIED, tool, &args)
    }

    fn ask(&mut self, tool: &str, args: Value) -> Value {
        self.call(tool, args.clone())
            .unwrap_or_else(|e| panic!("{tool} {args}: {}", e.message))
    }

    /// The error of a call that must be refused as an error of the call.
    fn error(&mut self, tool: &str, args: Value) -> String {
        match self.call(tool, args.clone()) {
            Err(e) => e.message,
            Ok(a) => panic!("{tool} {args} was answered: {a}"),
        }
    }

    fn queued(&mut self) -> Value {
        self.ask("queued", json!({}))
    }

    fn hash(&self) -> u64 {
        self.session.state_hash()
    }

    /// What the runner was handed, in order.
    fn handed(&self) -> Vec<TrialAsked> {
        self.asked.lock().expect("kept").clone()
    }

    fn preview(&mut self, args: Value) -> Value {
        self.ask("preview", args)
    }

    fn trial(&mut self, args: Value) -> Value {
        self.ask("trial", args)
    }

    fn trials(&mut self) -> Value {
        self.ask("trials", json!({}))
    }
}

fn take_c(extra: Value) -> Value {
    let mut c = json!({"intent_kind": "take_place", "hex": [0, 6]});
    c.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    c
}

fn order() -> Value {
    json!({"command": "set_intent", "intent_kind": "control_island", "island": 1})
}

/// Draft W, "with the waits".
fn draft_w() -> Value {
    json!({"command": "set_intent", "intent_kind": "control_island", "island": 1,
    "under": [
        {"intent_kind": "take_place", "hex": [0, 6],
         "limits": [{"limit": "forces", "task_forces": [FORCE_OF_C], "land_units": [3]}]},
        {"intent_kind": "take_place", "hex": [4, 0],
         "limits": [{"limit": "after", "other_intent": 2},
                    {"limit": "not_before", "from_day": 4}]}
    ]})
}

/// Draft N, "no waits": draft W without the two limits of the second
/// sub-intent.
fn draft_n() -> Value {
    json!({"command": "set_intent", "intent_kind": "control_island", "island": 1,
    "under": [
        {"intent_kind": "take_place", "hex": [0, 6],
         "limits": [{"limit": "forces", "task_forces": [FORCE_OF_C], "land_units": [3]}]},
        {"intent_kind": "take_place", "hex": [4, 0]}
    ]})
}

/// Draft O, "the order alone".
fn draft_o() -> Value {
    order()
}

/// The words of an answer, one string.
fn words(answer: &Value) -> String {
    answer["text"]
        .as_array()
        .expect("words")
        .iter()
        .map(|line| line.as_str().expect("a line"))
        .collect::<Vec<_>>()
        .join("\n")
}

fn numbers(v: &Value) -> Vec<u64> {
    v.as_array()
        .unwrap_or_else(|| panic!("a list: {v}"))
        .iter()
        .map(|n| n.as_u64().unwrap_or_else(|| panic!("a number: {n}")))
        .collect()
}

fn checked(answer: &Value) {
    let schemas = validator::Schemas::load(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../crates/sim-agent/schemas"),
    );
    let kind = answer["kind"].as_str().expect("a kind");
    let errors = schemas.check(&format!("{kind}.schema.json"), answer);
    assert!(errors.is_empty(), "{errors:?}\n{answer}");
}

fn course(steps: &[sim_agent::CourseStep]) -> Vec<CommandKind> {
    steps.iter().map(|s| s.command.clone()).collect()
}

// ------------------------------------------------------------ Reading answers

/// A command as the test reads a course: the engine's own command, in a line.
fn tag(c: &CommandKind) -> String {
    use sim_core::command::Limit;
    match c {
        CommandKind::SetLeave { leave, given } => format!("leave {} {given}", leave.name()),
        CommandKind::SetIntent { intent, parent, .. } => format!(
            "intent {} parent {}",
            intent.raw(),
            parent.map_or("none".to_string(), |p| p.raw().to_string())
        ),
        CommandKind::SetLimit(limit) => match limit {
            Limit::Forces { intent, .. } => format!("forces {}", intent.raw()),
            Limit::After { intent, other } => {
                format!("after {} {}", intent.raw(), other.map_or(0, |o| o.raw()))
            }
            Limit::NotBefore { intent, day } => {
                format!("not_before {} {}", intent.raw(), day.unwrap_or(0))
            }
            Limit::Risk { intent, risk } => format!("risk {} {risk:?}", intent.raw()),
            other => format!("{other:?}"),
        },
        CommandKind::SetPursuit { intent, .. } => format!("pursuit {}", intent.raw()),
        CommandKind::WithdrawIntent { intent } => format!("withdraw {}", intent.raw()),
        other => format!("{other:?}"),
    }
}

fn tags(commands: &[CommandKind]) -> Vec<String> {
    commands.iter().map(tag).collect()
}

fn strs(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

/// Draft W's six commands, in the batch's order.
fn batch_w() -> Vec<String> {
    strs(&[
        "intent 1 parent none",
        "intent 2 parent 1",
        "forces 2",
        "intent 3 parent 1",
        "after 3 2",
        "not_before 3 4",
    ])
}

const LEAVE: &str = "leave launch true";

/// Each queued entry of `queued`: its place in its queue and its command.
fn entries(q: &Value) -> Vec<(u64, Value)> {
    q["entries"]
        .as_array()
        .expect("entries")
        .iter()
        .map(|e| {
            (
                e["place"]["number"].as_u64().expect("a place"),
                e["command"].clone(),
            )
        })
        .collect()
}

/// A short name for a queued command as `queued` gives it.
fn entry_tag(command: &Value) -> String {
    let n = |key: &str| command[key].as_u64().unwrap_or(0);
    match command["kind"].as_str().expect("a kind") {
        "set_leave" => format!(
            "leave {} {}",
            command["leave"].as_str().unwrap(),
            command["given"]
        ),
        "set_intent" => match command["parent"].as_u64() {
            Some(p) => format!("intent {} parent {p}", n("intent")),
            None => format!("intent {} parent none", n("intent")),
        },
        "set_limit" => match command["limit"].as_str().unwrap() {
            "forces" => format!("forces {}", n("intent")),
            "after" => format!("after {} {}", n("intent"), n("other_intent")),
            "not_before" => format!("not_before {} {}", n("intent"), n("from_day")),
            other => format!("{other} {}", n("intent")),
        },
        other => other.to_string(),
    }
}

fn queued_tags(g: &mut Game) -> Vec<String> {
    entries(&g.queued())
        .iter()
        .map(|(_, c)| entry_tag(c))
        .collect()
}

/// The staff's reading of a preview: the tree's entries.
fn tree(p: &Value) -> Vec<Value> {
    p["with_change"]["tree"]["sub_intents"]
        .as_array()
        .expect("a tree")
        .clone()
}

/// The commands of a kept trial asked, as the second trial is given.
fn second_of(a: &TrialAsked) -> &sim_agent::SecondAsked {
    a.second.as_ref().expect("a second trial")
}

/// How an order went, given as the worlds it was met and kept in.
fn met_in(v: &Value, worlds: &[u64]) {
    assert_eq!(numbers(&v["worlds"]), [1, 2, 3, 4], "{v}");
    assert_eq!(numbers(&v["met_and_kept"]), worlds, "{v}");
    assert!(numbers(&v["met_and_lost_again"]).is_empty(), "{v}");
    let not_met: Vec<u64> = (1..=4).filter(|w| !worlds.contains(w)).collect();
    assert_eq!(numbers(&v["not_met"]), not_met, "{v}");
}

fn tally(beside: &Value, both: &[u64], yours: &[u64], staffs: &[u64], neither: &[u64]) {
    let w = &beside["world_for_world"];
    assert_eq!(numbers(&w["both"]), both, "{w}");
    assert_eq!(numbers(&w["with_your_plan_alone"]), yours, "{w}");
    assert_eq!(numbers(&w["with_the_staffs_alone"]), staffs, "{w}");
    assert_eq!(numbers(&w["neither"]), neither, "{w}");
    assert!(numbers(&w["apart"]).is_empty(), "{w}");
}

/// The five costs, each the lowest, the middle and the highest.
fn five_costs(c: &Value) {
    for key in [
        "men_lost_to_the_enemy",
        "men_lost_unfed",
        "ships_sunk",
        "ships_out_of_action",
        "aircraft_lost",
    ] {
        for end in ["lowest", "middle", "highest"] {
            assert!(c[key][end].is_u64(), "{key} {end} in {c}");
        }
    }
}

fn zero(c: &Value, key: &str) {
    assert_eq!(
        c[key],
        json!({"lowest": 0, "middle": 0, "highest": 0}),
        "{key} in {c}"
    );
}

/// The words of a trial: every line.
fn lines(answer: &Value) -> Vec<String> {
    answer["text"]
        .as_array()
        .expect("words")
        .iter()
        .map(|l| l.as_str().expect("a line").to_string())
        .collect()
}

fn says(answer: &Value, part: &str) {
    assert!(
        lines(answer).iter().any(|l| l.contains(part)),
        "no line has {part:?}:\n{}",
        words(answer)
    );
}

/// What is asked and answered in the first trial of statement 11: the
/// evening, draft W previewed as preview 1, `trial` of it at the size.
fn trial_of(draft: Value, size: &str, compare: Option<bool>) -> (Game, Value) {
    let mut g = evening();
    let p = g.preview(draft);
    assert_eq!(p["preview"], 1, "{p}");
    let mut args = json!({"preview": 1, "size": size});
    if let Some(c) = compare {
        args["compare"] = json!(c);
    }
    let a = g.trial(args);
    (g, a)
}

fn middle(went: &Value) -> u64 {
    went["met_days"]["middle"]
        .as_u64()
        .unwrap_or_else(|| panic!("a middle day in {went}"))
}

/// Everything an answer says that statement 11's first answer is read for.
fn same_fields(a: &Value, b: &Value) {
    for key in [
        "how_it_went",
        "last_day",
        "could_not_be_played",
        "cost",
        "enemy_lost",
        "beside",
    ] {
        assert_eq!(a[key], b[key], "{key}");
    }
}

/// Draft W approved on the evening: the leave and the six commands queued.
fn approved() -> Game {
    let mut g = evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1, "{p}");
    let a = g.ask("approve", json!({"preview": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    g
}

/// Draft O previewed and approved: the leave and the order, queued.
fn queued_order() -> Game {
    let mut g = evening();
    let p = g.preview(draft_o());
    assert_eq!(p["preview"], 1, "{p}");
    let a = g.ask("approve", json!({"preview": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    g
}

/// The take of B under intent 1, to go not before day 8, as a preview.
fn later_b() -> Value {
    json!({"command": "set_intent", "parent": 1, "intent_kind": "take_place",
           "hex": [4, 0], "limits": [{"limit": "not_before", "from_day": 8}]})
}

/// Draft W approved, and day 1 resolved: the evening of day 1.
fn in_force_w() -> Game {
    let mut g = approved();
    g.ask("end_day", json!({}));
    g
}

/// Statement 1: draft W previewed on the evening is preview 1, of intent
/// 1, with `under` giving intents 2 and 3, each with its command, its
/// parent and its limits; the staff's reading is of the whole draft (the
/// two standing entries, no take of the staff's, a landing under each);
/// nothing is queued and the hash is what it was.
#[test]
fn s01_the_draft_is_read_as_one() {
    let mut g = evening();
    let before = g.queued();
    let hash = g.hash();
    let p = g.preview(draft_w());
    checked(&p);
    assert_eq!(p["kind"], "preview");
    assert_eq!(p["valid"], true);
    assert_eq!(p["preview"], 1);
    assert_eq!(p["day"], 1);
    assert_eq!(p["intent"], 1);
    assert_eq!(p["command"]["kind"], "set_intent");
    assert_eq!(p["command"]["intent"], 1);
    assert_eq!(p["command"]["intent_kind"], "control_island");
    assert_eq!(p["limits"], json!([]));
    assert_eq!(p["shown_limits"], json!([]));
    let under = p["under"].as_array().expect("under");
    assert_eq!(under.len(), 2);
    for (u, (number, limits)) in under.iter().zip([(2u64, 1usize), (3, 2)]) {
        assert_eq!(u["intent"], number);
        assert_eq!(u["command"]["kind"], "set_intent");
        assert_eq!(u["command"]["intent"], number);
        assert_eq!(u["command"]["parent"], 1);
        assert!(u["shown"].as_str().is_some_and(|s| !s.is_empty()));
        assert_eq!(u["limits"].as_array().unwrap().len(), limits);
        assert_eq!(u["shown_limits"].as_array().unwrap().len(), limits);
    }
    // The staff's reading is of the whole draft: first his two entries, then
    // the staff's, numbered from 4.
    let t = tree(&p);
    for (entry, number) in t.iter().zip([2u64, 3]) {
        assert_eq!(entry["number"], number, "{entry}");
        assert_eq!(entry["parent"], 1, "{entry}");
        assert_eq!(entry["maker"], "commander", "{entry}");
        assert_eq!(entry["standing"], true, "{entry}");
    }
    let staffs = &t[2..];
    let numbered: Vec<u64> = staffs
        .iter()
        .map(|e| e["number"].as_u64().unwrap())
        .collect();
    assert_eq!(numbered, (4..4 + staffs.len() as u64).collect::<Vec<_>>());
    for entry in staffs {
        assert_eq!(entry["maker"], "staff", "{entry}");
        assert_ne!(entry["kind"], "take_place", "{entry}");
    }
    let landing = |parent: u64, force: u64, hex: Value| {
        staffs.iter().any(|e| {
            e["kind"] == "ready_landing"
                && e["parent"] == parent
                && e["task_force"] == force
                && e["hex"] == hex
        })
    };
    assert!(landing(2, FORCE_OF_C, json!([0, 6])), "{t:?}");
    assert!(landing(3, 1, json!([4, 0])), "{t:?}");
    let no_orders: Vec<&str> = p["with_change"]["tree"]["no_orders"]
        .as_array()
        .expect("no_orders")
        .iter()
        .map(|r| r.as_str().expect("words"))
        .collect();
    for line in [
        "intent:2, the commander's, plans the place at (0,6): the staff forms no take beside it.",
        "intent:3, the commander's, plans the place at (4,0): the staff forms no take beside it.",
    ] {
        assert!(no_orders.contains(&line), "{line}\n{no_orders:?}");
    }
    // As things stand intent 1 is not there.
    assert_eq!(p["as_things_stand"]["orders"], json!([]));
    assert_eq!(p["as_things_stand"]["commands"], json!([]));
    // Nothing is queued, and the game is what it was.
    let after = g.queued();
    assert_eq!(after, before);
    assert_eq!(entries(&after).len(), 1);
    assert_eq!(g.hash(), hash);
}

/// Statement 2: the four rows of the table of numbers, each a valid
/// preview with the numbers given.
#[test]
fn s02_the_numbers() {
    let case = |queue_hold: bool, order_number: Option<u64>, first: Option<u64>, second: Value| {
        let mut g = evening();
        if queue_hold {
            let a = g.ask(
                "set_intent",
                json!({"intent": 1, "intent_kind": "hold", "base": 1}),
            );
            assert_eq!(a["status"], "queued", "{a}");
        }
        let mut order = order();
        if let Some(n) = order_number {
            order["intent"] = json!(n);
        }
        let mut one = json!({"intent_kind": "take_place", "hex": [0, 6]});
        if let Some(n) = first {
            one["intent"] = json!(n);
        }
        order["under"] = json!([one, second]);
        let p = g.preview(order);
        assert_eq!(p["kind"], "preview", "{p}");
        assert_eq!(p["valid"], true, "{p}");
        let under = p["under"].as_array().unwrap();
        (
            p["intent"].as_u64().unwrap(),
            under[0]["intent"].as_u64().unwrap(),
            under[1]["intent"].as_u64().unwrap(),
        )
    };
    let after = |other: u64, number: Option<u64>| {
        let mut b = json!({"intent_kind": "take_place", "hex": [4, 0],
                           "limits": [{"limit": "after", "other_intent": other}]});
        if let Some(n) = number {
            b["intent"] = json!(n);
        }
        b
    };
    // Draft W.
    let mut g = evening();
    let p = g.preview(draft_w());
    let under = p["under"].as_array().unwrap();
    assert_eq!(
        (
            p["intent"].clone(),
            under[0]["intent"].clone(),
            under[1]["intent"].clone()
        ),
        (json!(1), json!(2), json!(3))
    );
    // A root hold of base 1, his intent 1, queued.
    assert_eq!(case(true, None, None, after(3, None)), (2, 3, 4));
    // The order as 4, the first sub-intent as 7.
    assert_eq!(case(false, Some(4), Some(7), after(7, None)), (4, 7, 8));
    // The second sub-intent as 9, naming 2.
    assert_eq!(case(false, None, None, after(2, Some(9))), (1, 2, 9));
}

/// Statement 3: the five drafts the engine would refuse, each answered
/// as that refusal with its place in the batch and the batch's size;
/// after all five nothing is queued and draft W is preview 1.
#[test]
fn s03_a_refusal_with_its_place_in_the_batch() {
    let c = || take_c(json!({}));
    let b = |extra: Value| {
        let mut b = json!({"intent_kind": "take_place", "hex": [4, 0]});
        b.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        b
    };
    let forces_c = || {
        take_c(
            json!({"limits": [{"limit": "forces", "task_forces": [FORCE_OF_C], "land_units": [3]}]}),
        )
    };
    let with_under = |mut order: Value, under: Vec<Value>| {
        order["under"] = json!(under);
        order
    };
    // (the draft, the command refused, its place, the size, words the reason has)
    let cases: Vec<(Value, Value, u64, u64, Vec<&str>)> = vec![
        (
            with_under(
                order(),
                vec![
                    take_c(json!({"limits": [{"limit": "after", "other_intent": 3}]})),
                    b(json!({})),
                ],
            ),
            json!({"kind": "set_limit", "limit": "after", "intent": 2, "other_intent": 3}),
            3,
            4,
            vec!["intent:3", "not in force"],
        ),
        (
            with_under(order(), vec![b(json!({})), b(json!({}))]),
            json!({"kind": "set_intent", "hex": [4, 0], "intent": 3}),
            3,
            3,
            vec!["already plans", "intent:2", "(4,0)"],
        ),
        (
            with_under(
                {
                    let mut o = order();
                    o["limits"] = json!([{"limit": "risk", "risk": "press"}]);
                    o
                },
                vec![c(), take_c(json!({"hex": [0, 0]}))],
            ),
            json!({"kind": "set_intent", "hex": [0, 0], "intent": 3}),
            4,
            4,
            vec!["cannot stand under", "intent:1"],
        ),
        (
            with_under(
                order(),
                vec![
                    forces_c(),
                    b(json!({"limits": [{"limit": "risk", "risk": "press"}]})),
                ],
            ),
            json!({"kind": "set_limit", "limit": "risk", "intent": 3, "risk": "press"}),
            5,
            5,
            vec!["wider", "intent:1"],
        ),
        (
            with_under(
                {
                    let mut o = order();
                    o["intent"] = json!(4);
                    o
                },
                vec![take_c(json!({"intent": 2}))],
            ),
            json!({"kind": "set_intent", "hex": [0, 6], "intent": 2, "parent": 4}),
            2,
            2,
            vec!["intent:2", "not a number"],
        ),
    ];
    for (draft, command, place, size, reason) in &cases {
        let mut g = evening();
        let hash = g.hash();
        let queue = g.queued();
        let a = g.preview(draft.clone());
        checked(&a);
        assert_eq!(a["kind"], "command", "{a}");
        assert_eq!(a["tool"], "preview", "{a}");
        assert_eq!(a["status"], "refused", "{a}");
        for (key, value) in command.as_object().unwrap() {
            assert_eq!(&a["command"][key], value, "{key} in {a}");
        }
        assert_eq!(
            a["of_batch"],
            json!({"position": place, "size": size}),
            "{a}"
        );
        let why = a["reason"].as_str().unwrap().to_lowercase();
        for part in reason {
            assert!(why.contains(part), "{part:?} not in {why:?}");
        }
        // Nothing numbered, nothing queued.
        assert_eq!(g.queued(), queue);
        assert_eq!(g.hash(), hash);
        let next = g.preview(draft_w());
        assert_eq!(next["preview"], 1, "a refusal is not numbered");
    }
    // All five on one evening, one after another.
    let mut g = evening();
    let queue = g.queued();
    for (draft, ..) in &cases {
        let a = g.preview(draft.clone());
        assert_eq!(a["status"], "refused", "{a}");
        assert_eq!(g.queued(), queue);
    }
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1, "{p}");
    assert_eq!(g.queued(), queue);
}

/// Statement 4: the errors of the call, each naming what is wrong, each
/// numbering and queueing nothing.
#[test]
fn s04_an_error_of_the_call() {
    let mut g = evening();
    let queue = g.queued();
    let c = || take_c(json!({}));
    let b = |limits: Value| json!({"intent_kind": "take_place", "hex": [4, 0], "limits": limits});
    let mut cases: Vec<(String, Vec<&str>)> = Vec::new();
    cases.push((
        g.error(
            "preview",
            json!({"command": "set_limit", "limit": "risk", "intent": 1, "risk": "press",
                   "under": [c()]}),
        ),
        vec!["under", "set_intent"],
    ));
    cases.push((
        g.error("preview", {
            let mut o = order();
            o["under"] = json!(5);
            o
        }),
        vec!["under"],
    ));
    cases.push((
        g.error("preview", {
            let mut o = order();
            o["under"] = json!([5]);
            o
        }),
        vec!["under[0]"],
    ));
    cases.push((
        g.error("preview", {
            let mut o = order();
            o["under"] = json!([take_c(json!({"parent": 1}))]);
            o
        }),
        vec!["under[0]", "parent", "stands under"],
    ));
    cases.push((
        g.error("preview", {
            let mut o = order();
            o["under"] = json!([
                c(),
                b(json!([{"limit": "risk", "risk": "press", "intent": 2}]))
            ]);
            o
        }),
        vec!["under[1].limits[0]", "intent"],
    ));
    cases.push((
        g.error("preview", {
            let mut o = order();
            o["under"] = json!([
                c(),
                b(json!([{"limit": "reserve", "base": 1, "supply_t": 10}]))
            ]);
            o
        }),
        vec!["under[1].limits[0]", "reserve"],
    ));
    for (message, parts) in cases {
        for part in parts {
            assert!(message.contains(part), "{part:?} not in {message:?}");
        }
    }
    // Each numbers nothing and queues nothing.
    assert_eq!(g.queued(), queue);
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1, "{p}");
}

/// Statement 5: `approve` of preview 1 queues the six commands in the
/// batch's order, with `limits_queued` 0 and `under_queued` as given;
/// day 1 takes in the seven, none refused, and the command state is
/// scenario 137's first morning.
#[test]
fn s05_approve_queues_all_of_it_in_order() {
    let mut g = evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1);
    let a = g.ask("approve", json!({"preview": 1}));
    checked(&a);
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["preview"], 1);
    assert_eq!(a["intent"], 1);
    assert_eq!(a["queued"], 7);
    assert_eq!(a["limits_queued"], 0);
    assert_eq!(
        a["under_queued"],
        json!([{"intent": 2, "limits_queued": 1}, {"intent": 3, "limits_queued": 2}])
    );
    let q = g.queued();
    let list = entries(&q);
    assert_eq!(list.len(), 7);
    for (i, e) in q["entries"].as_array().unwrap().iter().enumerate() {
        assert_eq!(e["place"], json!({"queue": "commands", "number": i + 1}));
    }
    let mut want = vec![LEAVE.to_string()];
    want.extend(batch_w());
    let got: Vec<String> = list.iter().map(|(_, c)| entry_tag(c)).collect();
    assert_eq!(got, want);
    // Day 1 takes in the seven and refuses none.
    g.ask("end_day", json!({}));
    let e = g.ask("events", json!({"day": 1}));
    let count = |kind: &str| {
        e["events"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|ev| {
                ev["event_kind"] == kind && ev["text"].as_str().unwrap().contains("the commander's")
            })
            .count()
    };
    assert_eq!(count("command_taken"), 7, "{e}");
    assert_eq!(count("command_refused"), 0, "{e}");
    let state = g.ask("command_state", json!({}));
    let intents = state["intents"].as_array().unwrap();
    let find = |n: u64| {
        intents
            .iter()
            .find(|i| i["number"] == n)
            .unwrap_or_else(|| panic!("intent {n}"))
    };
    for n in [1u64, 2, 3] {
        assert_eq!(find(n)["in_force"], true);
        assert_eq!(find(n)["maker"], "commander");
    }
    assert_eq!(find(1)["parent"], Value::Null);
    assert_eq!(find(2)["parent"], 1);
    assert_eq!(find(3)["parent"], 1);
    assert!(!find(2)["limits"]["forces"].is_null());
    assert!(find(2)["limits"]["not_before"].is_null());
    assert!(!find(3)["limits"]["after"].is_null());
    assert!(!find(3)["limits"]["not_before"].is_null());
    // The staff's intents are numbered from 4 and none is a take.
    for i in intents
        .iter()
        .filter(|i| i["number"].as_u64().unwrap() >= 4)
    {
        assert_eq!(i["maker"], "staff", "{i}");
        assert_ne!(i["kind"], "take_place", "{i}");
    }
}

/// Statement 6: something queued after the preview voids it, and
/// `approve` queues nothing of the batch.
#[test]
fn s06_or_none_of_it() {
    let mut g = evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1);
    let l = g.ask("set_leave", json!({"leave": "prepare", "given": true}));
    assert_eq!(l["status"], "queued", "{l}");
    let a = g.ask("approve", json!({"preview": 1}));
    checked(&a);
    assert_eq!(a["status"], "refused", "{a}");
    let why = a["reason"].as_str().unwrap();
    assert!(why.contains("void"), "{why}");
    assert!(why.contains("something was queued after it"), "{why}");
    assert!(why.contains("give the leave to prepare"), "{why}");
    assert_eq!(
        queued_tags(&mut g),
        strs(&["leave launch true", "leave prepare true"])
    );
}

/// Statement 7, the first bullet: `take_back` of the order takes
/// commands 2 to 7.
#[test]
fn s07_take_back_of_the_order_takes_its_written_part() {
    let mut g = approved();
    let q = g.queued();
    let t = g.ask(
        "take_back",
        json!({"queue": "commands", "number": 2, "as_of": q["as_of"]}),
    );
    let taken: Vec<u64> = t["taken_back"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["place"]["number"].as_u64().unwrap())
        .collect();
    assert_eq!(taken.len(), 6, "{t}");
    let mut sorted = taken.clone();
    sorted.sort();
    assert_eq!(sorted, [2, 3, 4, 5, 6, 7]);
    assert_eq!(queued_tags(&mut g), strs(&[LEAVE]));
}

/// Statement 7, the second and fourth bullets: `take_back` of the take
/// of B takes its two limits; then of the take of C its forces limit;
/// and a limit alone goes alone.
#[test]
fn s07_take_back_of_a_sub_intent_takes_its_limits() {
    let mut g = approved();
    let q = g.queued();
    let t = g.ask(
        "take_back",
        json!({"queue": "commands", "number": 5, "as_of": q["as_of"]}),
    );
    let mut taken: Vec<u64> = t["taken_back"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["place"]["number"].as_u64().unwrap())
        .collect();
    taken.sort();
    assert_eq!(taken, [5, 6, 7], "{t}");
    let mut want = vec![LEAVE.to_string()];
    want.extend(batch_w()[..3].iter().cloned());
    assert_eq!(queued_tags(&mut g), want);
    // Then the take of C and its forces limit.
    let q = g.queued();
    let t = g.ask(
        "take_back",
        json!({"queue": "commands", "number": 3, "as_of": q["as_of"]}),
    );
    let mut taken: Vec<u64> = t["taken_back"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["place"]["number"].as_u64().unwrap())
        .collect();
    taken.sort();
    assert_eq!(taken, [3, 4], "{t}");
    assert_eq!(
        queued_tags(&mut g),
        vec![LEAVE.to_string(), "intent 1 parent none".to_string()]
    );
    // A limit alone goes alone.
    let mut g = approved();
    let q = g.queued();
    let t = g.ask(
        "take_back",
        json!({"queue": "commands", "number": 7, "as_of": q["as_of"]}),
    );
    assert_eq!(t["taken_back"].as_array().unwrap().len(), 1, "{t}");
    assert_eq!(t["taken_back"][0]["place"]["number"], 7);
    assert_eq!(queued_tags(&mut g).len(), 6);
}

/// Statement 7, the third bullet: `take_back` of the take of C while the
/// "after intent 2" is queued is refused, naming command 6, and the
/// queue is as it was.
#[test]
fn s07_a_sub_intent_another_rests_on_is_not_taken_back() {
    let mut g = approved();
    let q = g.queued();
    let why = g.error(
        "take_back",
        json!({"queue": "commands", "number": 3, "as_of": q["as_of"]}),
    );
    assert!(why.contains("intent:3 after intent:2"), "{why}");
    assert!(why.contains("commands 6"), "{why}");
    assert!(why.contains("take that back first"), "{why}");
    // The queue and its as_of are as they were.
    assert_eq!(g.queued(), q);
}

/// Statement 8: under a queued order, a sub-intent previewed as one
/// `set_intent` with `parent` and its limits, approved; a limit of it
/// refused with its place; a second take of the place refused as
/// "Already planned"; and under an order in force, a take of a place
/// the staff has planned refused as "Already planned".
#[test]
fn s08_a_sub_intent_under_an_order_that_stands() {
    let sub = |limit: Value| {
        json!({"command": "set_intent", "parent": 1, "intent_kind": "take_place",
               "hex": [0, 6], "limits": [limit]})
    };
    let forces = || json!({"limit": "forces", "task_forces": [FORCE_OF_C], "land_units": [3]});
    // The order queued alone.
    let queued_order = || {
        let mut g = evening();
        let p = g.preview(draft_o());
        assert_eq!(p["preview"], 1);
        let a = g.ask("approve", json!({"preview": 1}));
        assert_eq!(a["status"], "queued", "{a}");
        g
    };
    let mut g = queued_order();
    let p = g.preview(sub(forces()));
    checked(&p);
    assert_eq!(p["kind"], "preview", "{p}");
    assert_eq!(p["valid"], true);
    assert_eq!(p["preview"], 2);
    assert_eq!(p["intent"], 2);
    assert_eq!(p["command"]["parent"], 1);
    assert_eq!(p["limits"].as_array().unwrap().len(), 1);
    assert_eq!(p["under"], json!([]));
    let a = g.ask("approve", json!({"preview": 2}));
    checked(&a);
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["limits_queued"], 1);
    assert_eq!(entries(&g.queued()).len(), 4);
    // A limit wider than its parent allows is refused with its place.
    let mut g = queued_order();
    let r = g.preview(sub(json!({"limit": "risk", "risk": "press"})));
    assert_eq!(r["status"], "refused", "{r}");
    assert_eq!(r["of_batch"], json!({"position": 2, "size": 2}), "{r}");
    assert!(r["reason"].as_str().unwrap().contains("wider"), "{r}");
    // A second take of C is "Already planned", naming intent 2: one command.
    let mut g = queued_order();
    let p = g.preview(sub(forces()));
    g.ask("approve", json!({"preview": p["preview"]}));
    let r = g.preview(
        json!({"command": "set_intent", "parent": 1, "intent_kind": "take_place", "hex": [0, 6]}),
    );
    assert_eq!(r["status"], "refused", "{r}");
    assert!(
        r["reason"]
            .as_str()
            .unwrap()
            .contains("intent:2 already plans"),
        "{r}"
    );
    assert!(r.get("of_batch").is_none(), "{r}");
    // Under an order in force: the staff has planned both places.
    let mut g = queued_order();
    g.ask("end_day", json!({}));
    let state = g.ask("command_state", json!({}));
    let staffs_take = state["intents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["kind"] == "take_place" && i["hex"] == json!([4, 0]) && i["maker"] == "staff")
        .expect("the staff's take of B")["number"]
        .as_u64()
        .unwrap();
    let r = g.preview(
        json!({"command": "set_intent", "parent": 1, "intent_kind": "take_place", "hex": [4, 0]}),
    );
    assert_eq!(r["status"], "refused", "{r}");
    assert!(
        r["reason"].as_str().unwrap().contains(&format!(
            "intent:{staffs_take} already plans the place at (4,0)"
        )),
        "{r}"
    );
}

/// Statement 9: the `TrialAsked` of draft W's trial: the standing
/// commands of both trials, the course of each, and that the first's
/// course less the written part is the second's, step for step.
#[test]
fn s09_the_second_trials_commands_are_the_firsts_less_the_written_part() {
    let (g, a) = trial_of(draft_w(), "small", None);
    let handed = g.handed();
    assert_eq!(handed.len(), 1, "one TrialAsked");
    let asked = &handed[0];
    let second = second_of(asked);
    assert_eq!(asked.intent.raw(), 1);
    assert_eq!(second.of, BesideOf::LeftToTheStaff);
    assert_eq!(second.of.name(), "left_to_the_staff");
    assert_eq!(a["asked"]["beside"], "left_to_the_staff");
    // The standing commands: the leave to launch in both.
    assert_eq!(tags(&asked.standing), strs(&[LEAVE]));
    assert_eq!(tags(&second.standing), strs(&[LEAVE]));
    assert_eq!(asked.standing, second.standing);
    // The course, all on day 1.
    assert!(asked.course.iter().all(|s| s.day == 1));
    assert!(second.course.iter().all(|s| s.day == 1));
    assert_eq!(tags(&course(&asked.course)), batch_w());
    assert_eq!(
        tags(&course(&second.course)),
        strs(&["intent 1 parent none"])
    );
    // Less the written part, and nothing else.
    let less: Vec<CommandKind> = course(&asked.course)
        .into_iter()
        .filter(|c| match c {
            CommandKind::SetIntent { parent, .. } => parent.is_none(),
            CommandKind::SetLimit(limit) => {
                !matches!(limit.intent().map(|i| i.raw()), Some(2) | Some(3))
            }
            _ => true,
        })
        .collect();
    assert_eq!(less, course(&second.course));
    // No leave is given by the trial in either.
    for steps in [&asked.course, &second.course] {
        assert!(course(steps)
            .iter()
            .all(|c| !matches!(c, CommandKind::SetLeave { .. })));
    }
    assert_eq!(a["leaves_given_to_the_side"], json!([]));
}

/// Statement 9, "The same leaves and the same settings in both" and
/// "With limits on the order".
#[test]
fn s09_the_same_leaves_and_the_orders_limits_in_both() {
    // On an evening with nothing queued: the batch, then what the trial gives.
    let mut g = bare_evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1);
    g.trial(json!({"preview": 1, "size": "small"}));
    let handed = g.handed();
    let asked = &handed[0];
    let second = second_of(asked);
    let first = course(&asked.course);
    let then = &first[6..];
    assert_eq!(tags(&first[..6]), batch_w());
    assert!(
        then.iter()
            .any(|c| matches!(c, CommandKind::SetLeave { given: true, .. })),
        "the trial gives the leave to launch: {:?}",
        tags(&first)
    );
    let second_course = course(&second.course);
    assert_eq!(tags(&second_course[..1]), strs(&["intent 1 parent none"]));
    assert_eq!(tags(&second_course[1..]), tags(then));
    assert_eq!(second.standing, asked.standing);
    // With limits on the order: the second's course is the order and its risk.
    let mut g = evening();
    let mut w = draft_w();
    w["limits"] = json!([{"limit": "risk", "risk": "press"}]);
    let p = g.preview(w);
    assert_eq!(p["preview"], 1, "{p}");
    g.trial(json!({"preview": 1, "size": "small"}));
    let handed = g.handed();
    let asked = &handed[0];
    let second = second_of(asked);
    let first = course(&asked.course);
    assert_eq!(first.len(), 7);
    assert_eq!(course(&second.course), first[..2].to_vec());
    assert_eq!(
        tags(&course(&second.course)),
        strs(&["intent 1 parent none", "risk 1 Press"])
    );
}

/// Statement 10: one asking, one trial's number and one size; the
/// pictures of worlds 1 to 4 assume the same things in the two plays;
/// and `beside` is the trial of the order alone, field for field, for
/// draft W and for draft N.
#[test]
fn s10_both_trials_play_the_same_worlds() {
    // One asking, one trial's number and one size.
    let (mut g, a) = trial_of(draft_w(), "small", None);
    let handed = g.handed();
    assert_eq!(handed.len(), 1);
    assert_eq!(handed[0].trial, 1);
    assert_eq!(
        handed[0].size,
        sim_agent::SizeAsked::Named("small".to_string())
    );
    // The pictures are the same: the test plays the two sets of commands.
    let s = load(&game_text());
    let play = |standing: &[CommandKind], steps: &[sim_agent::CourseStep]| {
        let played = sim_trial::play_window(
            &s.world.side_view(ALLIED),
            &sim_trial::Believed {
                forces: s.believed_forces(ALLIED).cloned().collect(),
                intents: s.believed_intents(ALLIED).cloned().collect(),
                men: s.believed_men(ALLIED).cloned().collect(),
            },
            s.trial_figures().unwrap(),
            s.world.ref_data(),
            s.world.map(),
            sim_trial::WindowAsked {
                trial: handed[0].trial,
                worlds: 4,
                days: 7,
                intent: handed[0].intent,
                standing: standing.to_vec(),
                course: steps
                    .iter()
                    .map(|st| sim_trial::CourseCommand {
                        day: st.day,
                        command: st.command.clone(),
                    })
                    .collect(),
                dates: Some(vec![1]),
            },
        )
        .expect("played");
        let worlds = played.played.into_iter().next().unwrap().played.worlds;
        worlds
            .into_iter()
            .map(|w| (w.number, w.assumptions))
            .collect::<Vec<_>>()
    };
    let second = second_of(&handed[0]);
    let yours = play(&handed[0].standing, &handed[0].course);
    let staffs = play(&second.standing, &second.course);
    assert_eq!(yours.iter().map(|w| w.0).collect::<Vec<_>>(), [1, 2, 3, 4]);
    assert_eq!(yours, staffs, "the pictures assume the same things");
    // The second trial is the trial of the order alone, for W and for N.
    let n = g.preview(draft_n());
    let o = g.preview(draft_o());
    assert_eq!(
        (n["preview"].as_u64(), o["preview"].as_u64()),
        (Some(2), Some(3))
    );
    let n_answer = g.trial(json!({"preview": 2, "size": "small"}));
    let alone = g.trial(json!({"preview": 3, "size": "small", "compare": false}));
    assert_eq!(alone["beside"], Value::Null);
    for answer in [&a, &n_answer] {
        let b = &answer["beside"];
        assert_eq!(b["how_it_went"], alone["how_it_went"]);
        assert_eq!(b["last_day"], alone["last_day"]);
        assert_eq!(b["could_not_be_played"], alone["could_not_be_played"]);
        assert_eq!(b["cost"], alone["cost"]);
        assert_eq!(b["enemy_lost"], alone["enemy_lost"]);
    }
}

/// Statement 11: the answer for draft W at `small`: what was asked, how
/// the order went with his plan and with the staff's, the tally, and the
/// words quoted.
#[test]
fn s11_the_answer_names_the_staffs_plan_and_gives_the_tally() {
    let (_, a) = trial_of(draft_w(), "small", None);
    checked(&a);
    assert_eq!(a["kind"], "trial");
    let asked = &a["asked"];
    assert_eq!(asked["preview"], 1);
    assert_eq!(asked["intent"], Value::Null);
    assert_eq!(asked["tried_intent"], 1);
    assert_eq!(asked["compare"], true);
    assert_eq!(asked["second_trial"], true);
    assert_eq!(asked["beside"], "left_to_the_staff");
    assert_eq!(asked["in_force"], false);
    assert_eq!(asked["dates"], Value::Null);
    assert_eq!(asked["standing"], json!(["give leave to launch"]));
    assert_eq!(asked["course"].as_array().unwrap().len(), 6, "{asked}");
    // The order with his plan: not met by day 7 in worlds 1 to 4.
    met_in(&a["how_it_went"], &[]);
    assert_eq!(a["last_day"], 7);
    let b = &a["beside"];
    assert_eq!(b["of"], "left_to_the_staff");
    assert_eq!(b["order"], 1);
    met_in(&b["how_it_went"], &[1, 2, 3, 4]);
    assert_eq!(b["with_your_plan"], a["how_it_went"]);
    tally(b, &[], &[], &[1, 2, 3, 4], &[]);
    // In words.
    says(
        &a,
        "Beside it, the same order with the landings left to the staff, in the same worlds",
    );
    for part in [
        "your plan against the staff's",
        "met and kept in both in none",
        "with your plan alone in none",
        "with the staff's plan alone in 4 worlds (worlds 1, 2, 3 and 4)",
        "in neither in none",
    ] {
        assert!(
            lines(&a)
                .iter()
                .any(|l| l.starts_with("Taken world by world") && l.contains(part)),
            "{part:?} in {}",
            words(&a)
        );
    }
    // No advice.
    assert!(!words(&a).contains("better"));
}

/// Statement 11's table: the tally for draft N at `small` and for draft
/// W at `standard`, and in the last the staff's day earlier than his.
#[test]
fn s11_the_tally_for_each_draft_and_size() {
    // N at small: both, in all four.
    let (_, n) = trial_of(draft_n(), "small", None);
    checked(&n);
    assert_eq!(n["beside"]["of"], "left_to_the_staff");
    tally(&n["beside"], &[1, 2, 3, 4], &[], &[], &[]);
    met_in(&n["how_it_went"], &[1, 2, 3, 4]);
    met_in(&n["beside"]["how_it_went"], &[1, 2, 3, 4]);
    // W at standard: both, in all four, his day later than the staff's.
    let (_, w) = trial_of(draft_w(), "standard", None);
    checked(&w);
    tally(&w["beside"], &[1, 2, 3, 4], &[], &[], &[]);
    met_in(&w["how_it_went"], &[1, 2, 3, 4]);
    met_in(&w["beside"]["how_it_went"], &[1, 2, 3, 4]);
    let (his, staffs) = (
        middle(&w["how_it_went"]),
        middle(&w["beside"]["how_it_went"]),
    );
    assert!(his > staffs, "his {his} against the staff's {staffs}");
    // Reported for the architect, not asserted beyond the relation.
    println!(
        "137b s11: standard, W: how_it_went {} ; beside {}",
        w["how_it_went"], w["beside"]["how_it_went"]
    );
    println!(
        "137b s11: small, N: how_it_went {} ; beside {}",
        n["how_it_went"], n["beside"]["how_it_went"]
    );
}

/// Statement 12: the five costs of each side in both trials, given
/// alike; the three that are 0; the second's equal to the trial of the
/// order alone.
#[test]
fn s12_the_costs() {
    let (mut g, w) = trial_of(draft_w(), "small", None);
    let n = g.preview(draft_n());
    assert_eq!(n["preview"], 2);
    let n = g.trial(json!({"preview": 2, "size": "small"}));
    let o = g.preview(draft_o());
    assert_eq!(o["preview"], 3);
    let alone = g.trial(json!({"preview": 3, "size": "small", "compare": false}));
    for answer in [&w, &n] {
        let b = &answer["beside"];
        for costs in [
            &answer["cost"],
            &answer["enemy_lost"],
            &b["cost"],
            &b["enemy_lost"],
        ] {
            five_costs(costs);
            for key in ["men_lost_to_the_enemy", "ships_sunk", "aircraft_lost"] {
                zero(costs, key);
            }
        }
        assert_eq!(b["cost"], alone["cost"]);
        assert_eq!(b["enemy_lost"], alone["enemy_lost"]);
        let beside: Vec<String> = lines(answer)
            .into_iter()
            .filter(|l| l.starts_with("Beside it, what "))
            .collect();
        assert_eq!(beside.len(), 2, "{}", words(answer));
        assert!(beside[0].starts_with("Beside it, what it cost your side"));
        assert!(beside[1].starts_with("Beside it, what the trial's enemy lost"));
    }
}

/// Statement 13: a new sub-intent under a queued order, previewed and
/// tried: the commands of both trials, the landing's own end, the
/// order's end in each and the tally; and, queued, the same answer.
#[test]
fn s13_a_new_sub_intent_under_an_order_that_stands() {
    let mut g = queued_order();
    let p = g.preview(later_b());
    assert_eq!(p["preview"], 2, "{p}");
    assert_eq!(p["intent"], 2, "{p}");
    let a = g.trial(json!({"preview": 2, "size": "small"}));
    checked(&a);
    let handed = g.handed();
    let asked = &handed[0];
    let second = second_of(asked);
    let held = strs(&[LEAVE, "intent 1 parent none"]);
    assert_eq!(tags(&asked.standing), held);
    assert_eq!(
        tags(&course(&asked.course)),
        strs(&["intent 2 parent 1", "not_before 2 8"])
    );
    assert_eq!(tags(&second.standing), held);
    assert!(second.course.is_empty());
    assert_eq!(asked.intent.raw(), 2);
    assert_eq!(second.of, BesideOf::LeftToTheStaff);
    met_in(&a["how_it_went"], &[]);
    assert_eq!(a["asked"]["beside"], "left_to_the_staff");
    let b = &a["beside"];
    assert_eq!(b["of"], "left_to_the_staff");
    assert_eq!(b["order"], 1);
    met_in(&b["with_your_plan"], &[]);
    met_in(&b["how_it_went"], &[1, 2, 3, 4]);
    tally(b, &[], &[], &[1, 2, 3, 4], &[]);
    says(
        &a,
        "Beside it, the same order with the landing at (4,0) left to the staff, in the same worlds",
    );
    // Queued, it reads the same.
    let q = g.ask("approve", json!({"preview": 2}));
    assert_eq!(q["status"], "queued", "{q}");
    let t = g.trial(json!({"intent": 2, "size": "small"}));
    checked(&t);
    let handed = g.handed();
    let asked = &handed[1];
    let second = second_of(asked);
    assert_eq!(
        tags(&asked.standing),
        strs(&[
            LEAVE,
            "intent 1 parent none",
            "intent 2 parent 1",
            "not_before 2 8"
        ])
    );
    assert!(asked.course.is_empty());
    assert_eq!(tags(&second.standing), held);
    assert!(second.course.is_empty());
    assert_eq!(second.of, BesideOf::LeftToTheStaff);
    same_fields(&t, &a);
}

/// Statement 14: a sub-intent in force tried with no `compare`: two
/// trials, the second handed its withdrawal; the tally; the words; and
/// nothing withdrawn in the game. With `compare` false, one. The order
/// in force is as it was.
#[test]
fn s14_a_sub_intent_in_force() {
    let mut g = in_force_w();
    let hash = g.hash();
    let a = g.trial(json!({"intent": 3, "size": "small"}));
    checked(&a);
    let handed = g.handed();
    assert_eq!(handed.len(), 1, "two trials in one asking");
    let asked = &handed[0];
    let second = second_of(asked);
    assert!(asked.standing.is_empty());
    assert!(asked.course.is_empty());
    assert!(second.standing.is_empty());
    assert_eq!(tags(&course(&second.course)), strs(&["withdraw 3"]));
    assert!(second.course.iter().all(|s| s.day == 1));
    assert_eq!(second.of, BesideOf::GivenBackToTheStaff);
    assert_eq!(second.of.name(), "given_back_to_the_staff");
    assert_eq!(a["asked"]["in_force"], true);
    assert_eq!(a["asked"]["beside"], "given_back_to_the_staff");
    assert_eq!(a["asked"]["for_day"], 2);
    let b = &a["beside"];
    assert_eq!(b["of"], "given_back_to_the_staff");
    assert_eq!(b["order"], 1);
    assert_eq!(a["last_day"], 8);
    met_in(&b["with_your_plan"], &[]);
    met_in(&b["how_it_went"], &[1, 2, 3, 4]);
    tally(b, &[], &[], &[1, 2, 3, 4], &[]);
    says(
        &a,
        "Beside it, the same order with the landing at (4,0) given back to the staff on the coming morning, in the same worlds",
    );
    says(&a, "hat was already ordered stands");
    says(
        &a,
        "The withdrawal is the trial's, not yours: intent 3 is not withdrawn in your game",
    );
    // After it intent 3 is in force and the hash is what it was.
    assert_eq!(g.hash(), hash);
    let state = g.ask("command_state", json!({}));
    let three = state["intents"]
        .as_array()
        .unwrap()
        .iter()
        .find(|i| i["number"] == 3)
        .unwrap();
    assert_eq!(three["in_force"], true);
    // With compare false one trial is played and beside is null.
    let one = g.trial(json!({"intent": 3, "size": "small", "compare": false}));
    assert_eq!(one["beside"], Value::Null);
    assert_eq!(one["asked"]["second_trial"], false);
    assert_eq!(g.handed().len(), 2);
    assert!(g.handed()[1].second.is_none());
    // The order itself, in force, is as it was.
    let order = g.trial(json!({"intent": 1, "size": "small"}));
    assert_eq!(order["beside"], Value::Null);
    assert_eq!(order["asked"]["second_trial"], false);
    assert_eq!(order["asked"]["in_force"], true);
    says(&order, "Your order is in force");
    says(&order, "\"compare\" true");
    assert!(g.handed()[2].second.is_none());
}

/// Statement 15: with draft W queued, the order tried reads as its
/// preview did; a sub-intent nothing names is tried beside the staff's
/// plan of its place; a sub-intent an "after" names is refused, naming
/// the command, and played with `compare` false.
#[test]
fn s15_a_queued_order_with_its_written_part() {
    // Statement 11's first answer, for the comparison.
    let (_, first) = trial_of(draft_w(), "small", None);
    let mut g = approved();
    // The order is read as its preview was.
    let a = g.trial(json!({"intent": 1, "size": "small"}));
    checked(&a);
    let handed = g.handed();
    let asked = &handed[0];
    let second = second_of(asked);
    let mut seven = vec![LEAVE.to_string()];
    seven.extend(batch_w());
    assert_eq!(tags(&asked.standing), seven);
    assert!(asked.course.is_empty());
    assert_eq!(
        tags(&second.standing),
        strs(&[LEAVE, "intent 1 parent none"])
    );
    assert_eq!(second.of, BesideOf::LeftToTheStaff);
    same_fields(&a, &first);
    // A sub-intent nothing else names.
    let t = g.trial(json!({"intent": 3, "size": "small"}));
    checked(&t);
    let handed = g.handed();
    let second = second_of(&handed[1]);
    assert_eq!(
        tags(&second.standing),
        strs(&[
            LEAVE,
            "intent 1 parent none",
            "intent 2 parent 1",
            "forces 2"
        ])
    );
    assert_eq!(t["beside"]["of"], "left_to_the_staff");
    assert_eq!(t["beside"]["order"], 1);
    tally(&t["beside"], &[], &[], &[1, 2, 3, 4], &[]);
    // A sub-intent another queued command names: refused, nothing played.
    let played = g.handed().len();
    for args in [
        json!({"intent": 2, "size": "small"}),
        json!({"intent": 2, "size": "small", "compare": true}),
    ] {
        let why = g.error("trial", args);
        assert!(why.contains("intent:3 after intent:2"), "{why}");
        assert!(why.contains("names intent 2"), "{why}");
    }
    assert_eq!(g.handed().len(), played);
    let one = g.trial(json!({"intent": 2, "size": "small", "compare": false}));
    assert_eq!(one["beside"], Value::Null);
    assert_eq!(g.handed().len(), played + 1);
    assert!(g.handed()[played].second.is_none());
}

/// Statement 16: where `compare` is not given, the rows stated here
/// (draft O, and draft W with `compare` false).
#[test]
fn s16_where_compare_is_not_given() {
    // Draft O, the side having no other plan: not played, as before.
    let (g, a) = trial_of(draft_o(), "small", None);
    checked(&a);
    assert_eq!(a["asked"]["second_trial"], false);
    assert_eq!(a["beside"], Value::Null);
    says(&a, "Your side has no other plan to weigh");
    assert!(g.handed()[0].second.is_none());
    // Draft W with compare false: one trial, the first answer's fields.
    let (_, first) = trial_of(draft_w(), "small", None);
    let (g, one) = trial_of(draft_w(), "small", Some(false));
    checked(&one);
    assert_eq!(one["asked"]["second_trial"], false);
    assert_eq!(one["beside"], Value::Null);
    assert!(g.handed()[0].second.is_none());
    assert_eq!(one["how_it_went"], first["how_it_went"]);
    assert_eq!(one["cost"], first["cost"]);
}

/// Statement 17: draft W described: two trials of 4 worlds for 7 days,
/// the words quoted, nothing played and nothing kept.
#[test]
fn s17_asked_before_it_is_played() {
    let mut g = evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1);
    let hash = g.hash();
    let queue = g.queued();
    let d = g.trial(json!({"preview": 1, "size": "small", "describe": true}));
    checked(&d);
    assert_eq!(d["kind"], "trial_described");
    assert_eq!(d["asked"]["beside"], "left_to_the_staff");
    let plays = d["plays"].as_array().unwrap();
    assert_eq!(plays.len(), 2, "{d}");
    assert_eq!(plays[0]["trial"], "first");
    assert_eq!(plays[1]["trial"], "second");
    for play in plays {
        assert_eq!(play["worlds"], 4);
        assert_eq!(play["days"], 7);
    }
    says(
        &d,
        "Two trials of 4 worlds of 7 days each, in the same worlds: the first with the draft, the second the same order with the landings left to the staff.",
    );
    assert_eq!(g.hash(), hash);
    assert_eq!(g.queued(), queue);
    assert!(g.handed().is_empty(), "nothing was played");
    let t = g.trials();
    assert_eq!(t["estimates"], json!([]), "{t}");
}

/// Statement 18: `dates` refused for a written part, each case with its
/// words, nothing played; and draft O given its window as before.
#[test]
fn s18_dates_is_refused_for_a_written_part() {
    let sentence =
        "A written landing is held to a day by its \"not before\", which can be previewed and compared";
    // Each alone, each an error with nothing played.
    let refused = |g: &mut Game, args: Value, begins: &str| {
        let hash = g.hash();
        let played = g.handed().len();
        let listed = g.trials();
        let why = g.error("trial", args.clone());
        assert!(why.starts_with(begins), "{args}: {why}");
        assert!(why.contains(sentence), "{why}");
        assert_eq!(g.handed().len(), played);
        assert_eq!(g.hash(), hash);
        assert_eq!(g.trials(), listed);
    };
    for dates in [json!("chosen"), json!([4])] {
        for describe in [false, true] {
            let mut g = evening();
            g.preview(draft_w());
            refused(
                &mut g,
                json!({"preview": 1, "dates": dates, "describe": describe}),
                "preview 1 carries landings you wrote: a window is not offered for a written part.",
            );
        }
    }
    for describe in [false, true] {
        let mut g = queued_order();
        let p = g.preview(later_b());
        assert_eq!(p["preview"], 2);
        refused(
            &mut g,
            json!({"preview": 2, "dates": "chosen", "describe": describe}),
            "preview 2 is of a landing you wrote under intent 1: a window is not offered for a written part.",
        );
        let mut g = approved();
        refused(
            &mut g,
            json!({"intent": 3, "dates": "chosen", "describe": describe}),
            "intent 3 is a landing you wrote under intent 1: a window is not offered for a written part.",
        );
        refused(
            &mut g,
            json!({"intent": 1, "dates": "chosen", "describe": describe}),
            "intent 1 has landings you wrote under it: a window is not offered for a written part.",
        );
        // A change to a queued written part.
        let mut g = approved();
        let p = g.preview(json!({"command": "set_limit", "limit": "not_before",
                                 "intent": 3, "from_day": 6}));
        assert_eq!(p["preview"], 2, "{p}");
        refused(
            &mut g,
            json!({"preview": 2, "dates": "chosen", "describe": describe}),
            "preview 2 is of a landing you wrote under intent 1: a window is not offered for a written part.",
        );
        let p = g.preview(json!({"command": "set_limit", "limit": "not_before",
                                 "intent": 1, "from_day": 2}));
        assert_eq!(p["preview"], 3, "{p}");
        refused(
            &mut g,
            json!({"preview": 3, "dates": "chosen", "describe": describe}),
            "preview 3 is of intent 1, which has landings you wrote under it: a window is not offered for a written part.",
        );
    }
    // In force: as an order in force is today, in the words that name a
    // "not before".
    let mut g = in_force_w();
    let why = g.error("trial", json!({"intent": 3, "dates": "chosen"}));
    assert!(why.contains("is in force"), "{why}");
    assert!(why.contains("not before"), "{why}");
    // Draft O is given its window as before.
    let mut g = evening();
    let p = g.preview(draft_o());
    assert_eq!(p["preview"], 1);
    let w = g.trial(json!({"preview": 1, "dates": [4], "size": "small"}));
    assert_eq!(w["kind"], "window", "{w}");
}

/// Properties: nothing is changed, a preview tried is not spent, asked
/// twice the same, one estimate each.
#[test]
fn nothing_is_changed() {
    let mut g = evening();
    let p = g.preview(draft_w());
    assert_eq!(p["preview"], 1);
    let n = g.preview(draft_n());
    assert_eq!(n["preview"], 2);
    let hash = g.hash();
    let queue = g.queued();
    let first = g.trial(json!({"preview": 1, "size": "small"}));
    assert_eq!(g.hash(), hash);
    assert_eq!(g.queued(), queue);
    let again = g.trial(json!({"preview": 1, "size": "small"}));
    // Asked twice, the same, but for the estimate.
    let unnumbered = |a: &Value| {
        let mut a = a.clone();
        a.as_object_mut().unwrap().remove("estimate");
        a
    };
    assert_eq!(unnumbered(&first), unnumbered(&again));
    assert_eq!(first["estimate"], 1);
    assert_eq!(again["estimate"], 2);
    // One estimate each: the three questions of statement 11.
    let n_answer = g.trial(json!({"preview": 2, "size": "small"}));
    let standard = g.trial(json!({"preview": 1, "size": "standard"}));
    assert_eq!(g.hash(), hash);
    assert_eq!(g.queued(), queue);
    let listed = g.trials();
    let estimates = listed["estimates"].as_array().unwrap();
    assert_eq!(estimates.len(), 4, "{listed}");
    for e in estimates {
        assert_eq!(e["kind"], "trial");
    }
    for (number, answer) in [(1, &first), (2, &again), (3, &n_answer), (4, &standard)] {
        let kept = g.ask("trials", json!({"estimate": number}));
        assert_eq!(&kept, answer);
    }
    // A preview tried is not spent: approve queues the six.
    let a = g.ask("approve", json!({"preview": 1}));
    assert_eq!(a["status"], "queued", "{a}");
    assert_eq!(a["queued"], 7);
    assert_eq!(queued_tags(&mut g).len(), 7);
}

/// Property: the live enemy is not read.
#[test]
fn the_live_enemy_is_not_read() {
    let (_, plain) = trial_of(draft_w(), "small", None);
    let mut text = game_text();
    text.push_str(
        "\n[[land_unit]]\nid = 4\nname = \"Placeholder Japanese\"\nside = \"japanese\"\nmen = 300\nq = 4\nr = 0\n",
    );
    let mut g = game_of(&text);
    queue_leave(&mut g);
    g.preview(draft_w());
    let with_enemy = g.trial(json!({"preview": 1, "size": "small"}));
    assert_eq!(
        serde_json::to_string(&plain).unwrap(),
        serde_json::to_string(&with_enemy).unwrap()
    );
}

/// Properties: the words hold no advice, the answers pass their schemas
/// and carry version 57 of the agent's interface.
#[test]
fn the_words_the_schemas_and_the_version() {
    let mut answers: Vec<Value> = Vec::new();
    let mut g = evening();
    answers.push(g.preview(draft_w()));
    answers.push(g.preview(draft_n()));
    answers.push(g.preview(draft_o()));
    answers.push(g.preview({
        let mut o = order();
        o["under"] = json!([take_c(
            json!({"limits": [{"limit": "after", "other_intent": 3}]})
        )]);
        o
    }));
    for args in [
        json!({"preview": 1, "size": "small"}),
        json!({"preview": 2, "size": "small"}),
        json!({"preview": 1, "size": "standard"}),
        json!({"preview": 1, "size": "small", "describe": true}),
        json!({"preview": 3, "size": "small"}),
    ] {
        answers.push(g.trial(args));
    }
    answers.push(g.ask("approve", json!({"preview": 1})));
    let mut g = queued_order();
    answers.push(g.preview(later_b()));
    answers.push(g.trial(json!({"preview": 2, "size": "small"})));
    let mut g = in_force_w();
    answers.push(g.trial(json!({"intent": 3, "size": "small"})));
    let kinds: BTreeSet<String> = answers
        .iter()
        .map(|a| a["kind"].as_str().unwrap().to_string())
        .collect();
    for kind in ["preview", "command", "trial", "trial_described"] {
        assert!(kinds.contains(kind), "{kind} in {kinds:?}");
    }
    for answer in &answers {
        checked(answer);
        assert_eq!(answer["version"], 57, "{answer}");
        let all = answer.to_string().to_lowercase();
        assert!(!all.contains('%'), "a per cent sign in {answer}");
        for word in [
            "chance",
            "points",
            "better",
            "should",
            "recommend",
            "advise",
        ] {
            assert!(!all.contains(word), "{word:?} in {answer}");
        }
    }
    assert_eq!(sim_agent::VERSION, 57);
}
