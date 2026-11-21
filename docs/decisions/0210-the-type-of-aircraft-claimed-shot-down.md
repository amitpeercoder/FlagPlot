# 0210: The type of the aircraft claimed shot down

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word at the table: "if we know what kind of enemy aircraft were shot down, we should let the player know what kind they were". It is what decision 0179 left for later (its rule 7). It is to be read by a reviewer who has not seen it written before anything is built.

Summary: A claim of aircraft shot down gives a number and says nothing of what they were. The men who fired now make out the types of the aircraft they claim, or do not, by the one chance and the one draw that already decide the types claimed destroyed on the ground. A type they name is a type that flew against them in that raid; the number against it may be more than the truth, as the claim's number already may.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

- **A claim names the types it claims on the ground** (decision 0179; rules chapter 3, What a raid's claim names): a list of pairs, a type and a number, made out or not once for the claim.
- **A claim of aircraft shot down names none.** The table says "4 aircraft shot down" where the men who shot them down would have said what they were.
- **Men did say.** Every action report on file that claims aircraft shot down names them by kind, rightly or wrongly; the game's own rule for what the attacked saw of a raid (decision 0167) already has a chance of making out a type.

## The rules

1. **A claim of aircraft shot down may name types.** It gains a second list of pairs, a type of aircraft and a number, in the order of the type's number, each number above 0. The list is empty or its numbers add to the aircraft the claim says were shot down.
2. **The field** of a side's claim in a raid is the enemy's aircraft its fighters and guns could fire on in that raid, added up by the type of each group, as they are counted when the raid's air combat begins and kept as first counted:
   - for the side raided, the aircraft of the raid: its strike groups and its escorts;
   - for the raiding side, the patrol over the hex that met the raid.
3. **What each claimed aircraft is.** Each one really shot down is of the type of the group it was taken from. The false ones are shared over the types of the field in proportion to their numbers there, as decision 0179 shares the false ones on the ground: each type is given the whole part of the false number times its share, and what is left goes one at a time to the types with the largest part cut off, a tie to the lower type number. No draw.
4. **Made out or not, once for the claim, by the draw there is.** The one draw of decision 0179, at `raid_recognised_chance`, decides both lists: made out, the claim names its types on the ground and in the air; not made out, neither. The draw is made where the chance is above 0 and the claim has something to name: aircraft claimed on the ground with a field there, or aircraft claimed shot down with a field in the air. It was made before only in the first case; in the second it is the same draw on the same stream, so no other draw moves.
5. **A surface fight's claim and a bombardment's claim name no type,** as now.
6. **The claim's words** give the types after the number, as they do on the ground: "4 aircraft shot down (3 of type T, 1 of type U)".
7. **Each unit's share of the claim** (decision 0180) is not parted by type: `by_unit` keeps its counts.
8. **Who sees it:** whoever sees the claim, and no one else.
9. **The claim's breakdown** gains the field in the air by type, the aircraft really shot down by type and the false ones by type. It is the umpire's.
10. **No new figure.** `raid_recognised_chance` is 0.75 in the live figures and 0 in the frozen step 1 figures, so every game on the frozen figures names no type and draws nothing new.

## What a side learns by it

- **A type named flew against it in that raid.** That is more than the count told: a side that claims a torpedo plane knows torpedo planes came. What the attacked saw of the raid (decision 0167) already tells the raided side the types that came, by the same chance, so for it this adds little. For the raiding side it tells what kind of fighter met it, which it did not know.
- **The number against a type may be above the truth,** and a type may be named of which none was really shot down, where false claims fall to it by its share of the field.
- **A claim with no type** is one not made out, or one of a field with nothing in it.

## Rule card

Reads: the groups in a raid's air combat and their types, the aircraft taken from each, the claim's counts. Writes: the claim's new list, which is state of the claimant's belief. Draws: none new; the draw of decision 0179 is made in more cases.

## Games on file

The list is new state of a claim, hashed in its place and named, so the layout rises and the golden files and daily hashes are remade for it. On the frozen figures no type is named and nothing is drawn, so every game on them gives the events it gave. On the live figures a claim of aircraft shot down now names types three times in four, so the words of those claims change in every game on the live figures, and in a claim that had nothing on the ground the new draw is made; the build reports whether any other event of any game on file moves, and stops if one does.

## What it does not do

- **It does not part a unit's claims by type,** nor an air group's tally.
- **It does not name the ship or the group** the aircraft came from.
- **It does not make out types one aircraft at a time:** all of a claim's types are made out or none.

## What it costs

- **One list of state in a claim, hashed; the layout raised; the claim's words, the agent's line and schema, the export and the table's diary changed** to give the types.
- **Rulebook:** chapter 3 (What a raid's claim names); the glossary's "Claim".

## Questions for the designer

None.
