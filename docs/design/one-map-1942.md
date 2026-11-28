# Design note: 1942 on one map

Status: a note for discussion, written by the architect on Nov 28, 2026 at the designer's asking, from a conversation of Nov 27. Nothing here is built or planned, and the note takes no decision number. It says what the next larger scenario is, what of history it keeps, and in what order it could be reached. It takes the place of the proposal in `larger-scenarios.md` (Guadalcanal to the end of November as the next step), which becomes one stage of this. Historical statements below are general and unsourced; any date or figure the game uses must come from sourced rows, as everywhere. The research it calls for is in `prompts-1942.md` at the root.

The designer's words, in the order he gave them:

- "I want to start looking into bigger, longer scenarios. [...] I want something substantial. It could be partial, for example covering 1942, or have a theme around it."
- Of the carrier war of 1942 as a theme and the South Pacific from May to November as a span: "I'd like to combine 3 and 4. They seem to create interest and engagement."
- "I actually want Midway on one wider map. I want all these to be on the map (Papua, Midway, and the other islands related to this scenario). Once we play a grander scenario, we can only preserve thus much. For example, reinforcement schedule, that may push the commander to take certain places by a certain time. If the place is not taken, the reinforcement is not coming or is coming to an already-occupied base (we need to think what we want there)."
- "Since it is a planning game now, each time the user gets a directive, they also get a timeline. We could try to keep timelines like in history, for example, that they need to plan Midway ahead, so that it is taken at a certain (historical) date. If the user is quicker, we could push the dates."

## The idea in one line

One game from 1 May to 30 November 1942, about 214 days, on one map from Papua to Midway, in which each side is given what its country sent it and what it was told to do, each with its date, and decides everything else.

The carrier war is its theme without any rule for it. In one world the ships and aircrew persist, so a deck or an air group lost in May is one the side does not have in October. No mechanism carries forces from one battle to the next, because there is one game and not a series.

## What exists to build on

Checked against the index of decisions and the rulebook on Nov 27, 2026.

| Needed | Exists | Standing |
| --- | --- | --- |
| A map that says where it is, and a second sea | Decision 0199: each map has a frame; the Midway map crosses the date line | Built. Two maps: the south-west Pacific (about 1,600 hexes) and the sea about Midway |
| Forces that come onto the map on a day | Decision 0195: a task force with what it carries, a land unit or an air group at a base | Built. The day is fixed by the scenario; nothing makes it depend on what has happened |
| A rear base fed from off the map | Decision 0196: so many tons a day, from a day to a day, while the base is its side's | Built, the simple form |
| Men made good | Decision 0197: a stream of replacement men at a base | Built |
| Fuel at sea | Decision 0201, and estimated figures for real oilers by decision 0207 | Built; real oilers are derived for the Coral Sea and the five days only |
| Submarines against warships | Scenarios 128 and 129 | Built |
| The battles of the span as games | The Coral Sea, Midway and the Eastern Solomons for play; the 39 days | Built, each on its own map and its own days |
| A mission in a superior's words | Decision 0157: a directive, replaced by a new one when the war changes | Built as text the player reads. Nothing in the engine holds a date or judges it |
| A mission judged once, by its date | `levels.md`, and decision 0237 for its first three rungs | A note; three rungs planned |
| What waiting buys | Decision 0177: the window tries the same course at two or three dates | Built |
| A plan the commander writes | Decision 0232 | Steps 1 and 2 built |
| An enemy that chooses its own operations | Decision 0161, step 10, the enemy's approaches | Not built |
| Facts | Order of battle for the Coral Sea, Midway, and Guadalcanal from August to November 1942 | On file. Nothing for Papua ashore, for the Central Pacific's bases, or for where every force stood on 1 May |

## The map

