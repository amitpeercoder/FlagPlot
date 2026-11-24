# 0220: An order against an enemy force

Status: a plan by the architect, written on Nov 22, 2026, on the designer's words at the table, looking at a contact's card: "From an enemy force card, should we add the option to destroy it? I'm not sure about that, but that seems natural. Maybe even the option to avoid it (in the future) or to delay it." The architect proposed destroy and shadow now, avoid later and no delay; the designer said: "Yes, please." It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Every order a commander can give is about a place or a base. None is about an enemy force, so a player who wants a force sunk must turn that into waters himself, and the order goes stale when the enemy moves. This adds two orders that name a contact: destroy it, and shadow it. The staff turns each into the tasks it already has, round where the contact is believed to be, and moves them as the reports move. The order says plainly when the contact is lost, when another report is taken to be the same force, and when it is taken for another kind of ship.

Standing: Built on Nov 23, 2026 (see "As built" at the end); planned on Nov 22, 2026; amended on review; the scenario's statements written on Nov 23, 2026.

## Why

What is so today (rules chapters 3 and 8; decisions 0012, 0123 to 0125, 0133, 0202 and 0214):

- **A contact is a number, a hex, a count of ships, a kind and an age.** A report within `merge_distance_hexes` of a fresh contact refreshes it, and it keeps its number; the report replaces its hex, count and kind. A contact not refreshed ages and is dropped at `contact_drop_age_pulses`. A strike that finds nothing drops it. A number is never used again: the same force seen later is a new contact.
- **An order to deny waters names a hex and a distance.** Under it the staff forms tasks to strike ships and to search waters from bases, a stand or carrier task for each task force its forces limit names, and, with no task force named, a carrier task for a free carrier force at a station the staff chooses.
- **A carrier force under an order closes on a carrier it has found** within the bounds of the order, and beyond them by the leave to pursue (decision 0214).
- **A contact's card at the table** offers to deny the waters round it and to operate a carrier force from its hex. Both are orders on the hex, and stay there when the contact moves.

## The rules

### The two orders

1. **Two root orders are new, each naming a contact the side holds:** destroy it, and shadow it. Each is given to the area, stands until it ends or is withdrawn, and has the limits and leaves any root order has.
2. **Each has waters:** the hexes within `contact_order_hexes` of the contact's estimated hex. They are read again each time the staff is asked, from the commander's own picture, so they move as the contact is refreshed.
3. **Destroy is served as an order to deny those waters is served,** by the same doctrine and the same tasks, with two differences:
   - the tasks are formed again where the waters have moved, the old ones ending as a task ends whose order changed;
   - a carrier force that serves it has leave to pursue that contact: the bounds of rule 10 of decision 0214 are lifted for it, and no leave is asked. A hard limit binds as there: an area limit, a hold, the forces limit.
4. **Shadow forms the tasks to search waters and no other.** No task to strike, no stand task and no carrier task is formed for it. Nothing stops a group from striking the contact under another order or its own mission.
5. **Where nothing can serve it the order says so, as a deny does:** no base with a bomber in reach, no carrier force free.

### When the contact is refreshed

6. **A report that refreshes the contact moves the order with it.** Nothing is recorded but the tasks formed again.
7. **Where a refresh changes the contact's kind from the kind it had when the order was given,** the staff hands in a notice: the contact is now taken for that kind. It is handed in once for each change. The order stands.

### When the contact is lost

8. **Where the side no longer holds the contact, the order is lost-contact from that dusk.** It keeps the last waters it had, and is served there by the tasks to search waters alone. The staff hands in a notice that the contact is lost and that it searches where it was last reported.
9. **A lost order takes a new contact.** At a dusk at which the side holds a contact that was made after the old one was dropped and whose estimated hex is within `lost_contact_reach_hexes` of the last waters' centre for each day since the loss, the order takes the nearest of them, the lowest number on a tie. Count and kind are not compared, as a report is not compared with a contact. The staff hands in a notice: contact M is taken to be the force of contact N, with its kind and where. The order is then as it was before the loss.
10. **A lost order ends at the dusk `lost_contact_days` days after the loss** with no contact taken. The staff hands in a notice that the order is ended, the force not found. An order that ends is as one withdrawn.
11. **The order never ends because the force is believed sunk.** A side does not know that. A force destroyed is no longer reported, and the order ends by rule 10.
12. **The commander may withdraw the order at any time,** and may refuse a contact taken by rule 9 by withdrawing it.

### Stops

13. **Each of the three notices (now taken for another kind, lost, ended) may stop the clock at the table,** by stops of the table's own in its panel, on unless the player unticks them. No interrupt of the engine is added.

## What a side learns by it

Nothing of the enemy that it did not hold. The waters, the kind and the taking of a new contact are read from the commander's own picture and from nothing else. With command levels on, the picture is the area command's, as in decision 0214.

## What is recorded

- **Two kinds of order, their codes added at the end.** An order's state gains the contact it names, the kind when given and last told, the last waters' centre, and the day it was lost. Each is hashed in its place and named; the layout rises.
- **Three notices, their codes at the end.** A side sees its own.
- **Figures, all placeholders and judgements:** `contact_order_hexes` 2, the distance the table offers today; `lost_contact_days` 2; `lost_contact_reach_hexes` 6 for each day, a force at 20 knots making some 12 hexes of 40 miles in a day and a half of that allowed.
- **The doctrine's version, the rules label and the agent's interface rise.** The tool that gives an order takes the two kinds and a contact's number; it refuses a contact the side does not hold.
- **The frozen copies do not gain the orders.**

## Rule card

Reads: the order's contact, in the commander's picture: its hex, kind and whether it is held; the side's contacts and the events that made them; the order's state; the figures; all that the deny doctrine reads. Writes: the order's state; sub-orders, as the deny doctrine writes them; notices. Draws: none.

## At the table

- **A contact's card offers DESTROY THIS FORCE and SHADOW THIS FORCE** above the orders on its waters.
- **The order's paper names the contact as the card does** ("the force reported at 35°26′N 178°30′E, 21 ships, taken for cruisers") and follows it: the chart draws the order's waters round the contact, and a lost order's waters dashed.
- **The notices are slips from the staff** in the words of rules 7 to 10.
- **An order given inside a day acts at 1800,** as decision 0219 has it.

## Games on file

None moves: no game on file gives either order. Recorded games are remade for the versions; the records of closed proofs are not altered.

## What it does not do

