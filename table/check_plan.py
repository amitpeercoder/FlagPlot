#!/usr/bin/env python3
"""A check, by script, of a plan the commander writes at the table: decision
0232, step 3, rules 27 to 31. It is run by `table/check_screen.py`, and alone:

    python3 table/check_plan.py

It needs `sim-cli` built (`cargo build --release -p sim-cli`) and node, uses
port 8786 (`--port` for another), and stops everything it starts. Standard
library only. Like the table's other checks it is not run by CI.

THE STATEMENTS. Written by the architect on Nov 27, 2026, before any code of
step 3. Nobody but the architect changes a statement or an expected value
below (the words in THE WORDS, the constants PLAN_IN_NODE, WRITTEN_SAMPLE and
every value a `check` compares with), and never to make a check pass: a
mismatch comes back to the architect. The build adds what a statement leaves
to it: the page, and the checks by the page's text that are marked "the
build's" below. A rule is cited by its number in decision 0232 and is not
restated. The table's wording rules hold (decision 0221, rules 14 to 16 and 22
to 23; `table/README.md`, "Words, stage 4"): one word for each thing, no
caption that explains a rule, no handle and no number of the engine's in what
is said, no points and no advice.

What is read. The page reads a maker, a parent, `standing`, a preview's
`under`, an approval's `under_queued`, the queue's `goes_with` and a trial's
`beside` as the tools give them (agent interface 57). It parses no words to
learn whose a node is. The one place it reads words is statement 21, where an
intent the engine names in a reason is said as a landing.

THE GAME. The 39 days as the table offers them (`guadalcanal-39-days`, the
person Allied, the engine Japanese), whose three files gain one island
(statement 1). The Allied seat holds Espiritu Santo (base 3); Lunga (base 1,
hex (2,-4)) and Tulagi (base 4, hex (3,-5)) are the enemy's. Task forces 9 and
10 at the seat are "Placeholder transports, Guadalcanal" and "Placeholder
transports, Tulagi"; land units 5 and 6 are the 1st Raider Battalion and the
1st Parachute Battalion. On the first evening leave to prepare is the side's
and leave to launch is withheld, so an order's plan waits for him.

THE WORDS, fixed here. A straight apostrophe below is the page's own curly
one (U+2019) wherever the page already writes "THE STAFF'S CHOICE" so.

  YOUR PLAN                        the part's name, and the head of its summary
  The staff plans every landing.   the summary with nothing written
  left to the staff                a place nobody wrote, in the open part
  WRITE IT                         the button on such a place
  LEAVE IT TO THE STAFF            the button on a landing of the draft
  Task forces, Land units, Sails from, Not before, Wait for, After,
  Knowledge, Risk, Done by         a landing's lines, in this order
  the landing at <place>           a landing, in a list or after "After:"
  your landing at <place>          a landing of his, where a sentence or a
                                   link names it
  This reading is worked with your plan.
  This reading is worked with the limits you set and your plan.
  Your plan: <the summary>         in SIGNED's line for the order
  YOURS, THE STAFF'S               the mark on each line of the plan's paper
  GIVE IT BACK TO THE STAFF        the button on a line of his in force
  given back to the staff          what SIGNED says of it
  Your plan, The staff's plan      the dry run's two columns
  The order met and held           the first row of those columns

A place is said by its base's name ("Lunga"), never by a hex or a number.

==========================================================================
The island (Amit's answer 2 of Nov 26, 2026)
==========================================================================

1. THE FILES. `scripts/table-rear-bases/make_files.py` makes each of the
   three files of `data/scenarios/table-rear-bases/` as it does today with
   one change more, its sixth. In `07b_slice_submarines.toml` and
   `07c_slice_submarines.toml`, after the block of island 2 (the `[[island]]`
   with `number = 2`, to the blank line that ends it) it writes a note of
   three lines that names decision 0232, step 3, and then

       [[island]]
       number = 3
       name = "Guadalcanal and Tulagi"
       places = [{ base = 1 }, { base = 4 }]

   and a blank line. `07_full_slice_submarines.toml`, the game of a person
   against an agent and of two agents, has no island at all today (found on
   Nov 27, 2026: the table's list offers "Seize and hold Guadalcanal" in
   that game too, and the engine refuses it, "island 1 is not an island of
   the scenario"). At its end, after a blank line, the script writes a note
   of two lines, then the blocks of islands 1 and 2, each `[[island]]`,
   `number`, `name` ("Guadalcanal" of `{ base = 1 }`, "Tulagi" of `{ base =
   4 }`) and `places`, as those two files have them, a blank line after
   each, and then the block of island 3. The header the script writes, which
   says the file is the file it copies "with these changes and nothing
   else", says this change after ruling 2's, in these words:

       #   Decision 0232, step 3 (the designer's answers of Nov 26 and 27, 2026):
       #   the island "Guadalcanal and Tulagi" (number 3, of base 1 and then base
       #   4), so that one order holds both landings; and, in the file that had no
       #   island, islands 1 and 2 as the other two files have them. Map data:
       #   nothing of an island is hashed, and a game that orders none plays as
       #   before. No file of record or of a proof has the third island.

   (The notes beside the islands were fixed word for word when this was first
   written; the build's shorter notes say the same and are accepted, and the
   header's paragraph, which the build left out, is restated here: amended by
   the architect on reading pull request 759.) Nothing is invented: the
   places are the bases islands 1 and 2 name in decision 0153, and an island
   has no figure. No other file under `data/` changes: not `table-boats/`,
   not `table-hunger/`, no file of record and no file of a proof. Tested in
   `tests/table_rear_bases_files.rs`, statements 2, 6 and 7 (restated and
   added by the architect in its doc comment).

2. NO GAME MOVES. For each of the three files, `sim-cli run FILE --events`
   gives, before the change and after it, output that differs in no line, the
   hash lines kept. The build shows the comparison; it is not a check of this
   file. (The file's own hash, which a saved game records, does change: a game
   kept from before is not taken up on the new file.)

3. THE LIST. `table/scenarios.json` gives the 39 days three islands:

       {"number": 1, "name": "Guadalcanal", "base": 1},
       {"number": 2, "name": "Tulagi", "base": 4},
       {"number": 3, "name": "Guadalcanal and Tulagi", "base": 1, "places": [1, 4]}

   `places` is new: the bases of an island of more than one place, in the
   island's order, the first of them its `base`. An entry without it is of
   its one `base`. No other scenario's entry changes (the five days keep
   their two islands). `serve.py --island` takes `3=Guadalcanal and
   Tulagi@1,4` and gives the same entry; `@1` gives an entry with no
   `places`, as today. The bridge hands the page the entries as they are.
   Checked: `the_island`.

4. THE ORDER IS OFFERED. A base's card offers "Seize and hold <island>" for
   every island one of whose places it is: Lunga's and Tulagi's cards each
   offer "Seize and hold Guadalcanal and Tulagi", after the island of their
   own. Checked in node: `islandPlaces`, `islandsOf`.

==========================================================================
Rule 27: the order's paper gains YOUR PLAN
==========================================================================

5. THE PART. The paper of an order to seize and hold an island has a part
   YOUR PLAN, folded, after LIMITS and before ACTS (`id="p-plan"`, its
   summary `id="p-plan-sum"`, its body `id="p-plan-body"`); for every other
   kind of order it is hidden. Its summary is "YOUR PLAN · " and then "The
   staff plans every landing." or what he wrote (statement 8).

6. THE PLACES. Open, it has a row for each place of the island that is not
   the side's, in the island's order (`placesToTake`): the place's name and
   "left to the staff" with WRITE IT, or the landing he wrote, its lines and
   LEAVE IT TO THE STAFF. A place the side holds has no row. Where the side
   holds every place the part says "You hold every place of it: there is no
   landing to write." WRITE IT is not offered until the staff's first reading
   of the draft has come, since the reading gives the order's number
   (statement 9).

7. A LANDING'S LINES are, in this order, each at the staff's choice until
   set and each with BACK TO THE STAFF'S CHOICE once set:

   | Line | Its choices | Sent |
   | --- | --- | --- |
   | Task forces | the staff's choice; use only these: the side's task forces by name | `task_forces` (and `ships: []`) of the landing's one `forces` limit |
   | Land units | the staff's choice; use only these | `land_units` of that limit |
   | Sails from | the staff's choice; a base of the side | `source` of that limit |
   | Not before | the staff's choice; a date | `not_before`, `from_day` |
   | Wait for | the staff's choice; a task force or land unit | `wait_for` |
   | After | the staff's choice; another landing written in this draft ("The landing at Tulagi"); another order of his to take a place or control an island, as the order's own After lists them | `after`, `other_intent`: the landing's number in the batch, or the order's |
   | Knowledge | as the order's own line | `knowledge` |
   | Risk | the staff's choice (the order's); preserve, balanced, press | `risk`; nothing where it is the staff's choice |
   | Done by | the staff's choice; a date | `by_day` of the landing's own `set_intent`, not a limit |

   A kind of forces has no "None" here, and single ships are not offered:
   rule 11 reads the task forces, the land units and the source. Air groups,
   Keep within, Strike first and Protected ships are not lines of a landing:
   nothing reads them on a take he wrote under an island (rule 10; decision
   0222, "a control is shown only where something reads that limit"). Every
   task force of the side is listed; one on the carrier or the bombardment
   mission is not the landing's, and the staff's reason says so (rule 10),
   which statement 21 leads back here. The risk offers all three: one wider
   than the order's is refused by the engine and said at the line (statement
   11). "After" on a landing does not offer a landing that waits on it.

8. WHAT IS SAID, in a line (`planLine`, checked in node): the landings in
   the order they are sent, joined by "; ", each "<place>: " and its set
   lines joined by " · ", or "<place>: the staff's choice in all" where none
   is set. A date is said as the page says a "not before" today.

9. WHAT IS SENT (`underBodies`, checked in node and taken by the engine,
   `the_draft_as_one`). With a landing written, the order's preview carries
   `intent`, the number the staff's last reading of this draft gave, and
   `under`: one entry a landing, each `{"intent": N, "intent_kind":
   "take_place", "base": B}` with `by_day` where a date is set and `limits`
   where any line is set, the limits in the order of statement 7 with the
   risk last. The landings are numbered from one past the order's, in the
   order they are sent: the order he wrote them in, but that a landing
   another waits on (its After) is sent before the one that waits. With
   nothing written the preview is the one sent today, with no `intent` and no
   `under`.

==========================================================================
Rule 28: one reading, one signature, one entry, one TAKE BACK
==========================================================================

10. THE STAFF'S READING is of the whole draft: one `preview` of the order,
    its limits and `under`, drawn by `readingView` from `with_change` as
    today. Each landing he wrote is a line of the tree marked YOURS with what
    the staff would form under it; each of the staff's is marked THE STAFF'S
    (statement 17). Where a landing is written the reading ends "This reading
    is worked with your plan.", or, with limits set too, "This reading is
    worked with the limits you set and your plan."

11. A REFUSAL NAMES ITS LINE. A draft the engine refuses is answered with
    its place in the batch (rule 20): the order is place 1, then its limits,
    then each landing followed by its own limits (`batchAt`, checked in
    node). The paper says "The staff refuses it: " and then, for an order's
    limit, as today; for a landing, "The landing at <place>: " and the
    engine's reason; for a landing's line, "The landing at <place>: <line>:
    " and the reason (`batchWords`). YOUR PLAN is opened with that landing's
    row, or line, marked. The engine's reason for a limit wider than the
    order above allows names that order by its handle; the page says "It is
    wider than the order above it allows." (added by the architect on
    reading pull request 759: no handle is said).

12. SIGN AND SEND is one `approve` of that preview and sends nothing else.
    The count that went is checked: `limits_queued` against the order's own
    limits sent, and `under_queued` against the landings and each one's
    limits; a difference is said ("Your plan: 1 of 2 landings went.").

13. SIGNED has one entry for the order (`signedParts`, checked in node on
    the queue the engine gave for such a draft): "Order: Seize and hold
    Guadalcanal and Tulagi", then " · " and the order's OWN limits where any
    (the `set_limit` entries of `goes_with` whose `intent` is the order's;
    today the line would say a landing's limits as the order's), then " ·
    Your plan: " and the summary of statement 8. Its landings and their
    limits are no entries of their own. One TAKE BACK takes all of it
    (`take_back` of the order's entry, as today).

==========================================================================
Rule 29: the dry run's two columns
==========================================================================

14. DRY RUN on a draft with a landing written is `trial` of the preview at
    the size chosen, with no `compare` given (rule 24 makes it true). The
    estimate's sheet, where `beside.of` is "left_to_the_staff" or
    "given_back_to_the_staff" (`planBeside`, checked in node on a sample
    answer), has under the heading "Your plan and the staff's plan":
    - one line of what the second column is: "Beside it: the same order with
      the landings left to the staff, in the same runs."; for one landing
      tried, "... with the landing at <place> left to the staff, in the same
      runs."; for a landing in force, "... with the landing at <place> given
      back to the staff on the coming morning, in the same runs.";
    - a table of two columns, "Your plan" and "The staff's plan", whose first
      row is "The order met and held", from `beside.with_your_plan` and
      `beside.how_it_went`, each as the window's "Met and held" row says a
      count today ("all 4 runs, day 5", "no run"), and whose other rows are
      the side's five costs, a row left out where it is nothing in both;
    - under "What it may cost the enemy of the dry run", the same two
      columns for `enemy_lost` and `beside.enemy_lost`;
    - one line run for run: "Run for run, the order met and held: with both
      plans in A; with your plan alone in B; with the staff's plan alone in
      C; with neither in D.", and where `apart` holds any, " 1 run could not
      be played and is left out." (or " N runs could not be played and are
      left out.").
    It calls neither plan the better. The three kinds of `beside` the sheet
    shows today are shown as today, and so is the plan's paper's own DRY RUN.

15. ON THE 39 DAYS the table's files name no trial's figures today, so the
    table offers no dry run there at all (found on Nov 27, 2026). Whether
    they are to name `data/reference/trial.toml`, as the table's three other
    battles do, is asked of the designer (the as-built file, "Asked of the
    designer"). `the_dry_run_beside` is run where the game offers `trial` and
    says so where it does not; statement 14 is held by the node check either
    way.

==========================================================================
Rule 30: the plan's paper
==========================================================================

16. A LANDING WRITTEN UNDER AN ORDER THAT STANDS (rule 22; the designer's
    answer 3: "before you give the island's plan its leave"). On the plan's
    paper of an order to seize and hold an island that is in force and whose
    plan is a draft or preparing, each take the staff would form (a
    `take_place` of the proposed tree whose maker is the staff) has WRITE IT.
    It opens the one order paper for that landing alone ("Write the landing
    at <place>, under " and the order, linked), with the lines of statement
    7; its After lists his other landings under the order. The reading is
    `preview` of `{"command": "set_intent", "parent": the order, "intent_kind":
    "take_place", "base": B}` with `by_day` and `limits`; SIGN AND SEND
    approves it; SIGNED says "Landing: <place>, under your order to seize and
    hold Guadalcanal and Tulagi" and then its lines, with TAKE BACK; DRY RUN
    is statement 14's, for one landing. Once the island's plan is released
    the staff's takes say THE STAFF'S and offer nothing (the designer's
    answer 3). Taken by the engine: `under_an_order_that_stands`.

17. THE MARK. Every task of the plan's paper, of a plan that waits (from
    `propose`) and of a plan in force (from the command state), and of the
    staff's reading in the order paper, has after its words YOURS where its
    maker is the commander and THE STAFF'S where it is the staff
    (`whoseTag`, checked in node). The order at the head has none.

18. A LINE OF HIS in force shows the landing's lines of statement 7 as the
    order's LIMITS part shows an order's: what is set, or the staff's choice,
    or what is read from the order above with "from" and that order linked;
    CHANGE on each but Done by, and BACK TO THE STAFF'S CHOICE where it is
    set on the landing. CHANGE opens the order paper for that one limit of
    that landing ("Change the limits of your landing at <place>"), read by
    `preview` of the one `set_limit` with the landing's number, as an
    order's limit is today. It does not say "Set by us" of a limit he set.
    Done by is shown and not changed here: a date is the landing's
    `set_intent` given again, which is left out of this build.

19. GIVE IT BACK TO THE STAFF, on a line of his in force while he may sign,
    opens the order paper: "Give the landing at <place> back to the staff",
    ACTS, SIGN AND SEND and TEAR UP, and DRY RUN where the game has a trial
    (`trial` of that landing's intent with no `compare` given: statement
    14's third line). It has no staff's reading, since no tool reads a
    withdrawal before it is given; the paper is the reading before signing,
    in place of the single press WITHDRAW THIS ORDER is. Signed, it sends
    `withdraw_intent` of the landing; SIGNED says "Your landing at <place>:
    given back to the staff", with TAKE BACK. Taken by the engine:
    `a_line_of_his_in_force`.

==========================================================================
Rule 31: a reason leads to his line
==========================================================================

20. A LIMIT OF HIS LANDING, in a reason's `limits` (`{kind, intent}` with
    the landing's number), is a link under YOUR LIMITS as today, in the
    words of its kind, followed by " on " and "your landing at <place>".
    For a landing in force it opens the plan's paper at that landing's line,
    with the limit's CHANGE in focus; for a landing of the draft in hand
    (its number is one the draft sends) it opens YOUR PLAN at that landing's
    line.

21. HIS LANDING NAMED IN A REASON'S WORDS. Where a reason says "intent:K"
    and K is a landing he wrote (in the command state: a `take_place` whose
    maker is the commander and that has a parent; or a number the draft in
    hand sends), the page says "your landing at <place>" as a link, to the
    plan's paper at that line or to YOUR PLAN at that landing, and drops the
    engine's words that say whose it is, which the link now says. The page
    marks the landing as it marks an order a unit serves today, with its
    number between the two brackets its links are made from, written [K]
    here (`writtenMarks`, checked in node on the engine's sentences of rules
    9, 10 and 14), by these readings and no other, each only where K is a
    landing he wrote:

    | The engine's words | Said as |
    | --- | --- |
    | "intent:K, the commander's, plans the place at (q,r)" | "[K] stands" |
    | "intent:K, the commander's take of the place at (q,r)," | "[K]" |
    | "intent:K, the commander's take under intent:M," | "[K]" |
    | "intent:K is the commander's take under an island" | "[K] is your landing under an island" |
    | any other "intent:K, the commander's," (added on reading pull request 759: the island's evidence read "your landing at Lunga, your, is in force") | "[K]" |
    | any other "intent:K" | "[K]" |
    | in a sentence so marked, "is his to take" and "carries out his date" | "is yours to take", "carries out your date" |

    Nothing else of a reason is reworded by this step.

Checks by the page's text (the build's): that each part above is in the
page under the names this file gives, as `check_screen.py` holds each part
of decision 0222; they are added in `the_page_holds_it` and hold nothing a
statement does not say. Tried in a browser by hand, and said in the as-built
file: the part open and closed, a draft of both landings read, signed and
taken back, the plan's paper before and after the leave, and the dry run's
sheet if the designer answers yes.
"""

