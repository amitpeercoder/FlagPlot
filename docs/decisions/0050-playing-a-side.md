# 0050: Playing a side

Status: confirmed by the architect, within the plan for the agent interface the designer approved on Nov 11, 2026.

## The gap

A side can be told its situation, ask for detail, ask why and measure (decisions 0047 to 0049). It cannot yet do anything. Every game so far ran from a scenario's scripted orders, start to finish, in one call. The spec asks that an agent play as a person will: read, give orders, end the day, and read again, through a server that holds the game open, with every order recorded so that the game replays.

## Decisions

### Seats

1. **Each side has a seat, and a seat is held by a player or by the script.** A player is whoever is on the other end of the tools: a model, a person or a test. The script is the scenario's own orders for that side, given day by day as they always were. Who holds each seat is recorded once in the saved game, not on each order.
2. **A seat sees and orders its own side only.** Every tool call is made from a seat. A call cannot name another side.

### Orders

3. **`issue_orders` takes a list of orders for the coming day and answers for each: queued, or refused with the reason.** It asks the engine's own check, the one that will be made when the day begins, against the state as it is now. A refused order is not queued. Queued orders are kept in the order given.
4. **The answer is a forecast of the engine's verdict, and the event is the verdict.** An order queued today can still be refused when the day begins, if the state has changed, and the briefing of the next day says so.
5. **An order for a unit the side does not own is refused as an order for a unit that does not exist is:** "not known", word for word. The engine has two reasons for the two cases; a player is shown one.
6. **A queued order can be withdrawn before the day ends:** `issue_orders` may be told to clear what the seat has queued. After the day ends an order stands until another replaces it, as always.
7. **Orders are JSON in the form saved games already use,** with a published schema, and each carries its origin: issued by the player, serving no intent yet.

### Time

8. **`end_day` says the seat has finished its orders for the coming day.** When every seat held by a player has ended, the day is resolved: each player's queued orders and the script's orders for that day are handed in, the day runs, and each player may read a new briefing. With one player the day resolves on that player's `end_day`.
9. **Between `end_day` and the resolution a seat can read and cannot order.** After the last day `end_day` says the game is over, and the seat can still read.
10. **Nothing happens in real time.** The game waits for the seats for as long as they take. No clock is read, so the same calls give the same game.

### The record

11. **A saved game holds the orders as they were handed in,** for both sides, by day, whoever gave them, and who held each seat. It replays to the same hashes with no player present (decision 0043). This replaces saving the scenario's script alone.
12. **Every game with a player keeps a transcript:** each call a seat made, in order, with its arguments and the whole answer it was given. It is numbered and carries the game day, and no time of day. It is the record of what the player knew when it decided.

### The server

13. **`sim-cli serve` holds one game open and speaks JSON lines on its standard input and output:** one request a line, one answer a line. A request names the seat, the tool and its arguments. It opens no network port.
14. **The seven tools of the spec are the server's whole vocabulary:** `get_briefing`, `list_forces`, `list_contacts`, `list_bases`, `get_entity`, `measure`, `issue_orders`, `end_day`, `explain`. (`list_*` are three.) A request for anything else is an error that names the tools.
15. **A malformed request is answered with an error and changes nothing.** The game cannot be put into a bad state from outside.

## As built

- **Only one of the engine's reasons for refusing an order is hidden:** "not your unit", which is shown as "not known", the same words as for a unit that does not exist. The other reasons depend on the map, on the order's own figures or on the side's own unit, and are shown as the engine words them.
- **A played game differs from the scripted game in one way.** An order for a unit that is not the side's own is refused by `issue_orders` and never reaches the engine, so the event that records its rejection in a scripted run is absent in a played one. The full slice sends one such order on purpose. In every other scenario tested, with a player on either side or both, in either order, the played game is the scripted game event for event and hash for hash.
- **An order can pass the forecast and be refused when the day begins.** A convoy told in one day to sail and to load has both queued, and the load refused at the start of the day because it is then under way. The next briefing says so.
- **Enemy bases are shown.** A strike on a base is ordered by naming the base, and the order is refused if there is no such enemy base. That would let a player find bases by guessing, except that both sides already know where every base is and who holds it (decision 0034, 2). So the tools show it: `list_bases` lists the enemy's bases by number, name, place and holder, and nothing else of them.
- **A game with no player has no use for the server,** and `serve` says so.
- **The server writes the saved game and the transcript when its input ends,** whether or not the game is over.
- **A saved game written before seats existed** is read as both seats held by the script, and replays.

## Why

- **The engine's own check, asked early.** A player needs to know now that a route is impossible, not tomorrow. Asking the same function the engine will ask means the answer cannot be more permissive or stricter than the rule.
- **One reason for "not yours" and "not there".** Otherwise giving orders to guessed numbers would map the enemy's forces.
- **Saved orders, not saved conversation, make the replay.** The transcript is evidence of what was read; the orders are what happened. A game must replay without the model that played it.
- **Standard input and output.** It needs no port, no authentication and no dependency, and the protocol that models speak can be wrapped round it in the next stage.

## What it costs

- A player that never ends its day holds the game for ever. That is a matter for whoever runs the player.
- Two players take turns at orders blind to each other and the day then resolves at once; there is no way yet for one side to react inside a day. That is how the game is defined, and the pulses' automatic decisions stand in for it.
- The forecast of a verdict can be wrong, and a player may not expect that.

## Deferred

- Intents and the staff: orders here are a player's own, unit by unit.
- A game saved in the middle and resumed.
- More than one game in a server, and a network transport.
- Orders the engine does not have yet: forming and disbanding forces, rebasing, target priority, a base's reserve levels.

## Rule card

No rule of the game is added. Orders enter through the engine's existing intake. Reads a side's view; writes orders into the queue the engine already has.
