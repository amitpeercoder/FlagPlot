# 0008: Orders

Status: needs review.

## The gaps in the spec

The spec says orders are validated against truth at intake, that they persist until replaced, and that each carries an origin of "player or staff". It leaves several details open.

## Decisions

1. **Orders are submitted per side, then taken in at the start of the day.** Sides are processed in a fixed order (Allied, then Japanese), and each side's orders in the order given.
2. **Every order gets an ID and is kept, accepted or not.** The event log records which it was, and the reason for a rejection.
3. **The issuer is "player" or "staff", as the spec says.** Scripted orders in test scenarios are marked "player". There is no third value for scripts or agents.
4. **"Persist until replaced" is per kind of order.** A new route replaces the old route. A new speed replaces the old speed. A route order does not cancel a speed order.
5. **A task force remembers which order set its route and which set its speed.** Movement events cite those orders as their causes.
6. **A route is rejected whole** if any waypoint is off the map or the path crosses a hex ships cannot enter.

## What needs review

- **Decision 3:** when an agent plays, should its orders be marked "player", or does the origin need an "agent" value? The transcript rule in the agent interface suggests it matters who issued what.
- **Validating against truth can leak information.** An order aimed at a hex blocked by something the side does not know about would be rejected with a reason. Nothing like that exists yet, since the only checks are map and ownership. It will matter once orders can target contacts.
- **Only three orders exist:** set route, set speed, set mission. The spec lists many more. Each arrives with the scenario that needs it.
- **Mission is stored but does nothing yet.**