import argparse
import json
import os
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)

from check_screen import Bridge, Person, begin, check, cut_function, get, page_text, post, run_in_node  # noqa: E402

GAME = "guadalcanal-39-days"
ISLANDS = [
    {"number": 1, "name": "Guadalcanal", "base": 1},
    {"number": 2, "name": "Tulagi", "base": 4},
    {"number": 3, "name": "Guadalcanal and Tulagi", "base": 1, "places": [1, 4]},
]
STAFFS = "the staff’s"

# The queue the engine gave, at the Allied seat of the 39 days with the island, for a draft approved: the order at press, the landing
# at Tulagi with its forces and its base, and the landing at Lunga after it, not before day 4, at no count, at balanced, done by day
# 9 (run by the architect on Nov 27, 2026; `acts` and the entries' words are left out, which the model does not read).
WRITTEN_SAMPLE = [
    {"place": {"queue": "commands", "number": 1}, "command": {"kind": "set_intent", "intent": 1, "intent_kind": "control_island", "island": 3},
     "goes_with": [{"queue": "commands", "number": n} for n in range(2, 10)], "rests_on": []},
    {"place": {"queue": "commands", "number": 2}, "command": {"kind": "set_limit", "intent": 1, "limit": "risk", "risk": "press"}, "goes_with": [], "rests_on": []},
    {"place": {"queue": "commands", "number": 3}, "command": {"kind": "set_intent", "intent": 2, "intent_kind": "take_place", "hex": [3, -5], "parent": 1},
     "goes_with": [{"queue": "commands", "number": 4}], "rests_on": []},
    {"place": {"queue": "commands", "number": 4}, "command": {"kind": "set_limit", "intent": 2, "limit": "forces", "task_forces": [10], "ships": [], "land_units": [5, 6], "source": 3},
     "goes_with": [], "rests_on": []},
    {"place": {"queue": "commands", "number": 5}, "command": {"kind": "set_intent", "intent": 3, "intent_kind": "take_place", "hex": [2, -4], "parent": 1, "by_day": 9},
     "goes_with": [{"queue": "commands", "number": n} for n in range(6, 10)], "rests_on": []},
    {"place": {"queue": "commands", "number": 6}, "command": {"kind": "set_limit", "intent": 3, "limit": "not_before", "from_day": 4}, "goes_with": [], "rests_on": []},
    {"place": {"queue": "commands", "number": 7}, "command": {"kind": "set_limit", "intent": 3, "limit": "after", "other_intent": 2}, "goes_with": [], "rests_on": []},
    {"place": {"queue": "commands", "number": 8}, "command": {"kind": "set_limit", "intent": 3, "limit": "knowledge", "knowledge": "none"}, "goes_with": [], "rests_on": []},
    {"place": {"queue": "commands", "number": 9}, "command": {"kind": "set_limit", "intent": 3, "limit": "risk", "risk": "balanced"}, "goes_with": [], "rests_on": []},
]

