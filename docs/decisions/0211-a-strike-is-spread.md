# 0211: A strike is spread over the ships of the kind it wants

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word after 1,000 games of Midway: "Do both", of spreading a strike and of fire that can destroy a carrier (decision 0212). It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today every strike group that attacks a force in a pulse attacks the same ship: the first of the kind its list ranks highest, and among several of that kind the lowest-numbered. So four carriers are attacked one at a time, and aircraft are spent on a ship already lost. This takes the groups that attack a force in a pulse in turn and gives each the next ship of the kind it wants, leaving for last a ship that already looks lost. A group is still not divided. It runs where a scenario's air combat figures switch it on, so no proof and no recorded game moves.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

- **The rule today** (decision 0026, item 9, kept by decision 0063; rules chapter 5; `airstrike/raids.rs`, `ship_to_attack`): a group attacks the best-ranked ship afloat in the force its raid found, the lowest ship number on a tie, chosen on the ships afloat when the pulse's air combat begins. Decision 0026 said of it: "It wastes aircraft on a ship already sinking and never touches the escorts. Spreading a strike is deferred."
- **What it does** (decision 0198, rules 107 and 108, counted on 1,000 games of Midway): in every pulse with two or more groups attacking a force they attack exactly one ship. Kaga is never attacked while Akagi is afloat. Of the groups after the first in a pulse, 86 in 100 go at a ship an earlier group of that pulse has already hit. At the Coral Sea in the game Yorktown is attacked by 1,135 groups in 200 games and Lexington, in the same force, by 145.
- **What the rows hold.** On the morning of 4 June three squadrons attacked three carriers: Kaga by nine dive bombers, Soryu by thirteen, Akagi by a number not stated (engagement events 8000 to 8002, preferred). At the Coral Sea on 8 May the Japanese attacked both American carriers of one force, and Lexington and Yorktown were both hit (engagement outcomes 4252, 4253, 4256). Against it: on the same day both American groups attacked Shokaku and none Zuikaku (events 2509 to 2511), for a reason no row gives. A strike was divided over the target by its leader at the Eastern Solomons (outcomes 336, 351) and at Santa Cruz American aircraft attacked two carriers, three cruisers and a battleship (events 2601 to 2606). No row states a doctrine for dividing a strike before launch.

## The rules

1. **The rule runs where a scenario's air combat figures give `spread_strikes = true`.** Without the key, or with it false, a group chooses its ship as today, in every rule below.
2. **The groups that attack one force in a pulse are taken in turn,** every group of the attacking side whose raid found that force, whatever ship or base it flew from, in the order of the group's number.
3. **A group's choice is among the ships of the kind it wants:** the ships afloat in the force, when the pulse's air combat begins, that its own target list ranks best. They are every ship of one kind, since a list ranks by kind.
4. **Ships that look lost are put last.** Those ships are put in order: first the ones whose fire damage is below `claimed_sunk_fire`, by ship number; then the ones at or above it, by ship number. That figure is the fire at which a crew already believes a ship lost (rules chapter 3). It is read on the state when the pulse's air combat begins, as the choice of ship is today.
5. **The n-th group to want that kind takes the ship in place n,** counting from nothing and going round: the first such group of the pulse takes the first ship of the order, the next the second, and a group past the last ship begins again at the first. Groups that want another kind are counted apart.
6. **A group is not divided.** All its aircraft attack the one ship, as today.
7. **Nothing else of the attack changes.** The force a raid attacks is chosen as today. The fighters, the guns, the shots and their draws are as they are: the attack's draws are keyed on the group and the force, not the ship, so the same shots hit or miss. What a hit does follows from the ship it lands on, as today.
8. **No draw.** It is the way a surface fight already gives each firing ship its target, the ship in place by number (rules chapter 4).

## What a side learns by it

Nothing new. The attacking side is told of its attack without the ship, as now, and its claim names the kind its contact was taken for. The side attacked sees which of its ships were attacked, as now.

## Rule card

Reads: the groups attacking a force in a pulse, each group's target list, the force's ships afloat and their fire damage, `claimed_sunk_fire`. Writes: nothing; the ship each group attacks. Draws: none.

## Figures

9. **One key, `spread_strikes`, in the air combat figures.** It goes into a figures file of its own, `data/reference/air_combat_play.toml`: the live `air_combat.toml` with the key added and nothing else changed, and a frozen counterpart. The live `air_combat.toml` and its frozen copy are not changed.
10. **The scenarios for play name that file:** the Coral Sea, the Eastern Solomons over five days and Midway, in their own files, and the table's copies of the 39 days, by the script that makes them. No file a proof or a validation measure plays names it.

## Games on file

- **No proof, no validation and no recorded game moves:** none names the new file.
- **The scenarios for play move,** and their bands are read again on 1,000 games each and reported: Midway's, the Coral Sea's, which stood inside the band at 574 to 426, and the Eastern Solomons over five days'. The build reports for each, before and after: who is ahead, carriers sunk by ship, hits on a carrier already sunk or with her deck out, and ships attacked in a pulse.
- **Tests on file that pin the one ship** play the frozen figures or the live `air_combat.toml`, where the rule is off, and are not changed.

## What it does not do

- **It does not divide a group among ships,** nor send aircraft at the escorts while a ship of the wanted kind is afloat.
- **It does not choose by a ship's size or by what she is doing,** only by her number and by whether she looks lost.
- **It does not carry the turn from one pulse to the next:** each pulse begins again at the first ship of the order.
- **It does not let a commander say how a strike is to be divided.**
- **It does not change which force a raid attacks** where two forces lie in one hex, as the two American forces at Midway do: every Japanese strike still goes at the force with the lower number.

## What it costs

- **One key and a change to how a group's ship is chosen;** the choice now needs every group attacking the force in the pulse, across raids, where today each group chooses alone.
- **A figures file of its own and its frozen counterpart,** named by the scenarios for play; the script that makes the table's copies rewrites a second line.
- **Rulebook:** chapter 5 (the ship a group attacks, and "One ship" among what is left out); the glossary's "Raid" where it says what a group attacks.

## Questions for the designer

None. He asked for it.
