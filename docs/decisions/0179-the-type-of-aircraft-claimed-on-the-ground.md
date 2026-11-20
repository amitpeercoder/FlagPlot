# 0179: The type of the aircraft claimed destroyed on the ground

Status: a plan by the architect, written on Nov 19, 2026, at the designer's word ("if the type is known, it should say which type"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review") before it was built. Built on Nov 20, 2026, with decision 0180 in one build (As built, at the end).

Summary: A raid on a base claims a number of aircraft destroyed on the ground and says nothing of what they were. The raiders now make out the types of the aircraft they claim, or do not, by one chance for the claim. A type they name is a type that was on the field; the number against it may be more than the truth, as the claim's number already may. A bombardment from the sea makes out none

Standing: A plan, written by the architect on Nov 19, 2026, and amended on review. Built on Nov 20, 2026, with decision 0180. Its readings were ruled on by the architect on Nov 20, 2026

## Why

The designer, at the table, of the line "1 aircraft destroyed on the ground": it should say the aircraft is the enemy's and, where the type is known, which type. The table can say the first. The second is not in the game: a claim holds counts and the kind of ship attacked (decision 0056), and no type of aircraft.

What is so today, read from the code:

- **A raid's claim of aircraft on the ground** is the aircraft its bombs really destroyed plus a draw for every other bomb, at `bomb_false_claim_chance` (decision 0056, 7; `airstrike/claims.rs`).
- **The aircraft on the ground at a base** are counted once, at the start of the air combat step: for each group at the base in order of number, its damaged aircraft and its ready aircraft not flying in the pulse (`airstrike/ground.rs`). Hits come off that count, the lowest group first.
- **A type of aircraft is already made out or not by a draw:** the people under a raid recognise the raiders' type at `raid_recognised_chance` (decision 0167), on a stream of their own.
- **A bombardment's claim** is of aircraft on the ground and nothing else (decision 0092, 17).
- **A claim is held by the side,** passed between commands whole (decision 0135), kept in a picture (decision 0170) and hashed.

## The rules

1. **A claim of aircraft destroyed on the ground may name types.** It gains a list of pairs, a type of aircraft and a number, in order of the type's number, each number above 0. The list is empty, or its numbers add to exactly the aircraft claimed on the ground.
2. **The field** is the aircraft on the ground at the raided base as they are counted at the start of the air combat step, before any raid of the pulse lands, added up by the type of each group. The count the hits are taken from is changed by each raid, so the field is kept apart, as it first stood.
3. **What each claimed aircraft is:**
   - an aircraft the raid's bombs really destroyed is of the type of the air group it belonged to, the group on the ground, not the group that bombed it; the number is the number really taken from that group;
   - the aircraft claimed falsely are shared over the types of the field in proportion to their numbers there: each type is given the whole part of the false number times its share; what is left is given one at a time to the types with the largest part cut off, a tie going to the type with the lower number. No draw.
4. **The types are made out or not, once for the claim,** by one draw at `raid_recognised_chance`, the chance with which men make out a type of aircraft in a raid (decision 0167). Made out, the list is every type of rule 3 with its number. Not made out, the list is empty. Where the field was counted empty, or nothing is claimed on the ground, there is no draw and the list is empty.
5. **The draw is its own:** `Draw::ClaimType`, code 18, added at the end; on a stream keyed as the claim's stream is (`rng_pair`, the intelligence subsystem, the raiding side's first unit to fire and the raid's first group), with that draw value. No other draw moves: a fight, a claim's numbers and every other claim are what they were at any setting of the chance.
6. **A bombardment's claim names no type.** Ships firing from the sea make out none.
7. **A claim of aircraft shot down names no type** by this record (later).
8. **The claim's words** give the types after the number: "5 aircraft destroyed on the ground (3 of type T, 2 of type U)", with a type named as decision 0167 names one, and the seat's tools naming it by its name in the data as they do there. Where the list is empty the words are what they were.
9. **Who sees it:** whoever sees the claim (decisions 0056 and 0135), and no one else. The other side is told nothing of it, as now.
10. **The claim's breakdown** gains the field by type, the aircraft really destroyed by type, the false aircraft by type, the chance and whether the types were made out. It is the umpire's, as every claim's breakdown is.
11. **No new figure.** `raid_recognised_chance` is 0.75 in the live figures and 0 in the frozen step 1 figures, so every game on the frozen figures names no type and draws nothing new. If a source ever asks for the two chances to differ, they are parted then.

## What a side learns by it