# A trial's answer, cut to what the two columns read: scenario 137b's statement 11, draft W at the small size (4 worlds for 7 days).
ALL4 = [1, 2, 3, 4]
NOT_MET = {"met_and_kept": [], "met_and_lost_again": [], "met_days": None, "not_met": ALL4, "worlds": ALL4}
MET = {"met_and_kept": ALL4, "met_and_lost_again": [], "met_days": {"earliest": 5, "middle": 5, "latest": 5}, "not_met": [], "worlds": ALL4}


def spread(low, middle, high):
    return {"lowest": low, "middle": middle, "highest": high}


def costs(men=(0, 0, 0), unfed=(0, 0, 0), sunk=(0, 0, 0), out=(0, 0, 0), air=(0, 0, 0)):
    return {"men_lost_to_the_enemy": spread(*men), "men_lost_unfed": spread(*unfed), "ships_sunk": spread(*sunk), "ships_out_of_action": spread(*out), "aircraft_lost": spread(*air)}


BESIDE_SAMPLE = {
    "how_it_went": NOT_MET, "last_day": 7, "cost": costs(men=(10, 20, 30)), "enemy_lost": costs(),
    "beside": {"of": "left_to_the_staff", "order": 1, "how_it_went": MET, "with_your_plan": NOT_MET, "last_day": 7, "cost": costs(men=(0, 5, 9), air=(0, 0, 2)), "enemy_lost": costs(),
               "world_for_world": {"both": [], "with_your_plan_alone": [], "with_the_staffs_alone": ALL4, "neither": [], "apart": []}},
}

