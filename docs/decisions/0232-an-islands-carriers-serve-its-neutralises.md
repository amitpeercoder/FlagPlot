# 0232: An island's carriers serve its neutralises, and the staff says what its carriers can and cannot do

Status: a plan by the architect, written on Nov 24, 2026 on the designer's finding at the table, and amended the same day on a review by a reader who had not seen it written ("Amended on review", at the end, says what changed on each finding). This is the plan as it stands after that review. Not built. Its statements and hand calculation are written from this text before anything is built. It has two questions for the designer. It settles row 59 of `docs/rules/differences.md`, changes decision 0202, rule 4, and adds to decision 0151, item 8a, where they are named.

Summary: Under an order to control an island the staff stations the side's carrier forces under the island's denial, in reach of the island's first place, and forms a "neutralise a base" for each enemy base near. Those neutralises read aircraft at bases and nothing else. So with two carrier forces 160 miles from an enemy base the staff says it cannot meet the neutralise of that base, names one far air base as the cause and says nothing of the carriers. This has a carrier force that works under an island's denial raid the neutralise targets of that island that its bombers reach, on a morning with no ship to strike; counts such a neutralise as served; and has every "cannot meet" of a deny or a neutralise under an island name each carrier force and why it does not serve.

Standing: Planned on Nov 24, 2026; reviewed and amended the same day. Next: its statements and hand calculation, then the build.

## Why

**The finding.** In "Guadalcanal: the first thirty-nine days", in the Allied seat, the designer ordered the island of Guadalcanal taken. The preview showed the tasks: capture Lunga, hold Lunga, deny the waters round Lunga with Task Force 16 and Task Force 11 working from (7,-5), 183 nm from Lunga, their bombers' least normal strike radius 190 nm; and neutralise Lunga, neutralise Tulagi and neutralise Shortland. Under each neutralise the page said "No air base can reach Tulagi: Espiritu Santo, 538 miles away, has no bomber group the staff may use". Tulagi is at (3,-5), four hexes from the two carrier forces. He said: "That seems not right!"

**The cause is known and was left open.** Row 59 of `docs/rules/differences.md`, found on the build of decision 0217 and "left to a ruling of its own, the architect's": the neutralise an island forms hands in "cannot meet" whenever no base reaches its target, even on days the island's own carrier force has its bombers raid that target (scenario 76's carrier variant: the notice on every day of 14, the raids on days 2 to 4). The question it left was whether a neutralise whose target a carrier force under the same island raids is met. This record rules that it is, and says what such a force does.

**What is so today,** read in the code on Nov 24, 2026:

- **The island's carriers are stationed under its denial** (`sim-staff/src/island.rs`, `island_carriers`): each carrier force the staff may take, with a carrier afloat and a group aboard, is given "operate a carrier force" under the denial at the covering hex of decision 0148, item 2, reckoned to the island's first place. The covering hex is set by the least normal strike radius of the bombers aboard. The station is not weighed for danger.
- **Such a force already serves two ends from that station** (`sim-staff/src/cover.rs`, `island_strike_take`; decision 0151, item 8a): its strikes on ships are the denial's, and on a morning with no fresh contact of ships in a bomber group's reach its bombers strike the enemy's base at a place the island is taking: the first take of the island's tree, in the order of its places, whose plan is released and whose place is not yet the side's and is within a bomber group's reach of where the force lies. Where that place has no enemy base, nothing is struck and nothing else is tried (`sim-staff/src/carrier.rs`, where a take found shuts out every other aim).
- **The island's neutralises never read a carrier** (`sim-staff/src/neutralise.rs`, `neutralises`): a neutralise is served by a base with a bomber in reach, or by a carrier force that decision 0202's pass gave it. That pass (`sim-staff/src/station.rs`, `carriers_for_ends`) reads only the commander's own orders (0202, rule 4), so it records nothing for a neutralise the staff formed under an island.
- **So the words say nothing of the carriers.** `cannot_neutralise` adds the carriers' clause only from what the pass recorded (`carrier_words`), which for an island's neutralise is nothing. The island's own denial has the same fault where no carrier task stands under it (`sim-staff/src/deny.rs`).
- **A commander's own neutralise of a base the island also wants is no better off.** The island forms none beside it (`island_neutralise` stands on it); the island's carriers are not free for 0202's pass, each having a carrier task; so that order too says "cannot meet" with the carriers in reach.
- **The engine can fly the raid** and the staff orders it already under a commander's own neutralise (0202, rule 21): `strike_base`, the take's routine given a base.
- **The table drops what the staff does say** (`table/index.html`, `plainFact`): of the sentence "No base ... may use has a bomber group that reaches ..." it keeps the first base and no other, prints "has no bomber group the staff may use" of that base whatever the staff's line said of it, and has no reading of the carriers' clause for a neutralise at all.
- **Once a landing is in, the island's carriers strike nothing ashore.** The take's plan is ended, and no rule sends them at any base the island is to neutralise.

