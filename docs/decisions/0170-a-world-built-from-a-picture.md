# 0170: A world built from a picture: the one new means of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 3 of its ten, section B, rules 3 to 5), which the designer confirmed. The hand calculation is written. Built on Nov 19, 2026 (As built, below).

Summary: A trial plays a course forward in worlds that are not the live one. Today a world can only be made at day 0 from a scenario: belief, the numbers of units made in play, the order book and the command state cannot be given to it, and every setup path refuses once a day has passed. This record adds a picture, plain data that holds everything a world needs on an evening, and one checked constructor in `sim-core` that builds a world from it. A world so built has no history before its day; what would cite an earlier cause cites the picture. Its draws come from a stream of their own, so that no trial reads the live game's dice

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Built on Nov 19, 2026

## Why

Decision 0100's experiment built worlds from a side's view and found four things the engine could not be given: the day, a claim or a dated estimate in belief, the numbers of the units made in play, and the order book. It used helpers that were never merged. Decision 0161 asks for one checked constructor in their place.

What is so today, read from the code:

- **A world is made by `World::new(seed, ref_data, map, truth)`** and nothing else. The day is a field of the state it is given and is not checked, so a state of day 5 is accepted. Belief is empty, the command state is empty, the counters start at their first numbers, the order book and the event log are empty.
- **Everything else is put in by setup calls that refuse after day 0:** starting contacts, opening estimates, command levels, commanders, radio silence. A contact's age can only come from the ageing of pulses replayed on day 0. No call writes a claim. An estimate goes in with day 0 and the grade "told".
- **The counters cannot be set.** The numbers of forces and air groups made in play start at 1,000,001 and 2,000,001; a state that already holds such a unit would be given its number again. The same holds for order numbers, a belief's contact numbers and a command state's next plan.
- **An event's number is its place in the log.** Numbers key draws and decide whether a sighting is heeded (a sighting older than the contact's last report is not). The state and belief hold event numbers: a force's arrival, what alerted it, its halt, its yard trip; a contact's last report; a claim's and an estimate's report.
- **An order's number is never looked up by a rule.** The state holds some twenty of them (a force's route, speed, cargo, readiness; an air group's mission and settings; a land unit's posture), used as the causes of what follows.
- **A few rules read the log of earlier days:** whether a force that is gone joined another or stood down (the words of a refusal), the ships lost on a day (an interrupt), yard trips (the score), and whether a note was already written (so as not to repeat it).
- **A saved game holds no state.** It is replayed from day 0. Nothing in the codebase builds a world at a later day.

## The rules

### A picture

1. **A picture is plain data** of one evening, the clock at 1800 between two days. It is a type of `sim-core` with public fields, built by whoever may build one; it is not state, is never saved and is never hashed. It holds:
   - **the day:** the last day resolved. The first day the world resolves is the day after;
   - **the state:** ships, task forces, bases, air groups, land units, formations, replacements and arrivals of both sides, as the engine's own state holds them;
   - **each side's belief:** its contacts (place, count, kind, age in pulses), its claims (place, day, pulse, and the figures a claim has) and its estimates of men ashore (place, men, day, grade). None carries an event's number;
   - **each side's command state,** whole: intents, plans, leaves, limits, holds, commitments, interrupts and the next plan's number;
   - **the orders in force:** a list. Every order the state or a command state refers to is in it, and is referred to by its place in the list, from 1;
   - **the counters:** for each side, the next number of a force and of an air group made in play;
   - **the objectives,** where the world is to have them.
2. **Nothing of a live world is in it.** It holds no event log, no order book of a live game, no daily hash and no seed. The numbers of its orders and its contacts are its own, counted from 1, so that a picture made from a side's view says nothing of how many orders or contacts the other side has had.
3. **An evening only.** A picture inside a day, at 0600 or 1200, is not in this record. A trial asked inside a day is for the step that builds the trial to settle.
4. **Command levels are off in a world built from a picture.** Each command's own picture, the reports on their way between commands, what a superior holds of his forces and radio silence are not in a picture in this record. A scenario that plays with command levels is tried without them, and the estimate says so (decision 0161, 13).

