# 0008: Orders

Status: confirmed by the designer, Nov 10, 2026. Decisions 3 and 7 were settled in that review.

## The gaps in the spec

The spec says orders are validated against truth at intake, that they persist until replaced, and that each carries an origin of "player or staff". It leaves several details open.

## Decisions

1. **Orders are submitted per side, then taken in at the start of the day.** Sides are processed in a fixed order (Allied, then Japanese), and each side's orders in the order given.
2. **Every order gets an ID and is kept, accepted or not.** The event log records which it was, and the reason for a rejection.
3. **The issuer is "player" or "staff", as the spec says.** It answers a game question: did the holder of the seat give this order by hand, or did their staff? Who holds the seat is a separate fact, recorded once per side in the saved game: human, agent (with the model's name) or script. Scripted orders in test scenarios are marked "player". An agent's own orders are marked "player", and orders from a staff working for an agent are marked "staff".
4. **"Persist until replaced" is per kind of order.** A new route replaces the old route. A new speed replaces the old speed. A route order does not cancel a speed order.
5. **A task force remembers which order set its route and which set its speed.** Movement events cite those orders as their causes.
6. **A route is rejected whole** if any waypoint is off the map or the path crosses a hex ships cannot enter.
7. **A rejection may only cite facts the issuing side can see:** its own units, the map and ownership. An order that would fail because of something the side does not know is accepted at intake and fails in execution. The side learns of it through an event it would plausibly receive. This amends the spec, which said orders are validated against truth state.

## Why

- **Decision 3:** the override rules in pillar 2 depend on telling a hand-given order from a staff order. That is true whoever holds the seat. A third issuer value for agents would need a fourth as soon as an agent has a staff.
- **Decision 7:** a rejection that cites unknown truth is free intelligence. A route refused because of an unseen minefield tells the side where the minefield is.

## Not built yet

- **The seat record.** Added to the saved game and the agent transcript when the agent server is built.
- **Execution-time failure.** Nothing needs it yet. Every check today is map or ownership, which the side can see.

## Still true

- **Only three orders exist:** set route, set speed, set mission. The spec lists many more. Each arrives with the scenario that needs it.
- **Mission is stored but does nothing yet.**
