# 0166: Deriving a transport, and the rulings on the ships of the runs

Status: settled by the architect on Nov 19, 2026. It is step 2 of decision 0165's plan, written after the second round of research (`docs/research/oob/class-facts-39-days.md`). It adds rules to the derivation and rulings on fact rows. It changes no rule of the engine and no scenario. Built in two parts (The build, below): part A, the Japanese destroyers, on Nov 19, 2026; part B, the transports, waits for fuel and endurance.

Summary: The rules by which a real transport, cargo ship and destroyer-transport become a ship class, and the rulings that let the Japanese destroyers of the troop runs be derived. A transport's kind is transport, its toughness is read from its light displacement, its cargo from a stated cargo capacity, its men from what it carried on 2 August, and its guns are left out. The Kagero, Shiratsuyu, Hatsuharu and Fubuki classes can be derived now; Oite and the American ships cannot yet, for want of ammunition, fuel and range, and stay placeholders until a source gives them

Standing: Settled by the architect, Nov 19, 2026

## Why

Decision 0165 puts the ships of the record into the 39 days, and a real ship may carry no invented figure. Two rounds of research filed what could be read. What is left is of three kinds: figures a rule has to say how to read, because no source gives them in the form the derivation asks for; disputes a ruling has to settle; and figures no source read gives at all. This record does the first two and names the third.

## The rules for a transport and a cargo ship

1. **Kind.** A class whose type is `transport` or `cargo_ship` is of the kind transport. A destroyer-transport keeps the type of the class she was built to, destroyer, and is of the kind destroyer.
2. **Toughness is read from the light displacement.** Standard displacement is a warship's measure, and no source gives one for a merchant hull. For a class of the kind transport the derivation reads `displacement_light_lt` where it reads `displacement_standard_lt` for a warship, and divides by the same figure. The two are the nearest like for like the sources allow: each is the ship without her fuel and her load. A merchant hull is weaker for its weight than a warship, and the rule does not allow for it: one figure turns displacement into toughness for every ship, and no row sizes another. This is named as a known weakness and not mended here.
3. **Cargo** is the class's or ship's stated cargo capacity (`cargo_capacity_lt`), by weight. A figure by volume, a deadweight or a load carried on one voyage is not read as a capacity. With no row the ship is not derived.
4. **Men.** A transport's or cargo ship's troop figure is the men she had aboard for the landings, read from the voyage of 2 August 1942 as the derivation list already allows, by the same road a destroyer's is read from her run. It is a load carried, not a capacity, and the sources file says so.
5. **Guns are left out.** A ship of the kind transport is derived with no main battery and no torpedo tubes. The transports of 1942 carried a single low-angle gun for their own defence; in a fight at sea the game gives a transport no part but to be hit, and one gun does not change that. This removes the need for a dated gun fit, a shell, a rate of fire and an outfit of ammunition for these ships, none of which the sources give. Her anti-aircraft guns are read as any ship's are.
6. **Radar** is read as any ship's is: a sourced fit, a sourced none, or the list's working assumption that unknown means none (decision 0019, 5).
7. **Speed.** The maximum speed is the ship's own row where she has one, then her class's. Where an official history's data line (DANFS) and a later compilation differ, the official line is read, by a written exception for the class.
8. **Fuel and endurance** are read as any ship's are: a fuel capacity in tons and an endurance with its speed. A capacity in barrels is not converted until a row gives the Navy's own factor for its fuel oil. See what still blocks them, below.

## The rulings on the Japanese destroyers

9. **Rounds per gun, Kagero, Shiratsuyu and Hatsuharu classes:** 150, the figure given for the 12.7 cm/50 gun, not for a class (ship_class_facts 2450 to 2452), by a written exception for each. It follows the exception already made for the Pensacola class's 8-inch gun: a gun's stated outfit is read for a class that mounts it where nothing nearer exists.
10. **Shiratsuyu class endurance:** 4,000 nm at 18 knots (2455, 2456). The other figure, 6,000 nm at 15 knots, is for another speed and does not contradict it; it is kept as a note, as the Kagero class's 15-knot figure is. Reading every destroyer class at or near 18 knots keeps them on one footing.
11. **Hatsuharu class endurance:** 4,000 nm at 14 knots (2460, 2461). The other reading, 6,000 nm at 15 knots, is the same figure its one source gives the Shiratsuyu class, a different hull, and is longer at a higher speed than the first; it is left disputed and not read.
12. **Hatsuharu class torpedoes:** six 61 cm tubes with the Type 90 torpedo, the reading three sources give. The readings of the Type 93 and of eight tubes are left disputed and not read.
13. **Fubuki class:** Murakumo, Uranami, Shikinami and Asagiri are derived as Fubuki and Shirayuki already are.
14. **Radar:** every destroyer of the runs is listed under the working assumption that unknown means none.
15. **Men:** each ship's troop figure is read from the run she made, the run's men over its ships, as decision 0093 (item 8) has it. Where the men of a run are disputed the list names the row it reads and why.
16. **The Kamikaze class's guns and tubes are not ruled here, and Savo Island stands.** Two sources say the class lost a gun and a pair of tubes in 1941 or 1942 (fits 1100 to 1103). Yunagi fought at Savo Island as derived with four guns and six tubes, and that battle is measured and closed. The rows stay disputed. The question is ruled when a Kamikaze-class ship is next derived, and whether Yunagi's figures should change is put to the next round that opens Savo Island, in `docs/open-work.md`.

