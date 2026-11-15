# 0122: Rearming: a ship in port takes ammunition through the hours of the day

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2 (decision 0113, decision 15), on the designer's answer to its question 6: rearming is built in stage 2. It writes the rule out with its rule card and its hand calculation. Not built. It is built in step 6 of the stage, with the staff's policy for when a force goes back to rearm, which gets its own record.

Summary: A base gains a stock of ammunition in tons, and a ship in port that lacks main battery rounds or torpedoes takes them from it in the port's queue, after its cargo and its fuel, at the port's rate at a berth or by lighter: rounds first, then torpedoes, whole ones only. A game with no ammunition at any base, or no rate, is the game it was

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Not built

## Why

A ship's main battery rounds are counted and spent, and each torpedo tube fires once in a game. Nothing gives either back. A force that has fought is weaker for the rest of the game however long it lies in port, and over the 39 days of the slice that is false: ships rearmed at Espiritu Santo and Noumea between the battles of October and November 1942 and fought again (`docs/research/oob/rearming.md`).

The research found where ammunition could be had and when, single dated cases with the rounds taken, and no rate. So the rule's shape rests on the record and its rates are placeholders.

It reuses the port's queue as refuelling does (decision 0112): the same berths, the same hours of a pulse, a third kind of work after cargo and fuel.

## The rule

1. **A base has a stock of ammunition,** in tons, given by the scenario and 0 where it gives none. It is a stock as supply and fuel are.
2. **What a ship lacks.** Rounds: its outfit, which is its main guns times the class's rounds for each gun, less the rounds aboard. Torpedoes: its tubes that have fired. A ship whose class carries the marker for ammunition that is not limiting (9,999 rounds a gun) lacks no rounds.
3. **Its weight.** A round weighs the figure for the ship's kind and a torpedo the figure for a torpedo: `surface_combat.toml: [rearming]`, `tons_per_round` by kind and `tons_per_torpedo`.
4. **The rate.** A port size gives ammunition worked at a berth and by lighter, in tons an hour. With none given for a port's size, no ship rearms there.
5. **In the queue.** A ship stopped at a base of its side that lacks ammunition, where the base has some and its size a rate, has work to do, and is given a berth as a ship with cargo or fuel is. In a pulse it works its cargo, then takes its fuel, then takes ammunition in the hours that are left: rounds first, then torpedoes.
6. **Whole rounds and whole torpedoes.** In the hours it has, a ship takes as many whole rounds, and then whole torpedoes, as the rate and the stock allow. A part of one is not taken and not carried to the next pulse.
7. **Where the stock is short,** ships take from it in the order the queue serves them.
8. **A torpedo taken reloads a tube that has fired,** the lowest numbered first, and that tube may fire again.
9. **Each ship's taking in a pulse is one event,** with the rounds, the torpedoes and the tons, caused as a refuelling is. A side sees its own.
10. **The list of forces says** what each force lacks in rounds, torpedoes and tons, and the hours it would take where it lies.

## The figures

| Figure | Live value | What it rests on |
| --- | --- | --- |
| Tons for a round, by kind: battleship, cruiser, destroyer, others | 1, 0.15, 0.04, 0.04 | Placeholders, sized to a shell with its charge: about a ton for a 14 or 16 inch round, 150 kg for an 8 inch, 40 kg for a 5 inch |
| Tons for a torpedo | 1.5 | Placeholder |
| Ammunition at a berth, by port size | 20 t an hour where there is a berth | Placeholder. No rate was found |
| Ammunition by lighter | 10 t an hour | Placeholder. No rate was found |

The frozen step 1 figures give no rate and no stock, so the hand-calculation tests built before this rule are unchanged, and this rule's test sets its own.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Rearming | A base's ammunition; a port's berths and rates; the ships stopped there, their rounds aboard, their tubes fired, their class and kind; the hours cargo and fuel leave in the pulse | Rounds aboard; tubes loaded; the base's ammunition; the event |

## What it leaves out

- **Anti-aircraft ammunition,** which is not counted.
- **Bombs and torpedoes for aircraft,** which a base's air supply covers (chapter 5).
- **Where torpedoes could be had.** The record has them from tenders at particular places. Here a base with ammunition gives both.
- **Ammunition carried as cargo** to stock a forward base. A stock is set by the scenario until a supply run can carry it.
- **Rearming at sea** or from another ship, which the record has as an expedient.
- **A reserve of ammunition at a base.** The staff's reserve limits cover supply and fuel.

## What it costs

- **State:** a base's ammunition, in the hash and named beside it; the state version rises. The rules label rises.
- **Data:** two columns of the port sizes file, the `[rearming]` table, and a scenario's ammunition at a base and torpedoes fired at the start.
- **The rulebook:** chapter 2 gains "Rearming" beside "Refuelling"; chapter 4 says a tube may be loaded again; the glossary gains ammunition as a stock.

## The hand calculation

`tests/scenario_52_rearming.rs`: a cruiser at a berth and a destroyer by lighter at one base, and a destroyer at a base that runs out, with every pulse's rounds, torpedoes and tons.
