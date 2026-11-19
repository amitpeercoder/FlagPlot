# 0173: What a side believed, and whose a class is: the second part of step 4 of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 4 of its ten, section C, rules 7 and 8), which the designer confirmed. The second of the three records of step 4: the data the enemy's half is built from. It adds two kinds of entry to a scenario and one reference file, and no rule of play. To be built.

Summary: To build the enemy's half of a picture a constructor needs three things no view gives it and no rule should guess: which classes of ship and types of aircraft are the enemy's; what forces the side believed the enemy had and had not seen; and what the side believed the enemy meant to do. This record gives each a place in the data: a file that says whose each class and type is, and in a scenario the forces a side believes exist and the intents it believes the enemy holds. The 39 days are given theirs, as placeholders until the record of what each side believed is researched

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. To be built

## Why

The experiment of decision 0100 built an enemy from "the lowest class of the contact's kind that the side does not itself use", and noted that it works only while the sides share no class. It found that a view with no contact gives a world with no enemy ships, "valid, and proves nothing", and asked for the scenario to state the forces a side believes exist and has not seen, with "not seen" and "believed absent" as different inputs. And it found that an invented enemy with no orders sits still.

Decision 0161 took each up (rules 7 and 8): the classes "the scenario's reference data marks as that side's"; believed forces with three standings; and what the enemy wants, "stated, not found out", in the staff's own vocabulary.

What is so today, read from the code and the data:

- **No class or type has a side.** The class files and the type files have no such column, and nothing lists the classes of a side. A ship has its class and its side in the scenario. Placeholder classes are used by both sides.
- **A scenario states what a side holds at the start** in two ways: `[[contact]]`, a place, a count and a kind with an age; and `[[estimate]]`, a place and a count of men. Both become belief. Neither can say "a carrier force exists and we do not know where".
- **An intent is written in a scenario** as a `[[command]]` of kind `set_intent`, with the keys of its kind.
- **The generated class files are never edited by hand,** and a missing fact is a gap, not a guess (`docs/engineering/data-rules.md`).

## The rules

### Whose a class is

1. **A file of sides.** A scenario may name one file, `class_sides`, a CSV of the classes of ship and the types of aircraft its reference files give, each with the side that uses it: `allied`, `japanese` or `both`. Its columns are `table` (`ship_class` or `aircraft_type`), `id`, `side` and `note`. It is a file of its own and written by hand, so that no generated file changes.
2. **It is whole and it is true of the scenario.** Where a scenario names the file, the loader refuses the scenario if a class or type its reference files give has no row, if a row names a class or type they do not give, if one is given twice, and if a ship or an air group of the scenario is of a class or type the file gives to the other side alone.
3. **What a row rests on.** A class or type derived from the order of battle takes the side of the ships or groups its derivation list gives it, and its note names the list. A placeholder class takes `both` unless the note says why not. The side of a real class is a fact of the navy that built it and needs no figure.
4. **The reference data holds it,** for a view to read as it reads a class: whose a class is and whose a type is. A scenario that names no file has none, and plays as before.

### What a side believed exists

