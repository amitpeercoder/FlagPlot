# 0190: What to strike first

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word ("Plan the rules first, then play the games"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A staff's aircraft take the most important kind of ship first: a carrier before a cruiser before a transport. A commander whose task is to stop a landing has no way to tell his staff that the transports come first, and an order of his own to a group takes the group out of the staff's hands. This adds one limit a commander may set on an intent, the kinds of ship to strike in his order, which the staff gives its groups in place of the usual order and which a carrier force reads when it chooses what to close on.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

## Why

Scenario 105, the Coral Sea (decision 0188). In 1,000 games of staff against staff the Allied carriers sank Shoho in 912 and a transport in 12, and the convoy landed whole in 978. The carriers stood within reach of the convoy's route. Their strikes went at the Main Force, the nearer contact and the more important kind, every day.

What is so today, from the rulebook:

- **A group's target list** is a standing order: the kinds it will launch at, in order. Its default is all six kinds in the order of importance: carrier, battleship, cruiser, destroyer, transport, auxiliary (chapter 5).
- **A group picks its contact** by the kind first on its list, then the nearest, then the lowest number; and in the force it attacks it picks the ship that ranks first by its list (chapter 5).
- **A staff sets the list** of a bomber it orders to strike ships from what the risk gives: carrier, battleship and cruiser at preserve the force; every kind, in the default order, at balanced and at press (chapter 8, Striking ships from a base).
- **A carrier force closes on a contact** of a kind its risk names: nothing at preserve the force, a carrier at balanced, any kind at press (chapter 8, A contact to close on).
- **A commander's order to an air group an intent may use holds the group:** the staff gives it nothing more (chapter 8, the override). The architect's order to six bomber groups in scenario 105 grounded them for five days.

## The rules

1. **A new limit, the targets limit,** set by `set_limit` with `limit = "targets"`, the intent, and `kinds`: one to six kinds of ship, each once, in the order they are to be struck. It is cleared by `set_limit` with no kinds, as the limits that wait are.
2. **It is read up the tree, the nearest set on the chain,** as the rank is. With none on the chain nothing changes: every list and every choice is what it is today.
3. **Where it binds a task that strikes ships** (a task to strike ships from a base, and the strikes of "operate a carrier force"), the staff sets each bomber's target list to the limit's kinds in the limit's order, at every risk, in place of the list the risk gives. A kind the limit leaves out is not launched at, as with any short list.
4. **Where it binds a carrier force's task,** the kinds the force closes on are read from it: at balanced, the first kind of the limit; at press, every kind of the limit; at preserve the force, nothing, as today. Among contacts of kinds closed on, the first is taken by the limit's order, then the nearest, then the lowest number. With no limit this is today's rule: the default list's first kind is the carrier.
5. **It does not hold a group.** It is a limit on an intent and not an order to a unit. The staff goes on ordering the groups it binds.
6. **A reason says so.** The reason of an order that sets a target list from the limit names the limit and the intent it is set on, and so does the reason of a force that closes, or does not, on a contact.
7. **Who may set it:** the commander, on any intent of his in force, a staff-made one too, as any limit. Set on an intent whose plan is approved it is an amendment of the plan, as any limit is.
8. **What it does not do.** It does not change what a group does when ordered directly, the default list, a raid on a base, the sizing of a strike, or what a surface force engages.

## Rule card

- **Reads:** the targets limit on the chain of a strike-ships task or a carrier task; the contacts the side holds and their reported kinds.
- **Writes:** the limit, in the side's command state; through the staff's existing orders, a group's target list and a carrier force's route.
- **Reuses:** the target list and the way a group picks a contact and a ship; the staff's order that sets a list; the rule by which a carrier force closes; limits read up the tree.
- **New:** one limit in the command state. No event of a new kind beyond the limit being set, which is recorded as the others are; no draw.

## What it costs

- **State:** the command state gains a field, so the state hash's version rises and the golden files and recorded games are remade (decision 0057).
- **The agent interface:** `set_limit` takes the new limit; `command_state` shows it; the version rises.
- **Rulebook:** chapter 8 (the limits, striking ships from a base, a carrier force's strikes and what it closes on) and the glossary.
- **Every game with no targets limit plays as before.** The proofs and the recorded scenarios set none.

## Scenario 105

9. **The Allied side's orders gain** a targets limit on its carrier force's intent: transport, carrier, battleship, cruiser, destroyer, auxiliary. The mission is to stop the landing. This is the order the architect tried to give by hand; the hand calculation of scenario 105 is amended by him, and the run of 1,000 games is made again and reported beside the first.
10. **The Japanese side's orders gain none.**

## Questions for the designer

None. The rule is a general one; its first use is the one above.

## Amended on review

A reviewer who had not seen the plan written read it against the rules and the code and played scenario 105 on forty seeds. The plan as written would not have done what it was for. Found right: that only the staff's strike step sets a list, for a strike-ships task and for a carrier force alike; that a group picks its contact by its list's order, then the nearest, then the number, on the contact's reported kind; that a balanced carrier force closes on a carrier; that an order to a group holds it; that a limit read nearest-first fits how the rank and the knowledge limits are read.

### Why a list of kinds is not enough

11. **A convoy is not reported as transports.** A report's kind is the most important kind in the force, passed through the table of mistakes. The Coral Sea's convoy has a cruiser with it, so it is reported as a cruiser seven times in ten and never as a transport; once the cruiser is sunk, as a destroyer. In forty games only ten gave the Allied side even one daylight report of it as transports. A list with transports first leaves the Main Force, reported as a carrier, ahead of a convoy reported as a cruiser.
12. **What tells the convoy apart is where it is:** on the way to the place. A list of kinds cannot say that. So the plan gains a second part, and keeps the first for what it does do: inside the force attacked, the aircraft take the ship that ranks first on their list, and a list with transports first has them attack the transports and not the escort.

### The second part: the waters come first

13. **A group gains a setting, the waters to strike first:** a set of hexes, empty by default. A contact whose estimated hex is in them ranks before every contact outside them, whatever its kind; within each of the two the order is today's (the list's order, the nearest, the lowest number). With the setting empty nothing changes. It is the engine's rule of which contact a group launches at (chapter 5), and a group may be given it by an order as it is given its list.
14. **The staff sets it from a deny.** For each "deny an area" in force whose plan is released, the bombers the staff orders under it have the deny's waters as their waters to strike first: the bombers of the task to strike ships that it forms under the deny, and the bombers of a carrier force that the deny's forces limit names, under whatever task that force has. Where two denies bind a group, the waters are both. A group no deny binds has none.
15. **A carrier force a deny names closes on what is in its waters.** At balanced, beside a carrier, the force closes on a fresh contact of any kind whose estimated hex is in the waters of a deny that names it; such a contact is taken before one outside the waters. At press every kind is closed on already, and a contact in the waters is taken first. At preserve the force it closes on nothing, as today. A bold or a cautious commander's reading of his own contacts (chapter 8, Initiative) moves the level as it does today, and the waters are read at that level.
16. **Strike sizing is as it is.** A strike sized to a contact reported as a cruiser of 15 ships sends what the sizing figures give for it, and the rest go on to the next contact in the order. That split is intended and is not changed here.
17. **A strike on the place** (chapter 8, the carrier doctrine's raid on a place) reads only contacts a group would launch at: with a short targets limit, a contact of a kind left off the list does not keep a group from the raid.

### The first part, corrected

18. **Rule 4 is replaced by rule 15.** The targets limit no longer decides what a force closes on. It sets the bombers' lists (rule 3) and nothing else.
19. **A child's targets limit is never refused as wider than its parent's:** the nearest on the chain replaces, as the rank does.
20. **Why a limit and not a hold of one setting.** The rulebook has a hold of the search arc alone, and a hold of the target list alone would let a commander's order to a group stand without grounding it. It is not taken: it is given group by group, and it would freeze a list the staff then could not set from a later limit. The limit is one statement on an intent, read by every group the intent commands. Two mechanisms then set a group's list, the commander's order and the staff's from a limit, as two set its mission today; the override decides between them as it does now.
21. **The rulebook's list of what amends a plan** names five kinds of limit, where the code takes every limit on an intent as an amendment. It is corrected in the same change.

### The worked example and the costs

22. **Statement 4's reason is the engine's,** not the staff's: the launch is decided pulse by pulse, and the test reads the strike's own record of why nothing was launched. Statement 10 drops "more than six", which cannot be once a kind named twice is refused, and "a word that is no kind", which the interface refuses before the game sees it. Statement 5 names its deny and its base. Statements are added for the second part, and three properties: a limit of all six kinds in the default order gives today's orders at balanced and at press; moving a kind earlier never lowers the aircraft sent at contacts of that kind; a contact moved into the waters never ranks lower than before.
23. **Costs not said:** the refusals at intake; the limit's code in the state hash and the new setting of a group there; how a saved game writes both; the scenario file's command and order; `docs/commands.md`; the rules label, so that a game with the limit replays; the table's command sheet; the trial, which plays staffs forward.

### Scenario 105, as it now stands

24. **The Allied side is given** a deny of the waters within 2 hexes of (-13,-2), where the convoy rounds the tip, its forces task force 1, and on it the targets limit transport, carrier, battleship, cruiser, destroyer, auxiliary. The carriers keep their station by their own intent. The hand calculation is amended by the architect before the build.
25. **Run in four cells** with decision 0191: neither rule, each alone, and both, so that what each does is seen apart.
26. **What the reviewer expects, recorded before any run:** this rule turns more strikes onto the convoy on the mornings it is in the waters and within reach, which in his forty games was one pulse or two. He does not expect it alone to make a contest.

## Amended on a second review

A second reviewer read the rework against the code and the same forty games. Found right: that the setting belongs on the group; that a contact's estimated hex and age make a clean test; that escorts, the initiative rule and "it stays while the first is in reach" fit it. What he found wrong changes the plan again, and this section replaces what is above it where they differ.

27. **The waters come first only where the commander says so.** As written the staff set the waters from every deny, and every game with a deny would have changed: the 39 days, the five-day scenario, the proofs. The targets limit gains a second part, `waters_first`, true or false, false where left out. It may be true only on a "deny an area" or on an intent beneath one. The staff sets a group's waters to strike first only from a deny whose chain carries a targets limit with `waters_first` true. With no such limit no game changes: no order, no ranking, no search arc.
28. **A deny that names a carrier force reaches its bombers.** That force serves a task of its own, which the deny is not above. Where the deny's forces limit names the force and the deny carries a targets limit, the force's bombers take from it both the list and, where `waters_first` is true, the waters; a targets limit on the force's own chain is read first for the list. The staff finds these denies by a reading it does not have today, "the denies in force whose nearest forces limit names this force", which the strike step, the carrier's closing and the carrier's search all use.
29. **The carrier's search covers those waters:** they are added to its arc's threats, as a cover's arc takes a deny's waters today, and only under rule 27's switch.
30. **A group launches once in a pulse.** Rule 16 was wrong: aircraft over what a sized strike needs do not go on to another contact; they stay. Other groups of the force take the next contact in their own order.
31. **Only contacts a group could launch at are ranked:** fresh enough, of a kind on its list and in its reach. A contact in the waters and out of reach does not keep a group from one outside them in reach.
32. **Closing is different, and is meant.** A balanced carrier force with an enemy carrier in reach outside the waters and a contact in the waters beyond reach leaves to close on the one in the waters. That is what the order says: these waters first.
33. **Who sets and who clears.** Each morning, for every bomber the staff may order, it works the waters over all the denies that bind the group under rule 27 (their hexes together), and gives the order where they differ from what the group has, the empty set among them, so a deny withdrawn clears its waters the next morning. The order serves the lowest-numbered such deny, or, clearing, the task the group serves. A group the staff may no longer order keeps what it has, as a target list does today; that is a known limit.
34. **The rule card, restated.** Reads: the targets limits on the chains above a strike task and on the denies that name a carrier force; the contacts the side holds. Writes: the limit, in the command state; a group's waters to strike first, in the group's state; through existing orders, a group's list and a carrier force's route and search arc. New: one limit with two parts; one setting of an air group and the order that sets it; one rule of the engine's choice of contact (chapter 5). No draw.
35. **Costs, beside those of 23:** the new order to a group, with its refusals (a hex off the map, a hex twice), in the scenario file, the agent's orders and schemas, the tool server, the replay export and the strike's own record, which cites the order as it cites the list's.
36. **Statement 12 is tested on the staffs' orders and the event logs,** not on daily hashes, which all move with the new fields.
37. **Scenario 105:** the Allied deny is of the waters within 1 hex of (-13,-2), which holds (-12,-3) and (-14,-2) and leaves the Main Force's station outside; its forces are task force 1; its targets limit is transport, carrier, battleship, cruiser, destroyer, auxiliary with `waters_first` true. The Japanese deny carries no targets limit and its aircraft do as they did.
38. **What the forty games say, before any run:** a contact of the convoy in those waters, fresh and within 180 nm of the carriers, is held in 26 games of 40, always on day 2, for one pulse or two. In 25 of those 38 pulses some or all of today's strikes went elsewhere. That is the whole of what this rule can turn, and many of those contacts come from the support group's night fight.
