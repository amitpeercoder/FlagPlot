# 0039: Base supply consumption

Status: confirmed. The designer approved on Nov 11, 2026 that supply is spent for each sortie and each day, and that a base without supply neither flies nor repairs. Decisions 4, 6 and 8 are choices that answer did not state; the designer confirmed them on Nov 12, 2026.

## The gap

A base has a stock of supply, a convoy can add to it (scenario 2) and a raid can destroy some of it (decision 0034). Nothing uses it. The spec says base supply consumption arrives by scenario 6 at the latest, because an objective such as "30 days of supply" cannot be measured without it.

## Decisions

### What is spent

1. **Each aircraft that flies a sortie from a base costs the base supply:** a figure for the sortie plus a figure for each 100 miles of it, the distance counted as for wear and fatigue (decision 0035, 4). It is counted on the aircraft sent, whether or not they come back.
2. **Each aircraft at a base, ready or damaged, costs the base a figure a day,** at the start of the day.
3. **Each repair costs a figure** (decision 0035, 9). A base repairs an aircraft only if it has that much supply, so repair stops when the stock runs short.
4. **Replacements cost the base nothing** *(confirmed by the designer, Nov 12, 2026)*. They are flown or shipped in from outside. (Decision 0038 said a replacement would draw on base supply. It does not.)

### When

5. **Sorties are paid for after the air combat of the pulse they flew in,** all the base's sorties of the pulse in one sum. The daily cost and repairs are paid in the night, at the start of the day: repair first, then replacements, then the daily cost.
6. **A payment the stock cannot cover takes what is left** *(confirmed by the designer, Nov 12, 2026)*. The stock does not go below nothing and nothing is owed. So the last pulse before a base runs dry is flown in full.

### Out of supply

7. **A base with no supply at the start of a daylight pulse flies nothing in that pulse.** Its groups keep their missions. Their aircraft are on the ground.
8. **Nothing else stops** *(confirmed by the designer, Nov 12, 2026)*: the base's guns still fire, its aircraft are not lost, and a group there still rests. A base flies again in the first pulse that starts with supply in stock.
9. **Aircraft aboard ships spend nothing yet.** A carrier's supply is not modelled.
10. **The figures are in a file of their own, and a scenario without it spends nothing.** Scenarios 1 to 6d1 do not name the file and run as before.

## Why

- **The same shape a third time.** Wear, fatigue and supply each cost a sortie a base amount plus an amount for distance. A player who has learned one has learned all three, and a long raid is dear in all three.
- **All or nothing by the pulse.** Rationing a short stock among groups needs an order of priority, and someone to set it. A base that flies in full until it is empty, and then not at all, needs neither. The staff's job, later, is to see the empty day coming.
- **Bombing supply now matters.** A raid that burns 50 tons takes a day's flying from a small base. Before this stage it took nothing.

## What it costs

- A base never flies at half strength for want of fuel. It is in supply or out.
- The last pulse is free beyond what was in stock.
- Three files of upkeep figures: wear, fatigue and supply. They are one file's worth of numbers and can be merged when the whole slice runs with all of them (scenario 7).
- The garrison eats nothing. Only aircraft cost supply.

## Deferred

- Supply for ships, troops and anti-aircraft guns.
- Fuel and ammunition as separate stocks.
- Days of supply as a figure a side can read, and an objective that uses it.
- Priority among groups when supply is short.

## As built

- **Whether a base can fly is fixed at the start of the pulse.** A base bombed to nothing during a pulse still flies that pulse.
- **The daily cost counts the aircraft after the night's repairs and replacements.**
- **A base with nothing in stock is asked for nothing by the day,** and no event is recorded.

## What the runs show

Over seeds 1 to 1000, six days. All figures are placeholders. Airfield A starts with 150 t.

| Airfield A runs dry | Runs |
| --- | --- |
| In the morning of day 4 | 468 |
| In the afternoon of day 4 | 56 |
| On day 5 | 398 |
| On day 6 | 42 |
| Never | 36 |

It flies 4.41 raids a run on average, against 5.95 with 1,000 t and none with no supply. It spends 130 t on sorties and 20 t by the day. Airfield B spends 191 t on sorties and 27 t by the day and loses about 150 t to bombs. A base that loses more aircraft early lasts longer, because fewer aircraft fly and eat.

## The legibility test

A cheap model was given the Japanese briefing and the log of the first three days of the scenario's own seed. It rebuilt the first morning's 39 t from its three parts. It predicted the fourth day exactly: 4 t in the night leaving 15.6 t, a raid of 11 bombers with 5 escorts and a patrol of 4, sorties asking 26 t, the base paying the 15.6 t it had, and nothing flying from that afternoon on. Asked how to stretch the stock it dropped the escort or the afternoon patrol, and said what each gives up.

It found one gap. No event says a patrol flew: the reader has to infer the six fighters over the airfield from the sum. With the wear or fatigue figures loaded their events name the patrol. With supply alone nothing does. An event for every patrol in every pulse would double the log of a quiet day, so the gap is left and noted: the supply event's breakdown lists the sorties by kind, and the briefing a side gets must show it.

## Rule card

| | |
| --- | --- |
| Reads | The sorties flown from a base in the pulse and how far; the aircraft at a base; the repairs made; the supply figures; a base's supply |
| Writes | A base's supply; whether its groups can fly; whether it repairs |
| Events | Base supply spent, with what it was spent on and what is left, and saying so when the base is out of supply |
| Randomness | None |