# The page's model of a written plan, run in node. The names are the contract between this file and the page: the model lies in the
# page between the lines "  // ---- a plan the commander writes (decision 0232, step 3) ----" and "  // ---- end of the written plan
# model ----", is pure, and reads of the page only the limits model, `M`, `num`, `cleanName`, `dateOf`, `baseByNumber` and `runsIn`.
#
#   LIMITS_OFFERED.written_landing   ['ships', 'land_units', 'source', 'landing', 'risk'] (in the limits model)
#   islandPlaces(island)             the base numbers of an island's places, in order: `places`, or its one `base`
#   islandsOf(base, islands)         the islands a base is a place of, in the list's order
#   placesToTake(island, own)        its places that are not among `own`, the side's base numbers
#   newPlan()                        { landings: [] }: the landings in the order he wrote them
#   newLanding(base)                 { base, lim, by_day: null }: `lim` is `newLim()` with `risk` null; its `after` is null, a number
#                                    (another order) or { landing: base } (another landing of the plan)
#   planBatch(plan)                  the landings in the order they are sent (statement 9)
#   underBodies(plan, n)             the preview's `under`, the order's number being n
#   landingLine(l)                   one landing's set lines in words, or null
#   planLine(plan)                   the summary (statement 8)
#   batchAt(sent, plan, position)    where a place of the batch belongs; `sent` is what `sentLimits` gives for the order
#   batchWords(at, why)              the refusal's words (statement 11); for the order itself, `why`
#   signedParts(entry, entries)      { limits: the order's own limits in a line or '', plan: the summary or '' } (statement 13)
#   whoseTag(maker)                  the mark (statement 17)
#   planBeside(j, place)             the two columns of a trial's answer (statement 14), or null where `beside` is of another kind
#   writtenMarks(text, written)      a reason's words with his landings marked (statement 21); `written(n)` says whether n is one
PLAN_IN_NODE = r"""
function whereIs() { return ''; } function dateOf(d) { return ({ 3: '10 AUG', 8: '15 AUG' })[d] || ''; } function sayIntent(i) { return i.kind === 'control_island' ? 'Seize and hold Guadalcanal and Tulagi' : 'Capture Rabaul'; }
const hexes = [];
function main(input) {
  M.names = { 'task_force:9': 'Placeholder transports, Guadalcanal', 'task_force:10': 'Placeholder transports, Tulagi', 'land_unit:5': '1st Raider Battalion', 'land_unit:6': '1st Parachute Battalion',
    'base:1': 'Lunga', 'base:3': 'Espiritu Santo', 'base:4': 'Tulagi' };
  M.bases = [{ base: { handle: 'base:3', name: 'Espiritu Santo' }, place: { q: 7, r: 6 } }];
  M.enemyBases = [{ base: { handle: 'base:1', name: 'Lunga' }, place: { q: 2, r: -4 } }, { base: { handle: 'base:4', name: 'Tulagi' }, place: { q: 3, r: -5 } }];
  M.state = { intents: [{ number: 7, kind: 'take_place', in_force: true }] };
  const out = {}, islands = input.islands, both = islands[2];
  out.places = islands.map(islandPlaces);
  out.islandsOf = [1, 4, 3].map(b => islandsOf(b, islands).map(i => i.number));
  out.toTake = [placesToTake(both, [3]), placesToTake(both, [3, 4]), placesToTake(both, [1, 3, 4])];
  // Nothing written.
  const none = newPlan(); out.noneLine = planLine(none); out.noneUnder = underBodies(none, 1);
  // One landing with nothing set.
  const bare = newPlan(); bare.landings.push(newLanding(4)); out.bareLine = planLine(bare); out.bareUnder = underBodies(bare, 1);
  // Draft W: he writes Lunga first and Tulagi second, and Lunga is to go after Tulagi.
  const w = newPlan(), lunga = newLanding(1), tulagi = newLanding(4); w.landings.push(lunga, tulagi);
  lunga.lim.after = { landing: 4 }; lunga.lim.not_before = 4; lunga.lim.risk = 'preserve'; lunga.by_day = 9;
  tulagi.lim.ships = { mode: 'only', task_forces: [10], ships: [] }; tulagi.lim.land_units = { mode: 'only', list: [5, 6] }; tulagi.lim.source = 3;
  out.batch = planBatch(w).map(l => l.base); out.under = underBodies(w, 1); out.under5 = underBodies(w, 5).map(u => [u.intent, (u.limits || []).filter(b => b.limit === 'after').map(b => b.other_intent)]);
  out.line = planLine(w); out.lungaLine = landingLine(lunga); out.riskUnset = newLanding(1).lim.risk;
  // After another order, and a wait: sent as the order's own are.
  const o = newPlan(), one = newLanding(1); o.landings.push(one); one.lim.after = 7; one.lim.wait_for = { task_force: 9 }; one.lim.knowledge = { knowledge: 'recent_count', within_days: 3 };
  out.other = underBodies(o, 2); out.otherLine = planLine(o);
  // Where a place of the batch belongs: the order with one limit of its own, then draft W.
  const sent = [{ body: { limit: 'not_before', from_day: 2 }, key: 'not_before' }];
  out.at = [1, 2, 3, 4, 5, 6, 7, 8].map(p => batchAt(sent, w, p));
  out.words = [batchWords(out.at[0], 'W'), batchWords(out.at[1], 'W'), batchWords(out.at[2], 'W'), batchWords(out.at[3], 'W'), batchWords(out.at[7], 'W')];
  // SIGNED, from the queue the engine gave.
  out.signed = signedParts(input.queue[0], input.queue);
  out.signedAlone = signedParts({ place: { queue: 'commands', number: 1 }, command: { kind: 'set_intent', intent: 1, intent_kind: 'control_island', island: 3 }, goes_with: [], rests_on: [] }, []);
  out.tags = [whoseTag('commander'), whoseTag('staff')];
  // The dry run's two columns.
  out.beside = planBeside(input.trial, null);
  const one_ = JSON.parse(JSON.stringify(input.trial)); out.besideOne = planBeside(one_, 'Lunga').line;
  one_.beside.of = 'given_back_to_the_staff'; one_.beside.world_for_world = { both: [1], with_your_plan_alone: [2], with_the_staffs_alone: [], neither: [3], apart: [4] };
  const back = planBeside(one_, 'Lunga'); out.besideBack = [back.line, back.runs];
  one_.beside.of = 'withdrawn'; out.besideOther = planBeside(one_, null);
  // A reason's words.
  const mine = n => n === 3;
  out.marks = input.reasons.map(t => writtenMarks(t, mine));
  return out;
}
"""

