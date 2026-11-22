# 0212: Fire aboard a ship

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word after 1,000 games of Midway: "Do both", of spreading a strike (decision 0211) and of fire that can destroy a carrier. It was read by a reviewer who had not seen it written, who built a throwaway of both rules and played 200 games of each scenario for play; the designer then ruled that the rules run in every battle, with values that may differ by side and date where doctrine differed (decision 0213), and research on that doctrine was gathered. This is the plan rewritten on all three. It is to be reviewed again and measured before anything is built.

Summary: Today fire aboard a ship does nothing but light her at night, and only flooding sinks a ship; a carrier takes about 27 bomb hits to sink where the record has carriers lost to two, three and four. This lets a raid whose bombs hit a carrier set her aircraft and their fuel alight, the likelier the more of her aircraft are aboard, and by a chance that differs by side as the two navies' handling of fuel did; lets a fire that has taken hold grow or be fought down pulse by pulse in any ship; and loses a ship whose fire, at the top of its scale, grows again. It runs in every battle.

Standing: Planned on Nov 22, 2026; reviewed and rewritten the same day; to be reviewed again and measured.

## Why

What is so today (rules chapter 4; `damage.rs`):

- **A ship has two tracks of damage, flotation and fire, each from 0 to 100, and only flotation at 100 sinks her.** A bomb's hit lands on flotation three times in four and on fire once.
- **"Fire does not grow, spread or burn out, and is not fought."** It lights a ship at night, and at 30 or more her enemy claims her sunk.
- **A hit on a carrier destroys no aircraft aboard short of sinking her.** The game knows, when a carrier is hit, how many aircraft of her groups are aboard and how many are flying in that pulse, and nothing of their being armed or fuelled.

What the rows hold (`docs/research/carrier-doctrine-by-navy-and-date.md`; rows 15300 to 15462 of `engagement_outcomes.csv` and those on file before):

- **Where a carrier's aircraft were tells the lost from the saved better than her navy does.** Japanese carriers bombed with armed and fuelled aircraft aboard, Akagi, Kaga and Soryu on 4 June, were all lost to fire, after one to four hits (15304, 15306, 15324, 9300 to 9314). Japanese carriers bombed with their strike away, Shokaku twice and Zuiho, were all saved ("no aviation fuel was active", 15418).
- **Hiryu was lost to fire with most of her aircraft gone:** what she had left was aboard (15314, 15402).
- **No American carrier was lost in 1942 to a fire a bomb set.** Yorktown at the Coral Sea and at Midway and Hornet at Santa Cruz had drained their gasoline lines and filled them with carbon dioxide before the attack (15340 to 15346); Enterprise's fires of 24 August were under control in 40 minutes (207). The one American carrier hit with fuelled aircraft in her hangar and a live gasoline system, Wasp, was lost to fire, after torpedoes (5100).
- **The Japanese armed and fuelled in closed hangars; the Americans fuelled on the flight deck** (the Naval Technical Mission's report A-11; Yorktown's reports). No row has a Japanese carrier clearing her lines before an attack in 1942.
- **The rows do not bear out that Japanese crews fought fires worse in 1942.** That verdict is American and post-war, from interviews, with no ship cited (training facts 15326), and the same report describes schools, a damage control officer in every ship and drills (15327 to 15332). Shokaku's crew saved her twice.
- **Fire lost ships that carry no aircraft too.** Of the American cruisers and destroyers of 1942 with major fires on file, about four were lost with fire the stated cause (Astoria, Cushing, Monssen, Northampton) and about four saved (San Francisco, Boise, Salt Lake City, Pensacola). What decided it was whether the fire mains and power survived.
- **The American navy's firefighting changed in dated steps:** fog nozzles and inert gas after Lexington in mid 1942; schools, the manual and diesel pumps in 1943; hangar fires put out in one to four hours in 1944 and 1945. A Japanese change is dated only from mid 1944, and no row says it helped.

## The rules

### Where it runs

1. **The rule runs in every battle,** on the figures below. In the frozen figures the two chances are nothing for both sides, so fire there is as it is today and the hand calculations stand.
2. **It is of ships in task forces.** A ship in a yard or in port in no force is as today.

### Fire among the aircraft

