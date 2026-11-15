# 0114: Two settings of a strike group: how old a contact it launches on, and a share held back

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2, that the designer confirmed (decision 0113, decisions 11 and 12 and his answers to questions 2 and 3). It writes the two rules out with their rule cards. Not built.

Summary: Two standing settings of an air group on a strike mission, each defaulting to the present rule: the oldest contact it will launch a naval strike on, in pulses, where today one figure serves every group; and a share of its aircraft it holds back from a strike, where today every aircraft that can fly goes

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Not built

## Why

The staff of stage 2 sets a strike group's standing orders and the men on the spot act on them inside the day. The three risk levels of the spec differ first in the evidence a strike is launched on, and the engine has one figure for it, the same for every group of both sides (`air_combat.toml: strike_contact_max_age_pulses`). The record has commanders who waited for a fresh report and commanders who sent strikes on reports hours old (`docs/research/oob/launch-decisions.md`). And every aircraft of a group that can fly goes on every strike, where the record has attack aircraft held until more was known.

Both rules reuse what exists: the test of a contact's age, and the share a fighter group keeps for patrol. A game in which neither setting is given is the game it was.

## The oldest contact a group launches on

1. **A group has a standing setting,** the oldest contact it launches a naval strike on, in pulses. With none set it is the figure `strike_contact_max_age_pulses`, as today.
2. **It replaces the figure in the one test that reads it** (rulebook chapter 5, Naval strike: the contact): a contact qualifies if its age in pulses is at most the group's setting. Nothing else of the choice changes: the reach, the target list, and the order in which a qualifying contact is taken.
3. **It is set by an order,** `set_strike_contact_age`, with a whole number of pulses from 0 up to one less than `search.toml: contact_drop_age_pulses`, since an older contact has been dropped. An order outside that is refused and says the range. A scenario may give a group the setting at the start.
4. **It does not apply to a base strike,** which needs no contact.
5. **What an age means.** A contact is not aged in the pulse it is reported in. So at 0 a group launches on a contact reported in this pulse or the one before; at 1 a group launches in the morning on a contact reported the afternoon before; at 2 it launches in the morning on a contact of the morning before.
6. **The strike choice a side can ask** (`strike`) reads the group's setting, since it is the same function.

## A share held back

7. **A group has a standing setting,** the share of its aircraft it holds back from a strike, from 0 to 1. With none set it is 0, as today.
8. **A strike launches the aircraft the group is able to fly, times one less the share, cut to a whole number toward zero.** It is applied once, after the limits of "The order of the limits", to the whole number they give. If that comes to none, the group does not launch and nothing is recorded.
9. **It applies to a naval strike and to a base strike,** and to nothing else: not to search, patrol or escort. A fighter group's patrol share is as it is.
10. **The aircraft held do nothing in that pulse.** They are not counted as having flown: no sortie, wear, fatigue or supply for them. They are as able to fly in the next pulse as any other.
11. **It is set by an order,** `set_strike_held_share`, with a share from 0 to 1 as a decimal. A scenario may give a group the setting at the start.
12. **The launch event gives the aircraft that went,** as now, and its breakdown gives the aircraft able to fly, the share held and the aircraft sent.

## Who sees what

13. A side sees both settings of its own groups whole, and the orders that set them. The enemy sees neither, and learns nothing new: it sees a raid's aircraft as it does today.

## Rule cards

| Rule | Reads | Writes |
| --- | --- | --- |
| The oldest contact a group launches on | The group's setting, or the figure; the side's contacts and their ages; the reach and the target list | The strike launched, as now |
| A share held back | The group's setting; the aircraft the group is able to fly | The aircraft of the strike launched; the launch's breakdown |

## What it leaves out

- **A count of aircraft** in place of a share. A share holds as a group shrinks; a count needs a rule for a group smaller than it.
- **A second report.** Waiting until a contact has been reported twice is a judgement the staff makes with its standing orders: it leaves a group off the strike mission, or narrows its target list, until it is satisfied.
- **Telling a contact of this pulse from one of the pulse before.** A contact's age does not tell them apart, and nothing here does.

## What it costs

- **Two fields of an air group's state,** in the hash and named beside it. The state version rises. The rules label rises: a saved game may hold an order the old rules could not.
- **Two orders,** in scenario files, the agent's orders and their schemas; two scenario keys for a group's starting settings.
- **The rulebook:** chapter 5 (Air group orders, Naval strike: the contact, Base strike), chapter 1's tables of orders and refusals, and the glossary.
- **Every existing scenario is unchanged** apart from the hash version.

## The hand calculation

`tests/scenario_46_two_strike_settings.rs`: three groups at one base that differ only in the age they launch on, two contacts of different ages and kinds, and no enemy, so that each strike finds nothing and no die is rolled; and three groups at a second base that differ only in the share they hold back.
