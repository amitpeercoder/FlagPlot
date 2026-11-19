# 0165: The 39 days with the ships of the record: a plan

Status: a plan by the architect, written on Nov 19, 2026, for the designer's confirmation. Nothing is built. It follows the research of the same day (`docs/research/oob/ship-names-39-days.md`) and takes up what decisions 0093 (item 8) and 0101 (items 16 and 17, ruling H) left as gaps.

Summary: A plan to replace the placeholder transports of the 39 days with the ships the record names. Its 78 placeholders that stand for real ships stand for 54 ships the record now names and 14 it does not, but a name is not what keeps them out: a real ship may carry no invented figure, and the transports, the destroyer-transports and three classes of Japanese destroyer still lack facts a class is derived from. Six steps: a second round of research for those facts, the derivation rules a transport needs, the classes, the American landings at the record's 15 and 8 ships, the Japanese runs as the destroyers that made them, and the baseline run again. It is done before stage 4's proof. With six questions for the designer

Standing: A plan for the designer's confirmation, Nov 19, 2026

## What is so today

The 39 days (`data/scenarios/07_full_slice.toml`) holds 93 ships named "Placeholder". Fifteen were invented to exercise rules and have no counterpart (the supply convoy, the night forces, a bystander and a dry destroyer). The other 78 stand for real ships:

| Force | Placeholders | What the record now gives |
| --- | --- | --- |
| The landings of 7 August, Guadalcanal | 21 | 15 ships, all named, each with its division and the units it carried |
| The landings of 7 August, Tulagi | 12 | 8 ships, all named: four transports and four destroyer-transports |
| The American runs after the landings | 15 | 3 named (Alhena, Fomalhaut, Betelgeuse); 12 not named by any source read |
| The Japanese runs | 28 | Every ship named, 31 August's by two sources, most by one |
| The Oka group's barges | 2 | Not found |

Why they are placeholders is written in the scenario and in `data/derivation/guadalcanal_landings.toml`, and none of it is the want of a name:

- **No class can be derived for them.** A real ship's figures are derived from fact rows by a written rule, and the derivation writes nothing if one figure of one ship cannot be read. The American transports and cargo ships lack a standard displacement, an endurance with its speed and a fuel capacity that can be read. The destroyer-transports have two readings of their guns after conversion and a disputed speed. The Kagero class lacks a torpedo tube fit and rounds per gun; the Shiratsuyu class an endurance as well; Oite rounds per gun and a radar row.
- **No rule turns a transport into a class.** Nothing maps the types "transport" and "cargo ship" to a kind, or derives a transport's cargo.
- **The placeholder counts are sums, not ships.** A placeholder troop transport carries 600 men, so the landings take 33 of them where 23 ships sailed; a Japanese run was sized by its men at 200 a ship, so the run of 18 August has five where six destroyers went.

Four real destroyers are already in the slice by the same road this plan takes: Fubuki, Shirayuki, Hatsuyuki and Amagiri, each with a troop figure read from the run it made.

## The plan

1. **A second round of research, for the facts a class needs.** One list, taken from what the derivation says it lacks: for each American transport and cargo ship of 7 August, standard displacement, fuel, endurance with its speed, speeds, guns; for the destroyer-transports, the guns after conversion and the speed; for the Kagero and Shiratsuyu classes and Oite, the tube fit, rounds per gun, endurance and radar. Each as fact rows with their sources, disagreements kept. This is commissioned now: it changes nothing in the game and every later step waits on it.
2. **The derivation rules a transport needs,** as their own decision record: the kind a transport and a cargo ship take; how cargo room is read; the troop figure of an American transport, read from the men it had aboard on 2 August (rows already on file) as a destroyer's is read from its run. A rule is written once for all of them, not ship by ship.
3. **The classes.** The derivation list gains every ship whose figures can all be read. A ship that still lacks one stays out, and stays a placeholder by name, with the missing fact in the gaps file.
4. **The American landings at the record's ships.** 15 ships off Guadalcanal and 8 off Tulagi, named, each carrying the unit of the scenario that the record puts most of in her. The scenario's land units are whole combat groups, and a ship carries the men of one unit, so each ship is given to one unit by a ruling written beside it; no rule of the engine changes. The runs after the landings take the three names that are known, and the twelve that no source names stay placeholders.
5. **The Japanese runs as the destroyers that made them.** Each run has the ships the record names, with their real guns and torpedoes, their men read from the run. They are then warships that meet what they meet: a run that is intercepted fights as destroyers, where a placeholder transport could only be sunk. The Oka group's barges stay placeholders.
6. **The baseline run again,** and the three arrangements and the recorded games remade. Every game on the 39 days plays differently from step 4 on.

