# 0212: Fire aboard a ship

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word after 1,000 games of Midway: "Do both", of spreading a strike (decision 0211) and of fire that can destroy a carrier. It takes up what decision 0198's rule 16 left for the designer, a carrier caught with her aircraft aboard, and what decision 0102 deferred, fire that spreads. It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today fire aboard a ship does nothing but light her at night: it does not grow or burn out, and only flooding sinks a ship. So a carrier takes about 27 bomb hits to sink where the record has four lost to two, three and four. This lets a bomb that hits a carrier set fire among the aircraft aboard her, more surely the more of them are aboard and not flying; lets a fire that has taken hold grow or be fought down pulse by pulse; and loses a ship whose fire reaches the top of its scale. It runs where a scenario's figures give it, so no proof and no recorded game moves.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What is so today (rules chapter 4; `damage.rs`):

- **A ship has two tracks of damage, flotation and fire, each from 0 to 100, and only flotation at 100 sinks her.** A bomb's hit lands on flotation three times in four and on fire once. A hit's points are 100 times the weapon's damage over the ship's toughness.
- **"Fire does not grow, spread or burn out, and is not fought."** It lights a ship at night, and at 30 or more her enemy claims her sunk.
- **A hit on a carrier destroys no aircraft aboard short of sinking her,** and the game keeps nothing of aircraft being on deck, armed or fuelled (decision 0198, rule 110). It does know, when a carrier is hit, how many aircraft are aboard her and how many of them are flying in that pulse.
- **So by the figures Akagi takes about 26 dive bomber hits to sink, Kaga 27, Soryu 11, Hiryu 12, a Yorktown 14 and Lexington 28.**

What the rows hold (`engagement_outcomes.csv`, `engagement_events.csv`):

| Carrier | Bombs | Torpedoes | What became of her | Rows |
| --- | --- | --- | --- | --- |
| Akagi, 4 June | 2, "neither fatal" | 0 | fire over the whole hangar, "induced explosions", pumps failed; abandoned in about 9 hours, scuttled. "Preparing her second attack when hit" | 9300 to 9304 |
| Kaga, 4 June | 4 at least (unconfirmed) | 0 | fire uncontrollable, spread to the fuel tanks; sank in about 9 hours | 9306 to 9309 |
| Soryu, 4 June | 3 | 0 | induced explosions in her magazines; abandoned in 20 minutes, sank in about 9 hours | 9311 to 9314 |
| Hiryu, 4 June | not counted on file (4 and 6 claimed, disputed) | 0 | "fires uncontrollable"; abandoned in about 10 hours, scuttled | 9316 to 9320, 9480, 9481 |
| Lexington, 8 May | 3 | 2 | an internal explosion an hour and a half after the hits; scuttled in about 8 hours | 4252, 4253, events 2514 |
| Shokaku, 8 May | 3 | 0 | flight deck wrecked, petrol fires; saved | 4218, 4220 |
| Yorktown, 8 May | 1 | 0 | saved | 4256 |
| Yorktown, 4 June | 3 | 2 | fires minor and out in half an hour; abandoned for her list, sunk later by a submarine | 9350 to 9367 |
| Enterprise, 24 August | 3 | 0 | fires under control in 40 minutes; saved | 181 to 184, 207 |
| Enterprise, 26 October | 2 | 0 | saved | 4427, 4428 |
| Zuiho, 26 October | 2 | 0 | saved | events 2601 |
| Astoria, a cruiser, Savo | | | "uncontrolled fires caused the loss", about 10 hours after | 3969 |

- **Of the carriers that were bombed, five were lost to fire or to what the fire set off and six were saved,** and the hits do not tell them apart: two bombs lost Akagi and three did not lose Enterprise.
- **What tells some of them apart is on file for one ship only:** Akagi was preparing her second attack. The Japanese carriers held back armed aircraft that morning (engagement events 2172, 2237, unconfirmed). What each ship's deck and hangar held when she was hit is not on file, nor is Japanese damage control, nor Lexington's fuel vapour as a row.

## The rules

### Where it runs

1. **The rule runs where a scenario's surface combat figures give its six figures,** in a table `[fire]`, all or none. Without the table fire is as it is today, in every rule below.

### Fire among the aircraft

2. **A bomb that hits a ship with aircraft aboard may set them on fire.** For each bomb hit on a ship whose class can carry aircraft, after the hit's own effect:
   - her aircraft aboard are the ready and damaged aircraft of the groups aboard her, less those of them flying in this pulse;
   - her share aboard is those aircraft over her class's capacity, no more than 1;
   - the chance is `deck_fire_chance` times the share times the share;
   - one draw; if it is under the chance, her fire damage rises by `deck_fire_points`, no higher than 100.
3. **The share is squared** because two things must both be so: the bomb must fall among aircraft, and they must be the ones fuelled and armed, and each is the likelier the more of her aircraft are aboard. A carrier whose strike is away has few aboard and is seldom set alight this way; one caught with nearly all aboard nearly always is.
4. **A torpedo sets no such fire,** and neither does a shell.

### A fire that has taken hold

5. **At the end of each pulse, every ship afloat whose fire damage is at or above `fire_takes_hold_at` makes one draw.** Under `fire_spreads_chance` her fire rises by `fire_spread_points`; otherwise it falls by `fire_fought_points`, not below nothing.
6. **A fire below `fire_takes_hold_at` is as fire is today:** it neither grows nor falls.
7. **A ship whose fire damage reaches 100 is lost.** She sinks as a ship whose flotation reaches 100 does, with her aircraft aboard and her men, and the record says she was lost to fire. This is so whether the last points came from a hit or from the fire's own growth.
8. **It is so for every ship, not carriers alone:** a cruiser's fire that has taken hold grows or is fought as a carrier's is. Only rule 2 is of ships that carry aircraft.