## What still cannot be derived

- **Oite** (the run of 16 August) and the **Mutsuki class:** no source read gives rounds per gun for the 12 cm gun. Oite's run stays a placeholder.
- **The destroyer-transports:** endurance not found, fuel only in barrels, rounds per gun not found, the standard displacement a lone low row, and the guns after conversion disputed. On the guns the ruling is ready for when the rest is found: the 4-inch/50, three to a ship, which one compilation gives and which the official history bears out for August 1942 (a 4-inch gun was blown off Colhoun on 30 August); the reading of three 3-inch/50 is undated and listed beside guns fitted later.
- **The American transports and cargo ships:** with rules 1 to 7 every figure can be read but fuel and endurance. The only rows are one compilation's, at low, and it gives the same range to different classes and the same fuel to three. They are not read by exception: a figure that looks copied from class to class is not one to put a real ship's name on. The source that would settle them is the Navy's own data book of 1945 (Ships' Data, U.S. Naval Vessels, volume III), which gives fuel and radius; it exists as a scanned file, which the research may not download without the designer's leave. It is asked for.
- **Bellatrix and Libra:** three sources give each a different class. A ship's own rows are read before her class's, so she is derived from her own rows and the class key is left as it is.

## The build

In two parts, each one pull request read by the architect.

- **Part A, now: the Japanese destroyers.** The exceptions of rules 9 to 12 in `rules.toml`; the Kagero, Shiratsuyu, Hatsuharu and Fubuki-class ships of the runs added to `data/derivation/guadalcanal_landings.toml` with their runs; the class file made again. No tool code changes. No scenario reads the new classes until decision 0165's step 5, so no game changes.
- **Part B, when fuel and endurance are found: the transports.** Rules 1 to 5 in `oob-derive`, with the transports and cargo ships listed.

## What it costs

- **The engine:** nothing.
- **The derivation:** rules 1 to 5 are new code in `oob-derive` (part B). Part A is rows in two data files.
- **Earlier lists:** none changes. The exceptions name classes no other list reads, and `oob-derive --check` on every list must say so.

## As built: part A

Built on Nov 19, 2026, on the branch `runs-destroyers`. Rows in two data files and the class file made again; no tool code, no fact row, no row's standing and no scenario changed.

- **The exceptions** (`data/derivation/rules.toml`, agreed 2026-11-19): `main_rounds_per_gun` of the Kagero, Shiratsuyu and Hatsuharu classes from ship_class_facts 2450, 2451 and 2452 (ruling 9); `endurance_nm` and `endurance_speed_kn` of the Shiratsuyu class from 2455 and 2456 (ruling 10); the same of the Hatsuharu class from 2460 and 2461 (ruling 11). Ruling 12 needed none (reading 4).
- **The ships** (`data/derivation/guadalcanal_landings.toml`), each under the working assumption that unknown radar means none (ruling 14), with the class ids pinned after 3010:

  | Class id | Ships | Class | Run read | Troops |
  | --- | --- | --- | --- | --- |
  | 3011 | Kagero, Arashi, Hagikaze, Hamakaze, Tanikaze, Urakaze | Kagero | 18 August, 916 men (troop_lift_facts 141) over 6 ships (138), by the architect's ruling 1 | 152 |
  | 3012 | Kawakaze, Umikaze, Suzukaze | Shiratsuyu | 31 August, 1,200 men (47) over 8 ships (155) | 150 |
  | 3013 | Yudachi | Shiratsuyu | 4 September, about 1,000 men (161) over 6 ships (162) | 166 |
  | 3014 | Yugure | Hatsuharu | 2 September, about 150 men (159) over 2 ships (160) | 75 |
  | 3015 | Ariake | Hatsuharu | 4 September, as above | 166 |
  | 3016 | Murakumo, Uranami, Shikinami | Fubuki | 4 September, as above | 166 |

  The run of 31 August, read already for Hatsuyuki and Amagiri, now names five ships of its eight; the run of 18 August is a new entry; the runs of 2 and 4 September are new entries. Fubuki, Shirayuki, Hatsuyuki and Amagiri keep the runs they were read from.