### The constructor

5. **One call builds a world from a picture,** the reference data, the map and a stream (item 12): `World::from_picture`. It is the only way to make a world that is not at its beginning with nothing believed. `World::new` and the setup calls are as they were.
6. **It refuses a picture that is not sound,** naming what is wrong, and builds nothing. It checks what `World::new` checks of the reference data and the state, the invariants of the state, and each side's command state by the invariants of the day's end. Beyond those:
   - every order the state or a command state refers to is in the picture's list;
   - no unit made in play has a number at or above its side's counter, and no counter is below its side's first number;
   - a command state's next plan is above every plan it holds;
   - a contact is on the map and has at least one ship; a claim's and an estimate's day is not after the picture's day; a belief holds no two estimates for one hex;
   - nothing is half done: no hits on men waiting to be worked, no order pending, no day in hand.
7. **The first event of the world is the picture.** The constructor records one event, on the picture's day with no pulse: the world was built from a picture of that evening. Both sides see it. It is event 1; the first thing that happens in the world is event 2.
8. **What would cite an earlier cause cites the picture.** Every number of an event that the state holds, where it holds one, is set to event 1: a force's arrival, what alerted it, its halt, its yard trip. Every contact, claim and estimate is held as reported by event 1. A force alerted before the picture is so held: alerted, by a cause the world does not have.
9. **Orders keep their place.** The order book is the picture's list in its order, numbered from 1, each as issued on the day the picture gives it; the next order taken in has the next number.
10. **Counters stand where the picture put them:** the made numbers, each belief's next contact number (one above its contacts, which are numbered from 1 in the picture's order), each command state's next plan.
11. **A world so built has no history.** Its log begins with the picture and its daily hashes with its first day. A rule that reads the log of earlier days finds none. So: an order to a force gone before the picture is refused in the general words; a ship lost before the picture is not counted by an interrupt that counts the day's losses, which reads only the day; yard trips begun before the picture are not in the score. A trial does not score (decision 0161, 15), and none of these moves a fight.

### The stream

12. **A world built from a picture draws from a stream of its own.** The key of every draw gains a domain: the number of a trial and the number of a world within it. The domain is mixed into the key after everything it holds today, and only where there is one, as the second entity and the rule's own draw were added: a live game has none, and every stream of a live game is what it was. The seed of a built world is given with its domain by whoever builds it; the constructor takes no seed from a live world, and nothing in `sim-core` passes one.
13. **The umpire may rebuild a world on its own stream.** With the umpire's key, a whole picture may be taken from a live world (both sides' halves as they are, belief and all) and a world built from it on the live game's seed with no domain. It is for the test that the constructor loses nothing, and for the umpire's tools. Nothing that plays the game can hold the key.

## What it leaves for later

- **The picture's own half from a view, and the enemy's half by rule:** step 4 (decision 0161, C). This record gives the type and the constructor; it does not say how a side fills one in. The copying of a side's own units from its view, with their orders renumbered, belongs in `sim-core`, where a new field of a unit cannot be forgotten.
- **Command levels in a picture,** and a picture inside a day (items 3 and 4).
- **What the built world should know of the days before:** known progress toward objectives, the forces a side knew and lost. A picture could carry them as data. Nothing needs them until a trial reports them.

## Rule card

- **Reads:** a picture; the reference data; the map; a seed and a domain.
- **Writes:** a new world. Nothing of any other world.
- **Reuses:** the state, belief and command state as they are, and their invariants; the event log, with one new kind of event; the order book.
- **New:** in `sim-core`, the picture, the constructor, one kind of event and the domain of a draw's key.

## What it costs

- **No live game changes.** No field is added to the state of a game begun from a scenario, no draw of such a game moves, and no hash. The domain is a field of the world: it is named with every field, and it enters the hash and a draw's key only where there is one. The state's layout version is not raised: the walk of a world with no domain is byte for byte what it was. (Decision 0057 raises the version where the layout changes; here it does not.)
- **One kind of event,** which `side_events.rs` must give to both sides, and which the viewer and the export show.
- **The agent interface** is not touched: no tool builds a world yet.
- **The rulebook:** chapter 1 gains "A world built from a picture" beside how a game begins, the event in its table, and the domain in the passage on draws; `docs/engineering/determinism.md` gains the domain.

