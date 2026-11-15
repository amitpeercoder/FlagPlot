# 0112: Refuelling that takes time

Status: a plan by the architect, written on Nov 15, 2026, as decision 0102 ordered it. Confirmed by the designer on Nov 15, 2026, who answered its four questions as the architect recommended; it is built before stage 2 of step 3. The hand calculation is written (`tests/scenario_44_fuelling_takes_time.rs`). Built on Nov 15, 2026: see "As built".

Summary: A plan for refuelling that takes time: a ship in port takes fuel through the hours of the day at the port's rate, at a berth or by lighter, in the same queue as cargo, where today every ship is filled at once at the start of the day

Standing: Confirmed by the designer, Nov 15, 2026. Built Nov 15, 2026 (scenario 44, rules label 11, doctrine supply-6, state version unchanged)

## Why

Today every force stopped at a base of its side is filled at the start of the day, whatever its size, at no cost in hours and with no berth (rulebook chapter 2, Refuelling in port). A carrier force that takes thousands of tons is ready the same morning as a destroyer.

It has not mattered while a player gave the orders. It matters once a staff plans by trial, which is stage 3 and stage 4 of step 3: a planner that finds fuel is free at a base will plan on it. It will turn forces round in a day that the record shows lying in port for longer, and it will never weigh a small port against a large one. Decision 0102 ruled it to be built before stage 3 for that reason.

## The rule

It reuses the cargo rule: the same berths, the same queue, the same hours.

1. **Fuel is taken through the day, not at its start.** A ship of a force stopped at a base of its side takes the fuel it lacks in the hours of each pulse, at the port's fuelling rate, as cargo is worked. The morning's refuelling at the start of the day is removed.
2. **It needs no order,** as now. Refuelling is still a rule: a ship that lacks fuel and lies where it can be had takes it. The reserve of decision 0111 binds it as it does today, and the bases of a side in a hex are drawn on in base order.
3. **It takes a berth or a lighter** (question 2). Berths are given out at the start of each pulse, to forces in number order, to a ship that has work to do: cargo to move, or fuel to take. A ship with no berth fuels by lighter at the lighter rate. So a force that fuels can keep another from the berth, and a small port fuels a large force slowly.
4. **A ship does one thing at a time: its cargo first, then its fuel.** A ship with a cargo order works it until it is complete and then fuels with the hours left in the pulse.
5. **The rate is a figure of the port's size,** of its own, beside the cargo rates: tons of fuel an hour at a berth, and by lighter (question 1). A rate not given is no limit: the ship takes all it lacks in the first pulse it lies there.
6. **A force that sails takes what it has.** An order to sail is not held back for fuel. The measuring tool and the briefing say, for a force in port, the fuel it lacks and the hours it will take at the present rate, so that a player or a staff can time the order. There is no order to sail when fuelled (question 3).
7. **A force out of fuel at a base** is restored when its first ship has taken fuel, as now, which is in the first pulse and not at the start of the day.
8. **Nothing else changes.** A ship in port in no force takes no fuel, as now. Fuel carried as cargo is still not burned. Aircraft still fly on supply.

## What it leaves out

- **Refuelling at sea,** from an oiler or from a larger ship. The comparison names it; decision 0102 deferred it. It waits for a slice in which a force must stay at sea longer than its fuel lasts.
- **A port damaged,** and a port's daily ceiling, which decision 0102 tied to this rule. A rate by port size is the ceiling this rule needs; damage to a port stays deferred.
- **Rearming.** Ammunition is not replenished at all today, and that is a rule of its own.

## Rule card

| | |
| --- | --- |
| Reads | The forces stopped at a base of their side and their ships' fuel; the port's size and its fuelling rates; the berths given in the pulse; each base's fuel and fuel reserve; a ship's cargo order and whether it is complete |
| Writes | A ship's fuel; a base's fuel; the berths given in the pulse; a force's status where it was out of fuel; the events of fuel taken and of fuel held at a reserve |

