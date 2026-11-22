# 0214: A force asks leave to go after what it has found

Status: a plan by the architect, written on Nov 22, 2026, on the designer's idea at the table: "If a unit finds something that it believes should be getting its attention, it could be very interesting (and realistic) if it changes its plans to the thing it finds more important. For example, a carrier task force that had an assigned order to deny water at a certain location found a carrier force in another location while searching all around. It could then have a plan proposed to the commander, and if accepted, it will delay or cancel the previous order and prioritize the new order." Asked whether to plan it he said yes. It was read by a reviewer who had not seen it written; two of the designer's three first answers rested on things the review showed to be wrong, they were put to him again, and this is the plan rewritten on the review and his answers.

Summary: Today a carrier force goes after a carrier it has found only as far as its order lets it, and what lies past that it leaves alone without a word. This has the staff say so in the evening, before the day is played: where a force has found something it would go after but for how far it is or for the waters its order puts first, the staff tells the commander what was found, where the force would go and what order it would leave, and stops the clock. On his yes the force sails that night, and when it has nothing left to go after it goes back to its order by itself. Each order has a setting of its own for this: ask, go and report, or stay.

Standing: Planned on Nov 22, 2026; reviewed and rewritten the same day; confirmed by the designer on the points put to him. Built on Nov 22, 2026 (see "As built"), for the architect to read.

## Why

What is so today (rules chapters 1 and 8; `sim-staff`, `carrier.rs`):

- **A carrier force already closes on what it finds, inside bounds.** At each dusk a force with a bomber that can fly closes on a fresh contact, three pulses old or less, of a kind its risk lets it close on (a carrier at balanced, any kind at press, nothing at preserve), if the contact is within a night's steaming at full speed and its bombers' reach.
- **Past the bounds it does nothing and says nothing:**
  - a contact too far for one night is left;
  - where the order says its waters come first, a contact in the waters is preferred to a carrier outside them.
- **The staff is asked once a day, at dusk,** so a sighting of the morning or the afternoon is weighed that same evening. Commands to the staff given inside a day wait for the next dusk (decision 0156, rule 6).
- **An interrupt stops a seat only after the day has been played.** A thing the staff says at dusk of one day is answered at dusk of the next, when the contact is four pulses old or more and no longer fresh.
- **The commander is told nothing of a contact left alone** but in the reason beside the day's orders.
- **Leaves exist:** leave to prepare and leave to launch, each a permission the commander gives or withholds on an order; "with no commander, as for the enemy, both are allowed" (`docs/step3-spec.md`).

What the rows hold (`engagement_events.csv`):

- **A standing word given beforehand:** Fletcher told Spruance to "proceed southwesterly and attack enemy carriers when definitely located" (2141).
- **Commanders on the spot chose for themselves** among what was found: on 5 June Spruance, "I chose the one to the northwest" (2160); the second carrier division announced its dusk attack and ordered it (7632, 7633).
- **They held a strike for better word:** Fletcher at the Eastern Solomons, "pending more definite word" (155); Yorktown's group at Midway (2151).
- **A superior turned a force from its task on what was found:** the convoy for Port Moresby (2503, 2518), Tanaka's at the Eastern Solomons (249, 390).
- **No row here shows a commander asking a superior and being answered.** The rows support a word given beforehand at least as well as an asking each time; the setting of rule 4 is that word, and the asking is the table's.

## The rules

### What a force would go after

1. **A find is the contact the closing rule would take were one bound of the order lifted.** It is looked for at dusk, for a carrier force where the closing rule runs (not held, not going home, not fuelling, with a bomber that can fly), under an order to operate carriers or to deny waters, and only where the rule as it stands closes on nothing and has nothing in reach. The rule is then worked again whole, with one bound lifted, and the contact it takes is the find:
   - **too far:** the night's steaming taken as `pursuit_nights` nights;
   - **the waters first:** the order's preference for its waters set aside.
   Where both give a contact the find is the one of the more important kind, and of two of a kind the nearer. Nothing else of the rule is lifted: the kinds a risk closes on, the fuel to go and come back at the risk's margin, and freshness are as they are.