- **Avoid is not in it.** Keeping forces out of an enemy force's reach comes after the staff's regard for submarine waters (decision 0218, rule 55), whose means it shares.
- **Delay is not in it.** The staff cannot tell an order to delay from a weak attack.
- **It does not send a surface force after a contact by itself.** A surface force serves a destroy order as it serves a deny: where the forces limit names it, by a stand task in the waters.
- **It does not name an enemy ship.** A side holds contacts, not ships.
- **It does not judge whether the force is worth the order.** The commander does.

## What it costs

- **Two kinds of order served by the deny doctrine with moving waters; the lost-contact state; three notices; the table's two buttons, paper, chart and slips.**
- **Rulebook:** chapter 8 (a section of its own after Denying an area; the intent tree; the notices; the tools) and the glossary ("Destroy order", "Shadow order", "Lost contact").
- **A scenario of its own and its test statements,** written by the architect before any code is delegated: a contact that moves and is followed; one that changes kind; one lost and found again under a new number; one lost and not found; a new contact too far to be taken; a carrier that closes past the order's bounds with no asking; an area limit that binds it; shadow forming no strike.

## Questions for the designer

None before the review.

## Amended on review, Nov 22, 2026

Read by a reviewer who had not seen it written. What follows is the plan as it now stands and replaces the rules, the figures and the table's part above.

### What the review found

- **A force on the move rarely keeps its contact's number from one day to the next.** A report refreshes a contact only within `merge_distance_hexes`, 2, of one no older than `fresh_contact_max_age_pulses`, 2; a hex is 40 miles, and a force at 15 to 20 knots makes 9 to 12 hexes in a day. A bases' search at balanced risk flies once a day. So each day's sighting is most often a new contact, while the old one is held, stale, for a day and a half more. The plan as written would have served waters round the stale one, refused the new one for having been made before the old was dropped, and ended "not found" with the force on the chart.
- **The staff never withdraws a deny's tasks and re-forms none.** The tasks to strike ships and to search waters name a base and read the waters each dusk, so they need no forming again; a stand task stands at the centre and would chase a point the force left in the night; a free carrier force's station is chosen once and kept.
- **Decision 0214's leave is a force's, not a contact's,** and lifting the bound for an order is its setting "go", which exists.
- **At balanced risk a carrier force closes on a carrier and on nothing else,** so an order to destroy a force taken for cruisers would have moved no carrier. A base under a deny's task strikes what its target list reaches, with no preference for the force named.
- **A contact's kind is wrong on some three reports in ten and is replaced by each,** so a notice on every change would have stopped the clock about once a day for each order.
- **With command levels on, a command's picture numbers its own contacts and records no event of making or dropping one.** A world made for a dry run numbers its contacts anew.
- **Shadow as written was a deny with the bombers left out.**

### The order

1. **One root order is new: destroy a force.** It names a contact of the commander's picture and a distance in whole hexes, the commander's choice as a deny's is. It is given to the area and stands until it ends or is withdrawn. It may be given as search only, which is the shadow of the first plan.
2. **It is a deny whose centre is read from the force it follows, each time the staff is asked.** All that serves a deny serves it, with these differences:
   - its waters are to be struck first, by the mechanism of decision 0190, for the bases and the carrier forces that serve it: a base's bombers take a contact in them before another, and a carrier force closes on a contact in them of any kind, at balanced risk and at press;
   - its leave to pursue is "go" unless the commander sets another;
   - no stand task is formed under it. A surface force is not sent after a force on the move;
   - a free carrier force's station is chosen again at a dusk at which the waters' centre lies more than `station_rechosen_hexes` from where it was chosen;
   - a task at a base that has not reached the waters at two dusks running is withdrawn by the staff, and formed again when the base reaches them.
3. **Search only** forms the tasks to search waters and no other, sets no waters to be struck first and gives no leave. Its searches fly in both daylight pulses at any risk.
4. **Where nothing can serve it the order says so,** as a deny does, by the notice a deny uses.

### Following

5. **The order keeps, for the force it follows: the contact's number in the commander's picture, the hex and the day and pulse of its last report, and whether it was taken for a carrier.** These are kept when the order is given and at each change of rule 7.
6. **While the followed contact is fresh, the order follows it:** the centre is its estimated hex.
7. **At a dusk at which it is not fresh, the order takes another contact** of the commander's picture, where one qualifies:
   - its last report is later than the followed contact's;
   - its hex is within the distance a force at `followed_force_knots` steams in the hours since the followed contact's last report, counted in hexes from that report's hex;
   - no other order to destroy follows it.
   Of those that qualify it takes one taken for a carrier where the followed was, and one not where it was not, before another; then the nearest; then the lowest number. Orders are worked by priority, then by number.
8. **A taking is recorded and told on the order's paper,** and is no stop of the clock, but where the contact taken is of the other sort (a carrier for a force that was not, or the other way): then the staff's notice says so and may stop the clock.
9. **Where none qualifies the order is lost-contact:** its centre stays the last report's hex and it is served by the tasks to search waters alone. The staff hands in a notice, once.
10. **It ends at the dusk `lost_contact_days` days after the last report** with nothing taken, by a notice that the force was not found. It never ends because the force is believed sunk.
11. **A contact's kind changing is not a notice.** The paper shows what the force is taken for.

### What the order cannot do, said plainly

- **The staff aims at where the force was last reported.** It is asked at dusk; by morning a force has gone six hexes or more. The waters to strike first may not hold it then, and the groups strike what they reach. A wider distance holds more of where it may be.
- **By night nothing serves it.** A force that comes and goes in the dark is not met by this order.
- **It adds most against a force that is found each day:** the searches turn to it, a free carrier force is stationed for it and closes on it of any kind, and the bombers take it first.

### Pictures, seats and dry runs

12. **The number is the commander's picture's.** With command levels on that is the area command's; a seat at a task force names a contact of its own picture, matched to the area's by hex and kind as decision 0214 does. A contact is later than another by its last report, which a picture holds; nothing is read from events.
13. **Given inside a day, the order keeps what rule 5 keeps at once,** and acts at 1800 (decision 0219): if the contact is no longer fresh then, rule 7 is worked from what was kept.
14. **In a dry run the followed contact is found again by its place in the order of the picture's contacts,** which a made world keeps.

### What is recorded

- **One kind of order, its code at the end,** with the distance and whether it is search only. What rule 5 keeps is state of the side's command, by order, not part of the order's kind, so that following is no amendment of a plan. It is written by the engine when it records the staff's notices of a taking and of a loss. All is hashed and named; the layout rises.
- **Three notices, their codes at the end:** follows another contact; contact lost; ended, not found. A refusal is new: the contact is not held.
- **Figures, placeholders and judgements, in the staff's figures:** `followed_force_knots` 20; `lost_contact_days` 2; `station_rechosen_hexes` 3.
- **The stops of the table are keys of the tool that runs time,** so that an agent has them: a taking of the other sort, lost, ended.
- **The doctrine's version, the rules label and the agent's interface rise.** The frozen copies do not gain the order.