## What it costs

- **Every scenario with a force in port changes** on the live figures: fuel is taken later in the day, and a force that sails the morning after it arrives may sail short. The baselines, the recorded games and the full slice are remade, and the slice's measures are run again. The rules label rises.
- **The hand calculations do not move.** On the frozen step 1 figures the fuelling rate takes a neutral value, no limit, under which a ship is filled in the first pulse it lies in port. Where a hand calculation reads fuel at the start of a day before any pulse, the architect checks it and restates it if the hour matters: scenarios 36, 39, 40 and 41 each state a morning's refuelling.
- **The staff's timing.** The doctrine of stage 1 counts a ship free the morning after it is back. It counts the fuelling hours instead, by the same rule as loading. The doctrine's version rises.
- **No new state.** A ship's fuel and a base's fuel are state already; the berths of a pulse are worked within it. The state version does not rise unless the build finds otherwise.
- **The rulebook:** chapter 2 (Refuelling in port, Cargo orders, Port size) and the table of the day's order in chapter 1; the glossary.
- **Research.** Done for a first value: `docs/research/oob/fuelling-in-port.md`, and "The first figures" below.

## The hand calculation

`tests/scenario_44_fuelling_takes_time.rs`, written by the architect before any code: two forces at one port of one berth, one loading cargo and then fuelling, the other fuelling by lighter until the berth is free; a cruiser at a port with no berth that sails short on the second morning and finishes where she arrives; and a reserve holding a destroyer short, recorded once.

Rulings made while writing it:

- **A berth given in a pulse is that ship's for the pulse,** as for cargo. A ship that finishes early does not hand it on until the next pulse.
- **A ship that arrives in a pulse fuels in the hours of it that are left,** as it works cargo.
- **A ship held at a reserve is recorded once,** in the pulse it is first held, and not again while nothing changes.
- **The tools say what a force in port lacks and the hours it would take** at that port's lighter rate, or its berth rate where the port has a berth: the list of forces and the briefing.
- **On the frozen figures** no fuelling rate is given, so a ship fills in the first pulse it lies in port. A force that begins a game in port short of fuel and sails on day 1 now sails as it began, where before it was filled first: the build lists every scenario where that is so, and the architect restates any hand calculation it touches.
- **The staff of stage 1** counts the hours a ship will take to fuel at the source as it counts loading, in whole days, before it calls the ship free.

## The first figures

Settled by the architect on Nov 15, 2026, from the rows of `docs/research/oob/fuelling-in-port.md` (`place_facts` 241 to 295). They are sized from rows, each resting on one event, and are not checked against any battle.

| Figure | Value | What it rests on |
| --- | --- | --- |
| Fuel by lighter: a ship with no berth, fed by an oiler, a tanker or a barge alongside | 130 t an hour, at every size of port | Thirteen fuellings of one ship by a Japanese oiler run from 46 to 259 t an hour with a median of 131. A heavy cruiser took at least 120 and at least 164 t an hour from tankers alongside at Espiritu Santo and Noumea (246, 241). Shore pumps outside the theatre gave 125 in practice (290) |
| Fuel at a berth | 200 t an hour, at every size of port that has a berth | A heavy cruiser took at least 204 t an hour from the dock at Pearl Harbor (248), between 107 and 429 from the dock at Pago Pago (247), and between 215 and 487 from a tanker at Noumea (242) |

- **One value for every size of port.** No row tells a large port's pumps from a small one's. A port's size acts through its berths, as for cargo.
- **By these figures** a destroyer of 500 t fills in about 4 hours by lighter, a heavy cruiser that lacks 1,400 t in 7 to 11 hours, and a carrier that lacks 3,000 t in 15 to 23. The record agrees in kind: a destroyer alongside an oiler took from half an hour to two and a half hours (249 to 251), and two heavy cruisers took 1,126 t and 1,360 t in 12 hours 38 minutes and 5 hours 15 minutes (277).
- **What they do not carry.** How many ships an oiler or a barge could serve at once, and how many were on station, is a gap: by this rule every ship without a berth is fed by lighter at once, as cargo is. No rate of a United States fleet oiler in harbour was found, nor any 1942 statement that fuelling held a berth another ship needed. Both are in `gaps.csv`.
- **Tons.** The rows turn barrels and gallons to tons at 277 gallons to the ton, read as long tons, and take Japanese tons as printed. The game's ton is not told apart from either.
- The hand calculation of scenario 44 sets its own rates in its port sizes and does not move.

