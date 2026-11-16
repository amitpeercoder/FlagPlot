# 0127: Going back to rearm: ammunition as policy

Status: settled by the architect on Nov 16, 2026, inside the plan for step 3, stage 2 (decision 0113, step 6 of its seven: "rounds kept in reserve as policy"). It is the staff's use of decision 0122's rule. It adds no rule of the engine and no intent: ammunition is policy, as fuel is (decision 0116, 5). Not built. It is built in step 6, with decision 0122.

Summary: A surface force that is short of ammunition by its risk level does not sortie until it has rearmed: the staff counts each ship's rounds against a share of its outfit and its fired tubes against a share of its tubes, keeps a short force in port where the base can rearm it, says the day from which it may be used, and hands in that it cannot meet the intent where no ammunition is to be had

Standing: Settled by the architect, Nov 16, 2026, inside decisions 0113, 0122 and 0124. Not built

## Why

With decision 0122 a ship that lies in port with ammunition ashore takes it aboard. The staff still sends a force out each night it believes in a threat, whatever is left in its magazines, and a force that sails before it has rearmed takes nothing. A staff has to know when a force is not fit to fight and how long until it is.

## The policy

1. **Short of ammunition.** A ship is short when its rounds aboard are below the share of its outfit its risk gives, or its fired tubes are more than the share of its tubes its risk gives. A ship whose class carries the marker for ammunition that is not limiting is never short of rounds. A force is short when any ship of it afloat is.

| Set by the risk level | Preserve the force | Balanced | Press |
| --- | --- | --- | --- |
| A ship is short of rounds below | Three quarters of its outfit | A half | A quarter |
| A ship is short of torpedoes with more fired than | None | A half of its tubes | Never |

2. **A force that is short does not sortie** (decision 0124, 10) and is not a free escort (decision 0126, 5). Its reason gives the ship, its rounds against the level and its tubes.
3. **Where it can rearm.** A short force stopped at a base of its side that holds ammunition and has a rate for it is left there, and rearms by decision 0122. The reason gives the day from which the force may be used: the day after the last of its ships will be no longer short, by the engine's own measure of the hours rearming takes there, behind its cargo and its fuel.
4. **Where it cannot.** Where the base it lies at has no ammunition or no rate, the reason gives no day, and the staff hands in that it cannot meet the intent. It does not send the force to another base: moving a force's home is the commander's (decision 0124, "What it leaves out").
5. **It is read with fuel,** in one pass: a force sorties when it is neither short of fuel (decision 0124, 11) nor short of ammunition, and the day it may be used is the later of the two.

The shares are placeholders that rest on no row.

## What it leaves out

- **Carriers,** whose main battery is not what they fight with; their aircraft's bombs and torpedoes are a base's or a ship's air supply.
- **Breaking off a fight when rounds run low.** The rules of engagement do not read ammunition.
- **Sending a force to where ammunition is.**

## What it costs

- **No new state.** The doctrine's version rises. The stand doctrine's reasons and the list of forces gain the ammunition figures.
- **The rulebook's chapter 8** gains "Ammunition as policy".

## The hand calculation

`tests/scenario_57_going_back_to_rearm.rs`: scenario 54's force and contact, with one cruiser short of rounds at a base that can rearm her: the force stays in on the first night and sorties on the second.
