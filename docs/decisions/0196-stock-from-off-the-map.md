# 0196: Stock from off the map

Status: a plan by the architect, written on Nov 20, 2026, on the designer's word (his queue of that day: "supply from off the map"). Read by a reviewer who had not seen it written, and amended on that reading ("Amended on review", which replaces what is above it where they differ) before any of it is built.

Summary: A base's supply and fuel come only from what ships unload there. Nothing refills a rear base, so a long game drains Rabaul and Espiritu Santo to nothing. This lets a scenario give a base a supply line: so many tons of supply and of fuel a day, from a day to a day, added to its stock at night while the base is its side's. It is the simple form. The form the design note asks for, cargo that arrives in real ships that can be attacked, is begun by decision 0195, which lets the ships come; nothing yet lets them leave, and that waits for a record of its own.

Standing: Planned on Nov 20, 2026 and amended on review; not yet built.

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

## Amended on review

A reviewer read the plan against the design note, the rules of a day and the code. Found right: that base stocks are exact decimals; how aircraft streams are tested and served; the figures quoted; the sums of the worked example; that no check of the engine's or the harness's counts on stock never rising where a line is.

7. **The line names its side.** "The side that held the base when the game began" is recorded nowhere. A `[[supply_line]]` has a `side`, the base's side at the start where left out, kept on the line. It gives on a night its base is that side's, as an aircraft stream does. So a base lost and taken back receives again, a scenario may give a line to a base a side will hold later, and a side is shown only its own lines. A base formed in play has no line: the scenario cannot name it.
8. **Where it falls in the night:** after aircraft replacements are received and before the night's aircraft and men are paid for, so that a night's men are fed from that night's tons. It is a step of its own in the night's order, with its moment for a watcher of the day.
9. **The order of a day, corrected.** A base changes side in the fight ashore at the end of a day, after that day's night. The worked example's statement on a base lost and retaken is restated by the architect in those terms, in events and not in tons, since a base taken keeps only a share of its stock.
10. **The staff counts its own lines.** As written the staff's forecasts would have been wrong: it would say a rear base runs dry on a day it never reaches, and send runs early to a base a line feeds. A side's view gives the side its own lines, and the staff adds a base's line to its stock day by day wherever it forecasts that stock: the day a source falls to its reserve, and the stock of a base a run is weighed for. Days of supply is today's stock over planned use, as it is, and is read before the night's tons are given; the reason says the line beside it.
11. **What a line does not do:** it scores nothing as supply landed, which counts what ships unload; it keeps a base open, which is judged on its stock. Its event cites no cause, as a stream of aircraft cites none: nothing in the game led to it.
12. **"A base's stock rises when it changes side" was wrong:** the taker keeps a share, and the stock falls.
13. **The loader:** tons are whole numbers or decimal strings ("12.5"); a line with no first day begins on day 1 (an aircraft stream begins on day 2; the difference is meant, and tested); refused are a first day below 1, a first or last day past the game's last, a last day before the first, tons below 0, neither supply nor fuel, and a base the scenario has not.
14. **Built after decisions 0194 and 0195 and before 0197,** one at a time: all four raise the state's version and change the night's order, the scenario file's reader and the briefing's lines of what is due.

### A question for the designer

His design note says the slice's supply lines arrive "in real ships that can be attacked" (`docs/design/economy.md`, answer 5). The same note says arrivals "should stay a rate fed from outside the theatre", and that a route the enemy cannot reach is a pool of tonnage and not named ships (answer 4). This rule is the second: a rate, for a base whose own line runs away from the fighting. Nothing in the game stops a scenario giving Lunga a line, which would undo the war on shipping. So: **a line is given only to a rear base, and each scenario's record says for each line why the enemy could not have cut it.** Where he could, the cargo comes in ships that enter by date (decision 0195). Is that acceptable, or should a line be refused by the game anywhere but at a base the scenario marks as a rear base? The architect proposes the first: a rule of scenarios, written in `docs/engineering/roles-and-tests.md`, and no new marking.
