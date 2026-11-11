# 0038: Pilots and replacements

Status: confirmed. The designer chose on Nov 11, 2026 that scenario 6 counts pilots only, that replacements fill seats at the same quality, and that replacements arrive at a set rate. Decisions 4, 8 and 10 are choices that answer did not state and are marked for review.

## The gap

An air group is a number of aircraft. A side that loses aircraft over its own field and one that loses them 500 miles from home lose the same thing. Nothing arrives to replace a loss, so every scenario longer than a few days ends with empty groups.

Base supply, which a replacement will draw on, is the next stage.

## Decisions

### Pilots

1. **An air group may have a number of pilots.** One pilot stands for the crew of one aircraft, whatever its size. A scenario that gives a group no pilots does not track them for that group, and the group flies as before.
2. **A group flies no more aircraft than it has pilots.** The aircraft it can fly are the smaller of its ready aircraft and its pilots, and fatigue and the flight deck then cut that number as they cut ready aircraft now.
3. **A pilot is lost with an aircraft shot down away from its group's own hex.** That is every bomber, scout or escort brought down by fighters or guns over the enemy.
4. **A pilot is saved when his aircraft is lost at home** *(needs review)*: a patrol aircraft shot down over its own hex, an aircraft written off after a sortie (decision 0035), and an aircraft destroyed on the ground. The group keeps the pilot and has one aircraft fewer.
5. **A group lost with its ship loses its pilots.**

### Replacements

6. **A scenario may give a side replacements at a base:** for one aircraft type, a number of aircraft and a number of pilots a day.
7. **They arrive at the start of each day, after repair.** Aircraft go to the groups at that base that fly the type, the lowest-numbered first, until each has as many aircraft, ready and damaged together, as it started the scenario with. Pilots go the same way, until each group has as many pilots as it started with. An aircraft arrives ready.
8. **What a day's replacements cannot place is lost** *(needs review)*. Nothing is held over to the next day. A pool that fills up while a group is at strength is a second stock to keep and explain, and in four weeks of raids no group stays at strength.
9. **Replacement pilots are as good as the ones they replace.** Experience is not modelled.
10. **A group's strength is what it started with** *(needs review)*, unless the scenario gives another figure. It is the ceiling for replacements and nothing else.

## Why

- **Where an aircraft falls decides who can keep fighting.** The defender over his own field gets most of his pilots back and needs only aircraft. The raider needs both, and pilots come more slowly. That one rule gives the attacker's side of a long air campaign its real cost without any figure for it.
- **Certain, not rolled.** A share of pilots saved at home and a share lost would be nearer the truth and would add two figures and a roll to every loss. The rule here can be read off the event: where was the aircraft lost.
- **Rates in the scenario, not in the rules.** How many aircraft and pilots reached Rabaul or Henderson Field in a week is history, to be taken from sources for each side. The rule only says where they go.

## What it costs

- No pilot is ever lost over his own field, and none is ever saved over the enemy's. Both happened.
- A bomber's crew of seven and a fighter's pilot count the same.
- Replacements cannot be saved up or sent to another base.

## Deferred

- Experience, training and the quality of replacements.
- A pool that holds replacements until they are wanted.
- Moving a group to another base.
- Pilots wounded, and pilots rescued at sea.

## As built

- **Nothing arrives on day 1,** whatever a group's strength.
- **Every patrol loss saves its pilot,** a carrier's patrol at sea too. Rescue at sea is not modelled, so this is generous to carriers.
- **A group whose pilots are not tracked takes aircraft and no pilots;** the day's pilots pass to the next group.
- **A replacement names no cause.** Nothing orders it.

## What the runs show

Over seeds 1 to 1000, six days, at the end of each day on average. All figures are placeholders.

| Day | Bombers launched | Japanese bombers, aircraft and pilots | Japanese fighters, aircraft and pilots | Allied fighters, aircraft and pilots |
| --- | --- | --- | --- | --- |
| 1 | 18.00 | 15.25 and 15.25 | 10.25 and 10.25 | 10.22 and 12.00 |
| 2 | 16.21 | 14.12 and 13.32 | 8.63 and 8.63 | 10.03 and 12.00 |
| 3 | 14.31 | 12.90 and 11.21 | 7.31 and 7.31 | 10.18 and 12.00 |
| 4 | 12.21 | 11.65 and 9.01 | 6.14 and 6.14 | 10.43 and 12.00 |
| 5 | 10.01 | 10.11 and 6.49 | 5.17 and 5.17 | 10.68 and 12.00 |
| 6 | 7.49 | 8.77 and 4.16 | 4.30 and 4.30 | 11.00 and 12.00 |

The bombers end with twice as many aircraft as pilots: the group received 9.55 aircraft and 4.94 pilots a run and lost one of each with every bomber shot down. The Japanese fighters, with no replacements, fall to a third. The Allied fighters, shot down over their own field, keep all twelve pilots and are back near strength by the sixth day on two aircraft a day.

## The legibility test

A cheap model was given a briefing for both sides and the log of the first two days of the scenario's own seed. It said which groups lost pilots and which lost aircraft only, and why. It saw that pilots, not aircraft, held the second raid to 17 bombers. It predicted the third night and morning exactly: one bomber and one pilot to group 1 with the second bomber lost against the ceiling, two fighters and one bomber to the Allies, 17 bombers launched, 5 escorts, a patrol of 11. It judged the Allied fighters the better placed over two weeks, and named fighter pilots as what the Japanese most lack. It found no gap in the log.

## Rule card

| | |
| --- | --- |
| Reads | A group's pilots, ready and damaged aircraft and starting strength; where an aircraft was shot down; the scenario's replacements; where a group is |
| Writes | A group's pilots; its ready aircraft |
| Events | Pilots lost, with the loss that caused it; replacements received |
| Randomness | None |