**What is on it.** In the west Lae, Port Moresby, north-east Australia, Truk, Saipan and Guam; in the east the Hawaiian Islands; in the north Midway and Wake; in the south Brisbane, Noumea, Fiji, Samoa and Tongatabu. Between them the Bismarcks, the Solomons, the New Hebrides, the Marshalls, the Gilberts and the line islands. From 30 S to 38 N and from 142 E to 153 W (decision 0242, which plans the map).

**What is off it.** Japan, the Aleutians, the American west coast, Sydney and Auckland, the Indies. A force from any of them comes in at an edge by decision 0195, and a force sent to one leaves the map. The Aleutians matter as a drain: what went north in June is not on the map in June.

**Its size.** At today's 40 nautical miles a hex, about 10,400 hexes against 1,600 now (worked for decision 0242). Nearly all of it is open sea. The hex stays 40 miles: every range in the rules is counted in hexes of that size.

**Its scheme is a decision of its own: decision 0242.** Decision 0017 held the error of its flat projection to 5 in 100 across the slice, and decision 0199 gave each map one factor for longitude. Measured for decision 0242, the best single factor for this map leaves the carrier battle off Midway 8 in 100 out east to west and the Solomons 7. A projection about the map's own centre holds every named place within 3.6 in 100, and is what that record plans; its price is a grid whose north is a few degrees off the compass away from the centre. The engine reads nothing of where a map is, so this is the map tool's and the table's work and no rule's.

**The present maps and their games stay as they are.** A frozen scenario, a closed proof and a validation measure are tied to the map they were made on. The wide map is a third map, and nothing that exists is moved onto it.

**What it buys.** Distance becomes a decision. Pearl Harbor to Noumea is more than a week's steaming, so carriers sent south are not at Midway. A series of battles cannot ask that question, and it is the question of the carrier war.

**What it exposes.**

- Oilers. Over these distances a force without one goes nowhere. Real oilers are derived for two scenarios, and the Japanese ones rest on estimates.
- The cost of a wide sea in a dry run: a path over 10,400 hexes, and a trial's many worlds. To be measured with `sim-cli time` before anything is changed for it.
- Empty weeks. Most days in most of the map nothing happens. The table has to pass them quickly.

## What is kept from history, and what is let go

The designer's point is that a grander game can only keep so much. The rule proposed: **history is kept as what a commander could not change, and let go as what he decided.**

| Kept | Let go |
| --- | --- |
| Where every force stood on the first morning | Where any of them goes after it |
| What arrives in the theatre and when: ships completed, repaired or released from elsewhere, aircraft, pilots, men, supply | Which place it is sent on to |
| What leaves the theatre and when, for a war elsewhere | When an operation starts, inside its directive |
| What each side was told to achieve, and by when | Which places are taken, and in what order |
| What each side believed on the first morning | What it learns after |

A battle of the record is then something to compare a game with, never something the game is made to reach. That is already the rule for validation.

## Reinforcements

**They arrive at the rear, not at the front.** A reinforcement comes onto the map at an edge or at a main base of its side: Pearl Harbor, Truk, Noumea, Rabaul, Brisbane. Moving it forward is the commander's operation, as it was. The 7th Marines reaching Guadalcanal in September was a convoy somebody ordered, not an arrival. This removes most of the question of a reinforcement bound for a place that was never taken: almost nothing is bound for such a place.

**Three cases are left, and each has a proposed answer.**

1. **Its base is the enemy's.** Never delivered there. The supply line of decision 0196 already stops when a base changes hands; an arrival follows the same rule.
2. **By default it waits.** A force whose base is lost, or whose forward base was never taken, comes to the nearest main base its side holds. The side has it, and not where it wanted it. Air groups make this plain without a rule: a group needs a field, and a field not taken is a group left at the rear.
3. **Where pressure is wanted, a release with a window.** The command above offers a force for a stated purpose by a date, and the directive says so in words. If the condition is not met by the date, the force goes to another theatre and does not come. This is what gives a date its weight. History did the like: the defeat at Midway ended the Japanese plan against Fiji, Samoa and New Caledonia, and the landings in North Africa took shipping the South Pacific had asked for.