3. **A raid whose bombs hit a ship that has aircraft aboard may set them alight.** For each raid that scores one bomb hit or more on such a ship in a pulse, if those hits have not sunk her:
   - her aircraft aboard are the ready and damaged aircraft of the groups aboard her, less those of them flying in the pulse, as a sinking counts them;
   - her share aboard is those aircraft over those aircraft and the ones flying, the aircraft of her own groups; nothing where she has no aircraft at all;
   - the chance for one bomb is her side's `aircraft_fire_chance` times the share; the chance for the raid is one less the chance that every bomb fails;
   - one draw; under the chance, her fire damage becomes 100.
4. **A torpedo sets no such fire,** and neither does a shell. A ship with no aircraft of her own is not touched by rule 3.

### A fire that has taken hold

5. **At the end of each pulse, each ship in a task force whose fire damage is at or above `fire_takes_hold_at` makes one draw.** Under her side's `fire_spreads_chance` her fire rises by `fire_step`; otherwise it falls by `fire_step`, not below nothing.
6. **A ship whose fire would rise above 100 is lost.** She sinks as a ship whose flotation reaches 100 does, with the aircraft aboard her and her men, and the record says she was lost to fire.
7. **A hit alone does not lose a ship to fire.** Hits raise fire as today, no higher than 100. It is the fire's own growth at the end of a pulse that loses her.
8. **A fire below `fire_takes_hold_at` is as fire is today.**

## Figures

9. Placeholders, each with what it rests on. They are set from the rows before any game is run with them, and none is moved to enter a band. `aircraft_fire_chance` and `fire_spreads_chance` are figures by side (decision 0213); the other two are one for all.

| Figure | Side and date | Value | Rests on |
| --- | --- | --- | --- |
| `aircraft_fire_chance` | plain value | 0.3 | no row; between the two below |
| | Japanese, from the start | 0.6 | a judgement: aircraft armed and fuelled in closed hangars with the fuel system live. The four carriers of 4 June, each with what aircraft she had aboard, were all lost to fire after one to four bombs |
| | Allied, from May 1942 | 0.15 | a judgement: fuelling on the flight deck, and the lines drained and filled with carbon dioxide before an attack, from the Coral Sea on (15340 to 15346). No American carrier was lost to a fire a bomb set in six bombings |
| | Allied, from January 1943 | 0.1 | a judgement on dated changes: inert gas, fog nozzles, schools and the firefighting manual |
| `fire_spreads_chance` | plain value | 0.5 | of the eight American cruisers and destroyers with major fires on file, four were lost and four saved |
| | Allied, from January 1943 | 0.4 | a judgement on the same dated changes |
| | Allied, from January 1944 | 0.3 | hangar fires controlled in one to four hours, Franklin and Bunker Hill saved |
| `fire_takes_hold_at` | | 30 | the fire at which a crew already believes an enemy ship lost (`claimed_sunk_fire`) |
| `fire_step` | | 25 | a judgement from the times: the Japanese carriers were abandoned in 20 minutes to 10 hours, one to three pulses |

No value by side is given for Japanese firefighting: the rows do not support one for 1942, and the change of 1944 has no row that says it helped.

What they come to, said before a run:

- **A fire at 100** is lost at the next pulse's end half the time and in the end 3 times in 4; at 75, once in 2; at 50, and for a fire that has just taken hold at 30 to 49, once in 4.
- **A Japanese carrier with all her aircraft aboard:** one bomb sets them alight 6 times in 10, two bombs 84 in 100, three 94. Three in four of those fires lose her.
- **An American carrier of 1942 with all aboard:** one bomb 15 in 100, three bombs 39. With half her aircraft away, three bombs 21.
- **A carrier whose aircraft are all flying** is not set alight this way.

## What is recorded and who knows

10. **A fire set among aircraft is recorded** for the ship, citing the raid's attacks: the bombs that hit, the aircraft aboard and flying, the share, the chance, and her fire after. **Each pulse's growth or fall is recorded** for the ship. **A ship lost to fire** is recorded as sunk with fire as the cause.
11. **A side sees these for its own ships and nothing of the enemy's.**
12. **Two draws are new,** each with its own value added at the end: one for a raid's fire among aircraft, on a stream of the ship and the raid's first group; one for a fire's growth, on a stream of the ship, by day and pulse.
13. **A cause is new on the record of a ship sunk,** and nothing is new in a ship's state: fire damage is a field she has.

## Rule card

