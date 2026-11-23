# 0225: Boats in the games for play

Status: a plan by the architect, written on Nov 23, 2026. The designer said on that day that boats need not be held back from the games he plays. The rows and gaps are in `docs/research/submarines-in-the-play-scenarios.md`; the rules are decision 0218's.

Summary: The record places few boats well enough to put on a map. This puts in those it does, in games of their own at the table so that no file a test, a proof or a copy reads is touched, beginning with Midway, where the stations are on file. It names the data that must be found before the Solomons and the Coral Sea can have theirs, and it measures each game before and after.

Standing: Planned on Nov 23, 2026; to be reviewed.

## What the rows allow

- **A boat can be placed only where** a ship row ties her to a class that is built (class ids 6000 to 6011), a row gives where she was on a day of the game, and her class has a speed.
- **Midway.** American: Nautilus, Tambor, Trout (the arc 240 to 360 degrees, 150 miles from Midway) and Gato (the arc 250 to 310 degrees, 200 miles), by Bates's disposition of 2 June (positions 16009 and 16010). Eleven more American boats are named on the arcs and have no ship row. Japanese: I-168 alone lies on the map; the two cordons lie east of its edge.
- **The Coral Sea.** American S-44 and S-47 off Rabaul and New Ireland, by named waters with no position. No Japanese boat can be placed: six are named in the gaps with no station.
- **Guadalcanal.** No row gives a station for any boat of either side. The boats that hit Saratoga and Wasp have no ship rows.
- **Each side's belief** has rows for Midway (the Allied side: one boat west of Midway, 29 May) and for the Allied side at Guadalcanal; the Japanese side at Guadalcanal has none.

## The rules

1. **Boats go into games of their own.** A folder `data/scenarios/table-boats/` holds a copy of each table game that gains boats, made by a script from the game it copies, with a name no other scenario has. The table offers it beside the game without boats. No file that a test, a proof, a baseline or a table copy reads is changed; the tests that list every file for play gain the new names and nothing else.
2. **Midway is first.** Its copies gain:
   - **Nautilus, Tambor, Trout and Gato,** each at the hex nearest her place on her arc. The row gives the arc and the boats on it, in an order, and no bearing for each. The rule: the boats a row names are spread evenly along its arc in the order the row names them, the first at one end and the last at the other, and each boat's place is the point at the arc's distance on her bearing. All the boats the row names are counted in the spread, those with no ship row among them, so that the four stand where the row would have them and the others' places are left empty. Each place is marked as estimated from the row by this rule.
   - **I-168** at the hex of her first position the rows give in the game's days.
   - **Each boat's home** is her side's base on the map nearest her, as a stand-in named as such: the true homes lie off the map.
   - **Each side's suspected waters** where a row dated on or before the game's first day gives them, by the rule of decision 0218 (25 and 47): told on the day of the row, or on day 1 if earlier.
   - **No order:** each boat keeps her station. A side the staff plays alone is given no patrol order here.
3. **The eleven American boats with no ship row are not placed,** and are named in the game's notes as left out. Their ship rows are research owed (rule 6).
4. **Each game is measured before and after** on seeds 1 to 100, staff against staff: ships sunk and damaged by cause and by side, carriers lost, the result, and for each boat the attacks made, the hits, and whether she was hunted, damaged or sunk. The figures are reported beside what the rows say those boats did. Nothing is changed for the result.
5. **The attack chance is watched, not tuned.** At 0.02 an hour a force that lies a whole day in a boat's hex is attacked in 38 runs of 100, where the rows give about 1 to 5 attacks in 100 boat-days. The measure counts force-hours in boats' hexes and attacks made, so that the figure can be judged against the rows on every game and by one rule, later, and not on this one.
6. **Before the Solomons and the Coral Sea have boats, three things are found:**
   - ship rows for the boats named in the rows that have none (the I-15 class boats, I-11, the RO boats, S-37, S-41, S-42, S-43, and the eleven at Midway);
   - a station by position and date for each boat, where any source gives one; a boat known only by "off Rabaul" is given a patrol order of those waters and no station of her own, once the staff's pattern for a patrol is built (decision 0223, stage A);
   - reliefs by date over the 39 days, since a boat that goes home does not return.

## What it does not do

- It adds no rule and changes no figure.
- It puts no boat in the one-day games, which have no staff.
- It does not say the games with boats replace those without. The designer chooses at the table.

## Before it is built

Reviewed by a fresh reader; then the architect's statements for the Midway copies: each boat's hex worked from its row by rule 2, and what each game must hold on day 1.