## The choice

Two ways were open: (a) a force already stationed for the island's denial also raids the island's neutralise targets in reach; (b) the island's denies and neutralises go through decision 0202's one pass. This plan takes (a) and leaves the island's stationing as it is. The reasons:

1. **(b) alone would not answer the finding.** In one pass the denial is served first and takes every free force (0202, rule 3: a force is one order's). The neutralises would then say "taken by the denial" and still not be raided. One force serving two ends is needed either way.
2. **One force serving two ends from one station is already the island's rule** (0151, 8a), with ships first. (a) gives that rule more aims. It forms no new task, allows no new pair and adds no second way of handing out carriers.
3. **(b) changes where the carriers stand in every game with an island order.** By 0202's choice a deny of waters 3 hexes wide puts the station within 70 nm of the first place where the bombers reach 190 nm, not at the covering hex 183 nm off; and at preserve it forms no station at all, since the enemy's base at the place casts its circle. That is the carrier cover of every landing in the 39 days. It is a change to how a landing is covered, not a mend to a neutralise, and it wants a plan of its own and the designer's word (the first question below).

## The rules

### Which carrier forces serve which neutralises

1. **A carrier task that stands under an island's denial serves each of the island's neutralises whose target it can reach.** It can reach a target that is an enemy base on the side's map within the normal strike radius of a bomber group it may use, measured in a straight line from the task's hex, as the engine measures a strike's reach (decision 0040). The task's hex is the covering hex the island gave it; this rule does not move it.
2. **A bomber group it may use** is a bomber group (not a fighter, with a weapon: the raid's test) aboard a carrier afloat of the force, that the commander does not hold and that the side may use on that day. It is the set the carrier doctrine gives orders to; a held group, or one before its day, reaches nothing.
3. **The island's neutralises** are each "neutralise a base" in force on the side, with its plan released, of an enemy base on the side's map within 15 hexes of a named place of the island: the ones the island formed and the ones it stood on, whoever made them, a commander's own and another island's among them. A neutralise of a base the island would not neutralise is not the island's.
4. **A forces limit is kept.** Where a limit that binds ships stands on the neutralise's own chain, the force serves it only if it is inside every such limit. An island's own neutralise has none unless the commander set one.
5. **Nothing is formed for it.** The carrier task stays under the denial, one task for a force, as now. No carrier task is formed under an island's neutralise.
6. **Decision 0202's pass is unchanged.** A commander's own neutralise still takes a free force through it, with a station chosen by risk. Where it is also the island's by rule 3, the island's carriers serve it too, from where they work.
7. **Which forces the island takes is unchanged:** each carrier force the staff may take (decision 0149, item 5), inside the island's forces limit where that names task forces, with a carrier afloat and a group aboard. Decision 0222, rule 2, and row 75 of `docs/rules/differences.md` are not touched.

### What the force does each morning

8. **Ships come first, as now.** A bomber group with a fresh contact of ships it would launch at within its reach is left for ships (0148, item 4; 0190, rule 17).
9. **Otherwise the force's bombers go at one base that morning: the first of its aims.** The aims of a force at a hex are, in this order, each only where its base is an enemy base on the side's map within the reach, from that hex, of a bomber group the force may use:
   - **(a) the take's strike, as now** (0151, 8a): the enemy's lowest-numbered base at the place of each take of the island's tree, in the order of the island's places, whose plan is released and whose place is not the side's;
   - **(b) the base at a named place whose landing has been launched and is over:** the target of an island's neutralise that lies in a named place's hex, where no take of the island's tree for that place has a plan that has not yet been released, in the order of the island's places;
   - **(c) the bases off the places:** the target of each other neutralise of the island, in the order of the neutralises' numbers.
10. **Two things in rule 9 are new beside the aims themselves.** A take whose place holds no enemy base is no aim, and the next is tried, where today it shuts the others out. And (a) is every such take in place order, of which the first in reach was already the one taken: the words now say what the code does.
11. **The carriers do not raid a place before its landing is launched.** By (b), the base at a named place the island is still to take is not a carrier's aim until that take's plan is released. A base of the side with a bomber in reach raids it from the island's release, as now. The bases off the places, (c), are aims from the morning the island's plan is released.
12. **One base a force a morning.** The groups that reach the first aim go by the raid's rules; a group that does not reach it flies no raid that morning and is not sent at another base. It is the rule a base's bombers already keep: a group serves one raid, the first in priority order (rulebook, Raiding a base). The next aim is taken when the first is no longer one: the base is the side's, or its neutralise is withdrawn.
13. **The strike is the take's routine given that base** (`strike_base`; 0148, items 4 to 6, as 0202, rule 21, reuses it): the raid's rules by the island's risk decide which groups go, what they send and what they hold back, and the fighters aboard stay the force's cover. Its reason says which neutralise the base is the target of, as under a commander's neutralise.
14. **Reach is read where the force lies.** The carrier doctrine works the aims from the hex the force is in that morning. A force that has closed on a contact, or gone home for fuel, and lies beyond reach of every aim flies no raid. Nothing else of the carrier doctrine changes: its patrol, its search, its closing, its fuel.
15. **One routine gives the aims of a force at a hex,** and both doctrines use it: the area's, with the task's hex, to say whether a neutralise is served and where it stands in the aims; the carrier's, with the hex the force lies in, to choose the morning's base.
16. **It ends as the neutralise ends:** on the morning the target is the side's the staff withdraws a neutralise it formed (0151, 8b), and the base is no aim.

### Whether the neutralise is met

17. **A neutralise of the island's is served where a base qualifies, or a carrier task serves it by rule 1,** or, for a commander's own, 0202's pass gave it a force. Where only the island's carriers serve it, the staff forms no raid task for a base, hands in a decision to give no order with the reason of rule 19, and gives no notice that it cannot meet the order, as a deny that only a carrier force serves gives none (decision 0217). Where a base qualifies too, the raid task is formed as now and the carriers raid beside it.
18. **Its measure is unchanged.** It already counts a raid on its target by any group of the side (0202, rule 22).

### What the staff says

19. **Where a carrier task serves it,** the area's reason gives, for each such force:
    - the task, its hex, that it works there under the island's denial, the distance from that hex to the target, and the bomber groups it may use that reach it, with their reach;
    - where the target stands in the force's aims as reckoned from the task's hex: each aim before it, by name, or that it is the first; and, for a base at a place whose landing is not yet launched, that the bombers strike it from the landing's launch;
    - **where the force lies now,** as the area's picture has it and, under command levels, as of when that was known: at its hex; or elsewhere, with that hex and its distance from the target, and, where the target is beyond the bombers' reach from there, that no raid is flown from there, with what the force is doing where the staff ordered it (closing on a contact, named; going home for fuel).
    It then gives each base's line as now. What would change it: the carrier task withdrawn, the force held, an aim ahead of it taken.
20. **Where nothing serves it, "cannot meet" names the bases and the carrier forces both,** and never the bases alone where the side has a carrier force with a carrier afloat and a group aboard. After the bases' lines, in the words decision 0202, rule 28, begins with ("and no carrier force can serve it:"), each such force of the side has one line, the first of these that is true of it:
    - it has a carrier task that does not serve this neutralise by rule 1, named with its number and whose it is: the commander's own, another order's, or one under the denial of another island that this neutralise is not also a neutralise of;
    - it is not the island's to take, in the words the test of a force the staff may take gives today (such as held, named by an order higher in priority, or before its day);
    - no sea route reaches a covering hex for it, so it has no task under the denial;
    - the island formed no denial of its own because one stands in another tree, named, and the carrier forces under that one are that order's;
    - it works from a hex under the denial and has no bomber group aboard;
    - it works from a hex under the denial and each bomber group aboard is held by the commander or may be used only from a later day, each named with which;
    - it works from a hex under the denial and is outside a forces limit on the neutralise's chain, named;
    - it works from a hex under the denial, and the target is so many nm from there, beyond the reach of each bomber group it may use, each reach given.
    What would change it then names a carrier force working within reach of the target, beside a bomber group at a base within reach.
21. **The island's own denial says the same.** Where no base reaches the waters and no carrier task stands under the denial, its "cannot meet" names each carrier force with the line of rule 20 that is true of it, where today it names none.
22. **A side with no carrier force gets the words it gets today,** so no reason of a game without carriers moves (as 0202's reading 6).
23. **The sentences have fixed beginnings,** stated in the test's doc comment with the hand calculation, so that the tools and the table read them without guessing: one for "a carrier task serves it", and the clause of rule 20. **The carriers' clause holds no full stop inside it:** distances and reaches are given in whole nm, and names are given as elsewhere, so that the clause ends at the sentence's end.

## At the table

The page says what is known, in the table's words (decision 0221, items 15 and 16): names and miles, no intent numbers, no caption that explains a rule. Each line below is a fact of this order or of this day.

24. **A neutralise the carriers serve is not shown as an order that cannot be met.** Under its task the paper names the forces and the distance, then what they do about it today. For the game of the finding, under "Neutralise Tulagi": "Task Force 16 and Task Force 11 can reach it, 160 miles from where they work." and then the day's fact, one of: "Their bombers go to it this morning."; "Their bombers go to Lunga this morning."; "No raid this morning: ships are in reach."; "They strike it once the landing at Lunga is launched." (for a base at a place, before its landing); "Task Force 16 is away, 420 miles off, going home for fuel." In a preview the day is the first the plan would act on.
25. **A neutralise nothing serves says every base and every carrier force, each with its own fact,** and is never the one line about one air base. Each base has the fact the staff gave of it: that it has no bomber group the staff may use, or that its bombers reach so far against a distance of so much, with the type. For Shortland, with the miles the staff gives: "Nothing we have can reach Shortland. Espiritu Santo, [n] miles away, has no bomber group the staff may use. Task Force 16 and Task Force 11 work [n] miles from it; their bombers reach 190." A force that is not the island's is said so with the order that has it, as a link, as the paper does for a deny today.
26. **The page keeps every base the staff names,** not the first, for a neutralise as it does for a deny.
27. **What would change it** is listed as now, with the carriers' line of rule 20 among it.
28. The exact wording is the manager's to settle inside the table's rules. What must not happen is settled here, for an order to control an island and everything formed under it, and for a commander's neutralise that is the island's by rule 3: a line that names an air base as the reason while the side's carrier forces go unnamed. A commander's own neutralise away from any island is decision 0202's, whose words name the carriers already wherever its pass reads the order; one whose forces limit says its forces work where they lie is passed over by that pass and still names bases alone, and is not mended here (What it does not do).

## Rule card

Reads: the side's own intents in force and their plans (the island, its denial, its takes, the neutralises that are the island's by rule 3, the carrier tasks under the denial and elsewhere), holds on forces and on air groups, forces limits, and the day each group may be used from; the island's places, from the map; the side's own task forces, their mission, ships afloat, air groups aboard and where each force lies, as the area's picture has it for the area's reason and as the force knows it for its own morning; aircraft types' role, weapon and normal strike radius; the enemy bases on the side's map, by place; the contacts of the force's picture, by place, kind and age (for ships first, as now); the raid's figures by risk. Writes: orders to air groups aboard a carrier force (mission, target base, held share), as the strike under a take writes them; reasons; one notice fewer ("cannot meet") where a carrier task serves. Forms no intent. Adds no state. Draws no random number. The engine is not changed.

## What it changes

- **Row 59 of `docs/rules/differences.md` is settled** and is taken out in the build's pull request: a neutralise whose target a carrier force under the same island can reach is served, by rule 17.
- **Decision 0202, rule 4.** Its first sentence stands: the one pass is for the commander's own orders. Its second is replaced: a neutralise that is an island's is served by the island's carriers from the station of the island's denial, by this record. Its third does not hold as written: the island's doctrine is changed here and the difference is not closed. **The island still stations its carriers without weighing danger.** That stays a known difference, kept in `docs/open-work.md` until the first question below is answered. The record of 0202 is kept as written.
- **Decision 0151, item 8a.** The strike on the place of a take stands and comes first; rule 9 adds the aims after it, and rule 10 says what else differs.
- **Decision 0217, statement 8,** and scenario 76's carrier variant: restated below.
- **Versions.** The doctrine version rises by one from what `main` has when this is built (`supply-62` today). The rules label, the state layout, the hash version and the agent interface version do not move: no state, no engine rule and no key is added. A game kept by `play --game` under the old doctrine cannot be resumed for more play, as with every doctrine change; a table game in progress is begun again.
- **The rulebook, chapter 8,** in the same pull request: Controlling an island (the tree's rows 4 and 5, and "The carriers"); Operating a carrier force, Under "control an island" (the aims and their order); Neutralising a base (when it is served, "No base qualifies" and its words); Denying an area (the words of rule 21); Carriers serving an order for an end, rule 1's last sentence; the doctrine version's paragraph. The glossary: "covering hex" and "station" are read again and say that a force at an island's covering hex raids the island's neutralise targets in reach; "aim" is added if the rulebook's text uses it as a term.
- **`docs/open-work.md`:** the island's stationing without weighing danger, as open; and the line that says what the staff chooses for a carrier force.

## How it is tested

**A new scenario, a placeholder** with its own classes and bombers, in a folder of its own and frozen beside the step 1 figures, as scenario 116 is (`data/scenarios/carriers-serve-an-end/`). It takes the next free number, 137 when this was written; the manager checks `origin/main` and the open pull requests first. Until its hand calculation is written it is kept out of the loops that walk `data/scenarios/*.toml` (decision 0072).

Its setup, in outline: an island of two places. The first, P, has an enemy base. A second enemy base, E2, lies off the places within the bombers' reach of the covering hex, and **has a lower number than P's base**, so that the order of the aims is shown to be rule 9's and not the order of the bases' numbers. A third, E3, is within 15 hexes of P and beyond that reach. The second place, Q, is a bare coast hex with no enemy base. A base of the side lies far off with no bomber. Two carrier forces the island may take; a third the commander has given an order of its own. Variants change one thing each.

The statements, to be written with the hand calculation, are to cover:

1. On the first morning the neutralises of P's base and of E2 are served, with no "cannot meet", and each reason names both forces, the hex, the distance, the reach, the aims before it and where each force lies.
2. E3's neutralise cannot be met, and its words name the base's line and each of the three carrier forces with its line of rule 20: two beyond reach from where they work, one with a task of another tree. The clause holds no full stop.
3. Before the take of P is released, with no ship in reach, the bombers of both forces raid E2 and not P's base. From the take's release they strike P's base under the take, as today.
4. After the take's plan has ended with P's base still the enemy's, they raid P's base under its neutralise, before E2. Once P is the side's, that neutralise is withdrawn and they raid E2.
5. A group with a fresh contact of ships in its reach is left for ships and raids no base that morning.
6. A group that does not reach the morning's base flies no raid, though it reaches another aim.
7. The take of Q, released, with no enemy base at Q, shuts out no aim: the next is struck.
8. A force gone home for fuel, beyond reach of every aim, flies no raid; the neutralise is still served from the task's hex, and its reason gives where the force lies, the distance and that no raid is flown from there.
9. A force held is let go as today and is named as not the island's.
10. A force whose only bomber group is held by the commander, or may be used only from a later day, does not serve: with no other force in reach the neutralise cannot be met, and the words name the group and which.
11. Where a base of the side has a bomber in reach, its raid task is formed as today and the carriers raid too.
12. The neutralise's measure counts the carriers' raids.
13. The island's denial with no carrier task under it and no base in reach names each carrier force and why.
14. The same game with the side's carriers taken out gives the words of today, to the letter.
15. **A commander's neutralise the island stood on.** With the commander's own neutralise of E2 in force before the island, the island forms none, the island's carriers serve the commander's, it is not said to be unmet, and they raid E2 under it. With a forces limit on it that names a task force other than theirs, they do not, and the words say so.
16. **Two islands.** With a second island ordered whose place is E2: it stands on the first island's neutralise of E2, the carriers under the first island's denial serve it, and nothing is formed twice. Ordered the other way round, the carriers stand under the second island's denial and serve the first island's neutralises that are also the second's.
17. A commander's own neutralise of a base far from any island, with a free force, is served by 0202's pass as today; scenario 116's tests do not move.

**Property tests beside it:** with the task's hex held fixed, a bomber group added aboard, of any reach, never turns a served neutralise into one that cannot be met (a longer reach in place of a shorter is not the test: the covering hex is set by the least reach aboard, and moves with it); a group with a ship in reach never raids a base; a force never raids two bases in a morning; the carrier tasks, their hexes and the island's tree are the same with this rule and without it; the hash of a game is the same with breakdowns on and off.

**The table:** `table/check_screen.py` gains checks, on the 39 days with the island of Guadalcanal ordered from the Allied seat, that the papers of the three neutralises read as rules 24 to 26 say, that each base carries its own fact, and that no paper names an air base while leaving the carrier forces out; and one, on a staff sentence fed to the page, that a base whose bombers fall short is not said to have none.

## Games on file

This changes what a staff decides, so games move. Read on Nov 24, 2026:

- **Scenario 76's carrier variant** (`tests/scenario_76_controlling_an_island.rs`, the doc comment's "A carrier" and `a_carrier`). Its statement today: group 3 "launches at base 2 on days 2, 3 and 4 and on no other day". By rule 11 nothing is flown at base 2 on day 1, before the take's release, so days 2, 3 and 4 stand, struck under the take. By rule 9 (b), on each later day that base 2 is still the enemy's and the take's plan has ended, group 3 raids it under the neutralise on the mornings the raid's rules send it. The statement is restated by the architect, in that file's doc comment, with the hand calculation and before the build: "it launches at base 2 on days 2, 3 and 4 under the take, on no day before, and after day 4 under the neutralise on the days worked below", with those days, or "on no day after" if base 2 is the side's or the game gives none. The notice that the neutralise of base 2 cannot be met, handed in on every day of the 14 today, is handed in on none. The test's other assertions (the task formed on day 1, no route on any day) are expected to stand.
- **Decision 0217's statement 8** says of that variant "Every order of every day is what it was." It was true of 0217's build. From this build it holds to day 4 and is restated so where the test carries it; the record of 0217 is kept as written.
- **The three arrangements of stage 3's proof** (`data/scenarios/stage-3-proof/`) and the table's copies (`table-hunger`, `table-boats`, `table-rear-bases`): ten files that hold a "control an island" and a carrier force. They hold islands 1 and 2 for both sides. Each side moves: the Allied carriers raid Tulagi from the first quiet morning and Lunga's base after the landing while it is the enemy's; the Japanese carriers, once under their island's denial and in reach, raid Lunga when it is Allied-held, after their own landing is launched, and Tulagi likewise. Raids, the losses and fatigue that follow, and fewer "cannot meet" notices.
- **Tests that build island orders and may pin a reason, a notice or an order:** `crates/sim-agent/tests/command_tools.rs`, `crates/sim-cli/tests/door.rs`, `crates/sim-cli/tests/pack_from_source.rs`, `crates/sim-data/tests/the_39_days_beliefs_and_sides.rs`, `tests/proof_stage_4_harness.rs`, `tests/scenario_77_forces_with_men_aboard.rs`, `tests/scenario_86_fed_with_the_lift.rs`, `tests/scenario_94_a_trial.rs`, `tests/scenario_132_a_source_the_staff_chooses.rs`, `tests/scenario_135_men_beside_a_place.rs`, the digest of scenario 130, and `table/check_screen.py`. Each is run before and after; one with no carrier force under an island's denial should not move.
- **Not expected to move:** every game with no island order, scenario 116 and every game of decisions 0202 and 0203 among them.
- **Not read:** the recorded games.

The build plays every scenario and recorded game on file before and after, and names each that moves to the designer before the change merges. Pins are remade; a statement that no longer holds goes to the architect and is not changed to make a test pass. The closed proofs are not rerun and their recorded games are not altered: a recorded game replays as it was saved.

## What it does not do

- **It does not move a carrier force to reach a neutralise's target.** The island's station is reckoned to its first place alone. A base beyond the bombers' reach from there, Shortland in the game of the finding, is not raided by the carriers, and the staff says so with the distance.
- **It does not weigh danger in the island's station,** and does not send the island's denies and neutralises through decision 0202's pass.
- **It does not spread a force's bombers over two bases in a morning,** or send one force at one base and a second at another.
- **It does not serve a neutralise that is not the island's** with the island's carriers: a commander's own neutralise of a base more than 15 hexes from every named place stays 0202's alone.
- **It does not mend a commander's neutralise whose forces limit says its forces work where they lie.** Decision 0202's pass passes it over, a carrier force named so raids nothing ashore, and its "cannot meet" names bases alone. That is a fault of the same kind in another rule, and is added to `docs/open-work.md`.
- **It does not count the bombardment forces.** A bombard task under an island's neutralise (0151, 8c) does not make that neutralise served, and the words of rules 19 and 20 do not name bombardment forces, as today.
- **It does not change which bases the island neutralises** (those within 15 hexes of a named place), the raid's rules, the bombard tasks under the island, or the engine.
- **It does not add a key to any tool's answer.** The table reads the staff's sentences, as it does now.
- **It does not weigh what a base holds.** A base with nothing on it is raided as an airfield is, as the bases raid it today.

## What it costs

- **The carriers spend aircraft ashore on every quiet morning** while an enemy base the island is to neutralise is in their reach, where today they wait for ships once the landing is in. The raid's rules by risk bound it (the rest level, the least weight, standing down after a costly raid), and ships still come first each morning; but a strike flown at a base in the morning is not on deck when a ship is found at noon. The commander's checks are the ones he has: a hold on the force, an order of its own for it, withdrawing the neutralise.
- **The carriers raid a base off the places before the landing sails,** from the morning the island's plan is released: Tulagi, in the game of the finding. The place itself is left until its landing is launched (rule 11). No rule of the game gives the enemy warning from a raid, and the station is not weighed for danger, so the staff counts no cost in it; the second question below asks whether the commander does.
- **Every force goes at the same base** each morning, the first of the aims, and a group that cannot reach it flies no raid. In the game of the finding both forces raid Tulagi until the landing at Lunga is launched, then strike Lunga until it is the side's, then Tulagi again. That is rule 12's price for one mechanism; the designer should see it in play.
- **A neutralise can be "served" on a day nothing is flown:** the force is away for fuel, ships are in reach, or the bombers rest. The reason says where the force lies and the table says the day's fact; no notice is given. A commander who wants the base struck every day has the bases, or an order of his own.
- **The closed proofs' arrangement plays differently from here on,** for both sides, so a figure of stage 3's proof is no longer what the same files give. The reports stand as records of their day.
- **A reason more each morning** for each neutralise a carrier serves, as each standing task gives one today.

## Questions for the designer

1. **Should the carriers of an island order weigh danger in where they stand, as the carriers of a plain "deny" do since decision 0202?** Today an island order sends them to the first hex within their bombers' reach of the place, whatever the risk level. Weighing it would mean that at preserve they do not go at all while the enemy holds a base there, so a landing would have no carrier cover unless the commander raises the risk or orders the force himself; at balanced and at press they would stand nearer the place than now. This plan leaves it as it is. A yes is a plan of its own, after this.
2. **Should an island's carriers raid the enemy's other bases near before the landing is launched?** This plan has them do so from the morning the order is approved (Tulagi, while the landing at Lunga is made ready), and leave the landing place itself alone until the landing is launched. The other way is to fly nothing ashore until the first landing is launched. This plan takes the first, since the order to neutralise is in force from that morning and the bases ashore already act on it. A no is one line of rule 9.

## Amended on review, Nov 24, 2026

Read by a reviewer who had not seen it written, against the code. Eighteen findings; all are taken, two of them in part with the reason given.

1. **Row 59.** Taken. It is given as the cause in Why, settled in What it changes, and removed in the build's pull request.
2. **Scenario 76's carrier variant, 0217's statement 8, and the tests that build island orders.** Taken. Listed in Games on file. The statement is restated there in its form, with days 2, 3 and 4 kept and the notice gone; the days after day 4 are worked in the hand calculation, which this record does not hold, and are written into the test's doc comment by the architect before the build. Rule 11 is what keeps day 1 as it was.
3. **The property test of a longer reach.** Taken. The hex is held fixed and the test adds a group; the reason is written beside it.
4. **Rule 15's holes** (now rule 20). Taken. Rule 2 says which bomber groups count; "no bomber group aboard" comes before "beyond reach"; a line is added for groups held or before their day, and one for a forces limit; the lines are put in an order in which each is reached only when those before it are false.
5. **One base or two in a morning.** Taken: one. Rule 12 says a group that cannot reach the morning's base flies no raid, and What it costs no longer says otherwise.
6. **A take whose place has no enemy base, and "the first take".** Taken. Rule 10 names both as new or as the code's, the words of rule 9 (a) follow the code, and statement 7 tests the first.
7. **The order of the neutralise aims.** Taken. Bases at named places come first, in place order (rule 9 (b)), then the others by number; the scenario numbers E2 below P's base on purpose.
8. **A commander's neutralise the island stood on.** Taken, by the first of the two ways offered: rule 3 makes it the island's, and the island's carriers serve it, inside its forces limit (rule 4). Statement 15.
9. **A force away for fuel.** Taken. Rule 19 gives where the force lies and says when no raid is flown from there; rule 24 gives the table's line; What it costs names it.
10. **The table's words explained a rule.** Taken. Rule 24 gives a fact of the order and a fact of the day.
11. **Each base's own fact.** Taken, in rule 25 and in the table's checks, and named in Why.
12. **Which hex the aims are reckoned from.** Taken. The area's reason reckons from the task's hex and gives where the force lies beside it, from the area's picture; the carrier doctrine reckons from where the force lies; one routine serves both (rules 14, 15 and 19).
13. **Rule 23's width** (now rule 28). Taken by narrowing: it binds island orders and what is the island's. The path of a commander's neutralise whose forces work where they lie is not closed here; it is named in What it does not do and goes to `docs/open-work.md`. Reason: such a force raids nothing ashore today, so mending its words means first deciding whether it should, which is 0202's and 0222's question and not this finding's.
14. **Missing statements.** Taken: statements 4, 7, 8, 10, 15 and 16, and the reversed numbers in the setup.
15. **Raids before the landing sails.** Taken in part. The place itself is not raided by carriers before its landing is launched (rule 11), which also keeps scenario 76's days. The bases off the places are raided from the island's release; that is named in What it costs and asked as the second question.
16. **Bombardment under an island's neutralise.** Taken: named out of scope.
17. **A full stop inside the carriers' clause.** Taken: rule 23, and statement 2 checks it.
18. **Both sides in Games on file.** Taken.