## Questions for the designer

Answered on Nov 15, 2026, each as the architect recommended.

1. **The rate:** a figure of its own for fuel, by size of port. It starts as a placeholder equal to the cargo rate until research gives a row.
2. **A berth:** a ship that fuels takes a berth that cargo could have had.
3. **Sailing short:** a force ordered to sail leaves with the fuel it has, and the tools say what it lacks and how long it would take. No order to sail when fuelled.
4. **When:** before stage 2 of step 3.

## As built

Built on Nov 15, 2026. Scenario 44's hand calculation came out as written, every number of its three tables, its end state and its properties, with no input changed. The architect's three points of doubt came out as the record has them: a ship that arrives half an hour into a pulse is given the berth in that pulse (the cargo rule gives berths after movement, to the forces then stopped); a scenario starts a force with no fuel at a base by giving its ship `fuel_t = 0`, which the loader and the invariants accept; and no scenario has a force that begins in port short of fuel and sails on day 1.

**What was built.**

- **The figures.** `PortSize` gains `fuel_berth_rate_t_per_h` and `fuel_lighter_rate_t_per_h`, each optional: a column absent or a cell empty is not given, which is no limit. A given rate must be above 0. The live `data/reference/port_sizes.csv` has 200 at a berth for sizes 1 to 10 (size 0 has no berth, and its cell is empty) and 130 by lighter for every size, with a comment pointing to "The first figures". The frozen `data/reference/step1/port_sizes.csv` has the two columns empty, as its README now says, since the frozen-figures test asks that a frozen file have every column of its live one. Scenario 44 names its own `port_sizes_placeholder_fuelling.csv`, in both folders.
- **The rule** (`crates/sim-core/src/cargo.rs`, `refuel.rs`). The morning's refuelling is gone from `take_steps_one_to_three`. The loading phase of each pulse takes every force stopped at a hex centre with a ship afloat that has a cargo order able to work there, or lies where a base of its side is. Berths are given out once, in the cargo rule's loop, to a ship with cargo to move or fuel to take. Each force works its cargo, and then each ship takes fuel in the hours its cargo left. `ShipRefuelled` now carries its pulse and a breakdown (hours worked, cargo hours, hours left, on berth, the rate where given, what the ship lacked, what the base held above its reserve; the result is the tons taken).
- **The staff** (`crates/sim-staff`, doctrine `supply-6`). A ship is free on the morning after the day it finishes fuelling at the source, by the architect's ruling under decision 0107's ruling 8. The hour it is back is the start of the sail day plus 24 hours for each day out and of unloading, plus the passage back. The fuelling hours are the longest ship's lack on return (its fuel now less the round trip at cruise) over the source's berth rate where it has a berth, else its lighter rate. Free is the day after the day that hour falls in, and never earlier than before. With no rate it is the day it was. No whole day is added for fuelling, as one is for loading. The reasons say when the ship is back and fuelled.
- **The tools** (`crates/sim-agent`). The force line of `list_forces` and the briefing, for a force stopped at a base of its side whose ships lack fuel, gains `fuel_lacked_t` (the tons its ships lack together), `fuelling_hours` (the longest ship's lack over the port's berth rate where it has a berth, else its lighter rate; left out where that rate is not given) and `fuelling_at` (the base). The briefing says "lacks 540 t of fuel: 13.5 hours at base:2 ...", or, with no rate, "lacks ... t of fuel, taken in its first pulse in port". The agent interface stays at version 7: these are fields added, which its rule and decision 0107's precedent (a version for new tools and kinds) do not count as a change. The schema of the force line names them.
- **Versions.** The rules label is 11. The state version is unchanged (`flagplot-state-v26`): nothing new is state, since the berths of a pulse are worked within it and the once-only record of a hold is read from the event log. The doctrine is `supply-6`, and the agent interface is 7.