**A release is design, not fact.** That a force arrived on a day is a fact with a row. That it would not have come had a place not been taken is a judgement. Each release is written with its reason and what in the record it rests on, as decision 0187 asks of anything set to make a contest, and is never entered in `data/oob` as a fact.

**What it needs built.** An arrival that depends on the state of the game: who holds a place on a day. Decision 0195's arrivals are by day only. It is a small rule, and a rule: a record with a rule card.

## Directives and their timelines

**Every directive carries a date.** It says what is to be achieved, why, by when, and what is released for it. The date turns a mission into a plan worked backwards: to land on a day the transports sail on an earlier one, so the cover fuels the day before that. The staff can work the chain out and show it; the commander decides what to cut when it does not fit. That is the planning the game is now about (`levels.md`).

**Two clocks, and only one may move.**

- **The world's clock never moves.** A carrier in dock is ready when it is ready. Ships, aircraft, pilots and men arrive on their days. Being quick brings none of them sooner.
- **The directive's clock can.** When a directive closes, met or failed or run out, the next is issued, early or on its day. Its date is counted from the day of issue by the span the record allowed, and is never earlier than what the forces named in it need to be there.

**So being quick rewards itself, and costs.** A side that closes a directive early moves on while the enemy's reinforcements are weeks off, and with less of its own. No bonus is needed and none is given. The window of decision 0177 already puts the choice: the same course at two or three dates, and what each side has more of at the later one.

**How a date behaves.**

| Case | What happens |
| --- | --- |
| Met early | The next directive is offered. He may begin at once or wait for what is coming |
| Met on the day | The next directive is issued on its day |
| Not met | Judged once, as a level is, and the game goes on. A release tied to it is withdrawn, the enemy's own directive runs on, and the next directive is a different one |
| He asks for more time | Perhaps, once, at a price. The record has slips of this kind, to be confirmed from sources before any is used |

**Who holds the date.** In May and June 1942 the dated directives are Japan's: take a place by a day. The Allied ones are to hold, and their timeline comes from intelligence: the enemy is expected about a day, so be in place before it. From August it turns about. A game for the Allied seat therefore begins as a game of being ready in time, which is still a plan worked backwards from a date, and the date is a belief that may be wrong.

**The dates are facts.** The day an operation was ordered, the day it was set for, each postponement and its reason are in the record, and each needs a row. So do the plans that were dropped, since they are the branches.

**What it needs built.** A directive the engine holds: an end, a date, what is released with it, what follows it met and not met. Today a directive is text. This is the same thing a level needs (`levels.md`: a mission from above, judged once), and it should be built once for both. A campaign is then a chain of levels in one world.

## The enemy

A free player means the enemy cannot follow a script either. If Port Moresby falls in May, the days on which the record's forces sailed for Guadalcanal mean nothing.

The proposal is that the enemy is given exactly what the player is: a chain of dated directives that branches on results, and the releases that go with them. For Japan it runs, in outline, Port Moresby, then Midway, then Fiji and Samoa if the carriers are whole, and the defence of what is held if they are not. His staff works to his dates and slips when it is not ready. History is then kept for him as intent and means, as it is for the player.

It asks more of the staff than it does today. It can form an operation for an end it is given. It does not choose among operations, and the enemy's approaches (decision 0161, step 10) are not built. A staff that takes each directive of a chain in turn is a smaller thing than a staff that plans a war, and may be enough.

## Intelligence

Midway was an ambush because one side read the other's signals. On a wide map with a free enemy nothing puts the American carriers north of Midway on the right morning, and without a rule the battle of the record is the least likely of outcomes.