## Figures

9. All are placeholders, each with what it rests on. They are set from the rows above and before any game is run with them, and are not moved to enter a band.

| Figure | Value | Rests on |
| --- | --- | --- |
| `deck_fire_chance` | 1 | a judgement: with every aircraft aboard, a bomb that hits sets them alight. The four carriers of 4 June were lost to two to four bombs each, Akagi to two |
| `deck_fire_points` | 30 | a judgement: a fire among aircraft has taken hold at once. It is the figure of `fire_takes_hold_at` |
| `fire_takes_hold_at` | 30 | the fire at which a crew already believes an enemy ship lost (`claimed_sunk_fire`), taken as the fire that is past being put out at once |
| `fire_spreads_chance` | 0.6 | a judgement from the count of the table: of twelve ships on file with fires worth a row, six were lost to them or to what they set off and six saved them. A fire that has taken hold is given a little better than even to grow in a pulse |
| `fire_spread_points` | 35 | a judgement from the times: the Japanese carriers were abandoned in 20 minutes to 10 hours, one to three pulses. From 30, two pulses of growth reach 100 |
| `fire_fought_points` | 20 | a judgement: Enterprise's fires were under control in 40 minutes and out in hours. Two pulses of fighting bring a fire from just past the mark to below it |

What they come to, said before a run:

- **A carrier with nearly all her aircraft aboard** (a share of 0.7): each bomb sets fire with a chance of 0.49. Two bombs, 0.74; three, 0.87; four, 0.93.
- **A carrier whose strike is away** (a share of 0.2): each bomb, 0.04. Three bombs, 0.115.
- **A fire that has taken hold:** from 30 it reaches 100 before it is fought below the mark in 49 cases of 100, and from 60, after two such fires, in 80. So a carrier with a share of 0.7 hit by three bombs is lost to fire about half the time or a little more, and one with a share of 0.2 hit by three about one time in twenty.

## What is recorded and who knows

10. **A fire set among aircraft is recorded** for the ship, citing the attack: the aircraft aboard, the share, the chance, and her fire after. **Each pulse's growth or fall is recorded** for the ship, citing the event that last changed her fire. **A ship lost to fire** is recorded as sunk, with fire as the cause, citing that.
11. **A side sees these for its own ships and nothing of the enemy's.** The enemy's claim already counts a ship with fire of 30 or more as sunk.
12. **Two draws are new,** each with its own value added at the end: one for a fire among aircraft, on a stream of the ship and the attacking group; one for a fire's growth, on a stream of the ship, by day and pulse.
13. **No new state.** Fire damage is a field a ship has. The rules label does not rise where no game on file plays differently.

## Rule card

Reads: a ship's fire damage, her class's aircraft capacity, the aircraft of the groups aboard her and those flying in the pulse, the figures. Writes: her fire damage; that she is sunk. Draws: two new values.

## Games on file

- **No proof, no validation and no recorded game moves.** The figures go into a file of their own, `data/reference/surface_combat_play.toml`: the live `surface_combat.toml` with the table added and nothing else changed, and a frozen counterpart. The live file and its frozen copy are not changed. The scenarios for play name it, with decision 0211's file: the Coral Sea, the Eastern Solomons over five days, Midway, and the table's copies of the 39 days.
- **The scenarios for play move,** and their bands are read again on 1,000 games each. The build reports for each, before and after, with decision 0211 on in both so that this rule's own effect is seen: who is ahead; carriers sunk by ship and by cause, flooding or fire; for each carrier lost to fire, the bombs that hit her, her share aboard when each hit, and the pulses from the first fire to her loss; other ships lost to fire.

## What it does not do

- **It does not keep what a carrier's aircraft are doing** beyond aboard and flying in the pulse. Aircraft aboard are taken as a danger whether struck below, fuelled on deck or neither: the game does not know.
- **It does not let a fire destroy aircraft** before the ship is lost, nor close a flight deck.
- **It does not tell one navy's damage control from another's,** nor a ship built with an armoured deck from one without.
- **It does not model fuel vapour,** which lost Lexington and Wasp after torpedoes.
- **It does not let a ship lost to fire be towed or saved by another,** nor be scuttled by choice: at 100 she is gone.
- **It does not change the fire a shell or a bomb sets today,** which lands on the fire track once in four hits and takes hold only once it has reached the mark.
- **It does not reach the proofs.** Savo's and Cape Esperance's cruisers, some of which were lost to fire, play as before. Whether the proofs take the rule is the designer's question, with those of hunger and ammunition.

## What it costs

- **Six figures, two draws, three records, a step at the end of each pulse and a check at each bomb hit on a ship with aircraft.**
- **A figures file of its own and its frozen counterpart;** the script that makes the table's copies rewrites a third line.
- **Carriers are lost far more easily where the rule runs,** on both sides.
- **Rulebook:** chapter 4 (what fire does, the end of a pulse, what sinks a ship, the figures, what is left out); chapter 5 where a hit on a carrier is described; chapter 1's order of a pulse, its list of draws and who sees what; the glossary's "Fire damage" and a new "Share aboard".

## Questions for the designer

None before the build. He asked for it. The six figures are placeholders, and what they do to each of the three battles is reported before anything is tuned.