## The hand calculation

It is in `tests/scenario_91_a_world_from_a_picture.rs`, written before the code. Three cases:

- **By hand.** Scenario 1's force, a cruiser and a destroyer at Noumea with a route to Espiritu Santo at cruise, given as a picture of the evening of day 4. The world's first day is day 5, and days 5 to 7 resolve as scenario 1's days 1 to 3 do, pulse for pulse and ton for ton, but for the day.
- **Nothing lost.** A live game stopped on an evening, its whole picture taken by the umpire, and a world built from it on the game's own stream, plays on as the live game does: the state at the end of each later day is the live game's, unit for unit.
- **What it refuses,** one picture for each line of item 6.

## As built

Built on Nov 19, 2026, on the branch `world-from-picture-build`, from the head of the record's own branch. Every statement of the hand calculation holds (`tests/scenario_91_a_world_from_a_picture.rs`, 17 tests). No number of the hand calculation was changed.

**The statement left to the build.** Scenario 1's frozen search figures drop a contact that reaches `contact_drop_age_pulses = 5` (`data/reference/step1/search.toml`). The contact of case 1, 2 pulses old in the picture, is aged to 3 at the end of the night of day 5 and to 4 at the end of the morning, and is dropped at the end of the afternoon of day 5, aged 5: the "contact dropped" event of that pulse, citing event 1, as a contact of that age is dropped in a game from day 0. So it is not held at the end of day 5. The claim and the estimate stay.

**Nothing lost.** Scenario 1 stopped on the evening of day 1 and scenario 86 on the evening of day 3, each rebuilt by the umpire from its whole picture on its own seed, play on as the live game does on every later day: the ships, task forces, bases, air groups and land units read the same field for field (an order's and an event's number read as `_` outside the engine, so they are left out by the reading itself), each side's command state reads the same, and the staff hands in the same orders, commands, decisions to give none and notices on every day (one order on day 4, two on day 5, none after). Scenario 86 ends with base 1 at 915 t, base 2 at 70 t and base 5 at 10 t. No rule was found that reads a history the built world does not have. The rules the record names that read the log of earlier days (the words of a refusal to a force gone, an interrupt that counts a day's losses, yard trips in the score, a note already written) are not reached by either game after its picture; nothing tests them across a picture.

**What changed where.**

