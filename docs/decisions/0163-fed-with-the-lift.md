# 0163: Fed with the lift: the second part of the first mend of stage 4

Status: settled by the architect on Nov 19, 2026, inside the plan for step 3, stage 4 (decision 0161, step 1 of its nine, "fed with the lift"). Step 1 is written in two records: decision 0162, the load of a supply run by night; and this one. It adds no rule to the engine: it changes where the staff finds its source of supply and where a lift goes when it has landed, and it gives one base of the 39 days a stock of fuel. Not built.

Summary: Under "control an island" the staff named no source of supply until a transport happened to lie empty at one of its bases, and took the first base where one did, whatever that base held. The ships that had carried the men went back where they came from and fed nobody. Now the source is the base with the most supply, named on the first morning; a lift that has landed its men goes on to that base and joins the ships that feed the place; and Rabaul, which had no fuel, has some, so that a supply ship can make a third run

Standing: Settled by the architect, Nov 19, 2026, inside decision 0161. Not built

## Why

Decision 0162 says how the hunger of decision 0159's proof was traced. Three of its causes are taken here. Each is read from the code and from the trace of seed 1.

**The source waits for a ship, and takes the first base with one.** Decision 0152, item 10, chooses the source of supply "on the first morning there is a base that fits decision 0151, item 8, with a transport force it may take lies stopped there with nothing aboard". Until then the hold names no source, and three things follow. The supply of the men ashore is not formed (decision 0159, rule A, needs a source). The check of whether the place can be held (decision 0144, items 10 to 15) is not made, so a take sails with no word on how its men will be fed. And the base chosen is the first at which a force lies empty: the code takes it even where it holds no supply at all, and keeps it. On seed 1 the Japanese men landed on day 13 and the source was named on day 16, when the force that had carried them lay empty at Rabaul.

**The ships that carried the men feed nobody.** A lift as it stands is routed to the place and back to where it came from (decision 0152, item 6). On seed 1 eight Japanese forces carried men from Shortland, landed them, and went back to Shortland, which holds no supply and is not the source. They lay there for the rest of the game. The place was fed by one force of five ships.

**No Japanese base holds fuel.** The scenario gives Rabaul supply and no fuel. Refuelling is the engine's rule and needs a stock (chapter 2, Refuelling in port); the staff's reckoning of a run assumes the ships fuel on their return. The five ships made two runs and had not the fuel for a third: from day 29 the staff said so each morning and sent nothing.

Decision 0161 words this mend as the supply "formed on the morning their lift is released, sized to the men the lift carries and the days the staff expects them to wait, and counted in the lift's shipping". Read against the code, it is built so:

- **Formed with the lift.** The source is named before the lift sails, so the check of decision 0144 is made at the release and its words say whether the place can be held; and the supply intent is formed on the first morning the men have a base ashore, as decision 0159 has it, with no wait for a ship.
- **Sized to the men and the days.** That is decision 0144's check, items 11 to 14, which now runs.
- **Counted in the lift's shipping.** The lift's own ships become the ships of the supply. A lift's load of supply beside its men is not raised: by night a ship lands no more than the night's hours allow (decision 0145, item 7), and its men fill that.

One thing this does not gain is days. A force found empty at the source on a morning joins the pool by a command of that morning, and the supply is served on the limit as it stood: the first run is ordered the morning after, as before.

## The rules

1. **The source of supply is named without a ship.** This replaces the test of decision 0152, item 10. Where the commander names none, the source is the side's base in no named place's hex that the staff may take with the most supply above its reserve, the lower number on a tie; chosen on the first morning there is one, and kept, as before. A base with nothing above its reserve cannot be the source. No transport force need lie there.
2. **The pool is as before** (decision 0152, item 11): each morning the forces limit of each hold the staff formed is set to the source and the transport forces it may take that lie stopped there with nothing aboard. On the morning the source is named it may be the source alone.
3. **With no ship in the pool the supply intent is served and says so:** the run it would order, and that no ship of its forces limit can carry it. It is the supply doctrine's reason for a run it cannot send, unchanged.
4. **A lift that has landed goes on to the source of supply.** A lift of "take a place", the first or a later one, and a force a hold sends by decision 0152, item 8, is routed to the place and on to the source of the hold of that place, where a hold of the place with a source is in force and every ship of the force has the fuel for the passage to the place and on to that source, by the engine's measure at the speed it goes. Else it is routed back to where it came from, as before. Where the two are one base nothing changes. A source named on a morning is read by the take and the hold of that same morning.
5. **It joins the pool by item 2** once it lies there with nothing aboard. Nothing else is done to it: a force with men left aboard, or supply it did not land, does not join.
6. **The reason of the route** names the base it goes on to and why.

## The data

7. **Rabaul holds fuel.** Base 2 of `data/scenarios/07_full_slice.toml` is given `fuel_t = 5000`, a placeholder: no row gives Rabaul's stock of fuel oil in August 1942, and `data/oob/gaps.csv` gains the gap. The figure is sized not to bind: a fast transport's round trip to Lunga burns about 139 t, so 5,000 t is thirty-six such trips. The comment on the base says so. The staff's setup and the three arrangements of stage 3's proof are made again by their scripts. The frozen copy is not changed.
8. **Why it is right to change it before the proof.** A side whose main base has no fuel cannot run supply for 39 days by any rule; the proof of this stage would measure the scenario's gap and not the staff. It is recorded here, before any run, and no figure is set from a run.

## What it leaves for later

- **A base that feeds ships but is far.** The source is the base with the most supply, not the nearest. On the 39 days that is Rabaul for the Japanese side and Espiritu Santo for the Allied side, which is right. A nearer base with enough is not weighed.
- **Supply moved between the side's own bases,** so that a forward base becomes a source.
- **The staff reading a base's fuel** before it reckons a run's return. It assumes fuelling, as before.
- **The bombing of the stock on the beach,** which no rule here allows for.

## Rule card

- **Reads:** the side's bases, their supply and reserves; the forces limits of the holds; a force's ships, what is aboard and their fuel; the engine's measure of a passage.
- **Writes:** nothing of its own: the source in a forces limit, and a route.
- **Reuses:** the source and pool of decisions 0151 and 0152; the check of decision 0144; the supply formed by decision 0159; the route orders of a lift. No new state, event, draw or belief.

## What it costs

- **State:** none. The doctrine version rises.
- **Earlier scenarios.** A game with no "control an island" and no hold whose source differs from where its lifts came from plays as before. Scenario 77 keeps its orders: its lift's home is the source. The build lists each scenario whose orders or reasons change, and its new numbers go to the architect, not to the developer.
- **The 39 days change** on both sides: the source is named on day 1, the check of decision 0144 is made, and the lifts go on to the source. The scripted slice changes by Rabaul's fuel wherever a Japanese force lies there short of fuel. The recorded games on the slice are remade.
- **The rulebook:** chapter 8, "Controlling an island" and "Forces with men aboard", and the scenario's notes.

## The hand calculation

It is in `tests/scenario_86_fed_with_the_lift.rs`, written before the code. It is scenario 77 with the two spare forces taken away and the lift's home made a base with no supply. As the rule stood, the lift would land its men, go home, lie empty there, and that empty base would be named the source: the men would eat what they landed with and then go hungry. Now the source is the base with supply, named on day 1; the lift goes on to it, joins the pool on day 3, and on day 4 loads the run that lands on day 5.