# The engine's sentences of rules 9, 14 and 10, as `sim-staff` writes them (island.rs, air.rs, cover.rs), and one that names no
# landing of his.
REASONS = [
    "intent:3, the commander's, plans the place at (2,-4): the staff forms no take beside it.",
    "intent:3, the commander's, plans the place at (2,-4), and its plan is cancelled: the staff forms no take beside it, and intent:1 cannot be controlled while the place is his to take.",
    "intent:1 is to be done by the end of day 6; intent:3, the commander's take of the place at (2,-4), is to be done by the end of day 9, 3 days later. The staff carries out his date.",
    "intent:3, the commander's take under intent:1, waits for the plan of intent:1: it is preparing, and a take under an island is served once the island's plan is released, so the staff gives no order under intent:3 and its plan stays a draft.",
    "intent:3 is the commander's take under an island",
    "intent:3, the commander's, is in force and its plan is preparing",
    "intent:4 holds base:1 (Lunga)",
]


def the_model_in_node():
    """Statements 4, 8, 9, 11, 13, 14, 17 and 21, by the page's own model run in node."""
    print("-- a plan the commander writes: the page's model in node", flush=True)
    text = page_text()
    start, end = "  // ---- a plan the commander writes (decision 0232, step 3) ----", "  // ---- end of the written plan model ----"
    check(start in text and end in text, "the page holds the model of a written plan between its two marked lines")
    limits = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the limits model ----")]
    code = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ["cleanName", "num", "hexDist", "hexWords", "baseByNumber", "runsIn", "commas"]) + limits + text[text.index(start):text.index(end)] + PLAN_IN_NODE
    said = run_in_node(code, {"islands": ISLANDS, "queue": WRITTEN_SAMPLE, "trial": BESIDE_SAMPLE, "reasons": REASONS})
    # statement 4
    check(said["places"] == [[1], [4], [1, 4]], f"an island's places are its `places`, or its one base ({said['places']})")
    check(said["islandsOf"] == [[1, 3], [2, 3], []], f"Lunga is a place of Guadalcanal and of Guadalcanal and Tulagi, Tulagi of Tulagi and of both, Espiritu Santo of none ({said['islandsOf']})")
    check(said["toTake"] == [[1, 4], [1], []], f"the places to take are those the side does not hold, in the island's order ({said['toTake']})")
    # statements 8 and 9
    check(said["noneLine"] == "The staff plans every landing." and said["noneUnder"] == [], "with nothing written the summary says the staff plans every landing, and nothing is sent under the order")
    check(said["bareLine"] == "Tulagi: the staff’s choice in all" and said["bareUnder"] == [{"intent": 2, "intent_kind": "take_place", "base": 4}],
          f"a landing written with nothing set is sent as a take of its place with no limits, numbered one past the order ({said['bareUnder']})")
    check(said["riskUnset"] is None, "a landing's risk is not set until he sets it")
    check(said["batch"] == [4, 1], f"a landing another waits on is sent before the one that waits, whichever he wrote first ({said['batch']})")
    under = [
        {"intent": 2, "intent_kind": "take_place", "base": 4, "limits": [{"limit": "forces", "task_forces": [10], "ships": [], "land_units": [5, 6], "source": 3}]},
        {"intent": 3, "intent_kind": "take_place", "base": 1, "by_day": 9,
         "limits": [{"limit": "not_before", "from_day": 4}, {"limit": "after", "other_intent": 2}, {"limit": "risk", "risk": "preserve"}]},
    ]
    check(said["under"] == under, f"draft W is sent as two takes numbered 2 and 3, Tulagi's with its one forces limit, Lunga's with its date, its two waits and its risk last ({json.dumps(said['under'])})")
    check(said["under5"] == [[6, []], [7, [6]]], f"under order 5 the landings are 6 and 7, and the After names 6 ({said['under5']})")
    line = ("Tulagi: Task forces: Transports, Tulagi · Land units: 1st Raider Battalion, 1st Parachute Battalion · Sails from: Espiritu Santo; "
            "Lunga: Not before: 10 AUG · After: the landing at Tulagi · Risk: preserve · Done by: 15 AUG")
    check(said["line"] == line, f"the summary says each landing by its place and its lines, by name and by date ({said['line']})")
    check(said["lungaLine"] == "Not before: 10 AUG · After: the landing at Tulagi · Risk: preserve · Done by: 15 AUG", "one landing's lines are said in the order of statement 7")
    check(said["other"] == [{"intent": 3, "intent_kind": "take_place", "base": 1,
                             "limits": [{"limit": "wait_for", "task_force": 9}, {"limit": "after", "other_intent": 7}, {"limit": "knowledge", "knowledge": "recent_count", "within_days": 3}]}],
          f"a wait for a force, an After that names another order and a count's age are sent as the order's own are ({json.dumps(said['other'])})")
    check(said["otherLine"] == "Lunga: Wait for: Transports, Guadalcanal · After: Capture Rabaul · Knowledge: a count no older than 3 days", f"and said so ({said['otherLine']})")
    # statement 11
    at = [{"where": "order"}, {"where": "limit", "key": "not_before"}, {"where": "landing", "base": 4}, {"where": "landing", "base": 4, "key": "ships"},
          {"where": "landing", "base": 1}, {"where": "landing", "base": 1, "key": "not_before"}, {"where": "landing", "base": 1, "key": "after"}, {"where": "landing", "base": 1, "key": "risk"}]
    check(said["at"] == at, f"each place of the batch belongs to the order, a limit of it, a landing or a line of a landing ({json.dumps(said['at'])})")
    check(said["words"] == ["W", "Not before: W", "The landing at Tulagi: W", "The landing at Tulagi: Task forces: W", "The landing at Lunga: Risk: W"], f"a refusal is said at its line ({said['words']})")
    # statement 13
    check(said["signed"] == {"limits": "Risk: press", "plan": "Tulagi: Task forces: Transports, Tulagi · Land units: 1st Raider Battalion, 1st Parachute Battalion · Sails from: Espiritu Santo; "
                             "Lunga: Not before: 10 AUG · After: the landing at Tulagi · Knowledge: nothing known · Risk: balanced · Done by: 15 AUG"},
          f"SIGNED says the order's own limits, and none of a landing's as the order's, and then the plan as the summary says it ({json.dumps(said['signed'])})")
    check(said["signedAlone"] == {"limits": "", "plan": ""}, "an order signed alone says neither")
    # statement 17
    check(said["tags"] == ["YOURS", "THE STAFF’S"], f"the mark is YOURS or THE STAFF'S, by the maker ({said['tags']})")
    # statement 14
    b = said["beside"]
    check(b["head"] == "Your plan and the staff’s plan" and b["line"] == "Beside it: the same order with the landings left to the staff, in the same runs.", f"the sheet says what the second column is ({b['line']})")
    check(b["met"] == ["The order met and held", "no run", "all 4 runs, day 5"], f"the first row is the order's end in each trial ({b['met']})")
    check([c[0] for c in b["cols"]] == ["Your plan", "The staff’s plan"] and b["cols"][0][1] == BESIDE_SAMPLE["cost"] and b["cols"][1][1] == BESIDE_SAMPLE["beside"]["cost"]
          and [c[0] for c in b["enemy"]] == ["Your plan", "The staff’s plan"] and b["enemy"][1][1] == BESIDE_SAMPLE["beside"]["enemy_lost"],
          "the two columns are your plan and the staff's plan, for the side's costs and for the enemy's")
    check(b["runs"] == "Run for run, the order met and held: with both plans in 0; with your plan alone in 0; with the staff’s plan alone in 4; with neither in 0.", f"and run for run ({b['runs']})")
    check(said["besideOne"] == "Beside it: the same order with the landing at Lunga left to the staff, in the same runs.", f"one landing tried says which ({said['besideOne']})")
    check(said["besideBack"] == ["Beside it: the same order with the landing at Lunga given back to the staff on the coming morning, in the same runs.",
                                 "Run for run, the order met and held: with both plans in 1; with your plan alone in 1; with the staff’s plan alone in 0; with neither in 1. 1 run could not be played and is left out."],
          f"a landing in force is set beside the landing given back, and a run that could not be played is said ({said['besideBack']})")
    check(said["besideOther"] is None, "another kind of second trial is not this sheet's")
    # statement 21
    marks = [
        "⟦3⟧ stands: the staff forms no take beside it.",
        "⟦3⟧ stands, and its plan is cancelled: the staff forms no take beside it, and intent:1 cannot be controlled while the place is yours to take.",
        "intent:1 is to be done by the end of day 6; ⟦3⟧ is to be done by the end of day 9, 3 days later. The staff carries out your date.",
        "⟦3⟧ waits for the plan of intent:1: it is preparing, and a take under an island is served once the island's plan is released, so the staff gives no order under ⟦3⟧ and its plan stays a draft.",
        "⟦3⟧ is your landing under an island",
        "⟦3⟧ is in force and its plan is preparing",
        "intent:4 holds base:1 (Lunga)",
    ]
    for got, want in zip(said["marks"], marks):
        check(got == want, f"a reason's words mark his landing and drop whose it is ({got})")
    check("'your landing at '" in text, "the mark of a landing of his is drawn as a link that says \"your landing at\" and the place")