- **A type named in a claim was on that field at the start of that pulse.** This is true knowledge of the enemy, and it is what a pilot over a field sees.
- **The number against a type is a claim:** never below the truth for that type, and it may be well above, since the false aircraft are spread over every type on the field by its share. A type with few aircraft there may be given none of the false ones, and its number is then the truth; the side cannot tell when.
- **A claim whose types are not made out** is three times in four, at the placeholder chance, a claim on an empty field, where they are never made out: one claim with no type says little, and many raids on one field with never a type say it is empty. That is knowledge a side would have: men over an empty field see no aircraft to name.

## What it leaves for later

- **Types of aircraft claimed shot down,** by the raiders and by those under the raid.
- **The experience of the raiding group** moving the chance, as a ship's experience moves recognition (decision 0168): air groups' experience waits on research (decision 0064).
- **What a staff does with the knowledge.** Nothing reads the types but the claim's words and the seat's listing.

## Rule card

- **Reads:** the count of aircraft on the ground at the raided base at the start of the air combat step; the types of the groups there; the aircraft a raid's bombs really destroyed, by group; `raid_recognised_chance`.
- **Writes:** the list of types on the raiding side's claim, and on the event that records its arrival.
- **Reuses:** the claim, its stream's key, the chance and the wording of a type (decision 0167), the passing of a claim between commands.
- **New:** one draw value, one field of a claim.

## What it costs

- **State:** one field of a claim, so the hash version rises and the golden files are remade (decision 0057). It is named in `hash::name_every_field` and hashed in the claim's hash, which also covers a claim in flight between commands. A picture carries it (decision 0170): a picture's claim can no longer be copied bit for bit and is cloned. No figures file changes.
- **In the code:** what a base's hits destroy is returned by group, in the numbers really taken (after the cap by what the group has); a raid's tally keeps them by type over all its strike groups; the field is kept as it first stood. Every place that builds or takes apart a claim, a picture's claim or the claim's event changes with the field: the surface fight's and the bombardment's claims (empty lists), the objectives, the reports passed, the replay export.
- **No game on the frozen figures changes** but for the hash version: the chance is 0 there and no draw is made.
- **The live figures:** games on them gain one draw a claim and the types in claims; no fight changes, and no other claim.
- **The agent interface:** a claim in the seat's briefing and report gains the list, with the types by name; its schemas change and its version rises, and the recorded transcripts are remade.
- **The table:** the line of a claim names the types.
- **Documents:** the rulebook's chapter 3 (claims), chapter 1's list of draws and its table of who sees what, chapter 8's versions; the glossary.

## The hand calculation

It is in `tests/scenario_98_the_type_claimed_on_the_ground.rs`, written before the code: the base raid of scenario 11's hand calculation, three mornings, whose field is one group; the same with that group of another type than the bombers that raid it; and the rule's own function on fields of two and three types.

## Amended on review

An independent reading of the plan and its hand calculation, against the code, found the following; each was weighed before it was taken.

1. **A real aircraft could be told from a false one.** Every false aircraft went to the most numerous type, so any other type named was the truth. The false aircraft are now shared over the field's types in proportion (rule 3), with no draw, and "What a side learns by it" says what is left.
2. **Making out each aircraft apart made a claim with no type near proof of an empty field.** The types are now made out once for the claim (rule 4). What that still tells is said plainly.
3. **"Its own group's type" could be read as the raiders'.** Reworded (rule 3), and the hand calculation has a case in which the two differ.
4. **The stream's key was given wrongly.** Rule 5 gives it as the code has it. The draw keeps a value of its own, as every new rule's does.
5. **The field as first counted is not kept today,** and what is really destroyed is not known by type where the claim is made. Both are named under "What it costs".
6. **A figure absent from a file is not possible,** and a new figure would have changed the frozen figures file and what is pinned to it. The rule now uses the chance decision 0167 already has (rule 11).
7. **No breakdown was named.** Rule 10.
8. **The property over many seeds assumed one type on the field.** It now tests each named type against the field of that pulse.
9. **The list of what changes was short.** "What it costs" names the rest.

## As built

Built on Nov 20, 2026, on the branch `claims-types-and-tally`, from the head of `seat-trial-build` (decision 0178's build, pull request 546) with main and `group-tally` (decision 0180's plan, pull request 547) merged in, together with decision 0180: one rise of the state layout (49) and of the agent interface (30). No number or statement of the hand calculation was changed, and every statement of `tests/scenario_98_the_type_claimed_on_the_ground.rs` holds as written at the first run.

**What was built.**

