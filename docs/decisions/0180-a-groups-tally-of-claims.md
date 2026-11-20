# 0180: Who made a claim, and an air group's tally of its claims

Status: a plan by the architect, written on Nov 20, 2026, at the designer's word ("it will be good to know which unit or air group did it ... keep record of that"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review") before it was built. Built on Nov 20, 2026, with decision 0179 in one build (As built, at the end).

Summary: A raid's claim is one number for a side: so many aircraft shot down, so many destroyed on the ground, so many hits. It now says which unit claims what, and each air group keeps a tally of what it has claimed since the game began. No draw is added and no claim's number changes: the claim is shared out among the units whose fire it rests on. What the tally does for a group's experience is left for the day air groups have one (decision 0064)

Standing: A plan, written by the architect on Nov 20, 2026, and amended on review. Built on Nov 20, 2026, with decision 0179. Its readings were ruled on by the architect on Nov 20, 2026

## Why

The designer, at the table, of "Pilots report 3 enemy aircraft shot down": it should say which unit did it, and the record should be kept, perhaps to count toward the group's experience.

What is so today, read from the code:

- **A raid makes one claim for each side that fired** (decision 0056): the raiders', and that of the patrols and guns under the raid. Each is what really happened plus false claims drawn for the whole shots that missed.
- **Aircraft shot down are already reckoned a firing at a time.** Each firing of fighters or guns is recorded with one unit, what it really shot down and its whole shots that missed, and the false claims are drawn for each firing in turn (`airstrike/claims.rs`, the loop over `air_misses`). Only the sum is kept.
- **A firing may be of several groups together.** The escorts of every escort group of a raid fire as one firing, recorded under the first of them; every group on patrol over the place fires as one, under the first; the guns of a force attacked and of its escorts in station fire as one, under the force attacked (`airstrike/work_out.rs`). Each aircraft is one shot, and each shot is one draw, in order.
- **Aircraft on the ground and hits on ships are reckoned for the raid:** the bombs of all its groups together, one drawing of false claims over all that missed.
- **What a raid's bombs really destroyed on the ground** is added up by the bombing group as it lands, and then summed.
- **The claim's event cites the firing events it rests on,** so the table can already ask who fired; it cannot say who claimed how many.
- **An air group has no experience that works** (decision 0064: it waits on research).

## The rules

1. **A raid's claim says who claims what.** It gains a list: for each unit whose fire the claim rests on, the aircraft it claims shot down, the aircraft it claims destroyed on the ground and the hits it claims on ships. A unit that claims nothing is left out. A unit is an air group, or, for guns, the task force or the base the firing is recorded under. The list is in the order in which the units first fired, a firing of several groups giving them in the order of their numbers.
2. **The sums are the claim's.** The aircraft shot down in the list add to the claim's, the aircraft on the ground to the claim's, the hits to the claim's. Ships claimed sunk are the claim's alone and are not shared out (later).
3. **A firing of several groups is parted among them, shot by shot.** Its whole shots are shared among the groups that fire in it in proportion to the aircraft each has in the firing: each group is given the whole part of the shots times its share, what is left going one at a time to the groups with the largest part cut off, a tie to the lower number. The shots are then taken in the order they are drawn, the group with the lowest number having the first of them, and each group has the hits of its own shots. Where the firing really shot down fewer than it hit, the cut comes off the last shots first. The draws are the draws of today, each in its place.
4. **Aircraft shot down, by unit:** for each firing, and for each group in it, what its shots really shot down and the false claims drawn for its shots that missed. The false claims of a firing are drawn group by group in the order of rule 3: the same draws, in the same order, as the one drawing of today, so that the firing's false claims and every later draw are what they were. Guns are not parted: a firing of guns is its recorded unit's.
5. **Aircraft on the ground and hits on ships, by group:** what a group's own bombs or torpedoes really did is its own, and the false claims are drawn for its own bombs or torpedoes that did not count (for a base, its bombs less the aircraft they destroyed; for ships, those that missed), group by group in the order the groups attacked: again the same draws in the same order as the one drawing of today, and the same sum.
6. **An air group keeps a tally:** the aircraft it has claimed shot down, the aircraft it has claimed destroyed on the ground and the hits it has claimed on ships, each the sum of its lines in every claim since the game began. It is the group's own state, one thing of three numbers. It is kept when the group moves base, is rested, loses every aircraft or is lost: it is a record. A group made for a trial's enemy starts at nothing.
7. **Guns keep no tally.** A task force's or a base's guns are named in the claim's list and nowhere else.
8. **A surface fight's claim and a bombardment's claim name no unit** by this record (later).
9. **Who sees it.** The list is part of the claim: whoever sees the claim sees it (decisions 0056 and 0135), and the other side never. The tally is the group's own: its side knows it as it knows the group, and where command levels are on a superior knows it by the group's state reports, as he knows its aircraft (decision 0136). So a superior may read in a group's tally a claim that was itself lost on its way to him: that is accepted, a group's score being its own return. No tool tells a side the enemy's tallies.
10. **The claim's words** are what they are today, with decision 0179's types where that record puts them, and then one clause at the end that names the units: "; claimed by U: 2 shot down; by V: 16 on the ground, 3 hits". Only what a unit claims is said. The event's own text names a unit as events do; the seat's tools name it as they name a unit, and one that is no longer in the game by its number, "since dissolved". Nothing earlier in the words moves.
11. **The tally changes with the claim,** and the claim's event is the event of the change: it holds the list, and cites the firings. The lines have no breakdown of their own: each is what the cited firings record as really done, and the false claims the claim's breakdown already counts, taken a group at a time.
12. **A picture carries the list and the tallies** (decisions 0170 and 0171). A world is not built from a picture whose claim names a unit that is not of the claim's side in that picture.

