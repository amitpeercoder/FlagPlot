# 0241: An invented game names its own things

Status: decided by the designer on Nov 27, 2026, and written by the architect. Rung 1 is renamed under it in the same pull request.

Summary: A game that is invented whole may give its made-up things made-up names without the word "Placeholder". The mark `placeholder` stays in every file. In a game with anything of the record in it, a made-up thing is still named "Placeholder". Rung 1's places are Saltmere and Thornwick, and its ship is Merrowdale.

Standing: Decided by the designer; amends the data rule "anything made up is named Placeholder" (`docs/engineering/data-rules.md`, `CLAUDE.md`); built for rung 1

## Why

The designer, having played rung 1 on Nov 27, 2026: "I think we may remove the 'placeholder' placeholder from everything. It's only confusing. We could call base T a made up name and the same for base S. It's also OK to give made up names for all other things in this scenario, because it is, by definition, fictionary."

The word is there so that no reader takes an invented thing for the record. In a game of the record that has invented things in it, the word is the only thing that tells the two apart: the transports of the Eastern Solomons and of Midway are found by it (`tests/scenario_103_the_eastern_solomons_over_five_days.rs`, `tests/scenario_114_midway.rs`), and the Coral Sea's test reads it (`tests/scenario_105_the_coral_sea.rs`). In a game where nothing is of the record there is nothing to tell apart, and the word on every name only puzzles a new player.

## The rule

1. **A game is invented whole** when its scenario file and its map are both marked `placeholder = true`, no base, unit, ship or date in it is of the record, and the new game screen says that it is invented.
2. **In a game invented whole, a made-up base, unit or ship may carry a made-up name without the word.** The name is not the name of a place, ship or unit in `data/oob`.
3. **The mark stays.** Every file of such a game is still marked `placeholder`, the scenario's own `name` still begins "Placeholder scenario", and the card on the new game screen still says the game is invented.
4. **Everywhere else the rule is as it was.** In a game with anything of the record in it, a made-up thing's name starts with "Placeholder". A class, a type or a figure in `data/reference` that is made up keeps the word wherever it is used: reference data is shared by every game.
5. **No real ship, aircraft or weapon gets an invented rating,** as before. This record changes names only.

The designer said "from everything". It is taken for every game invented whole, and not for the games of the record, for the reason above. If he means those too, the tests that find invented ships by the word are restated first.

## Rung 1

| Was | Is |
| --- | --- |
| Placeholder base S | Saltmere |
| Placeholder base T | Thornwick |
| Placeholder forward garrison | Thornwick garrison |
| Placeholder source garrison | Saltmere garrison |
| Placeholder fast transport 1 | Merrowdale |

The names are the architect's and are the designer's to change. None is in `data/oob`. The ship's class is still the placeholder fast transport with cargo, class 12. The far place and the empty place of the ladder's map are named when rungs 2 and 3 are built.

No figure, place on the map, order or test of the engine changes: `tests/scenario_138_one_ship.rs` names none of them.

## What it reads and writes

No rule card: no rule of the game, state, draw or event changes.