- **`sim-core`:** `BattleClaim::ground_types` and the event's `ground_types`, a list of (type, number); `claims::ground_claim_types`, rule 3's function (the false aircraft shared by largest remainder, a tie to the lower type); `Draw::ClaimType`, code 18, at the end. In the air combat step the field of each raided base is kept as it first stood, by type, before any raid's hits come off the count; `apply_base_hits` gives what it really destroyed by group (the bombardment sums it, as before); a raid's tally keeps what its bombs destroyed by the type of the group on the ground. The claim draws once on its own stream (`rng_pair`, intelligence, the side's first unit to fire, the raid's first strike group, `Draw::ClaimType`). The claim's words, the breakdown (rule 10), the hash (after `from_bombardment`), a picture's claim (no longer `Copy`), the surface fight's and the bombardment's claims (empty lists).
- **`sim-agent`:** a briefing's claim gains `ground_types` (`aircraft_type` by name, `aircraft`), left out where empty, and the words name the type by name.
- **`sim-cli`:** the replay export adds `ground_types` (`aircraft_type` by number, `aircraft`) to `claim_received` where it is not empty.
- **Tests:** `tests/scenario_98_the_type_claimed_on_the_ground.rs`, 11 tests, to every statement and property.
- **Generated:** the run baselines differ only in their hash lines and in the words of raids' claims (983 lines changed, each the same line with the types and decision 0180's clause taken out); the five recorded transcripts differ only in the version and in claims; the saves in the state version and the daily hashes; the pinned hashes and the batch fixture in hashes and the state version.

**Readings.** Numbered with decision 0180's, which shares the build.

1. **No draw where the chance is 0** (rules 4 and 11): the draw is made only where the field was not empty, something is claimed on the ground and `raid_recognised_chance` is above 0. So the frozen figures draw nothing new, as rule 11 says.
2. **The field** is the pool hits on aircraft are taken from (decision 0035, 11; 0036, 11): each group at the base, its damaged aircraft and its ready aircraft not flying in the pulse, added up by the group's type. Two raids on one base in a pulse read the one field as it first stood.
3. **The breakdown** (rule 10) has, where the draw could be made (a field and something claimed on the ground), pairs of factors `field_type` and `field_aircraft`, `real_type` and `real_aircraft`, `false_type` and `false_aircraft` (the type's number, then its aircraft, in order of type), then `raid_recognised_chance` and `types_made_out` (1 or 0). A factor's name is fixed, so a type is told by a pair. The false aircraft by type are given whether or not the types were made out.
4. **The words:** "15 aircraft destroyed on the ground (15 of aircraft_type:3)" in the log, "(15 of Placeholder dive bomber)" in the seat's tools (the type by its name in the data, as decision 0167 names one there); several types "(2 of T, 3 of U)".
5. **"A type named was on the field"** is tested against the groups at base 2 with any aircraft at the start of each day, read from the truth: every aircraft on the field at the start of the morning pulse is of one of them. That is a little wider than the field itself, which the truth does not hold once the step has begun.
6. **"The logs are those of scenario 11's variants as they run today"** is asserted as: at made out 0 every claim's words, without decision 0180's clause, are the words `claim_text` gives the claim's numbers with no type and no unit, and the fight and every claim's numbers are those at made out 1; scenario 11's own tests, unchanged, pass.

7. **Decision 0167's two properties on scenario 6a.** Rule 11 shares `raid_recognised_chance` with decision 0167, so on scenario 6a the raiders' claim now names types where 0167's test moves that chance, and the state's hash counts a claim's types. In `tests/scenario_88_what_the_attacked_saw.rs` "the attacker is told nothing new" now sets the attacker's own claim aside (the rest of its events are the same, word for word), and "no fight is resolved differently" compares the ships, air groups and bases of every day for both scenarios and the daily hashes for scenario 19a only, whose raid is on ships and names no type. No expected number moved. The architect ruled the properties restated so stand, and the chances are not parted.
8. **The world at the end, and decision 0180's tally.** Scenario 11's "the world at the end is the same at every setting of the chances" and the unit test that two raids taken in either order leave the same truth now compare the truth with every air group's tally set aside: the tally is the record of the claims, which the chances (and, in the unit test, the streams the order of the raids keys) move. Nothing else in either differs.
9. **The replay export test** (`crates/sim-cli/tests/export_pictures.rs`) is re-pinned: the ten exports of the commit before were made again and compared with these; they differ in the state version, the day hashes and, in scenarios 5, 31, 50 and 58, in the claims' words, `claimed_by`, `ground_types` and a claim's breakdown factors, and in nothing else.

**What the table will need.** A claim's line can name the types after the number on the ground, from `ground_types` in the briefing or the export.