2. **A find never crosses a hard limit.** A contact outside the order's area limit, read up the tree, is not a find. A force that is held has none. The forces limit binds as today.
3. **A force covering a landing has no find.** A carrier force that serves a take or an island stays with it.

### Ask, go, or stay

4. **Each order has a leave to pursue,** a third leave beside the leaves to prepare and to launch, set by the commander and read up the tree as they are. It has three values:
   - **ask:** the staff asks leave for each find (rules 6 to 9);
   - **go:** the staff takes the leave as given and tells the commander (rule 10);
   - **stay:** nothing changes from today.
   For a player it is ask unless he says otherwise. With no commander it is go, as the other leaves are given.
5. **A find by the waters first is asked about even at go.** The preference is a limit the commander set, and only he sets it aside. With no commander nobody answers and the force stays.

### Asking leave

6. **The staff hands in a notice that the force asks leave to pursue:** the force; the contact, with its kind, its age and where it was reported; the hex the force would close to and the hours to it by the engine's forecast; and the order the force would leave. It is raised only for a contact the commander's own picture holds.
7. **That notice fires a standing interrupt,** new, whose subject is the force, and which the commander may switch off. **It stops the clock in the evening, before the day is played:** a seat that runs time is stopped at the dusk at which the staff would ask, with the day not yet resolved.
8. **The commander answers for that force and that contact,** yes or no, by a command that is his alone. Given at that stop it is taken in at the same dusk, before the staff is asked, so the force sails that night. A no is kept for that contact while the side holds it. No answer is no leave; the force goes on with its order and the staff asks again at the next dusk if the find still stands.
9. **Switching the interrupt off does not give leave.** It leaves the asking in the notices and the force where it is. The setting of rule 4 is what gives or withholds leave, and the table offers that, not the switch, where a player wants to stop being asked.

### Going after it

10. **With leave, given or taken, the closing rule runs with that bound lifted for that force,** and takes what it then takes. The staff's reason says it is a pursuit, by whose leave (the answer's own event is a cause of the order), and what order waits. Where the leave was taken the staff hands in a notice that the force pursues, which stops nothing.
11. **The order the force leaves is not withdrawn and not changed.** The force is away from its hex; its search and the order's other forces are as the staff's rules leave them.
12. **A leave by answer ends at the first dusk at which the force has neither a find nor a contact to close on.** The force goes back to its hex by the rule that sends it back today, and the staff hands in a notice that the pursuit is ended. A contact seen again under a new number within the bound keeps the pursuit; the leave is the force's, not the number's. The commander may end it sooner.
13. **Cancelling or changing the old order is the commander's,** as now. The staff does not propose it.

## Figures

14. One, a placeholder.

| Figure | Value | Rests on |
| --- | --- | --- |
| `pursuit_nights` | 2 | a judgement, with no row: a force is not sent after a contact it could not reach in two nights. As freshness stands a pursuit steams one night on a report and goes on only if the contact is seen again |

## What is recorded and who knows

15. **New state of a side's command:** the leave to pursue on an order; for each force, a leave by answer; and the refusals, by force and contact. Each is hashed in its place and named, and the layout rises.
16. **Three notices are new** (asks leave to pursue, pursues, pursuit ended), **one standing interrupt** with a force as its subject, and **two commands,** the setting of the leave and the answer; each code is added at the end. A side sees its own and nothing of the enemy's.
17. **The table** shows the asking as a slip "FROM STAFF · FOR DECISION": what was found and where, where the force would go and in how many hours (whole), and the order that would wait, with ALLOW and REFUSE. It does not say which bound. The chart draws the way. The order's paper has the setting, in the words "Pursuit: ask me / go and report / stay". A pursuit under way is said on the force's card and in the diary.
18. **The doctrine's version, the rules label and the agent's interface rise.** The tools gain the two commands, the finds and leaves in the command state, the interrupt, the notices and the stop before the day.