### Rule card

Reads: the commander's picture's contacts, each one's hex, kind, age and last report; the order's kept state; the figures; all that the deny doctrine, the targets and the carrier doctrine read, with the order's waters taken from the followed force. Writes: the kept state, by the engine on a notice; sub-orders formed and withdrawn; notices. Draws: none.

### At the table

- **A contact's card offers DESTROY THIS FORCE,** with the distance, 2 or 4 hexes round, and SHADOW IT for search only.
- **The order's paper says what it follows now,** by the report: where, how many, taken for what, how old; and each taking in the order it happened.
- **The chart draws the waters round the followed force,** dashed where the contact is lost.

### Before it is built

A count is made on the games for play, of sightings of one force that refresh a contact against those that make a new one, day by day. It is a check of the review's first finding and of `followed_force_knots`, and tunes nothing.

### The scenario's statements

A force at 20 knots seen on two mornings, the second a new contact while the first is held, and the order follows it. Two forces in reach, a carrier and not, and the sort decides. Two orders and one new contact. A contact dropped by a strike that found nothing. A kind that changes and stops nothing. A carrier force that closes on a force taken for cruisers at balanced. A base that falls out of reach and its task withdrawn. A free carrier force's station chosen again. Lost, then ended. Search only forming no strike. Command levels on with the area's picture late, and a seat at a task force. A dry run of a side with the order. The frozen figures untouched.

## The scenario's statements, as written (Nov 23, 2026)

Written by the architect before any code. The statements and the hand calculation are the doc comment of `tests/scenario_133_a_force_followed.rs`, 53 of them, every test function empty and marked "statement written, not built". Nobody else changes a statement, a hex, a day or a number there; a mismatch goes back to the architect.

### The games

Scenario 133, in `data/scenarios/a-force-followed/`, kept out of the loops that walk `data/scenarios/*.toml` (decision 0072): the files name an order no loader reads until this is built. The map is `data/map/placeholder_a_force_followed.toml`. All is placeholder data.

| Game | File | What it shows |
| --- | --- | --- |
| A | `133_a_followed.toml` | Two cruisers at 20 knots seen on four mornings: refreshed, taken as a new contact twice, a base's strike task withdrawn out of reach and formed again, lost, ended |
| S | game A with `search_only` | No strike task, both daylight pulses, the same days |
| B | `133_b_which_contact.toml` and seven games made from it | The sort, the nearest, the lower number, at the distance and one hex beyond it, two orders and one contact, an order given inside a day |
| D | `133_d_a_strike_that_found_nothing.toml` | The followed contact dropped by a strike that found nothing, and rule 7 worked from what the order kept |
| E | `133_e_a_kind_that_changes.toml` | A contact refreshed as another kind: no notice and no stop |
| G | `133_g_a_carrier_force.toml` | A free carrier force stationed, its station chosen again, closing on cruisers at balanced, and its strike launched |
| P | `133_p_leave_to_pursue.toml` | The order's leave to pursue: go, ask, stay, an area limit, search only |

Command levels on (a report up a pulse late, `data/reference/reports_placeholder_scenario_133.toml`), a seat below the area, a dry run, the refusals, the stops, the hash and the games on file are statements 40 to 53.

**The figures the tests read.** The frozen step 1 figures, through `common::altered_reference`, with the report's errors taken out (`position_error_chance`, `count_over_chance` and `count_under_chance` 0; each row of `kind_confusion` its own kind at 1), and for game P `pursuit_nights` 2 as scenario 125 has it. Frozen, because these are statements of what the rule computes on known figures; altered, because a test cannot hand a side a contact after the start (scenario 125 found this), so the contacts come from real ships under a real search, and the search must then report what is there. Group 1 has 100 flying boats, a placeholder number that makes the chance to sight 1 at any distance and width. The three figures of this rule are constants of the doctrine and are in no file.

**What the architect checked against the engine as it stands.** Each game was run with a deny of the same waters in place of the order, on the altered frozen figures: the sightings, the contact numbers and ages at each dusk, the passages, the strike that finds nothing, the tasks a deny forms and their intent numbers, the carrier stations (5,0) and (16,0) and the orders of day 1 are as the hand calculation has them. That is a check of the rules already built, on which the statements stand; nothing of the new rule was run, and no number was taken from a run.

### Rulings on what the plan left open

Each is the architect's, with its reason. Where one differs from the words of the plan above, the ruling holds.

