# 0020: Game figures for real ships, derived from facts

Status: confirmed by the designer, Nov 10, 2026, as stage 4c1 of scenario 4. The scaling constants are model parameters and placeholders.

## The gap

Decision 0015 says no real ship gets an invented rating: a game figure is derived from sourced fact rows by a written rule, and can name the rule and the rows. It left the rules and the tool unwritten. Until now every ship class the simulation loaded was a placeholder.

## Decisions

### The tool

1. **A tool reads the facts in `data/oob` and writes the ship classes a scenario loads.** It is given a list of ships and a date. The output is a generated file in `data/reference`, with a second file beside it that names, for every figure, the rule and the fact rows it came from.
2. **It reads only rows whose standing is "preferred" and that are valid on the date.** A fact that is missing, disputed or unconfirmed is an error naming the ship and the fact. Nothing is filled in. Several preferred rows that agree are all cited. Preferred rows that disagree are an error.

   A ship's own row replaces its class's row only when the ship's row can itself be read. A ship row that is in doubt does not block the class row. It is listed beside the figure in the sources file, so the doubt stays in view.
3. **An exception has to be written down.** The rules file may say "for this class and this fact, read the unconfirmed row", with a reason and the date the designer agreed. The tool lists every exception it used in the output.
4. **One class row per distinct fit.** The simulation gives every ship of a class the same figures. Ships of one class that differ on the date, in radar or in torpedo type, get separate rows, named for the difference, such as "Bagley class, SC radar".
5. **Generated IDs start at 1000,** so they cannot collide with placeholder classes. (Nov 13, 2026: a list pins each class id to its ships in a `[class_ids]` table and the tool no longer numbers rows by counting groups of equal figures; see decision 0077, "As built".)

### The rules

Each figure the simulation needs, and where it comes from:

| Figure | Rule |
| --- | --- |
| Kind | From the class's type: heavy and light cruisers are cruisers, and so on |
| Top speed | The sourced top speed |
| Cruise speed, endurance | The sourced endurance and the speed it is quoted at |
| Fuel capacity | The sourced fuel capacity, in long tons |
| Main guns | The number of guns of the main battery fitted on the date |
| Rounds per gun per minute | The sourced practical rate of fire of that gun. A theoretical or maximum rate is not read, unless a written exception names it (decision 0023) *(Nov 14, 2026: decision 0070 reads the lesser of the sourced rate and the median rate achieved by that kind of battery)* |
| Rounds per gun | The sourced ammunition carried per gun |
| Torpedo tubes | The number fitted on the date *(Nov 14, 2026: decision 0074: the tubes that bear on one side)* |
| Radar | Yes if a search radar is fitted on the date. Unknown is an error, unless the scenario's working assumption (decision 0019, 5) is given to the tool *(Nov 14, 2026: decision 0077: yes only for a surface-search set whose display has a plan position indicator)* |
| Gun damage | Shell weight in kg x the gun constant |
| Torpedo damage | Warhead weight in kg x the torpedo constant |
| Toughness | Standard displacement in long tons x the toughness constant |

6. **A ton in the simulation is a long ton,** 2,240 lb, the unit the Allied sources use for fuel and displacement. No earlier scenario depended on which ton was meant.
7. **Rate of fire, gun damage, torpedo damage and toughness become decimal quantities** in the simulation. Real guns fire 3.5 rounds a minute, and a 118 kg shell does 19.666666, not 19. Truncating to whole numbers would cost a small ship several percent. Whole-number figures give exactly the results they gave before.

### The constants

8. **The three constants are model parameters, not history:**

   | Constant | Value | So that |
   | --- | --- | --- |
   | Gun | 1/6 per kg | A 118 kg shell does about 20 |
   | Torpedo | 0.4 per kg | A 490 kg warhead does 196 |
   | Toughness | 1/25 per long ton | A 10,000 ton cruiser has 400 |

   They are chosen so that real ships land near the placeholder cruiser the combat rule was first tested with: an 8-inch hit does about 5 points to a heavy cruiser and a heavy torpedo about 49. They live in the rules file, marked as placeholders. Step 2 tunes them against the recorded outcomes. They are the only invented numbers in a real ship's figures, and they are the same for every ship.

## Why

- **Shell weight, warhead weight and displacement** are sourced for nearly every class, and each is a fair first measure of what it stands for. They ignore armour, shell type and where a hit lands. Decision 0014 already defers those.
- **An error, not a default,** because a default is an invented figure with nobody's name on it.
- **Constants shared by all ships** keep the comparison between ships sourced even while the scale is not.

## Deferred

- Armour, shell type and penetration.
- Torpedo reloads, range and reliability.
- Aircraft, weapons other than the main battery and torpedoes, and anti-aircraft fire.
- Figures for bases and air groups.
- Tuning the constants. Step 2.

## Rule card

No simulation rule. The tool reads `data/oob` and writes `data/reference`. The simulation reads only `data/reference`, as before.