- `sim-core`, a new module `world_picture.rs`: the picture, `WorldPicture` (the day, the state as a `TruthState`, each side's `PictureBelief` of `PictureContact`, `PictureClaim` and `PictureEstimate`, each side's `CommandState`, the orders as a list of `PictureOrder`, each side's `PictureCounters`, the objectives); `World::from_picture(picture, ref_data, map, stream)`; the umpire's `World::whole_picture(&self, key)` and `World::rebuild_from_picture(picture, ref_data, map, seed, key)`; and one walk, `walk_held_ids`, over every number of an order or an event that the state and the command states hold, which names each struct that holds one with all its fields, so that a field added there does not compile until it is walked. The checks, the re-pointing to event 1 and the renumbering of the umpire's picture all go through it.
- `rng.rs`: `RngKey` gains `domain: Option<Domain>`, absorbed after the rule's draw, the trial and then the world, each with a tag of its own (`TAG_TRIAL`, `TAG_WORLD`), and only where there is one; `Domain { trial, world }` and `Stream { seed, domain }`. A unit test shows the pinned values read with no domain and four keys that differ by domain alone giving four streams. Every struct literal of a key in the tests gains `domain: None`; no number in them moved.
- `world.rs`: the world holds `domain`, used by `rng` and `rng_pair`, read by `World::domain`; `MadeIds::at` and `MadeIds::first`. `World::new` is as it was.
- `hash.rs`: the domain is named in `name_every_field` and written last in `hash_world`, after the count of events, and only where there is one. `STATE_VERSION` is not raised.
- `belief.rs`: `BeliefState::from_picture`, crate-private, makes a side's belief from the picture's entries, each held as reported by event 1.
- `events.rs`: the event `BuiltFromPicture { day }`, added at the end, with no unit and no cause; `side_events.rs` gives it whole to both sides, with a row in its table test; the export names it `built_from_picture` with its day; the batch record and the briefing leave it out.
- `sim-agent`, the test that reads the library's source: `from_picture`, `whole_picture` and `WorldPicture` are banned words beside `World::new`.
- The rulebook: chapter 1 gains "How a world begins" (a game from a scenario, and a world built from a picture), the event in the table of what a side sees, the domain in the key of a draw and in the passage on the hash, and the invariant on belief now names the belief a picture gives. `docs/engineering/determinism.md` gains the domain. The glossary gains "picture of a world", kept apart from a command's picture, and "domain (of a draw)".

**The public surface added to `sim-core`.**

1. The module `world_picture` with its seven plain types, all fields public.
2. `WorldPicture::order_at(place) -> OrderId`: the one way outside the engine to make an order's number, the order at a place of a picture's list. Nothing else of the opaque numbers is opened: no event number can be made outside the engine (reading 3).
3. `PictureCounters::first(side)`, the counters of a side that has made nothing.
4. `World::from_picture`, `World::rebuild_from_picture` (asks for the umpire's key), `World::whole_picture` (asks for it too), `World::domain`.
5. `rng::Domain`, `rng::Stream` and the field `RngKey::domain`.
6. The event `EventKind::BuiltFromPicture`.

**Readings the record did not make.**

1. *The name.* The type is `WorldPicture`, in a module of its own, and the rulebook says "a picture of a world", so that neither the code nor the words meet a command's picture (`crate::pictures`, decisions 0134 to 0138).
2. *The day is given once.* The state carries its own day; the picture refuses a state whose day is not the picture's rather than overwrite either.
3. *No event number from outside.* A state that holds an event's number where it is not optional (a halt's event, a yard trip's departure) cannot be written by a picture made outside the engine, which cannot make one; the umpire's whole picture holds them, set to 1, and the copying of a side's half from its view (step 4) is in `sim-core`, where it can. Nothing else was opened.
4. *A side not given.* A side missing from the beliefs, the command states or the counters believes nothing, has the empty command state with its next plan 1, or starts its counters at its first numbers.
5. *The words and the order of the refusals.* Every refusal of item 6 is `SimError::InvalidSetup` whose words begin "the picture is not sound:" and name the unit, the place in the list, the counter, the hex or the day at fault; what `World::new` refuses is refused in its own words, and a command state that breaks the day's end invariants is `SimError::InvariantsViolated` on the picture's day, as a state that breaks the state's invariants is. The checks run in this order: the day; the orders referred to; the counters; the next plans; belief; hits on men; then everything `World::new` checks; then the command states' invariants; then the objectives, as `set_objectives` checks them. Only the first fault is named.
6. *A unit made in play* is one numbered above 1,000,000, and it is held against its own side's counter of its kind.
7. *Not checked, as item 6 does not ask:* an order's day of issue against the picture's day; an order's side against its unit's; a contact's age against the age at which contacts are dropped (such a contact is dropped at the end of the first pulse).
8. *The belief is made, not changed.* `BeliefState::apply_report` stays the only way a belief changes. A built world's belief is made whole by `BeliefState::from_picture`, as a game's empty one is made by `BeliefState::new`: made by sightings, two contacts of a picture within the merge distance would have become one.
9. *The umpire's whole picture* holds the orders the state and the command states refer to, renumbered from 1 in the order of their numbers in the live game, and only those; every event number its state holds is set to 1; it is refused while a day is open; it leaves out, as the picture does, the orders, commands and staff's hand-in waiting for the next day and command levels.
10. *The event's name.* It is `BuiltFromPicture`, "world built from a picture" in words: the agent's briefing must name every kind of event, and the lint on the agent's library forbids the word `World` outside `session.rs`, so the kind may not carry it.
11. *The agent's lint* bans the new names outright, though the umpire's key already keeps the two umpire's calls from it: no tool builds a world yet, and `World::from_picture` needs no key.