def the_page_holds_it():
    """The checks by the page's text: the build's, added as each part is built (the module's last paragraph)."""
    print("-- a plan the commander writes: the page's text", flush=True)
    text = page_text()
    check('id="p-plan"' in text and 'id="p-plan-sum"' in text and 'id="p-plan-body"' in text and text.index('id="p-limits"') < text.index('id="p-plan"') < text.index('id="p-acts"'),
          "the order paper has the part YOUR PLAN after LIMITS and before ACTS")
    for words in ("'YOUR PLAN · '", "left to the staff", "'WRITE IT'", "'LEAVE IT TO THE STAFF'", "'GIVE IT BACK TO THE STAFF'", "This reading is worked with your plan.",
                  "This reading is worked with the limits you set and your plan.", "' · Your plan: '", "You hold every place of it: there is no landing to write.", ": given back to the staff", "It is wider than the order above it allows."):
        check(words in text, f"the page says {words}")
    # Rules 30 and 31 (statements 16 to 21): the papers that write a landing and give one back, the mark, and the link to his line.
    for words in ("'Write the landing at '", "'Give the landing at '", "'Landing: '", "'Change the limits of '", "whoseTag(t.maker)", "'your landing at '", "writtenMarks(t, isHisNo)", "data-landing"):
        check(words in text, f"the page holds {words}")


def places(person):
    """Lunga's and Tulagi's base numbers and hexes, from the seat's own list."""
    bases = person.call("list_bases")["ok"]
    at = {b["base"]["name"]: (int(b["base"]["handle"].split(":")[1]), [b["place"]["q"], b["place"]["r"]]) for b in bases["enemy_bases"]}
    check("Lunga" in at and "Tulagi" in at, "Lunga and Tulagi are the enemy's on the first evening")
    return at["Lunga"], at["Tulagi"]


def forces(person):
    """The seat's numbers of the two transport forces and of two land units, by their names."""
    listed = person.call("list_forces")["ok"]
    force = {f["task_force"]["name"]: int(f["task_force"]["handle"].split(":")[1]) for f in listed["task_forces"]}
    unit = {u["land_unit"]["name"]: int(u["land_unit"]["handle"].split(":")[1]) for u in listed["land_units"]}
    return force["Placeholder transports, Tulagi"], [unit["1st Raider Battalion"], unit["1st Parachute Battalion"]]


