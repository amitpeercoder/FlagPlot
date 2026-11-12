# Design note: the story of the day, and giving commands

Status: a note for discussion, written by the architect on Nov 12, 2026 at the designer's request. Nothing here is built. The three questions it first left open were answered the same day; the answers are below. It says how Flag Plot could tell the player what happened so that they care about it, how the player could give commands, and what both rest on that exists. Historical statements below are general and unsourced; any figure the game uses must come from sourced facts, as everywhere.

## The problem

The designer played War in the Pacific: Admiral's Edition and names two things it did badly: the interface, and the absence of a story. Its combat report is detailed and nobody is moved by it. The aim here is a player who still wants one more turn, and who cares about the logistics, the ships and the conditions as someone commanding them would.

Why that report tells no story:

- It is flat. Every event has the same weight, in the order it happened.
- It is not tied to what the player meant to do. Nothing says whether the day went to plan.
- Nothing carries over. Tomorrow's report does not know today's.
- Nothing in it has a history. A ship is a row.

A story is selection and cause first, and prose a long way third. The detail should stay, one step behind the story, for the player who wants it.

## What exists to build on

- **Every event names its causes,** and every computed quantity can show its factors (`docs/engineering/traceability.md`). A chain from an order to a loss can be walked today with `sim-cli events --why`.
- **An order carries its origin:** who issued it and the intent it serves (`docs/rules/`, chapter 1).
- **A side gets only its own events,** cut to what it may know (decision 0047), and a side's claims differ from the truth (decisions 0014, 0056).
- **The briefing** is a daily situation report for one side, built from its view alone, with the same parts in the same order each day (decision 0048). It is a report, not a story: it lists and does not rank.
- **Measuring** is by the engine's own functions: air distance, sea route, a force's passage with time and fuel, an air group's reach (decision 0049).
- **Objectives are counts read from state** (decision 0054).

## The idea in one line

A chronicle: a layer that reads a side's events, ranks them, joins them by cause into a few accounts, and remembers what is still open, with the full detail one step behind each account.

## The chronicle

It is built from a side's view and nothing else, like the briefing. It reads and never writes.

### Three moments

1. **Signals during the day.** As the day resolves, dispatches arrive: a sighting, a strike's report, a ship hit. They are partial and sometimes wrong, and each comes from a named sender. This is pillar 5 shown as it happens.
2. **The war diary at the end of the day.** Three to five accounts, not thirty. Each is a chain of causes told in order: what was ordered and why, what happened, what it cost, what it changes. Each opens into the full after-action detail and the breakdowns behind it.
3. **Corrections later.** A carrier claimed sunk on the 24th is sighted at Truk a week on, and the diary says so and points back to the day of the claim. This is layer 1 of `docs/design/intelligence.md`, and it is the strongest material the game has: the player learns that their own record was wrong.

### What makes an account worth telling

The chronicle ranks. An event scores higher when:

- it departs from a plan or an intent the player set;
- it cannot be undone: a ship sunk, a base lost, an experienced air group destroyed;
- it is rare for this game so far: the first carrier sighted, the first night action;
- it concerns a unit with a history, or one the player holds under direct control;
- it closes or opens a thread (below).

The ranking is the part most likely to be wrong, and it is the part to test first.

### Threads

A thread is something unfinished that the player has reason to follow: a convoy due the day after tomorrow, a contact not yet resolved, a base with four days of aviation fuel, a claim not yet confirmed. The diary ends each day with the open threads. That list is the "one more turn": the player ends the day knowing what tomorrow will answer.

Threads also feed the interrupts of pillar 3. A thread that turns bad is a candidate to stop the clock.

### Altitude

Pillar 1 says zoom level is command level. It is also the level of the story. The same events give a theatre diary, a task force's log and a ship's history, each ranked for its reader. A player who drops down to plan one carrier strike reads that strike's account in full and the rest of the theatre in a line.

### How it is written

- The chronicle produces a structured account: who, what, where, why, at what cost, pointing at the events it rests on. Text is rendered from that by templates.
- It is deterministic, so a replay tells the same story.
- A language model may be offered later as a way to render the same structured account in better prose. It is never the source of a fact, and the game runs without it. This follows the pillars: the in-game staff stays deterministic code.
- Templated prose goes stale over 180 days. That is a reason to spend the effort on ranking, layout and the map, and to keep the text short.

## Making the player care

Attachment comes from identity, history, scarcity and consequence. None of it needs flavour text.