**Where the build read the record more closely, or chose where it was silent.** The architect accepted the first five on Nov 15, 2026, while the build ran, and the last two when he read it the same day. On the seventh: the staff's count is exact for a run of one ship, which is every run of stage 1's scenarios, and can be short by hours where a run has more ships than the source has berths; it is listed in `docs/open-work.md` to be made exact when a doctrine sends several ships to fuel at once.

1. **Fuel alone claims a berth only where the port's lighter rate is given.** Where it is not given, fuelling by lighter has no limit and a berth cannot speed it. So on the frozen figures fuel takes no berth from cargo.
2. **A rate not given takes no hours.** A ship there takes all it lacks in the pulse, whatever hours its cargo used. With a rate given and no hours left, it takes nothing.
3. **A force that runs out of fuel in a pulse takes no fuel until the next pulse,** since the hour it ran dry is not known. The rulebook says so.
4. **A hold at a reserve is recorded only when a reserve holds the fuel back:** the base that served the ship, or with none any base of the side there, holds fuel but none above its reserve. A ship left short by its rate or its hours records no hold.
5. **"Recorded once" is read from the event log:** a hold is recorded unless the ship's latest fuel event (`ShipRefuelled` or `RefuellingHeldAtReserve`) is a hold at the same hex with the same shortfall. Taking fuel or burning it changes that.
6. **One base serves a ship in a pulse,** the lowest-numbered of the side in the hex with fuel above its reserve, as in decision 0111. A ship it cannot fill draws on the next base in a later pulse.
7. **The staff counts one rate for every ship,** the berth rate where the source has any berth. The engine gives a berth only to the first ships in its queue, so a staff may undercount the hours where a force has more ships than the port has berths.

**The frozen hand calculations were restated.** On the frozen figures no rate is given, so a ship that comes into port short now fills in the pulse it arrives, where before it filled at the start of the next day. The architect's expectation that no frozen number would move was wrong. Every frozen scenario in which a force arrives at a base of its side short of fuel changed in its fuel at the end of the arrival day, and in nothing else. Those scenarios are 1b, 2b, 36, 40a, 40b, 41a, 41b, 42a, 42c, 42d and the frozen copy of the full slice (day 4, ships 37 to 39 and base 1). 39 and 41c changed only in their events. By the architect's ruling the comments and the assertions of scenarios 36, 39, 40 and 41 were restated, each refuelling a day earlier with the fuel figures the run gives (39: the hold at the reserve is recorded once, on day 1). So were those of 1b and 2b, with their refuelling events now in the pulse of the arrival, their tons and end states unchanged, and the refuelling of 2b's variant with supply aboard on day 4 from the 400 t landed in its night pulse. Scenario 42's comment needed no change. The unit test that asked the staff after refuelling now says the staff is asked on the fuel of the day before (`the_staff_is_asked_after_the_commanders_commands_on_the_fuel_of_the_day_before`).

**On the live figures** only the scenarios with a force that comes into port short changed (1b, 2b, the full slice, 36, 39, 40a, 40b, 41a to 41c, 42a to 42d). In the full slice nothing a measure reads moved (`docs/validation/full-slice-39-days-after-0112.md`): the batch files of 1000 runs before and after are the same in every count. One force sails short where it sailed full: the transports at Guadalcanal land through every hour of days 1 and 2, so they no longer take Lunga's 200 t the morning of day 2, and they sail for Espiritu Santo 16 t short in each ship. Savo Island, the Eastern Solomons and the air campaign are unchanged over 1000 seeds, final hashes included.