def draft_w(order, tulagi, lunga, force, units, source):
    """Draft W as statement 9 has the page send it, the order's number being `order`."""
    return {"command": "set_intent", "intent": order, "intent_kind": "control_island", "island": 3, "under": [
        {"intent": order + 1, "intent_kind": "take_place", "base": tulagi, "limits": [{"limit": "forces", "task_forces": [force], "ships": [], "land_units": units, "source": source}]},
        {"intent": order + 2, "intent_kind": "take_place", "base": lunga, "by_day": 9,
         "limits": [{"limit": "not_before", "from_day": 4}, {"limit": "after", "other_intent": order + 1}, {"limit": "risk", "risk": "preserve"}]}]}


def the_island(bridge):
    """Statement 3: the list gives the 39 days its three islands and no other scenario's change."""
    print("-- the island of both places, in the list and in the game", flush=True)
    entries = {e["id"]: e for e in json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]}
    check(entries[GAME]["islands"] == ISLANDS, f"the 39 days have three islands, the third of both places ({entries[GAME]['islands']})")
    check(entries["eastern-solomons-five-days"]["islands"] == ISLANDS[:2] and all(len(e.get("islands", [])) <= 1 for k, e in entries.items() if k not in (GAME, "eastern-solomons-five-days")),
          "no other scenario's islands change")
    check(begin(bridge, GAME, "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    check(get(bridge.table + "/config").get("islands") == ISLANDS, "the page is given the three islands as the list has them")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_draft_as_one(bridge):
    """Statements 9 to 13 through the bridge: the engine takes what the page sends for draft W, as one preview, one approval, one
    entry of the queue that takes the rest with it, and one taking back; and refuses a landing's line by its place."""
    print("-- the draft as one: what the page sends, read, signed and taken back through the bridge", flush=True)
    check(begin(bridge, GAME, "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    person = Person(bridge.table)
    (lunga, lunga_hex), (tulagi, tulagi_hex) = places(person)
    force, units = forces(person)
    alone = person.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 3})["ok"]
    check(alone.get("valid") and alone.get("under") == [], f"the order alone is read, with nothing under it ({alone.get('status')})")
    order = alone["intent"]
    takes = [s for s in alone["with_change"]["tree"]["sub_intents"] if s["kind"] == "take_place"]
    check(sorted(s["hex"] for s in takes) == sorted([lunga_hex, tulagi_hex]) and all(s["maker"] == "staff" for s in takes),
          "left to the staff, the island of both places has the staff form a take of Lunga and a take of Tulagi: one order, two landings")
    shown = person.call("preview", draft_w(order, tulagi, lunga, force, units, 3))["ok"]
    check(shown.get("valid") and shown.get("intent") == order and [u["intent"] for u in shown.get("under", [])] == [order + 1, order + 2],
          f"draft W is read as one, the landings numbered as the page numbered them ({shown.get('status')}: {shown.get('reason')})")
    check([u["command"].get("hex") for u in shown["under"]] == [tulagi_hex, lunga_hex] and shown["under"][1]["command"].get("by_day") == 9 and [len(u["limits"]) for u in shown["under"]] == [1, 3],
          "a landing sent by its base is taken as a take of the base's hex, with its date and its limits")
    subs = shown["with_change"]["tree"]["sub_intents"]
    mine = [s for s in subs if s.get("maker") == "commander"]
    check([s["number"] for s in mine] == [order + 1, order + 2] and all(s.get("standing") is True and s.get("parent") == order for s in mine) and subs[:2] == mine,
          "the reading's tree has his two landings first, each the commander's and standing")
    check(not [s for s in subs if s["kind"] == "take_place" and s["maker"] == "staff"], "the staff would form no take beside them")
    ready = {s["parent"] for s in subs if s["kind"] == "ready_landing" and s["maker"] == "staff"}
    check(order + 1 in ready and ready <= {order + 1, order + 2}, f"and would ready a landing under his landing at Tulagi, and none but under a landing of his ({sorted(ready)})")
    wide = draft_w(order, tulagi, lunga, force, units, 3)
    wide["under"][1]["limits"][2]["risk"] = "press"
    refused = person.call("preview", wide)["ok"]
    check(refused.get("status") == "refused" and refused.get("of_batch") == {"position": 7, "size": 7}, f"a landing's risk wider than the order's is refused at its place, the last of seven ({refused.get('of_batch')})")
    signed = person.call("approve", {"preview": shown["preview"]})["ok"]
    check(signed.get("status") == "queued" and signed.get("limits_queued") == 0 and signed.get("under_queued") == [{"intent": order + 1, "limits_queued": 1}, {"intent": order + 2, "limits_queued": 3}],
          f"one approval queues the order and both landings with their limits ({signed.get('under_queued')})")
    queued = person.call("queued")["ok"]
    root = [e for e in queued["entries"] if e["command"].get("intent") == order and e["command"]["kind"] == "set_intent"]
    check(len(queued["entries"]) == 7 and len(root) == 1 and len(root[0]["goes_with"]) == 6, "the queue holds seven commands, and the order's entry takes the other six with it")
    first = [e for e in queued["entries"] if e["command"].get("intent") == order + 1 and e["command"]["kind"] == "set_intent"][0]
    back = person.call("take_back", {"queue": first["place"]["queue"], "number": first["place"]["number"], "as_of": queued["as_of"]})
    check("refused" in back or back.get("ok", {}).get("status") == "refused", "the landing another waits on is not taken back before the one that waits")
    held = person.call("queued")["ok"]
    taken = person.call("take_back", {"queue": root[0]["place"]["queue"], "number": root[0]["place"]["number"], "as_of": held["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], "one taking back of the order takes all of it")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def a_line_of_his_in_force(bridge):
    """Statements 17 to 21 through the bridge: once in force the tools say whose each task is, a limit of his landing is changed by
    one preview, the staff's reason names his landing in the words statement 21 reads, and a landing is given back by its withdrawal."""
    print("-- a line of his in force: the makers, a limit changed, the reason that names it, and giving it back", flush=True)
    check(begin(bridge, GAME, "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    person = Person(bridge.table)
    (lunga, lunga_hex), (tulagi, _) = places(person)
    force, units = forces(person)
    order = person.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 3})["ok"]["intent"]
    shown = person.call("preview", draft_w(order, tulagi, lunga, force, units, 3))["ok"]
    check(person.call("approve", {"preview": shown["preview"]})["ok"].get("status") == "queued", "draft W is signed")
    check(person.call("set_leave", {"leave": "launch", "given": True})["ok"].get("status") == "queued", "leave to launch is given")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends")
    state = person.call("command_state")["ok"]
    by = {i["number"]: i for i in state["intents"]}
    check(all(by[n]["maker"] == "commander" and by[n]["parent"] == order and by[n]["in_force"] for n in (order + 1, order + 2)), "both landings are in force under the order, the commander's")
    check(by[order + 1]["limits"]["forces"]["task_forces"] == [force] and by[order + 2]["limits"]["after"] == order + 1 and by[order + 2]["limits"]["not_before"] == 4 and by[order + 2]["by_day"] == 9,
          "each shows the limits and the date he wrote")
    check(not [i for i in state["intents"] if i["kind"] == "take_place" and i["maker"] == "staff"], "the staff has formed no take")
    tasks = person.call("order", {"intent": order})["ok"]["tasks"]
    check({t["maker"] for t in tasks} == {"commander", "staff"} and [t["intent"] for t in tasks if t["maker"] == "commander"] == [order + 1, order + 2],
          "the plan order gives each task's maker: his two landings and the staff's tasks")
    reasons = [r.get("worked", "") for r in person.call("staff_orders")["ok"].get("no_orders", [])]
    said = f"intent:{order + 2}, the commander's, plans the place at ({lunga_hex[0]},{lunga_hex[1]}): the staff forms no take beside it."
    check(said in reasons, "the island's reason names his landing at Lunga in the words statement 21 reads")
    change = person.call("preview", {"command": "set_limit", "intent": order + 2, "limit": "not_before", "from_day": 6})["ok"]
    check(change.get("valid") and change.get("status") != "refused", f"a limit of his landing is read changed, by one preview ({change.get('status')})")
    given = person.call("withdraw_intent", {"intent": order + 2})["ok"]
    check(given.get("status") == "queued", "giving the landing back is its withdrawal, queued")
    held = person.call("queued")["ok"]
    mine = [e for e in held["entries"] if e["command"] == {"kind": "withdraw_intent", "intent": order + 2}]
    check(len(mine) == 1, "and is under SIGNED")
    taken = person.call("take_back", {"queue": mine[0]["place"]["queue"], "number": mine[0]["place"]["number"], "as_of": held["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], "and taken back")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def under_an_order_that_stands(bridge):
    """Statement 16 through the bridge: the order signed alone is in force with its plan waiting for his leave, the staff has formed
    no take, and a landing written under it then is read, signed and stands as his."""
    print("-- a landing written under an order that stands, before its plan has his leave", flush=True)
    check(begin(bridge, GAME, "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    person = Person(bridge.table)
    (lunga, lunga_hex), (tulagi, tulagi_hex) = places(person)
    alone = person.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 3})["ok"]
    order = alone["intent"]
    check(person.call("approve", {"preview": alone["preview"]})["ok"].get("status") == "queued", "the order alone is signed")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends")
    state = person.call("command_state")["ok"]
    plan = [p for p in state["plans"] if p["intent"] == order]
    check(not state["leaves"]["launch"] and plan and plan[0]["state"] in ("draft", "preparing") and not [i for i in state["intents"] if i.get("parent") == order],
          f"the order is in force, its plan waits for his leave and nothing is formed under it ({plan[0]['state'] if plan else None})")
    would = [s for s in person.call("propose", {"intent": order})["ok"]["tree"]["sub_intents"] if s["kind"] == "take_place"]
    check(sorted(s["hex"] for s in would) == sorted([lunga_hex, tulagi_hex]) and all(s["maker"] == "staff" and not s.get("standing") for s in would), "the staff would form both takes: each is offered WRITE IT")
    shown = person.call("preview", {"command": "set_intent", "parent": order, "intent_kind": "take_place", "base": tulagi, "limits": [{"limit": "not_before", "from_day": 5}]})["ok"]
    check(shown.get("valid") and shown["command"].get("parent") == order and shown["command"].get("hex") == tulagi_hex, f"the landing at Tulagi is read under the order ({shown.get('status')})")
    check(person.call("approve", {"preview": shown["preview"]})["ok"].get("limits_queued") == 1, "and signed with its limit")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends")
    would = [s for s in person.call("propose", {"intent": order})["ok"]["tree"]["sub_intents"] if s["kind"] == "take_place"]
    check([(s["hex"], s["maker"], bool(s.get("standing"))) for s in would] == [(tulagi_hex, "commander", True), (lunga_hex, "staff", False)],
          "the plan that waits now shows his landing at Tulagi standing, and the staff's take of Lunga still to form")
    again = person.call("preview", {"command": "set_intent", "parent": order, "intent_kind": "take_place", "base": tulagi})["ok"]
    check(again.get("status") == "refused", "a second landing at Tulagi is refused: it is already planned")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_dry_run_beside(bridge):
    """Statements 14 and 15 through the bridge, where the game has a trial: the answer for draft W is the staff's plan beside his,
    with the order's end in each and the two run for run; and a window is refused."""
    print("-- the dry run of a draft with a landing written: the staff's plan beside it", flush=True)
    check(begin(bridge, GAME, "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    person = Person(bridge.table)
    if "trial" not in get(bridge.table + "/tools"):
        print("   not run: the table's 39 days name no trial's figures, so the game offers no dry run (statement 15)", flush=True)
        check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
        return
    (lunga, _), (tulagi, _) = places(person)
    force, units = forces(person)
    order = person.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 3})["ok"]["intent"]
    shown = person.call("preview", draft_w(order, tulagi, lunga, force, units, 3))["ok"]
    answer = person.call("trial", {"preview": shown["preview"], "size": "small"})["ok"]
    beside, worlds = answer.get("beside") or {}, answer["how_it_went"]["worlds"]
    check(answer["asked"]["second_trial"] is True and beside.get("of") == "left_to_the_staff" and beside.get("order") == order, "with no compare given the second trial is played, of the same order with the landings left to the staff")
    check(beside["with_your_plan"] == answer["how_it_went"] and beside["how_it_went"]["worlds"] == worlds, "the order's end is given for each trial, in the same runs")
    counted = beside["world_for_world"]
    check(sorted(sum((counted[k] for k in ("both", "with_your_plan_alone", "with_the_staffs_alone", "neither", "apart")), [])) == worlds, "run for run, every run is counted once")
    check(set(beside["cost"]) == set(answer["cost"]) and len(beside["cost"]) == 5 and set(beside["enemy_lost"]) == set(answer["enemy_lost"]), "the staff's plan is given the same five costs for each side")
    window = person.call("trial", {"preview": shown["preview"], "dates": "chosen"})
    check('A written landing is held to a day by its "not before"' in str(window.get("refused") or window.get("ok")), "a window is refused for a draft with a landing written")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def run(bridge):
    """Everything, on a bridge started with no scenario: as `check_screen.py` calls it."""
    the_model_in_node()
    the_page_holds_it()
    the_island(bridge)
    the_draft_as_one(bridge)
    a_line_of_his_in_force(bridge)
    under_an_order_that_stands(bridge)
    the_dry_run_beside(bridge)


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument("--port", type=int, default=8786)
    parser.add_argument("--only-bridge", action="store_true", help="the checks through the bridge alone, for the build of the island before the page is built")
    args = parser.parse_args()
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        bridge = Bridge(args.port, args.sim_cli, 3, keep)
        try:
            if args.only_bridge:
                the_island(bridge)
                the_draft_as_one(bridge)
                a_line_of_his_in_force(bridge)
                under_an_order_that_stands(bridge)
                the_dry_run_beside(bridge)
            else:
                run(bridge)
        finally:
            bridge.stop()
    from check_screen import COUNT
    print(f"all held ({COUNT['held']} checks)", flush=True)


if __name__ == "__main__":
    try:
        main()
    except AssertionError as error:
        print("FAILED  " + str(error), flush=True)
        sys.exit(1)
