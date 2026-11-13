# 0076: How long surprise lasts: no rule

Status: decided by the architect on the designer's delegation, Nov 13, 2026: no rule. This record adds no rule and changes none, so it has no rule card. It says what was looked at and why nothing is built.

## The question

The batch file's record of each fight (decision 0062, version 4) showed that in the rules a force is never surprised for more than one round: a force that is fired on has sighted the enemy by the next round. The record at Savo Island has seven minutes between the Japanese sighting the southern group and that group's first sight of them, and thirteen for the northern group, which is more than one round of five minutes in each case.

If surprise should last two or three rounds and the rules give one, the factor for firing at a surprised force would be asked, in tuning, to carry in one round what the record spread over several. That would be a figure covering for a missing rule, so it was looked at before any figure moved.

## The record

Rows are from `data/oob/engagement_events.csv` (events) and `data/oob/engagement_outcomes.csv` (outcomes). "(mine)" marks arithmetic across two rows; it is not a row.

**The long interval is before the first shot.** The northern group was sighted at 01:37 and first lit and fired on at 01:50 to 01:51: "illuminated Astoria bearing 033, range 7,800 yd, at 0150" (events 90, medium, preferred), "first main battery salvo about 0151, 500 yd short" (events 94, medium, preferred). The Japanese were busy with the southern group in between. A fight in the rules begins with its first round of fire, so that interval is outside it.

**Minutes from first being fired on to first reply, at Savo Island:**

| Ship | Minutes | Rows |
| --- | --- | --- |
| Canberra | about 4; her reply is hedged "may have" | events 59, 79 (mine) |
| Chicago | 0 to 4, star shell only | events 59, 77 (mine) |
| Patterson | 0 to 1 | events 74, 80 (mine) |
| Astoria | 1 to 3 | events 90, 94, 96 (mine) |
| Vincennes | 0 to 4; two rows disagree on her first main battery salvo | events 97, 1063, 1064, 1065 (mine) |
| Helm, Wilson | fired before any row has them fired on | events 95, 98, 99, 1057 |
| Quincy | about 8: first splash at 01:51, "She then at about 0159 fired her first main battery salvo" | events 1048, 1066 (medium, unconfirmed) (mine) |

Of Quincy the rows also say she "was delayed in firing her first salvo for about twelve minutes" (outcomes 3028) and that "approximately eight minutes had now elapsed since the sounding of 'General Quarters' and his ship had not as yet fired a single shot" (outcomes 3059), both from the Naval War College analysis, medium, unconfirmed. Both give the cause: her organisation under her condition of readiness.

**At Cape Esperance,** where the Japanese group was the one caught: American fire was "concentrated into a period of barely 7 minutes" before the Japanese replied (outcomes 1647, high, preferred); "for 7 minutes before his guns replied" (outcomes 2314); "six to seven minutes before the first Japanese ship opened fire with main battery and eight minutes before the first hit was obtained" (outcomes 2552). By ship, from the Japanese division's table (medium, unconfirmed, its clock converted by the researcher's inference): Furutaka's high-angle guns after 2 minutes and her main battery after 6 (events 727, 731); Aoba's torpedoes after 2 and her guns under local control after 4 (mine). The cause the rows give is not knowing who was firing and fear of hitting friends (outcomes 2315).

So, in the rules' own unit: a force under fire replied within one round in most ships, and after one round and part of a second in two cases, Quincy and the Japanese group at Cape Esperance. No row gives a general figure for how long a surprised force stays surprised. The note in `docs/research/oob/savo-island.md` that it took "two to five minutes from alarm to first salvo" has no row behind it.

## What the rules do now

`crates/sim-core/src/surface.rs`; `docs/rules/04`, "Sighting" and "Surprise".

- A force is surprised in a round if, after the sighting step, it has not sighted the enemy, the enemy has, and the enemy has shots to fire.
- A surprised force does not fire. Fire at it has `surprised_target_factor`, 1.5 today.
- Surprise ends in three ways: the force's own sighting roll succeeds; it was fired on in the round before, which gives it the sight with no roll; or the fight ends.
- Being fired on also makes a force at normal readiness alert (decision 0021), so any later roll of its own is at the full chance.
- Readiness (decision 0068) is fixed when the fight begins and none of this changes it: a force that was not ready fires the share of its guns in every round in which it fires.