Reads: a ship's fire damage and side, the aircraft of the groups aboard her and those flying in the pulse, the bombs a raid put on her, the figures. Writes: her fire damage; that she is sunk. Draws: two new values.

## Games on file

It runs everywhere, so games move wherever a carrier with aircraft aboard is bombed or any ship's fire is at 30 or more at a pulse's end. The reviewer's 200 games found ships at such a fire today in every scenario for play: destroyers, cruisers and transports as well as carriers.

- **The frozen figures and the frozen scenarios do not move,** nor the hand calculations on them.
- **Before it is built it is measured,** on a throwaway outside the repository, with decision 0211: 1,000 games of each scenario for play and of every validation file and proof, the surface battles among them, before and after, each rule alone and both together. What is reported: who is ahead; ships sunk by ship and by cause, flooding or fire; for each ship lost to fire, how the fire began (among aircraft or from hits), the bombs, the share aboard, and the pulses from the fire's beginning to her loss; ships of each kind lost to fire; and for each validation measure and each proof's condition, its value before and after and whether its verdict changes.
- **The records of closed proofs are not altered.** A proof or a measure whose reading moves is reported to the designer; it is his to say whether a proof is run again and whether a figure is to be looked at.

## What it does not do

- **It does not know what a carrier's aircraft are doing** beyond aboard or flying in the pulse. Both sides launch before either lands, so a carrier is "caught" only with what did not fly in that pulse. The moment of 4 June, strike aircraft back aboard and being rearmed as the bombs fell, is not a thing the game's pulse can hold; what stands in for it is the Japanese side's figure.
- **It does not let a fire destroy aircraft** before the ship is lost, nor close a flight deck.
- **It does not tell a ship with an armoured deck from one without,** nor one crew's training from another's but through her side's figures.
- **It does not model fuel vapour,** which lost Lexington after torpedoes and bombs and Wasp after torpedoes.
- **It does not let a ship lost to fire be towed, saved or scuttled by choice.**
- **It does not give the Japanese side a figure for fighting a fire.**

## What it costs

- **Four figures, two of them by side, two draws, three records, a step at the end of each pulse and a check for each raid that bombs a ship with aircraft.**
- **Ships of every kind can be lost to fire,** in every battle, the surface battles of the proofs among them.
- **Rulebook:** chapter 4 (what fire does, the end of a pulse, what sinks a ship, the figures, what is left out); chapter 5 where a hit on a carrier is described; chapter 1's order of a pulse, its list of draws and who sees what; the glossary's "Fire damage" and a new "Share aboard".
- **A scenario of its own and its hand calculation,** written by the architect when the measuring is read and before any code is delegated.

## Questions for the designer

None before the measuring. What it shows of each battle is put to him before anything is built.

## How the plan changed

- **The share aboard was squared, over the ship's capacity,** which made Akagi the hardest of the four to set alight and gave Hiryu, with few aircraft left and all of them aboard, a small chance. It is now the share of her own aircraft that are aboard, not squared.
- **The chance was one for both sides,** and in the game as played it fell on the American carriers, hit in the afternoon with everything back aboard, more than on the Japanese, hit in the morning with their strike away. The two navies handled fuel differently, by rows, and the chance is now a figure by side.
- **A fire among aircraft added 30 for each bomb,** so most carriers were lost at the instant of the hits. It is now one fire for a raid, and a ship is lost only by the fire's growth at a pulse's end.
- **A hit that took fire to 100 lost the ship,** which turned one hit on a destroyer's fire track into a sinking. Rule 7.
- **"Every ship afloat"** took in ships in a yard. Rule 2.
- **The count of fires lost and saved was miscounted** and left out Ryujo and Shokaku at Santa Cruz; the chance of spreading now rests on the cruisers and destroyers, eight ships with rows.
- **It ran only in the scenarios for play.** By the designer's ruling it runs in every battle.
- **It had no scenario of its own.** It will.

## Measured and reviewed again, Nov 22, 2026

The plan above was read by a second reviewer and played on a throwaway with decision 0211: 1,000 games of each scenario for play under each rule alone and both, and the validation batches and the proofs' batches before and with both. What follows is the plan as it now stands; where it differs from the rules above, this holds.

### What the games showed

Who is ahead, of 1,000 (the band: neither side in more than 650):

| | Before | This rule alone | Both rules |
| --- | --- | --- | --- |
| Midway | Japanese 867 | Japanese 722 | Japanese 521, Allied 479 |
| Coral Sea | Allied 574 | Allied 570 | Allied 622 |
| Five days | Allied 615 | Allied 768 | Allied 812 |

