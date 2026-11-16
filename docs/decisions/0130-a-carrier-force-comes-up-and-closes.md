# 0130: A carrier force comes up at speed and closes the range

Status: settled by the architect on Nov 16, 2026, on the designer's choice of the same day after stage 2's second proof did not pass (decision 0128, "As run"; `docs/validation/stage-2-second-proof.md`): the carrier doctrine is given what it left out, and the proof is run again with its pass marks unchanged. It adds two rules to decision 0125's doctrine, and no rule of the engine. Built on Nov 16, 2026 ("As built" below).

Summary: Two additions to the doctrine of a carrier force played from an end: it steams to its hex at full speed where cruising speed would not bring it there in the night, and on a morning it holds a fresh contact its bombers want and cannot reach, it steams to the nearest hex from which they can, by its risk level, and goes back to its hex when there is nothing to close on

Standing: Settled by the architect, Nov 16, 2026, on the designer's choice after the second proof. Built

## Why

The second proof played the Eastern Solomons from ends a thousand times. The Allied carriers never struck, and the Japanese carriers struck on 152 seeds, against the 800 the plan asked of each. The report gives the cause from the staff's own reasons. A carrier force is sent to its hex at cruising speed, so the Japanese carriers came up late or not at all. And a carrier force does not move toward an enemy it holds beyond its bombers' reach: the American carriers sat with their bombers on the strike mission and nothing in reach. Decision 0125 left both out and said so.

A carrier that will not come up and will not close does not fight. Both are the judgement of the man who commands the force, which is where this record puts them.

What this record cannot mend: orders are taken once a day, so a force closes overnight on what it knew in the morning. On a contact first made in the morning it does not move until the next day. A battle of one day is decided by where the forces were sent, not by closing.

## The two rules, at the force's level

1. **Speed.** Wherever decision 0125's doctrine routes the force (to its hex, to a hex it closes to, or home), the speed is cruising speed where that brings it there within the 12 hours of the coming night, and otherwise full speed. At preserve the force it is always cruising speed. The speed is set by order only where it differs from what the force has. The fuel the force needs (decision 0125, 8) is measured at the speed chosen.
2. **A contact to close on.** On a morning the force is not held, not on its way home for fuel and has a bomber group that can fly: a contact of ships the side holds, no more than 3 pulses old, of a kind its risk closes on, that is beyond the normal reach of the force's bombers from where the force is, and within a night at full speed and that reach of it. Where there are several, the kind first in the order of importance, then the nearest, then the lowest number.

| | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| It closes on | Nothing | A carrier | Any kind |

3. **Where it goes.** The first hex, along the shortest sea route from the force to the contact's hex, that is within the normal reach of the force's bombers of that hex. The reach is the least strike radius among the bomber groups aboard that can fly. It does not go nearer than its bombers need.
4. **Fuel comes first.** It does not close where the passage there and from there to its home port, times its risk's margin, is more than it has. The reason gives the figures.
5. **It stays while there is something in reach.** A force that has left its task's hex to close stays where it is on a morning it holds a contact no more than 3 pulses old, of a kind its risk closes on, within its bombers' normal reach. On a morning it holds none, in reach or to close on, it goes back to its task's hex, as decision 0125's rule 7 has it, and is given no route when it is there.
6. **Its search arc** is taken from the hex the force is in when the staff decides, as before.
7. **Readiness.** Battle readiness on a day it closes, as at its hex.
8. **Reasons** give the contact, its kind and age, its distance against the reach, the hex chosen and the hours to it, or why the force does not close: nothing fresh, nothing of a kind it closes on, already in reach, too far, or not the fuel.

## What it leaves out

- **Opening the range** after a strike, or keeping outside the enemy's reach while inside one's own. The staff holds no belief about the enemy's reach until stage 2b.
- **Closing inside a day.** It needs orders inside a day.
- **A surface force that closes.** The stand task has a station; a hunt is another task.
- **Where the hex of the task should be.** Still the commander's, or the scenario's.

## What it costs

- **No new state.** The doctrine's version rises.
- **The rulebook's chapter 8,** "Operating a carrier force", gains the two rules.

## The hand calculation

`tests/scenario_58_a_carrier_force_closes.rs`: one carrier at her hex and a carrier reported 480 nm away, over 3 days at press: she closes at full speed in the night, strikes in the morning, and goes back.

## The proof

The second proof is run again when this and decision 0131 are built, by decision 0128's test plan with its pass marks and its stated intents unchanged. Both runs are kept.

## As built

Built on Nov 16, 2026, in one step. The doctrine is `supply-15`; the state layout (`flagplot-state-v33`), the rules label, the saved game version and the agent interface are unchanged: there is no new state. The rules are in `crates/sim-staff/src/carrier.rs`: the pure parts are `closes_on`, `speed_for`, `closing`, `in_reach` and `close_to` (tested in a view built for them), and `serve_carrier` reads the side's view into them. Rulebook chapter 8, "Operating a carrier force", items 11 to 16; the glossary gains "Close the range". Readings the build took, each closest to the record it could find:

- **Speed (rule 1).** The hours are the engine's forecast of the passage from the start of the coming night (`hours_to`), cruise where it arrives within 12 hours, else full speed; at preserve the force always cruise. It applies to the route to the task's hex, to a hex the force closes to, and to the route back from its home port, whose fuel is measured at that speed. **Not to the passage home for fuel:** that route and the fuel it weighs keep decision 0125's "speed it has". The plain reading ("or home"; "the fuel the force needs (decision 0125, 8) is measured at the speed chosen") changes scenario 55: its passage home, (0,0) to (5,0), is 200 nm, 12.5 hours at cruise, so full speed would be chosen, which burns (30/16)^2 = 3.515625 times as much; the property "fuel takes her home" would then send her home with 81 t (the forecast has her run dry, and its burn is capped at what is aboard: 81 x 1.2 = 97.2 t needed), and give a speed order to full with 79 t aboard. **For the architect to rule.**
- **Left to close (rule 5)** is read without new state: a force not at its task's hex whose last route ends at a hex that is neither the task's nor its home port's. A force away from its hex for another reason (scenario 55's commander's hex) is routed back as before.
- **Fresh** is no more than 3 pulses old at the staff's hour. **The reach** is the least strike radius (`strike_range_nm`) among the bomber groups aboard with an aircraft able to fly, a held group and a group that searches among them. **A night at full speed** is the force's full speed by the engine's forecast of the passage to the first fresh contact of a kind beyond reach, times 12 hours. Distances are air distances from the force's hex.
- **At its hex** the force closes on a contact beyond reach even when another it closes on is within reach: rule 2 weighs only those beyond, and says "already in reach" when every fresh one of a kind is within. Rule 5's stay is for a force that has left its hex.
- **Fuel first (rule 4)** is the engine's forecast of the passage to the hex and on to the home port at the speed chosen, times the margin. A ship the forecast has run dry is short, whatever the figures: **the forecast caps a ship's burn at what it has aboard,** so with a margin of 1 (press) a ship that cannot make the passage shows its fuel against the same fuel needed. Decision 0125's passage home and decision 0124's sortie read the same capped figure and are not mended here: in scenario 58 at 50 t the passage home reads "ship:1 has 50 t aboard against 50 t needed: every ship has what it needs" and the force stays on its task, where the 66.666666 t the passage needs would send it home. **For the architect to rule.** The closing reason reads "ship:1 has 50 t aboard against more than 50 t needed, running dry short of the passage", and she does not close, as the hand calculation has it. With no home port the fuel is not weighed and the reason says so.
- **The closing is weighed** on a morning the force is not on its way home and is not fuelling at home: from its hex, from elsewhere, and from its home port with the fuel to go back.
- **Readiness (rule 7):** battle on a morning it closes, and on one it stays within reach.
- **Reasons (rule 8).** A closing route's reason gives the contact, its kind and age, its distance against the reach and against the night and the reach, the hex chosen, its distance and the hours to it, and the fuel. On a morning it does not close the words join the reason it stays on its task (or the one that says it has no home port), so no event is added and no event number moves; where there is no such reason they stand as one of their own.
- **The order of the orders** is unchanged: readiness, rules of engagement, speed, route, then each group.

**What it was checked against.** Scenario 58 (`data/scenarios/58_a_carrier_force_closes.toml`, on the frozen figures with scenario 53's table, on its own placeholder map with the port at (0,5)) came out as the hand calculation has it: the twelve orders of day 1, the two of day 2 and none on day 3; the strike of 10 at (12,0) in the morning of day 1 and no other; arrivals at (6,0) at hour 8 and at (0,0) at hour 32, stopped at full speed; the fuel equal to the same ship's ordered over the same passages at full speed. So did the properties: balanced (ten orders, no launch, no order on day 2 with the contact at age 3 and 240 nm, two on day 3, arrival at hour 56), preserve the force, a cruiser at balanced and at press, already in reach, too far (1200 nm against 610), not the fuel at 50 t, speed alone (hour 8 at balanced and press, hour 15 at preserve, hour 5 at (2,0) at every risk), the held force, the same start and two worlds. One test was added by the team lead: with 300 t she has the passage home but not the passage to (6,0) and home at full speed, and does not close. No test is ignored. `sh scripts/compare-scenarios.sh --against origin/main` shows no change of outcome: scenario 55 differs only in its two reasons for giving no order, which gain "task_force:1 does not close: contact 1, a carrier, 2 pulse(s) old, is 160 nm from (0,0), within the 250 nm normal reach of its bombers, ..." on day 1 and "task_force:1 does not close: the side holds no contact of ships 3 pulses old or less." on day 2; every other scenario is the same with the hashes cut. The recorded games differ in their doctrine version, and the model's second-proof week in the words on closing in its reasons. The second proof was not run again.