5. **`[[believed_force]]`** in a scenario states one force of the enemy's that a side believes in and does not hold a contact of. Its keys:
   - `side`: the side that believes it;
   - `name`: words, for the estimate to name it by;
   - `kind`: the kind of its most important ship, as a contact has one;
   - `ships`: how many ships it believes it has, 1 or more;
   - `standing`: `present`, `absent` or `unknown`;
   - where it was last known: `at_base` (a base on the side's map), or `q` and `r`, with `day`, the day it was known there (0 for before the first day, the default); or neither, where the side does not know where;
   - `placeholder`: true where the belief is not from a source, with a `note`.
6. **The three standings mean** (decision 0161, 7): believed present, it is there; believed absent, it is not, and the entry is the record that the side ruled it out; unknown, the side does not know whether it is there. The loader gives no chance to any of them.
7. **`[[believed_intent]]`** states one intent a side believes the enemy holds. It has `side`, the side that believes it, and then the keys of a `set_intent` command for the enemy: `intent_kind` and what that kind takes. It is checked as that command would be checked for the enemy's side on the side's own map: an intent that names a base names one on the map of the side that believes it. A kind the staff alone may give is refused, as it is in a command.
8. **They are the scenario's, not the world's.** A believed force and a believed intent are not state, are not belief, are never hashed and change no game. The loader checks them and keeps them with the scenario, for the constructor to be handed with a side's view. No rule of the engine reads them, and a seat is not shown them by this record.
9. **The loader refuses** an entry with an unknown key, a side or a kind it does not know, no ship, a standing it does not know, a base that is not on the believing side's map or a hex off the map, a day after the scenario's last, both a base and a hex, and an intent the enemy could not be given.

### The 39 days

10. **The full slice and the three arrangements of the proof name a file of sides** for their classes and types.
11. **They are given what each side believed on the evening before 7 August,** as placeholders, each marked and noted as resting on no source yet:
    - the Allied side believes a cruiser force present, last known at Rabaul before the first day, of 6 ships;
    - the Allied side believes a carrier force of 8 ships, standing unknown, where not known;
    - the Allied side believes the Japanese side means to hold Lunga and to hold Tulagi;
    - the Japanese side believes a carrier force of 8 ships, standing unknown, where not known;
    - the Japanese side is given no belief of what the Allied side intends.
12. **The research is owed.** What each command believed of the other's forces and purpose in the first week of August 1942 is in the record, and belongs in `data/oob` as rows before these entries lose their mark. Until then no estimate that rests on them is anything but a placeholder's.

## What it leaves for later

- **How a believed force becomes ships,** where one of unknown place is put, and what the enemy does with a believed intent: the third record of this step.
- **Belief that changes.** A believed force is the opening belief. What a side learns in play is in its contacts, claims and estimates; nothing here strikes a force out when it is found.
- **A seat shown what its side believes exists.** The directive from above (decision 0157) is where a commander would be told; it is not built here.

## Rule card

- **Reads:** a scenario's file and its reference files.
- **Writes:** nothing in a game. The scenario as loaded holds the believed forces and intents; the reference data holds whose a class is.
- **Reuses:** the loading of a contact's kind and place, of a base on a side's map and of a `set_intent` command.
- **New:** one reference file kind; two kinds of scenario entry; in `sim-core`, the plain data they are loaded into.

## What it costs

- **No game changes.** No state, no draw, no hash. A scenario that names no file and states no belief loads as before.
- **`sim-data`** loads the file and the entries; `docs/commands.md` and the scenario notes say how they are written; `docs/engineering/data-rules.md` gains the file of sides and what a row rests on.
- **The saved game** records the hash of each data file a scenario reads: the file of sides is one of them, so a saved game of a scenario that gains one is of other inputs. The build remakes what must be remade and says what.

## What the tests are to show

Written by the architect; the build writes the tests to them.

1. **A file of sides, by hand.** A scenario of the placeholder classes with class 1 given to the Allied side and class 2 to both: it loads, and the reference data answers Allied for class 1 and both for class 2. Refused, each with words that name the fault: a class the files give with no row; a row for a class they do not give; a class given twice; a Japanese ship of class 1.
2. **A believed force.** One entry of each standing loads and is kept as written: the side, the name, the kind, the count, the standing, the place and its day. An entry at a base is kept with that base's hex. Refused: no ship; an unknown standing; an unknown kind; a base that is the believing side's own; a hex off the map; both a base and a hex; a day after the last; an unknown key.
3. **A believed intent.** "The Allied side believes the Japanese side holds base 2" loads and is kept as a hold of base 2 for the Japanese side. Refused: an intent naming a base that is not on the Allied side's map; a kind only the staff may give; an unknown key.
4. **The 39 days.** The full slice and each arrangement of the proof load with their file of sides: every class and type they give has a row, and no ship or group is of a class or type given to the other side alone. Each holds the five entries of item 11 as written, each marked placeholder.
5. **Nothing changes.** Every scenario of the repository plays as it did: no run baseline and no pinned daily hash moves. What changes is listed: the saved games and fixtures that record the hash of a scenario file or name its data files.
