# 0180: Who made a claim, and an air group's tally of its claims

Status: a plan by the architect, written on Nov 20, 2026, at the designer's word ("it will be good to know which unit or air group did it ... keep record of that"). To be read by a reviewer who has not seen it written, and amended, before it is built.

Summary: A raid's claim is one number for a side: so many aircraft shot down, so many destroyed on the ground, so many hits. It now says which unit claims what, and each air group keeps a tally of what it has claimed since the game began. No draw is added and no claim's number changes: the claim is shared out among the units whose fire it rests on. What the tally does for a group's experience is left for the day air groups have one (decision 0064)

Standing: A plan, written by the architect on Nov 20, 2026. Under review

## Why

The designer, at the table, of "Pilots report 3 enemy aircraft shot down": it should say which unit did it, and the record should be kept, perhaps to count toward the group's experience.

What is so today, read from the code:

- **A raid makes one claim for each side that fired** (decision 0056): the raiders', and that of the patrols and guns under the raid. Each is what really happened plus false claims drawn for the whole shots that missed.
- **Aircraft shot down are already reckoned a firing at a time.** Each firing of fighters or guns is recorded with its unit, what it really shot down and its whole shots that missed, and the false claims are drawn for each firing in turn (`airstrike/claims.rs`, the loop over `air_misses`). Only the sum is kept.
- **Aircraft on the ground and hits on ships are reckoned for the raid:** the bombs of all its groups together, one drawing of false claims over all that missed.
- **What a raid's bombs really destroyed on the ground** is added up by the bombing group as it lands, and then summed.
- **The claim's event cites the firing events it rests on,** so the table can already ask who fired; it cannot say who claimed how many.
- **An air group has no experience that works** (decision 0064: it waits on research).

## The rules

1. **A raid's claim says who claims what.** It gains a list: for each unit whose fire the claim rests on, in the order of its first firing, the aircraft it claims shot down, the aircraft it claims destroyed on the ground and the hits it claims on ships. A unit that claims nothing is left out. A unit is an air group, or, for guns, the task force or the base that fired them.
2. **The sums are the claim's.** The aircraft shot down in the list add to the claim's, the aircraft on the ground to the claim's, the hits to the claim's. Ships claimed sunk are the claim's alone and are not shared out (later).
3. **Aircraft shot down, by unit:** for each firing, what it really shot down and the false claims drawn for it, as they are drawn today, credited to the unit that fired. No draw is added or moved.
4. **Aircraft on the ground and hits on ships, by group:** what a group's own bombs or torpedoes really did is its own. The false claims of the raid, drawn once as today, are shared among the raid's groups that dropped anything in proportion to each group's whole bombs or torpedoes that did not count (for a base, its bombs less the aircraft they destroyed; for ships, those that missed): each group is given the whole part of the false number times its share; what is left is given one at a time to the groups with the largest part cut off, a tie going to the group with the lower number. This is the sharing of decision 0179, rule 3.
5. **An air group keeps a tally:** the aircraft it has claimed shot down, the aircraft it has claimed destroyed on the ground and the hits it has claimed on ships, each the sum of its lines in every claim since the game began. It is the group's own state. It is kept when the group moves base, is rested, loses every aircraft or is lost: it is a record.
6. **Guns keep no tally.** A task force's or a base's guns are named in the claim's list and nowhere else.
7. **A surface fight's claim and a bombardment's claim name no unit** by this record (later).
8. **Who sees it.** The list is part of the claim: whoever sees the claim sees it (decisions 0056 and 0135), and the other side never. The tally is the group's own: its side knows it as it knows the group, and where command levels are on a superior knows it by the group's state reports, as he knows its aircraft (decision 0136). No tool tells a side the enemy's tallies.
9. **The claim's words** name the units after the numbers, as the seat's tools name a unit: "3 aircraft shot down (2 by air group A, 1 by the guns of base B)". Where one unit claims everything it is named alone.
10. **The tally changes with the claim,** and the claim's event is the event of the change: it holds the list, and cites the firings.

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
- **Reuses:** the claim and its draws, untouched; the sharing by proportion of decision 0179; the passing of a claim and of a group's state between commands.
- **New:** one field of a claim, three of an air group.

## What it costs

- **State:** one field of a claim and three of an air group, so the hash version rises and the golden files are remade (decision 0057); all are named in `hash::name_every_field`. A picture carries them (decision 0170), a side's own half its own groups' (decision 0171), and a state report a group's (decision 0136).
- **No draw and no number of any claim changes,** on any figures: every recorded game plays as it did but for the hash version and the words of a raid's claim.
- **In the code:** a raid's tally keeps, for each side, what each unit really shot down and its false claims, and for each strike group its bombs or torpedoes, what they did and what missed.
- **The agent interface:** a claim gains the list; an air group, in the seat's listings and its card, gains its tally; the version rises.
- **The table:** a claim's line names who claims what, and an air group's card shows its claims to date.
- **It is built with decision 0179,** which changes the same claim: one build, one rise of the hash version and of the interface.
- **Documents:** the rulebook's chapter 3 (claims) and chapter 5 (an air group), chapter 1's table of who sees what; the glossary.

## The hand calculation

It is in `tests/scenario_99_a_groups_tally_of_claims.rs`, written before the code: the base raid and the unopposed strike of scenario 11's hand calculation, with the false claim chances at 0 and at 1, a unit at a time; and the sharing's own function.