## Rule card

Reads: the side's picture of contacts; each carrier force's task, its order's risk, waters, area and leave to pursue read up the tree; the force's fuel and the engine's forecasts; the leaves by answer and the refusals. Writes: the leaves by answer and the refusals (by the commander's command); the force's orders, by the closing rule. Draws: none.

## Who answers where no one is there

- **A side with no commander** has the leave at go: its forces pursue a find that is too far, and stay for one by the waters first.
- **In a dry run** the side tried carries its orders' settings; an answer given to a numbered contact is not carried into a made-up world, whose contacts are numbered anew. The made-up enemy has no commander and goes.
- **A player who runs time and does not answer** gives no leave.

## Games on file

- **A side with no commander now pursues,** so games move wherever a carrier force of such a side holds a fresh contact beyond a night and within two. The build plays every scenario, band, validation batch and recorded game before and after and names each that moves to the designer before anything merges.
- **A recorded game played with `run`** is rebuilt from its calls, and a new stop would shift them. Such a game is remade with the interrupt switched off in its first call where that keeps it the same game; where it does not, it goes to the designer. The records of closed proofs are not altered.

## What it does not do

- **It is of carrier forces alone,** and not of one covering a landing.
- **It does not ask about a contact of a kind the order's risk does not close on.** A commander who wants every kind closed on says press, as now.
- **It does not let a commander's rating move ask, go or stay.** A man who goes without leave is the question of refusal that decision 0119 left to the designer.
- **It does not let the staff weigh one order against another,** form an order of its own, or change or cancel one of the commander's.
- **It does not ask the staff again inside a day.** Decision 0155, item 4, stands.
- **It does not make an unanswered asking lapse into a yes.**
- **It does not mend the closing rule's not reading the area limit,** which the review found; that is noted in `docs/rules/differences.md` for a ruling of its own.

## What it costs

- **One figure; a leave, two commands, three notices and an interrupt; state for the answers; a stop before the day; and the table's slip, setting, card and chart.**
- **Rulebook:** chapter 8 (operating a carrier force, the leaves, the notices, the interrupts, the commands, what is not built); chapter 1 (when a seat is stopped); the glossary gains "Find", "Leave to pursue" and "Pursuit".
- **A scenario of its own and its hand calculation,** written by the architect before any code is delegated.

## Confirmed on Nov 22, 2026

The designer was asked one question at a time.

1. **Carrier forces first.**
2. **The clock stops in the evening, before the day is played.** He had first said the staff should be asked inside a day; the review showed the staff is already asked each evening and the delay lay in when the seat is stopped. Asked again, he chose the evening.
3. **Ask, go or stay is a setting of its own on each order,** ask for a player and go where no one commands. He had first said by risk; the review showed that every order left at its default would then stop the clock, that press would set aside a limit he had set, and that a side with no commander would never pursue. Asked again, he chose the setting.

## How the plan changed on review

- **It said the staff is asked at dawn.** It is asked at dusk. The stops inside a day are dropped and the stop moved before the day.
- **It tied ask, go and stay to risk** and to the commander's rating. It is now a leave of its own, and the rating is left out.
- **It had four kinds of find.** "A station kept" was no bound (decision 0202, rule 19: "not a fence"). "The kind" would have asked about every ship sighted at balanced. Two are left.
- **It read a find contact by contact.** It is now the closing rule worked again whole with one bound lifted, and only where the rule as it stands does nothing.
- **It keyed a leave on a contact's number,** which changes when a force is seen again a day later. The leave is the force's.
- **It named the wrong hard limits.** It now reads the area.
- **It had a side with no commander ask and stay.** It goes.
- **It cited rows for a superior answering** that show a word given beforehand or a commander choosing for himself, and row 2160 for a figure it does not support.

## As built

