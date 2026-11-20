# 0196: Stock from off the map

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "supply from off the map"). To be read by a reviewer who has not seen it written, and amended on that reading, before any of it is built.

Summary: A base's supply and fuel come only from what ships unload there. Nothing refills a rear base, so a long game drains Rabaul and Espiritu Santo to nothing. This lets a scenario give a base a supply line: so many tons of supply and of fuel a day, from a day to a day, added to its stock at night while the base is its side's. It is the simple form. The form the design note asks for, cargo that arrives in real ships that can be attacked, is made possible by decision 0195 and is left for the scenario that needs it.

Standing: Planned on Nov 20, 2026; waits for its review.

## Why

`docs/design/larger-scenarios.md`: a campaign longer than the 39 days empties its rear bases. Rabaul starts with 10,000 t of supply and Espiritu Santo with 960 t, both placeholders sized for 39 days; decision 0163 found Rabaul's fuel "gone by day 26" at its first figure.

What is so today:

- **A base's stock rises** only when ships unload at it, when a march brings supply in, or when it changes side. It falls to aircraft, repairs, men fed, loading, fuelling and bombs.
- **`docs/design/economy.md`, answer 5:** "Cargo arrives at the map edge on a schedule from the record, in real ships that can be attacked, and a base's stock depends on it." Planned there for step 4.
- **Aircraft already arrive at a rate** from off the map (`[[replacement]]`), while the base is its side's.

## The rules

1. **A new table of a scenario, `[[supply_line]]`:** a base, tons of supply a day and tons of fuel a day (each a whole number or a decimal, at least one above 0), and optionally a first day and a last day (from day 1 to the game's last where left out). A base may have more than one.
2. **Each night,** in the pulse where replacements of aircraft are received, a line whose days hold the day and whose base is held by the side that held it when the game began adds its tons to the base's supply and fuel. A base held by the other side on that night receives nothing, and what it would have received is lost.
3. **An event,** `StockReceived`, with the base and the tons of each, seen whole by the side that holds the base and by nobody else.
4. **It is not cargo and no ship carries it.** It cannot be found, struck or cut. That is what this rule leaves out and why it is the simple form: it is for a base at the edge of the theatre whose own line runs away from the fighting.
5. **Nothing else changes.** The staff reads a base's stock as it does today; days of supply, reserves and the source of a run are worked on the stock as it stands each morning. A side is told of its lines in its briefing, under what is due.
6. **A scenario with no supply line plays as before.**

## Rule card

- **Reads:** the supply lines; the day; the holder of each base and who held it at the start.
- **Writes:** a base's supply and fuel.
- **Reuses:** the step and the pattern of aircraft replacements; the briefing's lines of what is due.
- **New:** one table of a scenario and its state; one event. No draw.

## What it costs

- **State and hash:** the lines are state; the hash version rises and the golden files are remade.
- **A figure for each line is data,** and none is on file: a scenario's lines are placeholders, marked, until the tonnage that reached Rabaul and the South Pacific bases is researched.
- **It hides the war on shipping.** A base fed this way cannot be starved by sinking what feeds it. Where that matters, the scenario brings its cargo in ships (decision 0195) and gives the base no line.
- **Rulebook:** chapter 2 (a base's stock), chapter 1 (the day's order; what a side knows), the glossary; the scenario file's keys; the agent's briefing.