Steps 4 and 5 are each one pull request with the scenario's notes rewritten, read by the architect before it merges.

## When

Before the proof of stage 4 (decision 0161, step 9), which is run on the 39 days and must not have the scenario moving under it. Steps 1 to 3 change no game and go on beside stage 4's work. Steps 4 to 6 land in one stretch, after stage 4's step 8 and before its proof is run; the conditions of that proof are read on the new baseline.

## What it leaves out

- **The escorts and the covering cruisers of the landings** (ruling H of decision 0101): a separate piece, with Savo Island's ships already derived.
- **The runs the scenario does not hold** and the research found: the Tanaka convoy of 19 to 25 August, the runs turned back on 27 and 28 August, Burrows and Kopara, the raids on Savo and Tasimboko. Each is a change to what the 39 days contains, not to its ships' names.
- **The twelve disputes** the research filed (which battalions sailed in which ship, whether the run of 6 September landed men, and the rest). A dispute is settled when a derivation or the scenario has to read it, with the reason on the row, and not before.

## What it costs

- **Rules:** none in the engine. The derivation rules of step 2 are data rules with their own record.
- **The scenario:** 33 American ships become 23 and the Japanese runs change their counts; ship numbers already given are not reused, and the new ships take new numbers.
- **Every result on the 39 days** is run again: the baseline, the arrangements of stage 3's proof, the recorded games, the opening estimates where they name a force.
- **Risk:** the research may not find every fact. Step 3 then leaves some ships as placeholders, and the plan still stands for the rest.

## Questions for the designer

1. **No real name on a placeholder's figures in the meantime.** The quick way to see names on the table is to rename the placeholders now. The architect recommends against it: the names would sit on invented speeds, fuel and capacities, in forces of the wrong size, and the rule that a real ship carries no invented figure is what lets a name on the table be trusted.
2. **A ship that cannot be completed stays a placeholder.** The architect recommends this over a working assumption for each missing figure. The one assumption already in use (a ship with no radar row is treated as not fitted) stays.
3. **The Japanese runs fight as destroyers.** The architect recommends it: it is what they were, and it is the difference between a run that is caught and sunk and a run that fights its way out.
4. **The four destroyer-transports of 15 August stay unnamed.** One source says "the four destroyer transports of Transdiv 12" and others name the division's four ships, but none says those four sailed that day, and the division lent ships elsewhere that week. The architect recommends leaving them placeholders over joining the two statements.
5. **Before stage 4's proof, not after.** The architect recommends it, so that the proof is run once on the scenario that will stand.
6. **Names for the ships that have none** (the designer's question of Nov 19, 2026: where a navy named its ships by a pattern, name the unnamed along it, with a cue that the name is not history). The architect recommends two answers, by what the ship is:
   - **A ship that was real and whose name the record does not give** (the twelve American ships of the later runs) is called what a staff would call her: her type and her run, "a destroyer-transport of Transport Division 12", with no name. She had a name, and a made-up one in the right style is the kind of invention most easily taken for fact.
   - **A ship that never existed** (the fifteen invented to exercise rules, and any scenario that is not history) may take a name in the pattern of her navy and type, chosen so that no ship of that navy bore it, and is marked as invented in the data by a field of her own in place of the word "Placeholder" in her name. The table shows the mark wherever it shows the name. This changes the data rule that a made-up thing is named "Placeholder", and is built only if confirmed.
   - **Barges and craft that had numbers and no names** keep numbers.