## What a side learns by it

Nothing of the enemy that it did not have. The list is the side's own claim, cut finer; every number in it may be above the truth, as the claim's is, and a unit's line is never below what that unit really did. A side still cannot tell a real aircraft from a false one.

## What it leaves for later

- **Experience.** A tally is a record and nothing reads it. When air groups have an experience that works, what feeds it is to be decided then; the architect's view is that it should rest on days in action or sorties flown, which are true, and not on claims, which reward the groups that claim most wildly.
- **Ships claimed sunk, by group,** and a tally of them.
- **A surface fight's and a bombardment's claims by ship,** and a ship's tally.
- **The real score beside the claimed one,** for the umpire's replay.

## Rule card

- **Reads:** a raid's firings, each with its unit, what it really did and what missed; each bombing group's bombs or torpedoes and what they really did; the false claims as drawn.
- **Writes:** the list on each raid's claim and on the event of its arrival; three numbers on each air group.
- **Reuses:** the claim and its draws, each in its place; the passing of a claim and of a group's state between commands.
- **New:** one field of a claim, one of an air group.

## What it costs

- **State:** one field of a claim and one of an air group, so the hash version rises and the golden files are remade (decision 0057); both are named in `hash::name_every_field`. A picture carries them (decision 0170), a side's own half its own groups' (decision 0171), and a state report a group's (decision 0136), which holds the group whole.
- **No draw and no number of any claim changes,** on any figures: every recorded game plays as it did but for the hash version and the words of a raid's claim.
- **In the code:** a roll of shots gives which shots hit, not only how many; a raid's tally keeps, for each side, what each unit really shot down and its false claims, and for each strike group its bombs or torpedoes, what they did and what missed. Every place that builds an air group, a claim, a picture's claim or the claim's event changes with the fields: `sim-core`, `sim-data`, `sim-staff`, `sim-agent`, `sim-trial` (the groups it makes for an enemy), the replay export and the batch record of `sim-cli`, and the tests that build them by hand.
- **The agent interface:** a claim gains the list; an air group, in the seat's listings and its card, gains its tally; the schemas and their golden answers change and the version rises.
- **The table:** a claim's line names who claims what, and an air group's card shows its claims to date.
- **It is built with decision 0179,** which changes the same claim: one build, one rise of the hash version and of the interface. Neither changes a number of the other's hand calculation; scenario 98's statements that the words are scenario 11's "to the letter" are amended to except this record's clause.
- **Documents:** the rulebook's chapter 3 (claims), chapter 5 (an air group), chapter 1's table of who sees what, chapter 8's versions and chapter 9 (a trial's enemy); the glossary.