- **Left out:** Asagiri (reading 3); Oite and the Mutsuki class, as the record says. The list's header says who is in and who is out and why.
- **The class file** `data/reference/ship_classes_guadalcanal_landings.csv` and its `.ships.csv` and `.sources.csv` were made again by `oob-derive`. The four classes of the list as it was are unchanged. `oob-derive --check` passes on every list, and the class files of Savo Island, the Eastern Solomons, Cape Esperance and Rabaul made again are as they were: the Eastern Solomons list reads Kagero-class ships, but with their surface weapons not modelled, so the new rounds per gun is not read there.
- **What changed in the games:** nothing. No scenario names a new class id. `sh scripts/regenerate.sh` remade the saves of two recorded games, `full_slice_five_days_sonnet` and `second_proof_week_sonnet`, each in one line, the hash of the class file they load; no run baseline, golden, daily hash or transcript changed.

Readings the record did not make:

1. **One run a ship.** The tool reads a ship for one run only, and most of these ships made several of the scenario's runs. Each is read from the first run of the scenario (task forces 31 to 38) that she made whose men and ships are not disputed; the four ships the list first had keep their runs. So Kagero is read on 31 August and not on 18 August, Kawakaze and Umikaze on 31 August and not on 29 August, Yudachi on 4 September (her 31 August rests on Zimmerman's "all the above craft" alone, row 1331, against combinedfleet's eight of row 155, which the run's figure reads). A ruling on a disputed run may move a ship to it.
2. **The run of 18 August is disputed in its men** (900 in troop_lift_facts 43, 44 and 179; 916 in 141, which the scenario reads; 917 in 703; about 1,000 in 142), so it is not read. Arashi, Hagikaze and Hamakaze made no other run of the scenario, and Tanikaze and Urakaze only the run of 1 September, which the scenario does not hold; all five can be derived in every other figure and wait for a ruling.
3. **Asagiri** is named by ruling 13, but her one run, Desdiv 20's of 28 August (550 men, troop_lift_facts 143, over 4 ships, 144), was aborted and is not a run of the scenario; with no run she would be derived with no troops, so she is left out until her run is ruled.
4. **Ruling 12 needs no exception.** The six Type 90 tubes are fits 1083 to 1085, already preferred and agreed, and the Type 93 and eight-tube readings (1086, 1087) are disputed, so the tool reads the six tubes by its own rule. The comment in `rules.toml` says so.
5. **The run of 4 September is read in spite of its dispute.** Its two pages disagree on which section led (ship_facts 2359, 2360), but both name the same six ships and the same 1,000 men, so the figure does not rest on which was which.
6. **No entry for the runs no ship is read from.** The runs of 29 August (beyond the two ships already read on it), 5 and 6 September are disputed, and the run of 11 September (637 men, 168, over 3 ships, 169) is made only by ships read on an earlier run; the tool takes no run without a ship, so none of them is listed.

## The architect's rulings on part A

1. **The run of 18 August carried 916 men in six destroyers** (Nov 19, 2026). The build left the run unread because its men are disputed. 916 is the one preferred row (troop_lift_facts 141) and the figure the scenario has read since decision 0101; the three official histories' 900 is the same number rounded; the second page's 917 is one man off and Miller's "about 1,000" is a round figure for the echelon. 916 over the six ships Zimmerman names (138) is 152 a ship. Kagero is read on this run, her first, and Arashi, Hagikaze, Hamakaze, Tanikaze and Urakaze with her: class 3011, six ships, 152 men. Reading 2 is closed by this.
2. **One run a ship stands for part A, and is not the last word.** A troop figure read from one run is a load carried, and sisters now differ by the run each is read on: Yugure has 75, from a run of two ships with "about 150 men". When the scenario is given these ships (decision 0165, step 5) a ship must be able to carry what she carried on each of her runs, and that needs a rule for a class's figure across its runs, with a change to the tool. It is written with step 5. Until then no scenario reads these classes.
3. **Readings 3 to 6 stand as made.** Asagiri stays out; ruling 12 needs no exception; the run of 4 September is read; no entry is made for a run no ship is read from.
4. **The other disputed runs are not ruled here** (29 August, 5 and 6 September, the ships of 31 August and of 11 September): no ship's figure rests on them in part A. They are ruled with step 5, where the scenario's runs must each be given their men.