Carriers lost with both rules, games of 1,000, to flooding and to fire:

| Carrier | Before | Flooding | Fire |
| --- | --- | --- | --- |
| Akagi | 891 | 87 | 719 |
| Kaga | 128 | 55 | 701 |
| Soryu | 17 | 288 | 437 |
| Hiryu | 1 | 170 | 456 |
| Enterprise (Midway) | 255 | 103 | 166 |
| Hornet (Midway) | 48 | 96 | 164 |
| Yorktown (Midway) | 0 | 45 | 2 |
| Yorktown (Coral Sea) | 310 | 21 | 142 |
| Lexington | 0 | 0 | 108 |
| Shokaku (five days) | 177 | 21 | 367 |
| Zuikaku (five days) | 1 | 4 | 256 |
| Ryujo | 419 | 209 | 484 |
| Enterprise (five days) | 270 | 216 | 84 |
| Saratoga | 15 | 8 | 75 |

- **The rule does at Midway what the record has:** the four carriers are lost to fire, and the scenario comes inside its band.
- **Four things it did that no row supports:**
  - **Ships that are not carriers were lost to fire:** cruisers of the Tone class 144 times in the five days, 137 of them set alight through their own floatplanes; destroyers 137 in the five days and 110 at the Coral Sea; transports 89 at the Coral Sea.
  - **Seven carriers in ten lost to fire went down at the end of the pulse they were bombed in.** The Japanese carriers of 4 June burnt for about nine hours.
  - **Shokaku and Zuikaku were lost to fire in 367 and 256 games of the five days.** In the record Shokaku was bombed twice in 1942 with her strike away and saved. In the game about six in ten of a Japanese carrier's aircraft were aboard when she was bombed, fighters and aircraft not sent among them.
  - **American carriers were lost to fire in 159 to 332 games of each scenario.** Six bombings of American carriers in 1942 lost none to a bomb's fire. A raid set an American carrier's aircraft alight in 13 to 30 bombings in 100, which over six bombings loses none about one time in two: the record does not refuse it, and does not support 30.
- **No validation verdict changes,** and no proof condition read from a batch. The Eastern Solomons' measure of American carriers sunk holds its record of none in 945 runs of 1,000 where it held in all.

### What the review found

- **The frozen figures would have moved:** with the chance of spreading at nothing every fire of 30 or more fell by a step each pulse.
- **A sinking's count of aircraft in the air is nothing where sorties are not noted,** as in the Eastern Solomons' validation file: every aircraft read as aboard.
- **The Japanese figure was said to stand in for what the pulse cannot hold,** which is a limit of the model both sides share, and it was set by side after a game was read. Decision 0213 forbids both.
- **The chance of spreading at 0.5 does not come to "four lost and four saved":** from a fire at the top it loses three in four. Two of the eight were torpedo fires in fuel, which the game cannot make.
- **Fire damage was made to mean two things,** damage to mend and a fire burning.
- **Citations:** the words "no aviation fuel was active" are outcome row 15425, not 15418; rows 15340 to 15346 are Yorktown on 4 June alone, and the Coral Sea's draining is event row 15383; Shokaku on 8 May had "the gasoline afire" (15318) and was saved.

### The rules as they now stand

1. **It is of carriers alone:** a ship with a flight deck, in a task force. A cruiser's floatplanes are not in it. A ship of any other kind, a ship in a yard and the fire damage of every ship are as today.
2. **A fire among her aircraft is a state of its own,** a level from nothing to `aircraft_fire_levels`. Fire damage is not changed by it and does not feed it. Rules 5 to 8 above are withdrawn: a fire from hits does not grow, in any ship.
3. **A raid whose bombs hit a carrier may set her strike aircraft alight.** For each raid with one bomb hit or more on her, after every attack of the pulse has landed and if she is afloat:
   - her strike aircraft are the ready and damaged bombers of the groups aboard her, dive bombers, torpedo bombers and level bombers;
   - those aboard are those not flying in the pulse, as the pulse's own count of aircraft flying has it, which is kept whatever figures are loaded;
   - the share is those aboard over all of them, and nothing where she has no bomber;
   - the chance for one bomb is her side's `aircraft_fire_chance` times the share, and for the raid one less the chance that every bomb fails;
   - one draw; under the chance, her fire among aircraft becomes `aircraft_fire_levels`, the top.