## The hand calculation

It is in `tests/scenario_99_a_groups_tally_of_claims.rs`, written before the code: the base raid and the unopposed strike of scenario 11's hand calculation, with the false claim chances at 0 and at 1, a unit at a time; and the parting of a firing of two groups, shot by shot.

## Amended on review

An independent reading of the plan and its hand calculation, against the code, found the following; the first was checked in the source before it was taken.

1. **A firing is often of several groups, recorded under the first.** As written, the first escort group or the first group on patrol would have been credited with the whole firing and the others with nothing. A firing is now parted among its groups shot by shot, with no draw moved (rules 3 and 4), and the hand calculation has the parting of a firing of two groups.
2. **Sharing the false claims of bombs by proportion was not needed.** They are drawn a bomb at a time, so they are now each the group's whose bomb it was (rule 5): no rounding, and nothing told of the true split. The sharing table is gone.
3. **The words were not placed,** and placed in the middle they would have broken statements of decision 0179's hand calculation and of scenario 11's test. They are one clause at the end (rule 10), and scenario 98's two statements are amended.
4. **No breakdown was named.** Rule 11 says why the lines need none.
5. **A claim that names units is new in a picture.** Rule 12, the trial's enemy groups in rule 6, and a unit no longer in the game in rule 10.
6. **The order of a list was left to the code.** It is given in rule 1 and in the hand calculation's tables.
7. **A superior may read a lost claim in a tally.** Accepted, and said (rule 9).
8. **The tally is one field,** not three, and "What it costs" names the places that change.

## As built

