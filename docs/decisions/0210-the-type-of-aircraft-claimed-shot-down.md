# 0210: The type of the aircraft claimed shot down

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word at the table: "if we know what kind of enemy aircraft were shot down, we should let the player know what kind they were". It is what decision 0179 left for later (its rule 7). It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation is `tests/scenario_124_the_type_claimed_shot_down.rs`.

Summary: A claim of aircraft shot down gives a number and says nothing of what they were. The men who fired now make out the types of the aircraft they claim, or do not, by the one chance and the one draw that already decide the types claimed destroyed on the ground. A type they name is a type of the aircraft that firing was at; the number against it may be more than the truth, as the claim's number already may.

Standing: Planned on Nov 22, 2026; its review folded in; its hand calculation written.

## Why

- **A claim names the types it claims on the ground** (decision 0179; rules chapter 3, What a raid's claim names): a list of pairs, a type and a number, made out or not once for the claim.
- **A claim of aircraft shot down names none.** The table says "4 aircraft shot down" where the men who shot them down would have said what they were.
- **Fighter pilots' claims on file name kinds; ships' gunners' mostly do not.** Fighting Five claimed "14 Aichi 99 and 3 Zeros" at the Eastern Solomons (`engagement_outcomes.csv`, row 238; so rows 239 and 309), where the ships' anti-aircraft claims of the same day are numbers alone (215, 242 to 245, 250 to 256). One of those fighter claims names a type that was not there (239). The rule gives every claim the same chance of naming types and names only types that were fired on; both are simplifications, said here.

## The rules

1. **A claim of aircraft shot down may name types.** It gains a second list of pairs, a type of aircraft and a number, in the order of the type's number, each number above 0. The list is empty or its numbers add to the aircraft the claim says were shot down.
2. **Types are worked firing by firing.** Each firing of fighters or guns that a claim rests on is at one pool of the enemy's aircraft, the pool its hits are taken from:
   - escorts fire at the patrol: every group of the raided side on patrol over the raid's hex, each with the aircraft it has on patrol when the air combat step begins;
   - a patrol fires at the escorts: the raid's escorts;
   - a patrol fires at the bombers, and ships' or a base's guns fire: the raid's strike aircraft.

   A pool is counted by type as it stands when the step begins and is kept as first counted.
3. **What each claimed aircraft of a firing is.** Each one really shot down is of the type of the group it was taken from. The firing's false ones are shared over the types of its pool in proportion to their numbers there, by the function decision 0179 shares the false ones on the ground with: each type is given the whole part of the false number times its share, and what is left goes one at a time to the types with the largest part cut off, a tie to the lower type number. No draw. The claim's list is the sum of its firings' lists, type by type.
4. **So guns name no escort.** A side whose guns alone fired names only types of the strike aircraft, and escorts are named only where a patrol fired at them.
5. **Made out or not, once for the claim, by the claim's own draw.** One draw at `raid_recognised_chance` on the claim's stream decides both of a claim's lists: made out, it names its types on the ground and in the air; not made out, neither. The draw is made where the chance is above 0 and the claim has something to name: aircraft claimed on the ground with a field there, or aircraft claimed shot down. For a claim that had the draw already it is the same draw, so its types on the ground are as they were. A claim that had none (the raided side's, and a raiding side's against ships) draws now, each on its own stream, opened for that claim and drawn once: no other draw moves. The draw's value, `ClaimType`, 18, is not changed; it is now the claiming side's, not only the raiding side's.
6. **It can disagree with what the attacked saw.** The raided side is also told the type of the aircraft that attacked each ship or base, or that it was not made out (decision 0167), by another draw for other men. A side may so be told that a ship did not make out her attackers and that the force's claim names them, or the reverse. Both stand: they are two reports.
7. **A surface fight's claim and a bombardment's claim name no type,** as now.
8. **The claim's words** give the types after the number, as they do on the ground: "4 aircraft shot down (3 of type T, 1 of type U)".
9. **Each unit's share of the claim** (decision 0180) is not parted by type: `by_unit` keeps its counts, and a group's tally is as it is.
10. **Who sees it:** whoever sees the claim, and no one else. A claim passed between commands keeps its types, and so does a picture.
11. **The claim's breakdown** gains, where the draw could be made and the chance is above 0, factors of their own names for the air: each pool's type and number, the aircraft really shot down by type and the false ones by type. The chance and whether the types were made out are given once. On the frozen figures no breakdown changes.
12. **No new figure.** `raid_recognised_chance` is 0.75 in the live figures and 0 in the frozen step 1 figures, so every game on the frozen figures names no type and draws nothing new.

## What a side learns by it

- **A type named was fired on by its men in that raid.** For the raided side that is the types of the bombers, which decision 0167 already tells it three times in four, and, where its patrol met them, the types of the escorts, which nothing told it before. For the raiding side it is the types of the fighters on patrol over the target, which nothing told it before.
- **The number against a type may be above the truth,** and a type may be named of which none was really shot down, where false claims fall to it by its share of its pool. Every type named was in the pool fired on.
- **A claim with no type** is one not made out. In the air it is never the sign of an empty pool, since nothing is claimed shot down where nothing was fired on.

## Rule card

Reads: the groups in a raid's air combat and their types, the aircraft taken from each by each firing, the claim's counts. Writes: the claim's new list, in the claimant's belief, in the event that gives the claim, in a claim passed between commands and in a picture. Draws: no new value; the claim-type draw of decision 0179 is now made for the raided side's claim and for a raiding side's claim against ships, each on its own stream.

## Games on file

- **On the frozen figures** no type is named, nothing is drawn and no breakdown changes: every game on them gives the events it gave, and its daily hashes but for the layout.
- **Two hand calculations on file change, by the architect, with this record.** Scenario 98 (decision 0179) said the Allied claims name no type at any setting, and gave the words of a Japanese claim that now also names what was shot down; scenario 88 (decision 0167) has a property whose test compares daily hashes across settings of the chance, and a claim's types are in the hash. Their statements are amended in their own files, and scenario 124 holds the new numbers.
- **On the live figures** a claim of aircraft shot down names types three times in four. The words of those claims change, the claimant's belief changes and with it the daily hashes of every game on the live figures that has such a claim. No ship, aircraft, base or order changes: nothing that plays reads a claim's types. The build shows that by running every scenario on file before and after with claims' types set aside, and stops if anything else moves.
- **Remade for it,** as decision 0179 remade them: the golden hashes, the run baselines, the pinned exports, and the five recorded games that are replayed in tests (their transcripts and saves), with a paragraph in their README. **Not touched:** the records of closed proofs (`legibility_*`) and `stage_2b_seat_sonnet`, which no test replays.
- **The versions:** the state's layout rises for the new list, and the agent's interface rises for the new field.

## What it does not do

- **It does not part a unit's claims by type,** nor an air group's tally.
- **It does not name the ship or the group** the aircraft came from.
- **It does not make out types one aircraft at a time:** all of a claim's types are made out or none.
- **It does not let gunners make out types less often than pilots,** though the claims on file suggest they did.
- **It does not name a type that was not there,** though one claim on file does.

## What it costs

- **One list of state in a claim, hashed and named; the layout raised.** The air combat keeps, for each firing, the pool it was at and the aircraft really taken from each group, which today it adds up and throws away, and each pool as first counted.
- **The claim's words, the agent's line, schema and interface version, the export, and the table's diary,** whose rewriting of a claim's bracketed types must take two lists in one line.
- **Rulebook:** chapter 3 (What a raid's claim names; the claim-type draw's key; the line that says types shot down are not built); chapter 1 (who sees what, the list of draws, the versions); chapter 5 where it says what a claim names; chapter 8's versions; the glossary's "Claim" and "Types made out", and a new entry, "Pool fired on", kept apart from "Field (of a raided base)".

## Questions for the designer

None.

## How the plan changed on review

A reviewer who had not seen the plan read it against the air combat's code, decisions 0167 and 0179, their tests and the claims on file, on Nov 22, 2026.

- **The raided side's field was the whole raid, escorts and all.** Guns fire only at strike aircraft, so a side with no patrol could have been told the type of escorts its guns never fired on. Types are now worked firing by firing, each over the pool it was at (rules 2 to 4).
- **The raiding side's field was "the patrol that met the raid",** which the code does not have: escorts' hits come from every group on patrol over the hex. Rule 2 says so.
- **Two hand calculations on file would have failed without a word.** They are amended by the architect with this record ("Games on file").
- **What is really shot down is not kept by type where a claim is made.** "What it costs" names the change.
- **The breakdown's new factors would have shared names with the ground's,** and would have changed breakdowns on the frozen figures. Rule 11.
- **A claim can disagree with what the attacked saw.** Rule 6 rules that both stand.
- **"The same draw on the same stream" was loose:** for the raided side it is a stream never drawn before. Rule 5 says what is the same and what is new.
- **The "Why" said every action report on file names kinds.** The fighters' do and the ships' mostly do not, and one names a type that was not there.
- **"Field" already has a meaning.** The word here is "pool fired on".
