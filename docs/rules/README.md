# The rulebook

The rules of Flag Plot as they stand in the code today, each stated once, in the present tense, in the order a day resolves. A person or a model should be able to take a scenario's start and this book and work out by hand what the first day does.

## What this book is and is not

- **It states the rules.** It says what the simulation does, in the order it does it, with the order of operations and the rounding where a hand calculation depends on them.
- **It does not give the reasons.** The decision records in `docs/decisions/` hold why a rule is the way it is. A rule here ends with the decisions it rests on, in brackets.
- **It does not hold the figures.** The data files in `data/reference/` hold them. The book names a figure by file and key, such as `surface_combat.toml: alarm_chance`, and copies a value only where a worked example needs it, such as a chance in the bombardment rule or the share of a hit that lands on men. A value copied is as the files had it when the page was written; the file is the figure. Every figure in those files is a placeholder unless a record says it was derived.
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
| 2 | Day start | Orders intake: numbering, checks, refusals; a yard order, a rebase order, a posture order, a march order, a prepare-landing order and an activity order are applied at their acceptance | Chapter 1, Orders; chapter 4, Repair; chapter 5, Rebasing an air group; chapter 7, The posture and its order; Marching overland; Preparing a landing; The day's activity |
| 3 | Start of each pulse | Alert from belief | Chapter 3, Alert from belief |
| 4 | Start of each pulse | Fatigue of task forces | Chapter 3, Fatigue of a force |
| 5 | Start of each pulse | Bases out of supply fixed, and the damage to each base's airfield read | Chapter 5, A base out of supply and A damaged airfield |
| 6 | Night pulse | Aircraft repair | Chapter 5, Repair |
| 7 | Night pulse | Aircraft arrivals (one due at a field that takes no aircraft waits) | Chapter 5, Aircraft that arrive on a day |
| 8 | Night pulse | Replacements, from day 2 | Chapter 5, Replacements |
| 9 | Night pulse | Daily supply for aircraft | Chapter 5, Daily supply |
| 10 | Night pulse | Men's supply: the men ashore are fed from every base of their side in the hex, the unfed are lost in part | Chapter 7, The men's supply |
| 11 | Night pulse | Stand-down and return of tired air groups | Chapter 5, Stand-down and return |
| 12 | Night pulse | Movement: halted forces whose work is done steam on, the straggler pass, forces move in order of clock time (a force with a marked waypoint and work there halts at it), fights where they meet | Chapter 2, Movement, Halting at a marked waypoint and steaming on, and Stragglers; chapter 4, Meeting |
| 13 | Night pulse | Night search and surface engagements: empty. Shore bombardment: a force on the bombardment mission, in the hex of an enemy base on its side's map (a halted force is in it), fires at the base: it destroys aircraft, supply and men there (the men less the men cover saves), wears the cover of the defenders and damages its airfield, the base's side is given a sighting report of the force, and the bombarding side a claim | Chapter 1, The night pulse; chapter 4, Shore bombardment; chapter 3, Claims |
| 14 | Night pulse | Loading and unloading, with men taken aboard and landed, units divided and bases formed; a force that lands men works at the rate its loading gives, and a unit's first men landing spend its rehearsal | Chapter 2, Cargo; chapter 7, Landing men; Preparing a landing |
| 15 | Morning and afternoon | Air search and its sighting reports | Chapter 3, Air search |
| 16 | Morning and afternoon | Strike decisions and launches | Chapter 5, The strike decision |
| 17 | Morning and afternoon | Air combat: escorts, patrols, guns, attacks (a hit on a base may land on men ashore there, which cover cuts and which tire them and wear the cover, or on its airfield), damage, reports, claims | Chapter 5, The raid, step by step |
| 18 | Morning and afternoon | Running out of fuel, then coming home to no deck, then wear, then fatigue, then supply for sorties | Chapter 5, The end of a daylight pulse for aircraft |
| 19 | Morning and afternoon | Movement, as in step 12 | Chapter 2, Movement; chapter 4, Meeting |
| 20 | Morning and afternoon | Surface engagements: an empty slot | Chapter 1, The morning and afternoon pulses |
| 21 | Morning and afternoon | Loading and unloading, as in step 14 | Chapter 2, Cargo; chapter 7 |
| 22 | End of each pulse | Contacts age and are dropped | Chapter 3, Ageing and dropping |
| 23 | After the third pulse | Ground combat: first the marches (a day marched, an arrival with its supply and any dump formed; a unit that fell back carries none); then for each hex with men ashore, the fight (each side's fire limited, an attack that fails, a defender's losses cut by its cover), fatigue (men under fire do not rest but tire), a spent attacker holding or falling back, a side beaten, a base that changes side, the beaten side's men falling back or surrendering, the ground reports; then the day's activity: each force's loading, each unit's rehearsal, training or work (the work of the airfield also mends its damage), the days of fighting counted; then digging (each fighting unit on defend that did not march gains fieldworks); last, the day's hits on men are emptied | Chapter 7, Marching overland; The fight ashore (Shots and chance, An attack that fails); What a spent attacker does; A base that changes side; A beaten side's men; What a side knows of the enemy ashore; The day's activity; Cover and fieldworks; Preparing a landing; Experience and training |
| 24 | After the third pulse | Logistics, air group upkeep, intelligence update: empty. Then the base count | Chapter 1, After the pulses |
| 25 | Day close | The day counter advances, the invariants are checked, the state hash is recorded | Chapter 1, Invariants; The state hash and saved games |