So an unready force caught by surprise fires nothing for one round, and part of its guns for the rest of the fight.

## What was tried

Three ways of letting surprise last longer, each prototyped in a throwaway copy and run over 1,000 seeds of Savo Island on the figures of Nov 13, 2026 (before decision 0070), with the northern group's break-off level at 1.

- **(a)** A force that is not ready is not given the sight by being fired on. It must roll, at the full chance now that it is alert.
- **(b)** Being fired on gives the sight only with a chance, `alarm_chance` (0.5), for every force; otherwise the force rolls as usual.
- **(b2)** The same as (b), for a force that is not ready only.

Each cell is the 5th to the 95th percentile, then the median. Shells are in hundreds.

| | Today | (a) | (b) | (b2) |
| --- | --- | --- | --- | --- |
| Northern group surprised two rounds or more, of 994 fights | 0 | 143 | 70 | 71 |
| Southern group surprised two rounds or more | 0 | 0 | 0 | 0 |
| Japanese force surprised two rounds or more, of 1,994 fights | 0 | 0 | 64 | 0 |
| Allied cruisers sunk | 0 to 4, 1 | 0 to 4, 1 | 0 to 4, 1 | 0 to 4, 1 |
| Gun hits on Allied ships | 37 to 235, 171 | 37 to 238, 173 | 35 to 233, 170 | 37 to 236, 173 |
| Gun hits on Japanese ships | 2 to 160, 56 | 0 to 159, 54 | 1 to 173, 55 | 1 to 160, 55 |
| Japanese gun hits for each thousand shells | 33 to 67, 53 | 33 to 73, 54 | 32 to 69, 53 | 33 to 69, 54 |
| Allied gun hits for each thousand shells | 11 to 56, 37 | 14 to 57, 38 | 11 to 59, 38 | 11 to 57, 38 |
| Japanese shells | 10 to 41, 31 | 10 to 41, 30 | 9 to 40, 30 | 10 to 41, 31 |
| Allied shells | 1 to 33, 14 | 0 to 32, 13 | 1 to 33, 13 | 1 to 33, 13 |

Who sighted first, the minutes of action, destroyers sunk and torpedo hits both ways were the same in all four.

Little moves because a force that has been fired on is alert, so its own roll succeeds at 0.67 to 0.88 a round and a second round of surprise is uncommon; and the southern fight is over in a round or two whatever is done.

## Decision

1. **No rule is added.** Surprise ends as it does today.
2. **The record's surprise under fire is about one round, which the rules give.** The longer interval in the record is the time between being seen and being shot at, which is not part of a fight in the rules.
3. **A longer surprise for a force that is not ready would charge the same unreadiness twice.** The two cases that ran into a second round have causes the rows name. Quincy's is guns not manned under her condition of readiness, which decision 0068's share already carries for the whole fight. A rule that also kept her from firing for a second round would count it once as no fire and once as the share.
4. **For tuning:** `surprised_target_factor` carries one round of surprise under fire, as the record has it. It does not carry rates of fire (decision 0070) or readiness (decision 0068), and no source gives it a size, so it stays marked a placeholder however well it fits.

## The gaps, named

- **A force seen and stalked before the first shot.** The Japanese held the northern group in sight for thirteen minutes before firing on it. In the rules a force that sights fires in the same round.
- **Identification under fire.** At Cape Esperance the Japanese did not reply for seven minutes for not knowing who was firing. The rules have no ship taken for a friend.

If a later battle's measure shows a need, (a) is the least mechanism: it reads the fight's readiness, which exists, and needs no state, no figure and no new version of the state hash. It would change one sentence of `docs/rules/04`, the hand calculations of scenarios 4a and 15a, the pinned night fight of scenario 16, and every run baseline with a night fight.

## What follows

- The Savo Island measures of rounds surprised (`data/validation/savo_island.toml`, written by the spec session, landing with batch version 4) are not judged, and each says in its reason that the record's longer surprise is out of the rules' reach and that no figure is to be asked to supply it.
- The Japanese group's seven minutes without reply at Cape Esperance is a measure for that scenario in the rules' terms: rounds without fire from the side that was not ready.