- **A service record for every unit.** A ship, an air group or a base has a history derived from the event log: where it has been, what it fought, what it claimed, what it lost. It needs no new state. A ship with a history is one whose loss is felt.
- **Named people.** Step 1 holds pilots as a count and has no commanders (decision 0052). A named task force commander and a named squadron leader are the cheapest source of attachment, and the natural voice for a dispatch or a staff explanation. This is the one addition to the simulation in this note. People enter as entities with names that no rule reads yet (answer 1 below).
- **Logistics as the cause, not as a table.** Nobody cares about tons of supply. They care that Henderson Field flew 12 sorties and not 30 because its fuel was down to two days. Supply belongs in the diary as the reason something failed or was possible, and in a plan as the constraint that shaped it (`docs/design/planning.md`).
- **Scarcity.** What cannot be replaced matters: carriers, tankers, experienced air groups. The experience rule of step 2 serves this directly.
- **Conditions.** Weather, light and the moon are out of step 1 (decision 0052). When they arrive they should appear the same way as supply: as causes in an account, and on the map.

## Giving commands

The pillars leave open how the player expresses intent: map gestures, structured forms or plain language. The proposal is one form with two ways in.

- **The structured intent is the real thing.** An objective with a measure and a date, the forces released to it, and the constraints: "Guadalcanal held at 30 days of supply by 1 October, no carrier risked within reach of Rabaul". It is what the staff plans from, what orders name as their intent, and what a saved game holds.
- **Map gestures for where, when and who.** Drag a route, draw a patrol or search area, drop a force on an objective. While the player drags, reach rings, passage time and fuel are shown from the engine's own measuring (decision 0049), so what is possible is seen before anything is committed.
- **Plain language as a way to fill the form, after the kill gate** (answer 3 below). The player types or says what they want; it is turned into the structured intent and shown back to be confirmed. It never acts on its own.
- **Direct control** of a held force uses the same gestures, producing orders and not intent.

## Tables and detail

The player who wants every figure has it. The pillars cut modal spreadsheet screens as the only route to a decision, not lists as such.

- Lists and summaries are a second view: ships, air groups, bases, convoys, losses, claims.
- Every row leads to the unit on the map and to its service record.
- The full after-action detail sits behind each diary account, and every figure in it opens into its breakdown.

## Rules it must keep

- **No peek at the truth.** The chronicle for a player is built from `SideView` alone. An account of the enemy is an account of contacts and claims. The leak tests of decision 0047 should cover it.
- **The umpire's history is separate.** After a game ends, a history built on the truth, set beside what each side believed on each day, is a reward worth having. It is the umpire's and nothing that plays the game reads it.
- **Deterministic and read-only.** The chronicle changes no state and no hash.
- **Every sentence can be opened.** An account names the events it rests on.
- **No clerical work.** The player does not write the diary or tend the threads.

## Risks

- **The ranking buries what mattered,** or leads with what did not. Test it early and by hand against days whose story is known.
- **Sameness.** If every day reads alike the player stops reading. Quiet days should be one line; pillar 3 already passes them in seconds.
- **Scope.** This is step 4 and step 5 work. Only the cheap test below should come before the kill gate's prototype.

## Order of arrival

1. **Now, at no cost to step 2:** write a diary by hand from a side's events for the Eastern Solomons and for a week of the full slice, to find out what ranking and what threads the events can already support, and what is missing from them.
2. **With step 3:** threads and plan against actual, since both need the plan as an object.
3. **With step 4:** signals, the diary and the map gestures in the prototype.
4. **With step 5:** service records in full, named people, plain language, the umpire's history.

## Answers from the product managers

Agreed by the designer on Nov 12, 2026, on the architect's suggestion. These are directions for the design, not built rules; each becomes a decision record when its part is planned.

1. **People are entities with names, and no rule reads them yet.** A task force commander or a squadron leader has a stable id and a real name taken from sourced fact rows, under the data rules like any other fact. A person can move between units and has a service record of their own. The diary, the dispatches and the staff's explanations use them. No quality or rating is given to a person now; if one is added later it is derived from facts by a written rule, as a ship's figures are, and never invented.
2. **The kill gate has a fourth question:** "do you read the diary, or skip it?" The step 4 prototype must therefore carry the diary. A diary the player skips means the ranking failed, and it is reworked before step 5. `docs/pillars.md` is changed to say so.
3. **Plain language comes after the kill gate.** The step 4 prototype gives intent by the structured form and by map gestures only. Plain language needs a language model, which the pillars keep out of the game that runs offline, and it would hide whether the structured form is good on its own.
