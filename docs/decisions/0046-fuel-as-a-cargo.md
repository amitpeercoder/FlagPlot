# 0046: Fuel as a cargo

Status: confirmed. The designer chose on Nov 11, 2026 to build this before the agent interface. The details are the architect's.

Summary: Fuel as a cargo: a ship carries fuel as cargo beside supply, a load order names its cargo, and fuel landed joins the base's stock

Standing: Confirmed

## The gap

A base's fuel is what its scenario gives it (decision 0045). Nothing carries fuel anywhere. A forward base refuels ships until its stock is gone and then never again, however many convoys reach it, because a convoy carries one kind of cargo: supply (decision 0011, 1).

## Decisions

1. **Fuel is a second kind of cargo, in tons.** A ship carries supply and fuel cargo together up to its class's cargo capacity. The fuel a ship carries as cargo is not the fuel it steams on: it cannot burn its cargo.
2. **A load order names its cargo: supply or fuel.** Supply if it does not say, so every existing order means what it meant. A load of fuel fills each ship's remaining cargo capacity from the base's fuel stock, by the rules of decision 0011: stopped at a base of its own side, ships in order of their number, stopping when the stock is gone.
3. **An unload order moves everything aboard,** supply first and then fuel, into the base's two stocks. It needs no cargo named.
4. **Fuel moves at the rate supply moves,** through the same berths and lighters, in the same loading phase. A ship works one cargo at a time: in a pulse it moves supply until it has none, then fuel, within the tons its berth or lighter allows for the hours it was stopped.
5. **Fuel landed is fuel in the base's stock,** and ships refuel from it by decision 0045, the landing ships among them.
6. **A scenario may start a ship with fuel cargo aboard,** as it may with supply.
7. **A scenario that loads no fuel runs as before.**

## Why

- **One mechanism.** The berths, the rates, the loading phase, the two orders and the order ships are worked in are those of decision 0011. Fuel is a second stock moved by them, not a second way of moving things.
- **Any cargo ship can carry it.** In 1942 most aviation fuel and much ship fuel reached Guadalcanal in drums in ordinary transports. Tankers and bulk fuel are a refinement: they change the rate and the capacity, not the rule.
- **Cargo is not bunkers.** A ship that could burn its cargo would make every transport its own tanker and every fuel figure a sum of two. Keeping them apart keeps "how far can this ship go" a property of the ship.

## What it costs

- A force carries one kind of cargo in a trip unless it already has some aboard: a load order fills to capacity. A mixed load by weight needs an amount on the order, which is deferred.
- Fuel in drums and fuel in a tanker's tanks move at one rate.
- A ship low on fuel with a hold full of it must reach a base and land it before it can use it.

## Deferred

- Tankers, bulk fuel and their rates.
- A load of a stated weight, and so mixed loads.
- Refuelling at sea, from a tanker or from another ship.
- Aviation fuel as a stock of its own; air sorties still draw on supply (decision 0039).

## As built

- **The existing loading and unloading events say which cargo.** No kind of event is added. The text for supply is as it was.
- **A load of supply leaves room for fuel cargo already aboard,** as a load of fuel does for supply.
- **An unload of both cargoes records one event for each cargo,** supply first.
- **The full slice of scenario 7 does not carry fuel.** No new kind of event forced it in. The fuel run of scenario 2b is where the rule is proved.

## Rule card

| | |
| --- | --- |
| Reads | The standing cargo order and its cargo; a ship's cargo capacity, supply and fuel cargo aboard; the base's supply and fuel stocks; berths and rates |
| Writes | A ship's fuel cargo; a base's fuel stock |
| Events | The loading and unloading events of decision 0011, saying which cargo |
| Randomness | None |
