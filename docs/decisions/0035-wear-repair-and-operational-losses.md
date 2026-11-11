# 0035: Wear, repair and operational losses

Status: confirmed by the architect, within the plan for scenario 6 the designer approved on Nov 11, 2026. Decisions 2, 9 and 12 are choices the plan did not state and are marked for review.

## The gap

An air group has ready aircraft and nothing else. An aircraft is either fit to fly or shot down. Flying costs nothing, so a group can fly every pulse for ever, and the distance to the target matters only to whether it can be reached. Scenario 6 has to show a force worn down by four weeks of flying 565 miles each way.

Stage 6b adds aircraft that come back unfit to fly, their repair, and aircraft lost without the enemy's help. Fatigue, pilots and the supply a repair uses come in stages 6c and 6d.

## Decisions

### Damaged aircraft

1. **An air group has damaged aircraft as well as ready ones.** A damaged aircraft does not fly. Everything that counts the aircraft a group can fly counts the ready ones, as before.
2. **Fighters' and guns' hits still shoot an aircraft down** *(needs review)*. Aircraft damaged in combat that get home are not modelled apart: they are part of the wear below.

### The wear roll

3. **After the air combat of a daylight pulse, every group that flew rolls for wear.** A group flew if it searched, struck, escorted or kept a patrol in that pulse. The aircraft that roll are those that flew and came back.
4. **The chance an aircraft is worn is a base chance plus a chance for each 100 miles of the sortie.** The sortie's distance is twice the distance to the hex a strike or its escort was sent to, twice the range of a search arc, and nothing for a patrol.
5. **Each aircraft is rolled on its own** (decision 0033). The worn aircraft leave the ready ones.
6. **A share of the worn aircraft are written off:** each worn aircraft is rolled again at the write-off share. One written off is lost for good: this is the operational loss. The rest are damaged.
7. **A fighter group on combat air patrol rolls twice,** once for the patrol it kept and once for the escorts it sent, because they flew different distances. A patrol flies in every daylight pulse, raid or not; escorts fly only when they are sent.
8. **The figures are in a file of their own, and a scenario without it has no wear.** Scenarios 1 to 5 do not name the file and run as before.

### Repair

9. **A base repairs a number of damaged aircraft each day,** given for the base, at the start of the day *(needs review)*. It repairs the lowest-numbered group's first. A repaired aircraft is ready. A carrier repairs a number given once for all carriers, for the groups aboard it.
10. **Repair costs nothing yet.** The supply it uses comes with base supply consumption in stage 6d.

### On the ground

11. **Damaged aircraft are on the ground,** whatever their group is doing, and a bomb hit on aircraft can destroy them (decision 0034, 8). A group's damaged aircraft are destroyed before its ready ones.
12. **Aircraft worn in a pulse are not on the ground in that pulse** *(needs review)*: the hits of a raid are worked out on the state at the start of the air combat step, and wear comes after it.

## Why

- **One roll for range.** Distance enters once, in the chance an aircraft is worn. Operational losses rise with distance because they are a share of the worn. A second distance rule for losses would be a second thing to explain for the same effect.
- **Wear, not combat damage.** A hit that damages and does not kill would change every air fight of scenario 5 and add a draw to each hit. What the campaign needs is a count of aircraft that cannot fly tomorrow, and wear gives it.
- **Repair by the day and by the base.** The number a base can repair is what stood behind an airfield: mechanics, parts, shelter. It is the one figure that lets a well-found base outlast a poor one, and stage 6d will make it cost supply.
- **Why the patrol is not free.** If keeping fighters in the air cost nothing, the answer to every raid would be to fly everything all day. A small base chance of wear is the first cost of that; fatigue will be the second.

## What it costs

- A raid's survivors are never shot up: an aircraft is whole or gone until the wear roll. Bombers that fought through a patrol are worn no more than bombers that met nobody.
- The base chance and the chance for distance are two figures to find sources for.
- Scenario 5 has no wear. A carrier battle is a day or two, so it changes little, but the two scenarios do not yet run on the same rules. Scenario 7 runs the whole slice with the file.

## Deferred

- Aircraft damaged in combat, and damaged aircraft that fail to get home.
- The pilot of an aircraft written off (stage 6d).
- Supply and time for a repair by type of aircraft.
- Small airfields that wear aircraft faster.

## As built

Three things the decisions above did not state, settled when the stage was built.

- **A ship that sinks takes its damaged aircraft with it,** and a carrier's capacity counts ready and damaged aircraft together.
- **A group that loses damaged and ready aircraft on the ground in one raid has two events,** the damaged first. The event of stage 6a is unchanged.
- **A repair names as its cause the group's latest wear roll that left an aircraft damaged.** A patrol's wear roll names the group's mission and share orders, since no event says a patrol flew.

## What the runs show

Over seeds 1 to 1000, five days, an aircraft a run on average. All figures are placeholders.

| Group | Starts with | Worn | Written off | Repaired | Ready at the end | Damaged at the end |
| --- | --- | --- | --- | --- | --- | --- |
| 1, bombers raiding at 200 nm | 18 | 6.34 | 1.28 | 4.38 | 4.63 | 0.67 |
| 2, their fighters | 12 | 3.68 | 0.72 | 1.94 | 4.01 | 1.01 |
| 3, the defending fighters | 12 | 3.50 | 0.75 | 2.33 | 4.48 | 0.40 |
| 4, resting on the ground | 10 | 0 | 0 | 0 | 7.01 | 0 |
| 5, scouts searching to 300 nm twice a day | 6 | 8.08 | 1.55 | 5.22 | 3.13 | 1.31 |

The scouts, which nobody shoots at, end with half their aircraft ready: they fly the longest sortie twice a day and stand last in the repair queue. Without repair at airfield B the defending fighters end with 2.75 ready, not 4.48.

## The legibility test

A cheap model was given the Allied briefing and the log of the first three days of the scenario's own seed. It accounted for every scout (seven worn, one written off, three repaired, two ready and three damaged) and said why they shrink faster than the fighters. It predicted the fourth night and morning exactly: one fighter and two scouts repaired, with the third scout left because the base's three were used up in group order; four scouts searching; nine enemy bombers launched. Asked how to keep more scouts ready it offered a shorter search arc and searching once a day, with what each costs. It found no gap in the log.

Searching in one pulse only is not an order the game has. It is noted for the staff rules.

## Rule card

| | |
| --- | --- |
| Reads | What each group flew in the pulse and how far; ready and damaged aircraft; the wear figures; each base's repairs a day; where a group is |
| Writes | Ready and damaged aircraft of a group |
| Events | Aircraft worn (flown, distance, chance, expected, worn, written off); aircraft repaired |
| Randomness | Upkeep subsystem, keyed on the air group: one draw for each aircraft that flew, then one for each worn aircraft. A group that rolls twice rolls its patrol first |