The result of a game is worked out from the final state and the log. It is not a phase and no rule reads it (chapter 6).

## The chapters

1. **The day, orders, and what a side knows.** Time and the pulses, the order a day resolves in, every order and every reason one is refused, truth and belief, what a side sees of an event, fixed-point numbers and chance, the random keys, the invariants, the state hash and saved games.
2. **Movement, fuel and cargo.** The map as the rules read it (with its land links), task forces and speed, the budget of a pulse, legs and position, the order of movement, arriving, halting at a marked waypoint and steaming on, turning back, fuel burn, refuelling in port, cargo, berths and lighters (with the port factor of a day's work), and stragglers.
3. **Search, contacts and claims.** Air search, the sighting report and its errors, belief, contacts and their ageing, a strike that finds nothing, who is alert, the alarm, fatigue of a force, claims (after a fight, a raid and a shore bombardment), and the ground report and the estimate of the enemy ashore.
4. **Surface combat and damage.** When a fight begins, its rounds, the hit mechanism shared with air attack, damage tracks and named systems, breaking off, what a fight leaves behind, repair, and shore bombardment.
5. **Air operations.** Air groups, missions, rebasing a group to another base (one with an airfield that takes aircraft), a group lost with its base, what a group can fly, a damaged airfield and what it keeps on the ground, air distance, the night's upkeep, the strike decision, escorts and patrol, the raid step by step (with the shares of a hit on a base that land on men and on the airfield), wear, fatigue and the supply that flying costs.
6. **Objectives and results.** A scenario's mission and objectives (including a base held and men lost), points, the margin and the seven outcomes, what a side knows of its progress, and the result at the end.
7. **Land units.** The land unit and troop capacity, the load order with a unit, men worked as cargo, a landing, a base formed by it, a unit divided, men lost with a ship, the men's supply and the loss of the unfed, the posture of a unit and its order, marching overland, the day's fight ashore and the limit on the men engaged, the cut of a defender's losses by cover, fatigue and the strain of fire, an attack that fails, what a spent attacker does, a side beaten, a base that changes side, a beaten side's men falling back or surrendering, the day's activity, units that work and what they build (the airfield and the mending of its damage, the port, aircraft repair and defences, with their three grades), cover and fieldworks, preparing a landing, experience and training, what a land unit is good at (its skills), men lost to the fire of bombs and shells (cut by cover, counted for the strain, wearing cover), and what a side knows of its own men and of the enemy's.

## Kept true

A pull request that adds a rule, changes one or removes one changes this book in the same pull request: a new rule gets its section and, if it has a phase of its own, its row in the order of a day; a changed rule has its section rewritten and the sections that point to it checked; a removed rule has its section and the pointers to it taken out. New terms go in the glossary, changed ones are corrected there and unused ones removed. A row of `differences.md` that a change settles is removed. A change to a figure in a data file needs no change here. `CLAUDE.md` states the rule and `docs/engineering/rule-complexity.md` holds it in full. Where the book and the code differ, the code is right, and the difference is a bug to report.

The legibility test gives a cheap model this book and a scenario's starting state and asks it for a hand calculation of the first day: who sights whom, with what chance, and what the fight or the raid costs. If it cannot follow a rule, the rule or its section is the thing to simplify before the next scenario starts (see `CLAUDE.md`, Rule complexity).