`intelligence.md` has the direction: a standing source that tells a side, late and in part, what the enemy means to do. For this scenario its first form can be small: a directive to the Allied seat that says what is believed of the enemy's next operation, its place and its day, with the doubt stated. It reads the enemy's directive and nothing of where his ships are, and what it gives is a belief. Whether it is ever wrong, and how often, is for the designer.

## What it needs that is not built

In the order proposed. Each is a record of its own.

| Need | Kind | Note |
| --- | --- | --- |
| The wide map and its scheme | A plan with a review; the map tool and the table | First. Nothing else can be tried without it |
| The position on 1 May 1942 | Data | Where every force stood, both sides, across the map |
| Places and bases of the Central Pacific and the islands between | Data | Positions, fields, ports, garrisons |
| Real oilers for both sides, and fuel at the main bases | Data, with estimates under decision 0207 | The map is too wide to play without them |
| A directive the engine holds, with a date and what follows it | A rule | Shared with the levels |
| An arrival that depends on who holds a place | A rule | Small |
| The enemy's chain of directives | The staff | The largest unknown |
| The first form of intelligence | A rule | Small in its first form |
| The history read once, not every day | A mend, no rule | Before any game much past the 39 days (`docs/open-work.md`) |
| Papua ashore: the facts, and terrain in the fight ashore | Data and a rule | Not needed before July of the game |
| Experience for air groups | A rule that waits on research (decision 0064) | The theme leans on it: trained aircrew lost and not made good |
| Quiet days passed quickly at the table | The table | Before a person plays 214 days |

Weather stays deferred, as decision 0102 ruled, until a game shows what it is missing.

## Stages

Each stage is a game that can be played, and none is thrown away by the next.

1. **The map.** Its scheme decided and measured, its coast and places drawn, the table showing it. No scenario.
2. **Two seas, six weeks: 1 May to about 10 June 1942.** The Coral Sea and Midway in one game, both sides on directives with dates. It tries distance, oilers, a directive the engine holds, a release, the enemy's chain and the first intelligence. It needs no fight ashore in Papua, and its two battles have facts on file. About 40 days, so the history need not be mended first.
3. **To the end of November.** The same game carried on: Papua ashore from July, Guadalcanal from August, the autumn battles. It needs the history mended, Papua's facts and terrain, and what stage 2 showed was missing. Guadalcanal to November, the earlier proposal, is the last part of this stage and its facts are already in.

A scenario of record comes before a scenario for play at each stage, as the Coral Sea plan did.

## What this does not settle

- **Whether a directive's date can be wrong for the player.** The record's dates were set by men who did not know what the player's game holds. A date carried over to a game that has gone another way may be impossible or idle. The proposal counts dates from the day of issue and bounds them by what the forces need; whether that is enough is found by playing stage 2.
- **How far a game may leave the record before the releases stop making sense.** A side that has lost every carrier by June is still sent what history sent. That is right for ships that were building, and doubtful for forces that were sent because of how the war was going.
- **The Japanese seat.** Everything above is written for either seat. Whether both are offered, or the Allied one first, is the designer's.
- **How long a person will play.** The 39 days is already hours at the table. Stage 2 is the same length and stage 3 five times it.
- **What an agent costs at this size.** No time or cost of an agent playing the 39 days is on record. It is measured before a longer game is offered to one.
- **When.** Stage 4's proof, step 3 of decision 0232 and the first rungs of the levels come first by the standing order of work. The research can run beside them, since every stage needs it and none of it waits on a ruling. The map's plan is the first piece of building and is for the designer to place.

## Open questions for the designer

1. Is a release with a window wanted from stage 2 on, or does stage 2 use only the plain rule (a force waits at the rear) to see first how a date feels without it?
2. Can a commander ask for more time, and what does it cost him?
3. Is intelligence ever wrong about the enemy's day and place?
4. Which seat is built first for play?
5. Is 1 May the right first morning, or should the game open earlier, with the carrier raids of the spring and the Japanese carriers still in the Indian Ocean?
