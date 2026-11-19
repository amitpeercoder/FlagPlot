# 0179: The type of the aircraft claimed destroyed on the ground

Status: a plan by the architect, written on Nov 19, 2026, at the designer's word ("if the type is known, it should say which type"). To be read by a reviewer who has not seen it written, and amended, before it is built.

Summary: A raid on a base claims a number of aircraft destroyed on the ground and says nothing of what they were. The raiders now make out the type of each aircraft they claim, or do not, by a chance. A type they name is a type that was on the field; the number against it may be more than the truth, as the claim's number already may. A bombardment from the sea makes out none

Standing: A plan, written by the architect on Nov 19, 2026. Under review

## Why

The designer, at the table, of the line "1 aircraft destroyed on the ground": it should say the aircraft is the enemy's and, where the type is known, which type. The table can say the first. The second is not in the game: a claim holds counts and the kind of ship attacked (decision 0056), and no type of aircraft.

What is so today, read from the code:

- **A raid's claim of aircraft on the ground** is the aircraft its bombs really destroyed plus a draw for every other bomb, at `bomb_false_claim_chance` (decision 0056, 7; `airstrike/claims.rs`).
- **The aircraft on the ground at a base** are counted once, at the start of the air combat step: for each group at the base in order of number, its damaged aircraft and its ready aircraft not flying in the pulse (`airstrike/ground.rs`). Hits come off that count, the lowest group first.
- **A type of aircraft is already made out or not by a draw:** the people under a raid recognise the raiders' type at `raid_recognised_chance` (decision 0167), on a stream of their own.
- **A bombardment's claim** is of aircraft on the ground and nothing else (decision 0092, 17).
- **A claim is held by the side,** passed between commands whole (decision 0135), kept in a picture (decision 0170) and hashed.

## The rules

1. **A claim of aircraft destroyed on the ground may name types.** It gains a list of pairs, a type of aircraft and a number, in order of the type's number, each number above 0. The numbers add to no more than the aircraft claimed on the ground; what is left over is aircraft whose type was not made out.
2. **What each claimed aircraft is, before any is made out:**
   - an aircraft the raid's bombs really destroyed is of its own group's type;
   - an aircraft claimed falsely is of the type most numerous on the field as it was counted for the pulse, before any raid of the pulse (the count of the second bullet above, added up by type); where two types are equally numerous, the one with the lower number;
   - where the field was counted empty, a false claim has no type and is never made out.
3. **Each such aircraft is made out or not,** by a chance, `claim_type_made_out_chance` of the air combat figures: for each type in order of its number, the number made out is drawn as shots are (`roll_shots`: that many whole draws at the chance). Those made out are named in the list under their type; the rest are not named.
4. **The draws are the claim's own:** a stream keyed as the claim's stream is (the raid's first group and the base), with a draw value of its own, `Draw::ClaimType`, added at the end. No other draw moves: a fight, a claim's numbers and every other claim are what they were at any setting of the chance.
5. **A bombardment's claim names no type.** Ships firing from the sea make out none.
6. **A claim of aircraft shot down names no type** by this record (later).
7. **The claim's words** give the types after the number: "5 aircraft destroyed on the ground (3 of type T, 2 of a type not made out)", with a type named as decision 0167 names one. Where no type is made out the words are what they were.
8. **Who sees it:** whoever sees the claim (decision 0056, 0135), and no one else. The other side is told nothing of it, as now.
9. **The figure** is a placeholder: 0.75 in the live figures, the figure decision 0167 gave for recognising a raid; 0 in the frozen step 1 figures, so that every game on them names no type and is what it was.

## What a side learns by it

A type named in a claim was on that field at the start of that pulse: this is true knowledge of the enemy, and it is what a pilot over a field sees. The number against a type is a claim: never below the truth for that type made out, and it may be well above. A side cannot tell a real aircraft from a false one by its type, since both are made out by the same chance and a false one is given a type that was there. Where no type at all is made out among several claimed, the field may have been empty; that is as much as the rule gives away.

## What it leaves for later

- **Types of aircraft claimed shot down,** by the raiders and by those under the raid.
- **The experience of the raiding group** moving the chance, as a ship's experience moves recognition (decision 0168): air groups' experience waits on research (decision 0064).
- **What a staff does with the knowledge.** Nothing reads the types but the claim's words and the seat's listing.

## Rule card

- **Reads:** the count of aircraft on the ground at the raided base at the start of the air combat step; the types of the groups there; the aircraft a raid's bombs really destroyed, by group; the figure.
- **Writes:** the list of types on the raiding side's claim, and on the event that records its arrival.
- **Reuses:** the claim, its stream's key, `roll_shots`, the wording of a type (decision 0167), the passing of a claim between commands.
- **New:** one figure, one draw value, one field of a claim.

## What it costs

- **State:** one field of a claim, so the hash version rises and the golden files are remade (decision 0057). It is named in `hash::name_every_field`. A picture carries it (decision 0170), and a saved game.
- **No game on the frozen figures changes** but for the hash version: the chance is 0 there.
- **The live figures:** games on them gain the draws of the new stream and the types in claims; no fight changes, and no other claim.
- **The agent interface:** a claim in the seat's listings and briefing gains the list; its version rises.
- **The table:** the line of a claim names the types.
- **Documents:** the rulebook's chapter on air combat (claims) and chapter 1's table of who sees what; the glossary; the figures' notes.

## The hand calculation

It is in `tests/scenario_98_the_type_claimed_on_the_ground.rs`, written before the code: the base raid of scenario 11's hand calculation, three mornings, whose pool is one group of one type, with the chances at 0 and at 1; and the rule's own function on small fields of two types.