- **R1. Fresh, in rules 6 and 7, is the picture's own figure,** `fresh_contact_max_age_pulses`, 2. At dusk that is a contact last reported in the day just played. It is the test under which a new report would refresh the contact, which is what the review's first finding turns on; the carrier doctrine's 3 pulses is a test of its own and is not this one. If the staff cannot read the figure through its view, that comes back to the architect.
- **R2. The pulse of a contact's last report is read from its age:** age + 1 pulses before the dusk. A picture holds an age and no event, and a report heard late carries the age it has, so this is the same in every picture. A contact a side starts with at age 0 is as one reported in the afternoon of day 0.
- **R3. What the order keeps is kept anew at every dusk at which the followed contact is held and its last report is not the one kept,** by one notice, **follows**, which the staff hands in and on which the engine writes. The notice says whether the number changed (a taking) and whether the contact taken is of the other sort. Only a taking is told on the order's paper, and only a taking of the other sort may stop the clock; the sort the order keeps changes only at a taking. Reason: rule 5 as written kept the report only at the giving and at a taking, and a strike sent to a neighbouring hex can drop the followed contact on the morning it was refreshed; rule 7 would then be worked from a report days old and a hex the force had long left. Where the followed contact is held, its last report and hex are read from the picture; where it is not, from what was kept.
- **R4. The distance of rule 7 is counted between the two reports:** the hours from the start of the pulse of the followed contact's last report to the end of the pulse of the other contact's last report, at `followed_force_knots`, in whole hexes cut, by hex distance from the followed contact's hex; equal is within. Reason: "the hours since the last report", counted to the dusk, would let the same two reports qualify at a later dusk with nothing new learnt. The start of one pulse and the end of the other, because a report says its pulse and not its hour; that gives a daylight report 3 hexes of room over what 20 knots makes between two pulses' starts, which holds a report a hex out at each end.
- **R5. At a dusk an order is worked in this order:** follow a fresh contact; else take one; else, on the dusk that begins day D + `lost_contact_days` + 1, D being the day of the last report, end; else be lost. That dusk is the first at least `lost_contact_days` days after the report. So with the figure at 2 an order is lost for one day and then ends, and a contact that qualifies on the dusk it would end is taken.
- **R6. A lost order's strike tasks and carrier tasks are withdrawn by the staff** on the dusk it is lost, and its search tasks stand; they are formed again by the deny's own rule on a taking. A group whose task is withdrawn keeps its settings, the waters to strike first among them, as the rulebook already has it for a group the staff may no longer order.
- **R7. "Two dusks running" is read without new state:** a base's task is withdrawn where nothing the task may use at the base touches the waters as they are at this dusk, nor the waters round the hex the order kept before this dusk. A bomber's normal reach for a strike task, a searcher's range for a search task, by the deny's own test. It is formed again by the deny's rule on the first dusk a reach touches.
- **R8. An order that ends is withdrawn by the engine** when it records the notice, with everything beneath it, the notice's event as the cause. The staff may not withdraw an intent the commander made.
- **R9. Search only reads its leave to pursue as stay,** forms no carrier task and no stand task, and where no base can search the waters says it cannot be met, as a deny does.
- **R10. A station is chosen again at a dusk at which it lies more than `station_rechosen_hexes` hexes beyond the farthest a station may be from the centre** (the candidates' air distance of decision 0202, rule 5, in whole hexes cut: 4 for a reach of 250 nm and a distance of 2). The task is withdrawn and formed again in the same hand-in, under a new number. This differs from the plan's words, "from where it was chosen": nothing keeps where the centre was when a station was chosen, and to keep it would be state written on a staff's command. The test on the station itself needs none and also catches a centre that creeps a little each day. If the carrier pass cannot form a task for a force whose task it withdraws in the same hand-in, that comes back to the architect.
- **R11. The three stops are keys of `run` and `advance`, off unless asked for:** a taking of the other sort, lost, ended. They stop after, never before a day: `advance` at the end of the first pulse of the day the notice is handed in, `run` at the dusk after it. Reason: none of the three waits for an answer, and a stop before the day needs the staff asked on a copy, which decision 0214 keeps for the one notice that does wait. The table asks for all three unless the player unticks them.
- **R12. The keys.** In a scenario and in the tool: `intent_kind = "destroy_force"` with `contact` (the number in the commander's picture), `distance` (whole hexes, 0 or more) and `search_only` (false where left out). A contact that is held and not fresh may be named: the order keeps it and rule 7 is worked at its first dusk. A number the picture does not hold, or none, is refused: the contact is not held.

### What it changes, and the numbers

- **State and the hash.** One kind of intent, its code at the end, with its distance and whether it is search only; and, in the side's command state by order: the contact followed, the hex, day and pulse of the report kept, whether taken for a carrier, and whether lost. Each is hashed in its place and named in `hash::name_every_field`. The state layout rises, and with it the saved game's version.
- **Codes at the end:** three notices (follows; contact lost; ended, not found) and one reason a command is refused (the contact is not held). No interrupt, no event kind and no draw is added.
- **The doctrine's version, the rules label and the agent's interface rise.**
- **The numbers** (restated on Nov 23, 2026, after decisions 0226, 0227 and 0228 merged). On `origin/main` the state layout is v59, the saved game's version 27, the rules label 32, the doctrine `supply-59` and the agent's interface 53; stage D1 of decision 0223, in build, will take an interface number. The builder merges `main` first, takes the next free number of each then (layout, saved game, rules label, doctrine, interface), and says in the pull request which it took. No number is written here to be built to.

### Owed, and not proved by the statements

- **The count on the games for play** is stated below ("The count, as stated") and is made before the build.
- **Lost is handed in once** is not shown by a game: with `lost_contact_days` at 2 an order is lost for one dusk only.
- **A group keeps the waters to strike first of a task withdrawn under it** (R6). It is a known limit of the strike doctrine and is not mended here.
- **The table's part** is proved by the page's check, stated below ("The page's check"), not by this scenario.
- **Chapter 8 and the glossary** change in the same pull request.

## The count, as stated (Nov 23, 2026)

Owed by "Before it is built". It is the umpire's: it reads the log of each game, which is the truth, and nothing that plays reads it. It tunes nothing and changes no figure, statement or file of the game.

**What is counted.** A sighting is a `ReportReceived` that a search made (its cause a `SearchAttempt`), taken with the enemy task force that attempt names, the side told, the day and pulse, the hex reported and whether it made a contact or refreshed one. For each side and each enemy task force the sightings are put in order of time. Each sighting after the first of its force is one **later sighting**, paired with the sighting before it. Two sightings of one force in one pulse are one, the first.

1. **Refreshed or new.** Of the later sightings, how many refreshed a contact and how many made a new one, in all and by the days between the pair (the same day, the next day, two days on or more).
2. **The old one still held.** Of those that made a new contact, in how many the side still held, at the dusk after, the contact the earlier sighting made or refreshed.
3. **Within the distance.** For each pair, the hex distance between the two reported hexes against ruling R4's distance: the hours from the start of the earlier sighting's pulse to the end of the later one's, at 20 knots, in whole hexes cut. How many pairs are within it, in all and by the days between.
4. **The knots it would take.** For each pair, the hex distance times 40 nm over the hours between the starts of the two pulses: the greatest, and the value that nine pairs in ten fall at or below.

**The games.** Every scenario for play on file (`for_play = true`) that has a search, staff against staff as its band is read, each side given the orders its mission implies, on the live figures, seeds 1 to 100. The counts are given for each scenario and summed.

**The check of the counter, worked by hand.** Before it is run on the games, the counter is run on game A of scenario 133 with the order's three lines replaced by a deny of the waters within 2 hexes of (8,0), on the altered frozen figures of that scenario's test. The hand calculation of `tests/scenario_133_a_force_followed.rs`, game A, gives the sightings of Japanese task force 1 by the Allied side: the mornings of days 1, 2, 3 and 4, at (8,0), (14,0), (14,0) and (8,0). So there are 3 later sightings, each paired with the morning before:

| Pair | Hexes apart | Result | R4: hours, hexes | Within | Knots it would take |
| --- | --- | --- | --- | --- | --- |
| Day 1 to day 2 | 6 | new contact | 30, 15 | yes | 240 / 24 = 10 |
| Day 2 to day 3 | 0 | refreshed | 30, 15 | yes | 0 |
| Day 3 to day 4 | 6 | new contact | 30, 15 | yes | 10 |

The counter must give: 3 later sightings, all of the next day; 1 refreshed and 2 new; of the 2 new, 2 with the old contact still held at the dusk after (contact 1 at dusk of day 3, contact 2 at dusk of day 5); 3 of 3 within the distance; the greatest 10 knots. The Japanese side has no sighting. A counter that gives anything else is wrong and is mended before it is believed.

**How it is read.** It is a check of two things and sets neither. The review's first finding holds if most later sightings of the next day make a new contact while the old is held. `followed_force_knots` and R4 stand if at least nine pairs in ten are within the distance. Where fewer are, or where the first finding does not hold, the counts come back to the architect before anything is built; no figure is moved to meet them by anyone else. The counts are written under this section as "The count, as made", with the commit and the seeds.

## The page's check (Nov 23, 2026)

A function of `table/check_screen.py`, by the page's text, by the page's own functions run in node on an answer, and through a bridge on a real game, as its other checks are. Its statements:

1. **Where it is.** The card of a contact the seat holds has two buttons above the orders on its waters, in this order: DESTROY THIS FORCE and SHADOW IT. With DESTROY THIS FORCE goes a choice of the distance, 2 or 4 hexes round. A card of anything that is not a held contact has neither.
2. **What is sent.** DESTROY THIS FORCE sends one order of the kind destroy a force with the contact's number as the seat's picture has it, the distance chosen, and search only off; SHADOW IT sends the same with search only on. Neither sends a hex. No other call is made to give the order.
3. **Given at any hour.** Both buttons are shown and work at every stop, at dusk and inside a day. An order given inside a day is drawn at once with the hour from which the staff acts on it, as decision 0219's other orders are; none is refused or hidden for the hour.
4. **The paper names the force by its report.** The order's paper says what it follows now in the words the contact's card uses: where, how many ships, taken for what, how old. It gives no contact number and no handle of the form word:number, and the place is a link made by the function the cards use.
5. **Each taking, in order.** Under that, one line for each time the order took another report for the same force, the earliest first, with the day; a report that only refreshed the contact adds no line. A taking of the other sort says what it is now taken for and what it was.
6. **A kind that changes** changes the words "taken for" on the paper and on the card and adds no line and no slip.
7. **Lost.** A lost order's paper says the force is lost and where it was last reported, with the day. A search-only order's paper lists no strike and no carrier force.
8. **The chart.** The order's waters are drawn round the place of the report it follows, at the order's distance, and move when the answer's place moves. A lost order's waters are drawn dashed round the last place. An order ended or withdrawn draws none. The ring is drawn from the order as the tool gives it; the page works no centre of its own.
9. **The slips.** Each of the three notices that may stop the clock is a slip from the staff, in the staff's voice as "we": that we now take the force for a carrier force, or no longer do; that we have lost it and are searching where it was last reported; that we have not found it and the order is ended. The slip names the force as the paper does, by its report.
10. **The stops.** The panel has three stops, each ticked unless the player unticks it: a carrier where there was none or none where there was one, lost, ended. A call of time from the page asks for the ones ticked and for no other of these; unticked, the call does not ask for it. A taking of the same sort stops nothing and makes no slip.
11. **Nothing can serve it.** Where the staff says the order cannot be met, the paper says so in the staff's words as given, as a deny's paper does.
12. **Words.** Run in node on answers with an order followed, taken twice, lost and ended, the text drawn holds "order" and never "intent"; says "the enemy" and never "his"; has the staff speak as "we"; and holds no sentence that explains a rule: none saying how far a force may have steamed, when a contact counts as fresh, how many days until an order ends, why a report is taken for the same force, or that the staff aims where the force was. It gives no figure of the rule (no knots, no count of days left, no distance but the order's own).
13. **No points and no result.** The same text holds none of "points", "victory", "margin" or "score".
14. **A watched game.** The paper, the ring and the slips are drawn for the seat watched, through the watcher's read of the same tools; the watcher is shown no button to give the order.
15. **Through a bridge.** On the table's game of scenario 133's game A, at the Allied seat: on the first evening the card of the contact at (8,0) has both buttons; with the order given at 2 hexes and three days played, the paper says the force is followed at the place of (14,0), 2 ships, taken for cruisers, with one taking of day 3, and the chart's ring is round that place; with six days played the paper says lost, the ring is dashed round the place of (8,0), and the slip of the loss was drawn; with seven played the order is ended, its slip was drawn and no ring is drawn. Run with the three stops ticked, time stopped for the loss and for the end and for nothing else of this order.


## The count, as made (Nov 23, 2026)

Made before the build, on today's code (`e7bdc8c2`, release build), as "The count, as stated" says. It is the umpire's; it tunes nothing and moves no figure. Facts only: the nine-in-ten mark and the first finding are read by the architect.

**The counter** is `scripts/0220-count/count.py` (standard library Python; `run.sh` makes the games). It reads `sim-cli export` files and counts as the section above states, with these readings where the words left a choice: a sighting is a `report_received` whose cause is a `search_attempt`; the pulses are night 0 to 12, morning 12 to 18, afternoon 18 to 24 of the day's hours; "held at the dusk after" is no `contact_dropped` or `contact_not_there` event for the old contact on the later sighting's day or before it (so a contact dropped in the night of the next day is held, as the hand calculation has it); the nine-in-ten value is the nearest rank.

**The check.** Game A of scenario 133 with the order's three lines replaced by `intent_kind = "deny"`, `hex = [8, 0]`, `distance = 2`, on the frozen figures altered as the test alters them (`position_error_chance`, `count_over_chance`, `count_under_chance` 0, every `kind_confusion` row its own kind at 1), seed 1942 as the file has it: 3 later sightings, all of the next day; 1 refreshed and 2 new; 2 of 2 with the old contact held; 3 of 3 within the distance (30 hours, 15 hexes, against 6, 0 and 6); the greatest 10 knots. Exactly the numbers stated; nothing was mended.

**The games.** Every file with `for_play = true` that gives both sides orders (the `a` files of `coral-sea`, `five-days` and `midway`, and the `a`, `b` and `c` files of `table-boats`), 15 files, on the live figures the files name, seeds 1 to 100 (`sim-cli export FILE --seed N`). Left out: files that give one side orders or none (`b`, `c`, `d` of the other folders, `table-hunger`) and the three `base_*` files, which hold no commands. Commands:

```
cargo build --release -p sim-cli
scripts/0220-count/run.sh OUT
python3 scripts/0220-count/count.py sum OUT/*.jsonl
```

Several files give identical counts: `01a_midway` and `01a_midway_submarines`; `01a_coral_sea` and `02a_coral_sea_oilers`; `02a_eastern_solomons_oilers` and `02a_eastern_solomons_oilers_submarines` (and `01a_eastern_solomons`); `03a_coral_sea_both_oilers` and its submarine file. The sums count each file once, so those games weigh as many times as there are files. `01b_midway_submarines` and `03b_coral_sea_both_oilers_submarines` have no later sighting.

Per file and side (the counts are of later sightings over seeds 1 to 100; "Within" is R4's distance; the knots are the pair's distance over the hours between the two pulses' starts):

| Scenario | Side | Later | Refreshed | New | New, old held | Within the distance | Worst knots | Nine in ten at or below |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 01a_coral_sea | allied | 552 | 384 | 168 | 42 of 168 | 552 of 552 | 13.33 | 6.67 |
| 01a_coral_sea | japanese | 747 | 496 | 251 | 35 of 251 | 747 of 747 | 40.00 | 8.33 |
| 01a_eastern_solomons | allied | 1054 | 748 | 306 | 120 of 306 | 1054 of 1054 | 33.33 | 13.33 |
| 01a_eastern_solomons | japanese | 292 | 208 | 84 | 19 of 84 | 292 of 292 | 13.33 | 6.67 |
| 01a_midway | allied | 684 | 416 | 268 | 40 of 268 | 684 of 684 | 15.00 | 9.44 |
| 01a_midway | japanese | 71 | 46 | 25 | 0 of 25 | 71 of 71 | 13.33 | 6.67 |
| 01a_midway_submarines | allied | 684 | 416 | 268 | 40 of 268 | 684 of 684 | 15.00 | 9.44 |
| 01a_midway_submarines | japanese | 71 | 46 | 25 | 0 of 25 | 71 of 71 | 13.33 | 6.67 |
| 01b_midway_submarines | both | 0 | 0 | 0 | 0 of 0 | 0 of 0 | - | - |
| 01c_midway_submarines | allied | 54 | 40 | 14 | 5 of 14 | 54 of 54 | 13.33 | 6.67 |
| 02a_coral_sea_oilers | allied | 552 | 384 | 168 | 42 of 168 | 552 of 552 | 13.33 | 6.67 |
| 02a_coral_sea_oilers | japanese | 747 | 496 | 251 | 35 of 251 | 747 of 747 | 40.00 | 8.33 |
| 02a_eastern_solomons_oilers | allied | 1054 | 748 | 306 | 120 of 306 | 1054 of 1054 | 33.33 | 13.33 |
| 02a_eastern_solomons_oilers | japanese | 292 | 208 | 84 | 19 of 84 | 292 of 292 | 13.33 | 6.67 |
| 02a_eastern_solomons_oilers_submarines | allied | 1054 | 748 | 306 | 120 of 306 | 1054 of 1054 | 33.33 | 13.33 |
| 02a_eastern_solomons_oilers_submarines | japanese | 292 | 208 | 84 | 19 of 84 | 292 of 292 | 13.33 | 6.67 |
| 02b_eastern_solomons_oilers_submarines | japanese | 246 | 193 | 53 | 13 of 53 | 246 of 246 | 13.33 | 6.67 |
| 02c_eastern_solomons_oilers_submarines | allied | 509 | 461 | 48 | 11 of 48 | 509 of 509 | 13.33 | 6.67 |
| 03a_coral_sea_both_oilers | allied | 551 | 383 | 168 | 42 of 168 | 551 of 551 | 13.33 | 6.67 |
| 03a_coral_sea_both_oilers | japanese | 739 | 493 | 246 | 34 of 246 | 739 of 739 | 40.00 | 8.33 |
| 03a_coral_sea_both_oilers_submarines | allied | 551 | 383 | 168 | 42 of 168 | 551 of 551 | 13.33 | 6.67 |
| 03a_coral_sea_both_oilers_submarines | japanese | 739 | 493 | 246 | 34 of 246 | 739 of 739 | 40.00 | 8.33 |
| 03b_coral_sea_both_oilers_submarines | both | 0 | 0 | 0 | 0 of 0 | 0 of 0 | - | - |
| 03c_coral_sea_both_oilers_submarines | allied | 220 | 153 | 67 | 16 of 67 | 220 of 220 | 13.33 | 6.67 |
| All 15 | allied | 7519 | 5264 | 2255 | 640 of 2255 | 7519 of 7519 | 33.33 | 8.33 |
| All 15 | japanese | 4236 | 2887 | 1349 | 208 of 1349 | 4236 of 4236 | 40.00 | 6.67 |
| All 15 | both | 11755 | 8151 | 3604 | 848 of 3604 | 11755 of 11755 | 40.00 | 6.67 |


By the days between the pair, over all 15 files:

| Days between the pair | Later | Refreshed | New | New, old held | Within the distance |
| --- | --- | --- | --- | --- | --- |
| same day | 4464 | 3759 | 705 | 115 of 705 | 4464 of 4464 |
| next day | 5915 | 4092 | 1823 | 706 of 1823 | 5915 of 5915 |
| 2+ days | 1376 | 300 | 1076 | 27 of 1076 | 1376 of 1376 |

### The architect's reading of the count (Nov 23, 2026)

Read from the tables above; the counter's check gave the hand calculation's numbers, so the counter is believed.

1. **The review's first finding does not hold as it was worded, and holds as far as the rule needs.** It said a force on the move "rarely" keeps its number from one day to the next and that each day's sighting is "most often" a new contact with the old one held. Of the 5,915 pairs a day apart, 4,092 refreshed the contact and 1,823 made a new one: three in ten, not most. Of those 1,823 the old contact was still held in 706, four in ten. The forces of these games are mostly carrier forces the staff keeps at a station, which move little from morning to morning; the finding was reasoned from a force steaming all day. What the rule rests on is not "most often" but "often enough that an order which could not change its number would fail": 3,604 of 11,755 later sightings changed the number, and over two days or more 1,076 of 1,376. An order that followed a number alone would have been lost at each. Rules 5 to 9 stand as amended.
2. **`followed_force_knots` and ruling R4 stand.** Every pair, 11,755 of 11,755, is within the distance, against a mark of nine in ten; pairs a day apart needed 15 knots at most. The 40 knots is of pairs in one day, where a report a hex out at each end is most of the distance and R4's pulse of room is what holds them. The count shows the distance never refuses the same force. It does not show that it refuses another force: that is the sort, the nearest and "no other order follows it", which scenario 133's game B states.
3. **Decision 0220 goes to build,** to the statements of scenario 133 as written. No figure, ruling or statement is changed by the count.
4. **The two readings the developer chose are confirmed.** "Held at the dusk after" is the dusk that ends the later sighting's day: no drop of the old contact on that day or before it. That is the dusk at which rule 7 is worked, and it is the reading of the hand calculation (contact 1 at dusk of day 3). The nine-in-ten value by the nearest rank is as good as any, and nothing is judged on it.
5. **The files left out are rightly left out.** A file that gives one side orders, or neither, is not "staff against staff, each side given the orders its mission implies"; `table-hunger` and the `base_*` files give no commands. One caution on the sums: several files are the same game counted again (the oiler and submarine files of one battle), so the totals weigh those games more than once. No share above changes its side of a mark with each game counted once; the per-file rows are what to read.
6. **The counter stays in the repository,** at `scripts/0220-count/`, as the proofs' scripts are kept: it is the umpire's, reads export files, is read by no test, no build step and nothing that plays, and is run by hand. It is not run again as a gate; if `followed_force_knots` or R4 is ever questioned, it is the tool to ask.
7. **Owed.** The review's sentence in "What the review found" is left as written, being the review's; this reading is the record of what the count found against it. No game here has a surface force running by night, the case the order says plainly it does not meet; the count says nothing of it.


## As built (Nov 23, 2026)

Built on the branch `destroy-shadow`, in three commits, to the statements of scenario 133 as written. Five points the build left to the architect are at the end.

### What was built, and where

- **The engine** (`sim-core`, `sim-data`, `sim-cli`). `command.rs`: the intent kind `DestroyForce { contact, distance, search_only }` (code 17, name `destroy_force`, addressed to the area, a root), `Followed` (what an order keeps: the contact, the hex, the day and pulse of the report, whether taken for a carrier, whether lost, kept in the command state by order), `FollowNotice` and the three notice kinds `Follows`, `ContactLost` and `ForceNotFound` (codes 13 to 15, names `follows`, `contact_lost`, `force_not_found`), the refusal `ContactNotHeld`, `report_pulse` (R2) and `report_distance_hexes` (R4), and the order's leave to pursue read as go, or stay where search only. `command/intake.rs`: the check of the contact, `kept_at_giving`, and `apply_follow_notice`, which writes the kept state on a notice and, on `force_not_found`, withdraws the order and everything beneath it with the notice's event as the cause (R8). `hash.rs` hashes the new state in its place and names it. `sim-data/src/scenario.rs` loads `intent_kind = "destroy_force"` with `contact`, `distance` and `search_only`, and refuses a `hex` or a `base` with them. `sim-cli/src/save.rs` and `export/` write the order, the notices and their `follow`.
- **The staff** (`sim-staff`). `follow.rs`: `follow_forces`, run at dusk before every other pass, by priority then by number (R5): follow a fresh contact, else take another by the sort, the nearest and the lowest number, else end, else be lost; `waters_of`; the figures `followed_force_knots` 20, `lost_contact_days` 2 and `station_rechosen_hexes` 3 as constants of the doctrine. `deny.rs` serves the order as a deny with its waters read from `waters_of`, withdraws a base's task out of reach (R7), forms no strike for a shadow or a lost order. `targets.rs` puts its waters first; `carrier.rs` has a force close on a contact in them of any kind and take the order's own leave; `station.rs` chooses a free carrier force's station again (R10) and stations for no shadow and no lost order; `stand.rs` forms no stand task; `search.rs` flies a shadow's searches in both daylight pulses at any risk.
- **The tools** (`sim-agent`, `sim-cli`, `sim-trial`). `commands.rs` and `staff_tools.rs`: `set_intent` takes `destroy_force`. `follow.rs` of `sim-agent`: the order's paper in `command_state` as `following`, and the staff's words for the three notices. `told.rs` and `session.rs`: the three stops. `sim-trial` finds the followed contact again by its place in the order of the picture's contacts in a dry run (R14). The schemas and tool descriptions are changed to say all of it.
- **The rulebook.** Chapter 8 has "Destroying a force" after "Denying an area", with the kind in the intent list, the refusal "Contact not held", the pairs that may stand under it, the three notices and the tools; chapter 1 names the three stops; the glossary has "Destroy order", "Shadow order (search only)", "Followed force", "Taking (of a contact)" and "Lost contact (of a destroy order)"; `docs/commands.md` has the tool keys; `docs/rules/differences.md` has rows 78 to 82.

### The versions taken

State layout `flagplot-state-v60`; saved game version 28; rules label 33; doctrine `supply-60`; agent interface 55. (On `origin/main` they were v59, 27, 32, `supply-59` and 53, and stage D1 of decision 0223 took interface 54.)

### The tool keys

- `set_intent` with `intent_kind` `destroy_force`: `contact` (required, the number in the commander's picture), `distance` (required, whole hexes, 0 or more), `search_only` (optional, false where left out); no `hex`, no `base`.
- `command_state`: an order of this kind has `contact`, `search_only` and `following`, its paper.
- `run`: `stop_on`, a list of `follows_other_sort`, `contact_lost` and `force_not_found`, off unless asked; stops after the day, `stopped` `told`.
- `advance`: the same three beside its five; stops at the end of the first pulse of the day the notice is handed in.
- The notices `follows`, `contact_lost` and `force_not_found`, each with a `follow`; the refusal "Contact not held".

### Open points, to the architect

Each is a row of `docs/rules/differences.md`, marked "Open: to the architect". None changes a statement of scenario 133 as written.

- **(a) Row 78.** Statement 42: a seat at a base or a task force has no `set_intent` (`NO_INTENTS_HERE`, decision 0138), so a seat below the area cannot give the order, which rule 12 and the statement have it do.
- **(b) Row 79.** Rule 13, "kept at once": a command given inside a day is taken in at dusk (decision 0219), so the report is kept at dusk. It is the same as at once unless the contact is refreshed or dropped inside the day; a dropped one is refused at dusk.
- **(c) Row 80.** R10: the staff withdraws the old station only where a new one can be chosen in the same hand-in, so a force for which none can be chosen keeps its station though it lies too far.
- **(d) Row 81.** `follow_forces` works an order whose plan is not yet released, a draft, so a notice of following can be handed in before leave to launch is given.
- **(e) Row 82.** The notices of lost and of ended, where the contact is no longer held, carry 0 ships and a kind taken from the sort the order kept (carrier or cruiser), not the count and kind last reported.

### After the build (Nov 24, 2026)

- **Statement 40.** The following first read the side's own contacts; it now reads the area command's picture (`follow.rs`, `held_contacts`), as rule 12 has it, and statement 40 holds. No other doctrine's reading was changed.
- **Scenario 130's comparisons, to the architect.** Scenario 133 is out of part F's skip list. Part F (statements 7, 8 and 24) and part D (statement 20's digests of 86, 81 and 82) fail only because `StaffNotice` and its event gained the field `follow`, which their `Debug` text says as `follow: None`: with that set aside, all 119 games of part F give the text they gave on `origin/main`. Part D's state hashes are remade for the layout v60. The tables `BEFORE` and the pinned digests are not changed; whether the digests leave `follow` out of their text, as they leave a reason's list of limits out, is the architect's.

### The architect's rulings after the build (Nov 24, 2026)

Read against the branch at `6a34d4d2`: `sim-agent/src/session.rs` (`NO_INTENTS_HERE`), `sim-core/src/command/intake.rs` (`kept_at_giving`), `sim-staff/src/station.rs` (`station_rechosen`), the lead's rows 78 to 82 and scenario 130's digests. Statements 1 to 41 and 43 to 53 hold as I wrote them.

1. **Row 78, statement 42: the code stands, and the statement is restated by me.** No seat below the area states an order of any kind (decision 0138), and this decision is not the place to give one that power. Statement 42 now says the seat's `set_intent` is answered as it is for every kind and nothing is queued; its test is the builder's to write and is no longer ignored. The second half of rule 12 (a seat at a task force names a contact of its own picture, matched by hex and kind) is **not built** and waits for the decision that lets a seat below the area state an order. Row 78 closes as "the record is amended".
2. **Scenario 130's comparisons: the digest leaves a notice's `follow: None` out of its text.** I changed the two digest functions myself (`tests/scenario_130/part_f.rs`, `without_no_follow`; `part_d.rs`, `digests`) and their doc comments; `BEFORE` and every pinned digest are untouched. The field is `None` in every game with no order to destroy a force, so with it left out the text of such a game is the text it had, and the digests go on saying whether a game moved; a notice of following is written whole. A hand-written `Debug`, as decision 0222 used for a reason's limits, would not serve here: the event's field is in a derived enum. Run after the change: all 49 tests of scenario 130 pass.
3. **Row 79, rule 13: the code stands, and rule 13 is amended.** An order given inside a day is taken in at the next dusk, as every command given inside a day is (decision 0219), and keeps its report then. Where the contact was refreshed in between, the fresher report is the better one to keep. Where it was dropped in between, the order is refused at that dusk as "Contact not held": to keep it "at once" the queue would have to carry a report, which is state for an order on a contact the side no longer holds. Statement 23's first sentence is restated by me to say when it is kept; its days and hexes are unchanged. Owed: the table shows that refusal as a slip at the dusk, since the order was accepted at the hour it was given.
4. **Row 80, R10: the code stands.** A station is withdrawn only where another can be chosen in the same hand-in; a force is not left with no task by this rule. Owed, if not already so: on such a dusk the day's reason of the carrier task says the station lies too far and that none can be chosen.
5. **Row 81: the code stands.** An order whose plan is not yet released follows its force, is lost and ends by the same clock; its tasks wait for the release as a deny's do. Following is the order's own record and forms nothing, and an order released late must serve where the force is, not where it was when the order was drafted.
6. **Row 82: to be mended before the pull request is merged.** A notice must not say what was never reported: "0 ships" and a kind made from the sort tell the player something false. The order keeps, beside the hex and the pulse, the **ship count and the kind of the report it keeps**, written when the rest is written (at the giving and on a notice of following), hashed in their place and named; the notices of lost and of ended carry them where the contact is no longer held. The sort is still kept as it is and still changes only at a taking. The layout is v60 on this branch and is not yet on `main`, so it stays v60 with its golden files remade. Statements 9, 26 and 27 have a sentence each added by me that says so.
7. **The table's choices are confirmed.** The limits offered on the order are ships, air groups and risk, as a deny's paper offers them, with the leave to pursue where a deny's paper has it; a targets limit is not offered, the order putting its own waters first. The three stops go on `run` where the game's tools have no `advance` (R11 names both). A contact an order already follows shows a link to that order in place of the two buttons; statement 1 of the page's check is read so.
8. **The scenario's files** no longer say they are not to be run: the header of each of the six games now names the order as built.

Rows 79, 80 and 81 close as "the code stands; the record is amended"; row 82 stays open until its mend is in.

### The mends after the rulings (Nov 24, 2026)

Ruling 6 is in: `Followed` keeps the count of ships and the kind of the report beside the hex, written at the giving and on every notice of following, hashed in its place and named (the layout stays v60); the notices of lost and of ended carry them where the contact is no longer held, and row 82 of `differences.md` is closed. Ruling 4's owed line is in: on a dusk where a carrier task's station lies too far and none can be chosen, the task's reason of the day says so (`station.rs`, with a unit test). Statement 42 has its test, no longer ignored. No game on file holds an order to destroy a force, so no golden file or pinned hash moved.

### The architect's rulings on the finished build (Nov 24, 2026)

Read against the branch at `d0bca845`.

1. **The slip of the dusk refusal stands, and it stops the clock.** An order given inside a day was accepted at that hour; where it is refused at dusk as "Contact not held" the commander has no order where he believes he has one, and only he can give another. So the slip is drawn at the first stop after that dusk and the clock is held there. It is the page's own stop, read from the day's events of the side, with no interrupt of the engine and no key of the tools; it is not one of the three stops a player may untick, being the refusal of his own order and not a notice of the staff's. **A known limit, owed:** the page names the place from what it held when the order was given, so a reload before the slip loses the place and the slip then says only that the order on a force we no longer hold was not taken up. It is mended when a refused command's event carries what the command named, which is work on decision 0219's queue and not on this rule. It does not hold the merge.
2. **The count of named kept fields, 6 to 8, is the developer's and is right.** Statement 50 says "each kept field is named in `hash::name_every_field`" and gives no number; the test's count was the developer's way of holding that, and it rose with `ship_count` and `kind` by ruling 6. No statement or expected number of mine was changed.

### The architect's ruling on statement 53's guard (Nov 24, 2026)

The test of statement 53 compared `data/reference/step1` and `data/scenarios/step2-frozen` with commit `e9944c16`, by `git diff`, and failed on `main` when stage 4's proof (pull request 729) added a frozen file and a line of the folder's `README.md`. Nothing frozen was changed: the difference is one file added and one line added.

- **The comparison with a past commit is taken out of the test, by me.** Statement 53 is about what the build of this decision did, and that was shown once, with its pull request. As a standing check it is wrong three ways: it fails on every frozen file a later decision rightly adds; it reads the history, so it is skipped in a shallow clone, which is why CI passed on 729; and it read a Markdown file, which no test may. A filter for files changed or deleted would still have failed on the `README.md` and would still rest on the history.
- **What holds statement 53 from now** is what the test already asserts beside it: no file of `step2-frozen` is a copy of this scenario, and no figures file, frozen or live, names the order or its three figures. The statement's doc comment has a sentence added that says how it is read. No expected number changes.
- **That a frozen figure is never edited** is the rule of `data/reference/step1/README.md` and of review; `crates/sim-data/tests/frozen_figures.rs` holds the folder's shape. No test of one scenario guards the folder.

