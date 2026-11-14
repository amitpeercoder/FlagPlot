# 0036: Fatigue and rest

Status: confirmed. The designer chose on Nov 11, 2026 that fatigue cuts the number of ready aircraft that fly and does nothing else, and that the player sets one threshold for a group and the staff rests it. Decisions 4, 5 and 9 are choices that answer did not state; the designer confirmed them on Nov 12, 2026.

## The gap

Stage 6b made flying cost aircraft. It still costs the people nothing: a group flies as hard on the twentieth day as on the first. Nothing gives the player a reason to hold a group back, and nothing wears a force that is never hit.

## Decisions

### The number

1. **An air group has a fatigue from 0 to 100.** It starts at 0 unless the scenario says otherwise.
2. **Fatigue cuts the aircraft that fly, and does nothing else.** A group flies its ready aircraft times (100 less its fatigue) over 100, rounded down. On a carrier the flight deck's factor multiplies it too, and the product is rounded down once. Everything a group flies is cut this way: search, strike, escort and patrol. Fatigue changes no chance to hit and no chance of wear.
3. **The figures are in a file of their own, and a scenario without it has no fatigue.** Scenarios 1 to 6b do not name the file and run as before.

### Rising and falling

4. **After the air combat of a daylight pulse, a group that flew gains fatigue** *(confirmed by the designer, Nov 12, 2026)*: a figure for the sortie plus a figure for each 100 miles of it, the distance counted as for wear (decision 0035, 4). A fighter group that kept a patrol and sent escorts gains for the longer of the two, once.
5. **A group that did not fly in a daylight pulse loses a rest figure** *(confirmed by the designer, Nov 12, 2026)*, and not below 0. Nothing changes at night. So a group that flies one long sortie in the morning gets part of it back in the afternoon, and a group that flies both pulses gets nothing back.
6. **Fatigue gained in the morning cuts what flies in the afternoon.**

### Rest

7. **A group can be given a rest threshold.** The order names the group and a fatigue.
8. **At the start of each day the staff stands down every group whose fatigue is above its threshold,** and returns to flying every stood-down group whose fatigue is not. A group with no threshold is never stood down.
9. **A stood-down group flies nothing that day** *(confirmed by the designer, Nov 12, 2026)*: no search, strike, escort or patrol. It keeps its mission and takes it up again when it returns. It loses the rest figure in each daylight pulse, as any group that did not fly.
10. **One threshold, no second figure for coming back.** A group that rests goes back to flying on the first morning its fatigue is at or under the threshold. With the rest a day gives, that is a rhythm of days flown and days rested, and the threshold sets how hard the group is flown.

### On the ground

11. **The aircraft on the ground at a base are the ready aircraft that are not flying in the pulse, and the damaged ones.** This restates decision 0034, 8, which listed missions. The aircraft a tired group leaves behind, the half of a fighter group that neither patrols nor escorts, and a stood-down group are all on the ground. No result of scenarios 6a and 6b changes: every group at the raided base there flies all its ready aircraft or none.
12. **A group's damaged aircraft are still destroyed before its ready ones,** as decision 0035, 11. The lowest-numbered group still comes first.

## Why

- **One effect.** Fewer aircraft in the air is something the player sees in every event: 12 launched where 18 are ready. A penalty to hitting would be a second effect, hidden inside a chance.
- **No memory of yesterday.** Rising and falling pulse by pulse needs no record of what a group did the day before. The state is the one number.
- **Rest is the staff's work.** In War in the Pacific the player watched each group's fatigue and stood it down by hand (manual sections 7.1 and 7.2.1.7). Here the player says how hard a group may be flown and the staff does the rest. An event says when a group is stood down and when it returns, and names the order.
- **Why a tired group still flies.** A hard stop at a level would make fatigue invisible until it bit. A cut that grows shows the cost each day and leaves the choice of when to rest to the threshold.

## What it costs

- A group at fatigue 5 already leaves an aircraft in twenty behind. With small groups the rounding bites early: 6 scouts at fatigue 1 fly 5.
- The whole group tires together, however few of its aircraft flew.
- A group cannot be told to fly in the morning only. The legibility test of stage 6b asked for it. It belongs with the staff rules.

## Deferred

- Fatigue for a task force held at battle readiness (the next stage, with the same number and the same file). *(Nov 14, 2026: built by decision 0037)*
- Pilots as people, and experience.
- Rest that is better at a well-found base.
- Weather and climate.

## As built

- **Without the fatigue file nothing of this applies.** A starting fatigue is ignored and a rest order is accepted and stored but does nothing.
- **A group flew if it sent an aircraft,** whether or not any came back.
- **An event is recorded only when a fatigue changes.** A group at 0 that rests records nothing.
- **A search group with no arc does not fly** and is on the ground.

## What the runs show

Over seeds 1 to 1000, eight days. All figures are placeholders.

| Day | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Bombers launched, on average | 18.00 | 14.05 | 10.78 | 8.33 | 5.81 | 4.47 | none | 3.66 |
| Escort | yes | yes | yes | none | yes | none | | none |

The raid shrinks from 18 to under 4 in a week: losses in the air, and a few aircraft more left behind each day as fatigue rises. A run destroys 3.03 aircraft on the ground on average, more than half of them fighters the tired defending group left behind, and 154 t of supply. The day-by-day tables of the hand calculation hold in 852 runs for the bombers, 848 for the escort group, 929 for the defending fighters and all 1,000 for the scouts; in the others a group ran out of aircraft to fly and the test works the rule.

## The legibility test

A cheap model was given the Japanese briefing and the log of the first three days of the scenario's own seed. It explained from the log's numbers why 15 bombers and 5 escorts flew on the second day where 16 and 11 were ready. It predicted the fourth day exactly: the escort group stood down at 42 against a threshold of 40, nine bombers launched without escort, the enemy patrol's four shots all at the bombers, and both groups' fatigue after each pulse. Asked for a change of orders it raised the threshold to 45, worked out the three escorts that would fly, and saw the cost: the group would be stood down the next day instead. It found no gap in the log.

## Rule card

| | |
| --- | --- |
| Reads | A group's fatigue, ready aircraft and rest threshold; what it flew in the pulse and how far; the fatigue figures; the flight deck |
| Writes | A group's fatigue; whether it is stood down |
| Events | Fatigue gained or rested, for each group whose fatigue changes in a daylight pulse; group stood down; group returned to flying |
| Randomness | None |