4. **A fire among aircraft is fought from the next pulse.** At the end of each pulse after the one in which it was set or last raised, a carrier with such a fire makes one draw: under `fire_spreads_chance` it rises a level, otherwise it falls one. At nothing it is out.
5. **A carrier whose fire among aircraft would rise above the top is lost,** sunk as a sinking sinks her, with fire as the cause on the record.
6. **A carrier with a fire among aircraft flies nothing off and lands nothing on,** as one whose deck is out. Her deck is as it was when the fire is out.
7. **The rule is off where its figures are absent.** The frozen copies do not gain them, and every frozen test stands byte for byte.

### Figures

| Figure | Value | Rests on |
| --- | --- | --- |
| `aircraft_fire_chance`, Japanese | 0.6 | a judgement: aircraft armed and fuelled in closed hangars with the fuel system live (the technical mission's report A-11; event rows 15395 to 15406). The rows give the direction and no size |
| `aircraft_fire_chance`, Allied | 0.15 | a judgement: fuelling on the flight deck, and the gasoline lines drained on warning of a raid (event row 15383; outcome rows 15340, 15383). The rows give the direction and no size |
| `fire_spreads_chance` | 0.5 | no row; the middle. With three levels a fire set at the top is lost at the end of the next pulse one time in two and in the end three in four. The four Japanese carriers set alight on 4 June were all lost |
| `aircraft_fire_levels` | 3 | a judgement from the times: the carriers of 4 June were abandoned in 20 minutes to 10 hours |

- **The two values by side are a question for the designer** (below). They were first set after a game had been read, and the plan said so wrongly of the Japanese one that it stood in for the pulse. As they now stand each rests on the navy's handling of fuel and on nothing else, and neither has been moved since the measuring.
- **No value is given by date.** The American changes of 1943 and 1944 have no battle on file to be seen in and no row dated to the month.

### What is recorded and who knows

- **New state of a ship:** her fire among aircraft and the pulse it was set or raised. Hashed in its place and named; the layout rises.
- **Three records:** the fire set, citing the raid's attacks, with the bombs, the bombers aboard and flying, the share, the chance and the draw; each pulse's rise or fall; the ship sunk, with fire as its cause. A side sees its own. The enemy sees the sinking as it sees any.
- **Two draws,** each with its own value added at the end, in the air combat subsystem: for a raid's fire, on a stream of the ship and the raid's first group; for the fire's fighting, on a stream of the ship.
- **The rules label rises.**

Rule card: reads a carrier's fire among aircraft and side, the bombers of the groups aboard her and those flying in the pulse, the bomb hits a raid put on her, the figures. Writes her fire among aircraft; that she is sunk, with the aircraft and men lost with her; that she cannot fly. Draws: two new values.

### What it does not do, added

- **It does not know a bomber armed from one struck below empty.** A bomber aboard is taken as armed or soon to be.
- **It does not let a carrier be warned of a raid.** The game has no radar and no warning; the Allied figure carries the draining of lines that warning allowed.
- **It does not lose any ship but a carrier to fire.** The cruisers and destroyers lost to fire in 1942 were lost to many shell hits and to torpedoes in fuel; that is another rule.

### Questions for the designer

1. **The two values by side,** 0.6 and 0.15: whether the difference in handling fuel, sourced in direction and not in size, is enough to set them apart, or one value should serve both.

Answered on Nov 22, 2026, with the first measuring before him: by navy. The two values stand as judgements on the navies' handling of fuel; they are not moved on what a game shows.

It is measured again with decision 0211 as amended before anything is built, and what it shows is put to the designer.

## The amended rules measured, Nov 22, 2026

Decisions 0211 and 0212 as amended above were played on the throwaway, 1,000 games of each setting, with nothing moved. With both switches off the events are byte for byte those of the build without them.

Who is ahead, of 1,000 (the band: neither side in more than 650):

| | Neither | 0211 alone | 0212 alone | Both | Both, one chance of 0.4 |
| --- | --- | --- | --- | --- | --- |
| Midway | Japanese 867 | Japanese 955 | Japanese 781 | Japanese 579, Allied 421 | Japanese 661 |
| Coral Sea | Allied 574 | Allied 702 | Allied 561 | Allied 670 | Allied 617 |
| Five days | Allied 615 | Allied 706 | Allied 741 | Allied 816 | Allied 744 |

Carriers lost with both rules, games of 1,000, to flooding and to fire: Akagi 59 and 627; Kaga 48 and 635; Soryu 269 and 414; Hiryu 245 and 445; Enterprise at Midway 83 and 145; Hornet 69 and 196; Yorktown at Midway 33 and 1; Yorktown at the Coral Sea 15 and 86; Lexington 0 and 134; Shoho 871 and 39; Shokaku in the five days 10 and 291; Zuikaku 11 and 285; Ryujo 232 and 510; Enterprise in the five days 217 and 72; Saratoga 6 and 86.

- **No ship that is not a carrier is lost to fire,** in any setting.
- **No carrier is lost in the pulse she is bombed.** Of the fires that lost a carrier about six in ten lost her at the end of the next pulse; a fire put out took three pulses or more.
- **A raid's bombing set a fire** in 60 of 100 bombings of a Japanese carrier at Midway and 34 of an American one; at the Coral Sea 23 and 13; in the five days 58 and 21. The share of bombers aboard at a bombing was 0.68 for the Japanese carriers at Midway (0.52 in the morning, 0.91 in the afternoon) and 0.75 in the five days.
- **Midway is inside its band. The Coral Sea is outside it by 20 games and the five days by 166,** both toward the Allied side. Decision 0211 alone moves both that way too.
- **No validation verdict moves** (Savo, the Eastern Solomons, Cape Esperance, the air campaign, the landings), and the Eastern Solomons' American carriers sunk stays none in all 1,000 runs. **No proof condition read from a batch changes whether it holds.**
- **Not measured:** the proofs' conditions that need events and not a batch; legibility; the hash, saves and frozen tests, which the throwaway does not carry.

The readings the measurer chose where the record was open, each to be ruled before a build: a ship with a flight deck is a ship of the carrier kind; a strike aircraft is one of a type with the bomber role; a carrier bombed again while burning goes back to the top; a burning carrier is closed to launching and landing and is not counted as crippled or as looking lost; a fire ends when she sinks by flooding.

What it shows of each battle is put to the designer before anything is built.

## The designer's word, and the readings ruled, Nov 22, 2026

With these numbers before him the designer said: "Yes, build them", of both rules as measured, the five days' balance to be looked at as work of its own and not by moving a figure of this rule.

The measurer's readings are ruled as measured, so that what is built is what was played:

- A ship with a flight deck, for this rule, is a ship of the carrier kind.
- A strike aircraft is one of a type with the bomber role.
- Bombers aboard and flying are counted once, at the start of the pulse's air combat.
- A carrier bombed again while burning goes back to the top, and that pulse is the one her fire was last set in.
- A burning carrier is closed to launching and to landing, her own aircraft and others'; she is not counted as crippled for it, and she does not look lost for it under decision 0211. That a burning carrier should look lost to a raid is likely right and was not what was measured; it is left for a later record with its own measuring.
- A fire ends when she sinks by flooding. A carrier lost to fire is sunk by the ordinary sinking, with the fire's record as its cause.

The hand calculations are scenarios 126 (this rule) and 127 (decision 0211).

## As built, Nov 22, 2026

Built with decisions 0211 and 0213 in one pull request, to the rules of "Measured and reviewed again" and the readings ruled above. The code is `crates/sim-core/src/airstrike/carrier_fire.rs`, with the state `Ship::aircraft_fire` (its level, the day and pulse a raid last set it, and its record; hashed after `available_from`, the layout `flagplot-state-v56`), the last phase of every pulse `PulsePhase::AircraftFires`, the records `AircraftFireRoll` and `AircraftFireFought`, the draws `AircraftFireSet` (20) and `AircraftFireFought` (21), and `Ship::deck_for_flying`, which every launch, patrol, landing and the side's view of them read. The rules label is 28 (decision 0214, merged first, took 27 and the layout v55). Scenario 126 is its hand calculation.

Where the build reads the record, and where it departs from it:

- **A carrier lost to fire** is sunk by the ordinary `ShipSunk`, unchanged in its fields; "fire as its cause" is its cause, the `AircraftFireFought` record that rose above the top (marked `lost`). No field was added to `ShipSunk`.
- **The raid's record** names the carrier and her force, not the raid's group, which is the enemy's; it cites the raid's attacks on her, which the carrier's side is given cut.
- **The bombers counted** are each bomber group's ready aircraft less those the pulse's count has flying, plus its damaged aircraft, aboard; and the ready ones flying. Aircraft away from an earlier sortie and not back count in neither. Where none is aboard no draw is made and nothing is recorded.
- **The chance** is worked in the game's six decimals, each product cut in turn: scenario 126's 1 - 0.925^3 = 0.208546875 (the Allied side, half away, three bombs) is held as 0.208547, as the architect restated it.
- **A fire set closes her deck at once,** as a deck that goes out does: her aircraft in the air in that pulse look for another deck or a base.
- **A fire is fought wherever she is afloat,** in a force, in port or at a yard, by the architect's ruling below (`docs/rules/differences.md`, row 58, ruled and mended). A carrier lost to it out of a force has no `ShipSunk`: the fought record, naming no force, is her loss, and what goes down with her cites it.
- **A side's view and a dry run** read the side's own `aircraft_fire_chance` for both sides (decision 0213).

### The scenarios for play, 1,000 games each, as built

| | Measured ("The amended rules measured", both) | As built |
| --- | --- | --- |
| Midway, who is ahead | Japanese 579, Allied 421 | Japanese 571, Allied 429 |
| Coral Sea | Allied 670 | Allied 679 |
| Five days | Allied 816 | Allied 827 (Japanese 172, one level) |

Carriers lost, games of 1,000, to flooding and to fire (measured in brackets): Akagi 53 and 632 (59, 627); Kaga 56 and 650 (48, 635); Soryu 274 and 422 (269, 414); Hiryu 232 and 481 (245, 445); Enterprise at Midway 70 and 152 (83, 145); Hornet 79 and 172 (69, 196); Yorktown at Midway 30 and 0 (33, 1); Yorktown at the Coral Sea 19 and 95 (15, 86); Lexington 0 and 149 (0, 134); Shoho 870 and 41 (871, 39); Shokaku at the Coral Sea 0 and 4; Zuikaku at the Coral Sea 0 and 3; Shokaku in the five days 7 and 304 (10, 291); Zuikaku 9 and 302 (11, 285); Ryujo 228 and 507 (232, 510); Enterprise in the five days 220 and 70 (217, 72); Saratoga 7 and 82 (6, 86). No ship that is not a carrier is lost to fire.

The difference from the measured numbers has one cause found: the first raid of a kind at a force draws its ship as scenario 127's hand calculation has it, a draw d below one and the place floor(n x d), where the throwaway drew a whole number below n. The same seed therefore takes another ship in some pulses, and the games that follow differ. Nothing was moved. These are the scenarios' new numbers, in place of those recorded in decision 0198 and its like before these rules: Midway is inside the band of decision 0187 (neither side in more than 650); the Coral Sea is outside it by 29 games and the five days by 177, both toward the Allied side, as measured. The five days' balance is work of its own, by the designer's word above ("The designer's word, and the readings ruled").

Scenario 104's statement 19, a game of the five days' file a on one seed, was moved by both rules and is restated by the architect: without the objective the Japanese side has 2.67 and the Allied side 42.59, a margin of 39.92; with it 17.67 and 42.59, a margin of 24.92, a decisive victory each way.

### The architect's rulings, Nov 22, 2026

Read on pull request 675. Accepted as built:

- No cause field on `ShipSunk`: the fought record marked lost is its cause.
- The roll record does not name the raiding group, which is the enemy's.
- Aircraft still away from an earlier sortie are counted neither aboard nor flying.
- The deck is closed at once when a fire is set: her aircraft in the air look for another deck or a base.
- Scenario 126's figures are built in the test, from the frozen copy, with no committed figures file.
- `advance`'s "loss" stop takes in a fire set, and the Eastern Solomons expectation in `crates/sim-agent/tests/advance.rs` that follows from it.

Ruled and mended:

- **A burning carrier out of a task force:** the fire is fought at the end of every pulse while she is afloat, wherever she is, in a force, in port or at a yard. Setting a fire stays as built, a raid's on a carrier. Tested in `crates/sim-core/src/airstrike/tests.rs`; chapter 4 says so.
- **Scenario 126, "The other navy":** the last figure is restated as 0.208547, 1 - 0.925^3 = 0.208546875 held to six places, and asserted.
- **Scenario 104, statement 19:** restated as above.
