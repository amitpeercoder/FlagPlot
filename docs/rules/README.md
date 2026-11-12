# The rulebook

The rules of Flag Plot as they stand in the code today, each stated once, in the present tense, in the order a day resolves. A person or a model should be able to take a scenario's start and this book and work out by hand what the first day does.

## What this book is and is not

- **It states the rules.** It says what the simulation does, in the order it does it, with the order of operations and the rounding where a hand calculation depends on them.
- **It does not give the reasons.** The decision records in `docs/decisions/` hold why a rule is the way it is. A rule here ends with the decisions it rests on, in brackets.
- **It does not hold the figures.** The data files in `data/reference/` hold them. The book names a figure by file and key, such as `surface_combat.toml: alarm_chance`, and never copies its value. Every figure in those files is a placeholder unless a record says it was derived.
- **The code is the authority.** If the book and `crates/sim-core/src/` ever differ, the code is right and the difference is a bug in the book or in the code. Report it. `differences.md` lists the places where a decision record and the code differ today, and the oddities nobody decided, for the architect to rule on.
- **It is not the spec.** `docs/step1-spec.md` says what step 1 is for. This book says what step 1 does.

## How to read a rule

- **Bracketed numbers** close a rule or a paragraph: `[0014, 0033]` names the decision records it rests on. `[0052 item 15]` names an item in a record. Find the title of a record in `docs/decisions/README.md`.
- **"A side sees:"** closes a rule that changes what is true or what a side is told. It says which events or reports a side is given of it, whole, cut down or not at all (chapter 1, What a side sees of an event). A rule with no such line has no event a side sees.
- **Where a figure comes from.** A figure is in a data file, in the scenario file, or in the code. A data file figure is named `file: key`, such as `search.toml: fresh_contact_max_age_pulses`. A scenario file figure is named by its entry and field, such as a base entry's `anti_aircraft` or an air group entry's `pilots`. A figure with no file is in the code, and the text says where. A rule whose data file the scenario does not name is not loaded and does nothing (chapter 1, Where figures come from and when a rule is loaded).
- **Cross-references** give a chapter and a section by its title, as "chapter 3, Sighting reports".
- **"Limit:" and "Not built:"** name what a rule leaves out. **"Known limit"** is the same.
- **Words** have one meaning each, given in `docs/glossary.md`.
- **Numbers** are fixed-point, and every product and quotient is cut toward zero at six decimals (chapter 1, Numbers and chance). Where the order of operations changes the last digit, the rule says the order.

## The order of a day

Chapter 1 owns the sequence. This table is its summary. Each phase names the chapter and section that hold its rules.

| Step | When | What runs | Where |
| --- | --- | --- | --- |
| 1 | Day start | Refuelling in port | Chapter 2, Refuelling in port |
| 2 | Day start | Orders intake: numbering, checks, refusals | Chapter 1, Orders |
| 3 | Start of each pulse | Alert from belief | Chapter 3, Alert from belief |
| 4 | Start of each pulse | Fatigue of task forces | Chapter 3, Fatigue of a force |
| 5 | Start of each pulse | Bases out of supply fixed | Chapter 5, A base out of supply |
| 6 | Night pulse | Aircraft repair | Chapter 5, Repair |
| 7 | Night pulse | Aircraft arrivals | Chapter 5, Aircraft that arrive on a day |
| 8 | Night pulse | Replacements, from day 2 | Chapter 5, Replacements |
| 9 | Night pulse | Daily supply for aircraft | Chapter 5, Daily supply |
| 10 | Night pulse | Stand-down and return of tired air groups | Chapter 5, Stand-down and return |
| 11 | Night pulse | Movement: the straggler pass, forces move in order of clock time, fights where they meet | Chapter 2, Movement and Stragglers; chapter 4, Meeting |
| 12 | Night pulse | Night search, surface engagements, shore bombardment: empty | Chapter 1, The night pulse |
| 13 | Night pulse | Loading and unloading | Chapter 2, Cargo |
| 14 | Morning and afternoon | Air search and its sighting reports | Chapter 3, Air search |
| 15 | Morning and afternoon | Strike decisions and launches | Chapter 5, The strike decision |
| 16 | Morning and afternoon | Air combat: escorts, patrols, guns, attacks, damage, reports, claims | Chapter 5, The raid, step by step |
| 17 | Morning and afternoon | Wear, then fatigue, then supply for sorties | Chapter 5, The end of a daylight pulse for aircraft |
| 18 | Morning and afternoon | Movement, as in step 11 | Chapter 2, Movement; chapter 4, Meeting |
| 19 | Morning and afternoon | Surface engagements: an empty slot | Chapter 1, The morning and afternoon pulses |
| 20 | Morning and afternoon | Loading and unloading | Chapter 2, Cargo |
| 21 | End of each pulse | Contacts age and are dropped | Chapter 3, Ageing and dropping |
| 22 | After the third pulse | Ground combat, logistics, air group upkeep, intelligence update: all empty | Chapter 1, After the pulses |
| 23 | Day close | The day counter advances, the invariants are checked, the state hash is recorded | Chapter 1, Invariants; The state hash and saved games |

The result of a game is worked out from the final state and the log. It is not a phase and no rule reads it (chapter 6).

## The chapters

1. **The day, orders, and what a side knows.** Time and the pulses, the order a day resolves in, every order and every reason one is refused, truth and belief, what a side sees of an event, fixed-point numbers and chance, the random keys, the invariants, the state hash and saved games.
2. **Movement, fuel and cargo.** The map as the rules read it, task forces and speed, the budget of a pulse, legs and position, the order of movement, arriving, turning back, fuel burn, refuelling in port, cargo, berths and lighters, and stragglers.
3. **Search, contacts and claims.** Air search, the sighting report and its errors, belief, contacts and their ageing, a strike that finds nothing, who is alert, the alarm, fatigue of a force, and claims.
4. **Surface combat and damage.** When a fight begins, its rounds, the hit mechanism shared with air attack, damage tracks and named systems, breaking off, and what a fight leaves behind.
5. **Air operations.** Air groups, missions, what a group can fly, air distance, the night's upkeep, the strike decision, escorts and patrol, the raid step by step, wear, fatigue and the supply that flying costs.
6. **Objectives and results.** A scenario's mission and objectives, points, the margin and the seven outcomes, what a side knows of its progress, and the result at the end.

## Kept true

A pull request that adds a rule, changes one or removes one changes this book in the same pull request: a new rule gets its section and, if it has a phase of its own, its row in the order of a day; a changed rule has its section rewritten and the sections that point to it checked; a removed rule has its section and the pointers to it taken out. New terms go in the glossary, changed ones are corrected there and unused ones removed. A row of `differences.md` that a change settles is removed. A change to a figure in a data file needs no change here. `CLAUDE.md`, under "Rule complexity", holds the rule. Where the book and the code differ, the code is right, and the difference is a bug to report.

The legibility test gives a cheap model this book and a scenario's starting state and asks it for a hand calculation of the first day: who sights whom, with what chance, and what the fight or the raid costs. If it cannot follow a rule, the rule or its section is the thing to simplify before the next scenario starts (see `CLAUDE.md`, Rule complexity).