Built on Nov 20, 2026, with decision 0179, on the branch `claims-types-and-tally` (decision 0179's As built says from what). One rise of the state layout (49) and of the agent interface (30). No number or statement of the hand calculation was changed. Every statement of `tests/scenario_99_a_groups_tally_of_claims.rs` holds as written. The last cell of the fifth row of the parting table first said 4 and 2, which the rule does not give (reading 1); the architect corrected it to 4 and 1, and it is asserted.

**What was built.**

- **`sim-core`:** `claims::UnitClaim` (a unit, aircraft shot down, on the ground, hits) and `BattleClaim::by_unit`, the same on the event; `state::ClaimTally` and `AirGroup::claims`, added at the end; `claims::part_firing` (rule 3) and `claims::strike_lines` (rule 5), the rules' own functions. A roll of shots now gives which shots hit (`roll_hits`, the same draws as `roll_shots`); a roll of fighters carries the groups in the firing with their aircraft; a raid's tally keeps each firing's parts, each strike group's bombs or torpedoes and what they did, and the units that claim in the order they first fired, apart from the commands that hold the claim first (decision 0135, 1), which are unchanged. The false claims are drawn unit by unit and group by group on the claim's stream, the same draws in the same order as the one drawing before. Each group's tally takes its line as the claim is recorded. The words (one clause at the end), the hash (the claim's list after its types; the group's tally after `available_from`), `name_every_field`, `side_events.rs` (the event's fields named and decided: whole to its own side), a picture's claim and its check (rule 12), the surface fight's and the bombardment's claims (empty).
- **`sim-data`, `sim-trial`:** a group starts at nothing, a trial's enemy's too.
- **`sim-agent`:** a briefing's claim gains `claimed_by` (each unit's handle, `since_dissolved` where set, and its three numbers), left out where empty, and the clause in its words; an air group's line (`list_forces`, the briefing, `get_entity`) gains `claimed_to_date`, left out while nothing is claimed, and the text ", claimed to date: 7 shot down"; the seat's `events` and `explain` say a claim with the types by name and the units as the seat names them. Version 30, every schema and the schemas' README.
- **`sim-cli`:** the replay export adds `claimed_by` (`unit`, `shot_down`, `on_ground`, `hits`) to `claim_received` where it is not empty. The batch record is unchanged.
- **Tests:** `tests/scenario_99_a_groups_tally_of_claims.rs`, 14 tests; the tests that build a claim, an event of one or a group by hand gain the fields.
- **Documents:** the rulebook's chapters 1, 3, 5, 8 and 9, the glossary, `docs/commands.md`, and the recorded games' README.

**Readings** (decision 0179's are numbered 1 to 6 in its own As built; these are this record's).

1. **The fifth row of the parting table** (the architect ruled the build right, and corrected the table's last cell to 4 and 1, now asserted). Group 3 has shots 1 to 4 (the 1st hits), group 7 shots 5 and 6 (both hit); the firing really shot down 2, so the cut comes off the 6th shot, and group 7 has 1 real and no shot that missed. By the rule, and as the code has claimed since decision 0056, a hit that is cut is neither a real aircraft nor a miss and is never claimed: at chance 1 group 7 claims 1, and the firing 5 (2 real and 3 missed). The table's 4 and 2 add to 6, which is not the firing's own; "no claim's number changes" and "in every row ... the claims add to the firing's own" both give 4 and 1. Built as 4 and 1; the test asserts the shots and the real aircraft of the row and that the claims add to the firing's own, and not the table's last column. For the architect.
2. **A firing is one recorded firing event.** The patrol's shots at the bombers before the attack and after it (decision 0087) are two events and are parted each on its own, its first shot to the lowest group.
3. **The aircraft in a firing:** for the escorts' fire, each escort group's escorts with the raid; for the patrol's, each group's whole patrol over the hex (not the raid's share of it, which is not by group). The shares are in proportion either way.
4. **The units that claim are kept apart from the holders.** The commands that hold a claim first (decision 0135, 1) are still the first patrol group's, as before, so that with command levels on no report moves; the list names every group of the firing.
5. **Rule 12** (the architect ruled this reading to stand) is read for each kind of unit: an air group or a task force named must be in the picture and of the claim's side (neither changes side, and neither leaves the state); a base must be in the picture, of either side, since a base may change hands after its guns claimed; any other kind of unit is refused. A unit the picture does not hold is refused.
6. **"Since dissolved"** is said of a unit the seat's view no longer gives: an own air group or task force not in it, or a base not on the side's map. Groups and forces never leave the state today, so it is not met in any game played.
7. **The event's units, on the architect's word** (his ruling on the build's report, Nov 20, 2026): a claim's event names as its entities the units of its list, in the list's order, and nothing else, so `sim-cli events --unit` and a seat's `events` give a claim under each group, force or base it names. Only the side that owns the claim is given the event, as before, so the enemy never sees them. Entities are not hashed: what moved is the event listings (the export's `entities` of a raid's claim, a claim under a unit in `events --unit`, the `entities` of a claim in a seat's `explain` and `events`), not a daily hash. As first built the event named no entity.
8. **"1 hit"** in the clause, as the rest of the claim says it.
9. **"A tally is kept"** is tested by a group rebased on day 3 to a base added in the variant (it keeps 12, what it claimed on days 1 and 2), and by a group whose every aircraft is taken away in a picture after day 2 and that then plays day 3 (it keeps 0, 31, 0).
10. **With command levels on** the property is tested on the base raid with command levels on and scenario 63's report figures: every command that holds group 3 holds its tally as of the moment it is held at.
11. **"No answer of a seat's tools holds a tally of an enemy group"** is tested on the Allied briefing after the three days: the view gives only own groups, and neither Japanese tally is in it.

**Figures the hand calculation asks the build to report.** The base raid's first morning with a second Allied group (group 6, 6 fighters, on patrol over base 2), the Allied claim's list: at chances 0, group 3: 3 shot down, group 6: 1; at chances 1, group 3: 12, group 6: 6, base 2: 20; at the placeholder chances, group 3: 4, group 6: 2, base 2: 1. Days 2 and 3: at 0, 1 and 2, then 3, 1 and base 2 3; at 1, 9, 6, 20, then 7, 6, 20; at the placeholder chances 1, 2, 1, then 3, 1, 4. In each the patrol groups' lines add to the claim less the guns', and neither group is credited with more shots than it had on patrol.

**What the table will need.** A claim's line can say who claims what from `claimed_by` (the briefing's, or the export's), each unit as the seat names it; an air group's card can show `claimed_to_date`. A claim of a side's own never names an enemy unit, and no answer carries an enemy group's tally.
