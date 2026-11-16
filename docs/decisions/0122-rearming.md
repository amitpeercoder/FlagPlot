# 0122: Rearming: a ship in port takes ammunition through the hours of the day

Status: settled by the architect on Nov 15, 2026, inside the plan for step 3, stage 2 (decision 0113, decision 15), on the designer's answer to its question 6: rearming is built in stage 2. It writes the rule out with its rule card and its hand calculation. Built on Nov 16, 2026, in step 6 of the stage, with the staff's policy for when a force goes back to rearm, decision 0127 ("As built" below).

Summary: A base gains a stock of ammunition in tons, and a ship in port that lacks main battery rounds or torpedoes takes them from it in the port's queue, after its cargo and its fuel, at the port's rate at a berth or by lighter: rounds first, then torpedoes, whole ones only. A game with no ammunition at any base, or no rate, is the game it was

Standing: Settled by the architect, Nov 15, 2026, inside decision 0113. Built

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

## As built

Built on Nov 16, 2026, with decision 0127, in step 6 of stage 2. The rule is `crates/sim-core/src/rearm.rs`, called from the cargo queue (`cargo.rs`, `handle_cargo`) after the fuel.

**Versions.** The state version is 33 (a ship's `torpedoes_fired` after its `torpedoes_one_side_fired`, a base's `ammunition_t` after its `fuel_reserve_t`). The rules label is 16. The saved game stays at version 8 (no new order). The agent interface is version 13 (a force line's `ammunition_lacked`).

**How a tube is stated as fired.** A ship's torpedoes were a flag (ready or not) and a second flag for a class that fires one side first (decision 0083). The build keeps both and adds a count, `torpedoes_fired`, the tubes fired and not loaded again; `Ship::tubes_fired` reads the three together (all with none ready, at least the first side once it has fired). A salvo fires the tubes of its side, no more than the tubes loaded, and adds them to the count. A torpedo taken lowers the count, makes the ship ready while any tube is loaded and clears the first-side flag. For a class whose tubes all bear on one side that is the rule exactly; for one with two sides, a ship part loaded fires from the side the flag names the tubes it has loaded, up to that side's: the record's "lowest numbered first" is not followed tube by tube there. A ship whose tubes were never reloaded fires as it always did. A scenario states a ship's tubes fired with `tubes_fired` (the highest numbered), and its rounds with `gun_rounds`; a base's stock with `ammunition_t`.

**What a ship lacks** is whole rounds: the outfit less the rounds aboard, cut to a whole number, so that a part of a round left by fire is never asked for. The marker is 9,999 rounds a gun or more.

**The queue.** A ship that lacks ammunition claims a berth where the port gives an ammunition rate at a berth and a base of its side in the hex holds ammunition; the first base of the side there that holds any serves it. `take_fuel` now gives back the hours each ship used, cargo and fuel (fuel taken with no rate takes none), and the ship takes ammunition in the hours left: whole rounds first, then whole torpedoes, within the rate times the hours and the stock. A ship on a berth takes at the berth rate only; with no berth rate given it takes none there. A force out of fuel at a base rearms as it refuels.

**Data.** The port sizes file has two optional columns, `ammunition_berth_rate_t_per_h` and `ammunition_lighter_rate_t_per_h`; the live file gives 20 where there is a berth and 10 by lighter, the frozen file gives the columns empty. `surface_combat.toml` has an optional `[rearming]` table: `tons_per_torpedo` and `[rearming.tons_per_round]` with `battleship`, `cruiser`, `destroyer` and `others`; the live file has the record's figures and the frozen file none (`FROZEN_WITHOUT` in `crates/sim-data/tests/frozen_figures.rs`). Scenario 52 names its own files, `port_sizes_placeholder_rearming.csv` and `surface_combat_placeholder_rearming.toml`, with a copy of each among the frozen figures.

**The list of forces** (rule 10): a force stopped at a base of its side that lacks ammunition has `ammunition_lacked`, its rounds, torpedoes and tons, and the hours to take them there, the longest ship's, behind its fuel (`SideView::ammunition_work`): a berth is given as the queue would give it to the side's ships in the hex that lack fuel or ammunition, cargo aside; no hours where the base holds less than the force lacks or the port gives no rate.

**What a side is told** (the architect's ruling of Nov 16, 2026, after the build): the briefing's force changes gain `rearmed`, a line for each ship that took ammunition, and `charges_lost`, a line for each escort whose charge was lost (decision 0121, 7), since each changes the side's own force and the side would otherwise be told only through its events. `StationKept` and `MeetingNotHeld` are not folded: the force lines say where each force is, and a meeting not held changes no force of the side. Both new parts are left out when empty, so a briefing with neither is as it was. Chapter 1's phase table says that in the loading phase ships take fuel and then ammunition.

**A base that changes side** keeps its ammunition whole: the record says nothing, and the code that cuts supply and fuel to the captured share was not changed.

**What it was checked against.** Scenario 52 (`data/scenarios/52_rearming.toml`, on the frozen figures with its own files) came out as the hand calculation has it in every number: the four events at S and U, the base's ammunition after each pulse (10, 4, 4 t at S; 0 at U), the tubes still fired, 13.2 hours and 5 hours in the view and in the list of forces, and every property (no ammunition or no rate; after cargo and fuel, full at hour 23.2; whole ones only; short stock in the queue's order; the marker; she fires again, a salvo of 8, and without ammunition she fires none). A reading of the test: "routed to U on the morning of day 1" cannot be given, since orders are taken at the start of a day; the test rearms on day 1 to 5 tubes loaded and routes on day 2, and she leaves with 800 rounds and 5 tubes loaded.

**Compared with earlier scenarios.** `sh scripts/compare-scenarios.sh` against step 5's branch: 176 runs, 172 the same with the hashes cut; the 4 that differ are scenarios 52 and 57, live and frozen, which are new.