Built on Nov 22, 2026 (branch `pursuit-team`). State layout 55, rules label 27, saved game version 24, agent interface version 43, doctrine version `supply-53`.

- **The leave to pursue is an order's** (`CommandState::pursue_leaves`, set by `set_pursuit`, read up the tree, nearest set). The leaves to prepare and to launch are a seat's, not an order's and not read up the tree, so "as they are" in rule 4 holds for the default alone: ask where leave to launch is withheld, go where it is given.
- **State:** `pursue_leaves` by intent; `pursuits` by force, each by answer (naming the answer's `CommandTaken` event) or taken, with the contact it began on and the day; `pursuit_refusals` by force and contact. A taken pursuit is recorded too, so that its end can be told and the table can show it; the engine writes it when it records a `pursues` notice and removes either kind on `pursuit_ended`. A refusal is kept: contact numbers are never reused, so one for a contact no longer held binds nothing.
- **Commands:** `set_pursuit` (intent, ask, go, stay or none) and `answer_pursuit` (task force, contact, yes or no), the commander's alone. A no also ends a pursuit under way (rule 12, "the commander may end it sooner").
- **Notices** `asks_leave_to_pursue`, `pursues` and `pursuit_ended` (codes 10 to 12) carry the force, the order left and, for the first two, the contact (by the commander's number), its kind, age and hex, the hex the force would close to and the hours. **The standing interrupt** `asks_leave_to_pursue` (code 6) has a task force as its subject (code 3).
- **The answer is among the route's causes** through `Reason::answer_of`: the reason names the force, and intake cites that force's answer from the state, so a saved game carries no event number.
- **The find** is looked for only where the closing rule gives "too far" (every fresh contact of a kind it closes on beyond a night and the reach), and not for a force under a take or an island. It must pass the route and the fuel as a closing does. The area limit is read on the contact the lifted rule takes. The commander's number is his own picture's (the area command's), matched by hex and kind where reports travel; where he does not hold it there is no asking, and a go still pursues. A pursuit also ends at a dusk at which the closing rule does not run (going home, fuelling, no bomber able to fly).
- **The stop** is worked in `sim-agent` on the preview before a day is opened, for `run`, `advance` and a watched game's steps, where the interrupt would fire (not switched off, and armed); nothing changes in the world, and the next call plays that day. A firing of this interrupt on a day played does not stop `run` again after the day. `end_day` does not stop.
- **The frozen figures** have `pursuit_nights = 1`, at which no find is made, so every frozen test is as it was. Scenario 125's test sets 2.
- **Dry runs:** the side tried is given its orders' settings as `set_pursuit` commands where the course gives leave to launch, and answers and refusals are cleared from a made-up world.

### Where the build met the plan's words

- **The waters first cannot arise.** The closing rule takes the first contact within a night and the reach before it prefers the waters, so where it is too far there is nothing the waters' lift could bring within. The lift is built and unit-tested; scenario 125's property "The waters first" is ignored, the code closing on the carrier at (12,0) with no asking. `docs/rules/differences.md` row 58, for the architect.
- **"Seen again under a new number"** cannot be written: no scenario or test hook hands a side a contact after the start. Ignored, for the architect.
- **"A force covering a landing"** is built (no find under a take or an island) but not tested in scenario 125: it needs a take, a landing and a map long enough for a find. Ignored.
- The closing rule does not read the area limit (row 57), as the review found; not mended.

### Games on file

Batches of 200 seeds of every scenario with a carrier or deny order, before and after: all the same in every run but Midway file a (4 runs differ; Japanese ships sunk 219 to 216, Allied 61 to 60; outcomes unchanged), Midway file c and its copy in `table-hunger` (40 runs differ; Japanese ships sunk 0 to 11; stalemate 200 to 189, with 9 minor and 2 major Allied victories), and the second proof's week file b (1 run differs; Japanese ships sunk 325 to 327; outcomes unchanged). No recorded game became a different game.
