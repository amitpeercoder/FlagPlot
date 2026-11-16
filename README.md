# Flag Plot

An operational wargame of the Pacific War where the player commands by intent and a staff carries it out.

The name comes from the compartment on a flagship where the admiral and his staff tracked the battle and issued orders.

This page says what the game is for, what makes it different, where it stands and what is left. It was last brought up to date on Nov 16, 2026. The [design pillars](docs/pillars.md) are the source for the first two sections; everything else points at the page that holds the detail.

## The goal

The great Pacific War games model every ship, air group and pilot across the theatre, day by day, and make the player enter the orders for all of it by hand. A campaign is more than 1,600 daily turns of that.

Flag Plot keeps that depth in the simulation and takes it out of the player's hands. The player says what he wants: "hold Henderson Field", "deny these waters", "keep the garrison supplied at 30 days". A staff works out the forces, the patrols, the searches, the strikes and the convoys, gives the orders, and can say why it gave each one. The player holds whatever he chooses to hold himself, and hands it back when he is done.

It is a passion project for an audience of one, with no commercial target and no deadline. It succeeds if commanding this way is engaging and the interface is a pleasure to use.

## What makes it different

- **You command by stating ends, at any level.** An intent is a state of the world you want, not an activity. The staff breaks it into tasks for the commands beneath, and they into orders. You can state an intent to the whole area, to one base or to one task force, or reach down and give a unit an order yourself.
- **The staff shows its reasoning.** Every order, and every day on which the staff decides to give none, carries a reason: what it serves, up the chain to an intent you stated; what the staff knew; what limit shaped it; the working; and what would change the decision. Which level of command decided is part of the reason.
- **You know the enemy only through reports, and reports are wrong.** A sighting gives a place, a count and a kind of ship, and any of them may be mistaken. What your own people say they sank is a claim, never corrected. The staff plans on the same imperfect picture and is tested to make sure it never uses anything else.
- **Time runs until something matters.** A day resolves on its own. You set what is worth stopping for.
- **Every result can be traced.** Each event names what caused it, back to the order behind it, and each number can be opened to show what produced it.
- **It is checked against the record, and says where it misses.** Battles are run thousands of times and compared with what happened, measure by measure. A figure that rests on no source is marked as a placeholder, and a measure the game cannot meet is named as a miss with its cause, not tuned away.
- **The same game twice is the same game.** The same inputs and seed always give the same result, which is what makes testing, replays and bug reports possible.

## The ambition, as it has grown

The pillars of November 9, 2026 set the game out. Three things have been added by the designer since, each recorded as a decision.

- **Intents are ends, and they form a tree** (decision 0116). The first plan had the player choosing among activities: cover this, strike that, escort the other. Those are the staff's work. The player states an end, the staff forms the tasks under it, and the tree of an end with its order in time is the plan. What a plan needs to know is part of it: reconnaissance is something the staff plans because you asked for a place to be taken, not something you must remember to order (`docs/design/intelligence.md`).
- **A staff at every level of command, each with its own picture** (`docs/design/command-levels.md`, decision 0119). Each commander interprets the intent he is given and hands narrower intents down, to the level that acts. Each level knows what it has seen and what has been reported to it, about the enemy and about its own forces, and reports upward what it saw and did. The top commander sees the war late and second-hand, which is the reason he must command by intent. Commanders differ in initiative, and one who departs from his orders without cause may be relieved. Today one staff does this work on one picture a side, with every doctrine written for a single level so that the rest can be added without rewriting it.
- **A seat at every level, later** (decision 0129). Each level of a side is a seat that a player may fill, a player being a person or a model, and one player may fill several. He sees only what that level sees, and carries out and splits the intents of the levels above him. The game's own staff fills every seat no one takes.

The first playable slice is unchanged: the South Pacific from the Guadalcanal landings in August 1942 to the Japanese evacuation in February 1943.

## How it is built

Five steps, with a kill gate after the fourth, where a human plays a prototype for a few weeks and answers whether commanding by intent is fun, whether the staff can be trusted, whether the clock produces tension, and whether the day's story is worth reading. A "no" means reworking the design before the long work of the full build begins.

| Step | What it is | Where it stands |
| --- | --- | --- |
| 1 | A headless, deterministic simulation | Complete |
| 2 | Checking it against the record and tuning its figures | Complete, closed Nov 13, 2026 |
| 3 | The staff: the layer that turns intent into orders, for the player and for the enemy | In progress: stage 2 of its stages, all but its last proof |
| 4 | A prototype of the command loop with a minimal interface | Not started |
| 5 | The full build: the real interface, the complete order of battle, more of the war | Not started |

## Where things stand

**The simulation runs the war at sea, in the air and ashore.** Ships steam, burn fuel, carry cargo, refuel and rearm in port over the hours it takes, and are repaired. Aircraft search, strike ships and bases, fight, wear out, tire and rest. Forces fight on the surface by night. Troops are carried, landed, fed, fight, dig in, fall back and surrender, and an airfield is damaged and mended. A base changes hands. The rules as the code has them are in [`docs/rules/`](docs/rules/), in the order a day resolves.

**It has been checked against four battles.** Savo Island, the Eastern Solomons, the air campaign of August and September 1942 and Cape Esperance, the last run once as a check and never tuned on. Of 62 judged measures, 43 agree, 8 agree at the edge and 11 miss, each miss named with its cause ([`docs/validation/step2-close.md`](docs/validation/step2-close.md)). Thirty-nine days of the Guadalcanal campaign, from the landings to the battle of the ridge, run as one game.

**The staff can fight a side from stated ends.** Step 3 so far:

| Stage | What it is | Where it stands |
| --- | --- | --- |
| 0 | The choices a unit makes inside a day, as questions a side can ask | Built |
| 1 | The command contract: intents and limits, plans, holds and overrides, interrupts, reasons, running time; a staff that keeps a base supplied | Built |
| 2 | The fighting doctrines, on both sides | Built, but for its second proof |
| 2b | Command levels that differ in what they know and report | Planned; not built |
| 3 | Taking and holding a place: the operation and its phases, landing, reconnaissance | Not started |
| 4 | The trial: the staff plays a plan forward to estimate what it needs and costs, and lays out courses to choose between | Not started |
| 5 | The proof of step 3, and the interface the prototype needs | Not started |

What a commander can state today, and what the staff does with it:

| You state | The staff |
| --- | --- |
| Keep a base supplied | Plans and times the supply runs from a pool of shipping, gives a run an escort when it believes the run is threatened, and can post a covering force |
| Hold a place | Flies its fighters on patrol, points its searches at what is known, rests its groups, and can stand surface forces off it by night |
| Neutralise a base | Gives each of your bases in reach the task of raiding it; each base decides which mornings its bombers go, from their fatigue, the weight they can send, their escort and what the last raid cost |
| Deny an area | Tasks the bases and the carrier forces in reach to search those waters and strike ships; sends named surface forces to stand there on the nights it believes in a threat |

Fuel and ammunition are policy, not intents: a force short of either stays in until it has what it needs, and the staff says from which day it can be used. The risk level you give an intent changes what the staff does with the same information.

**The first proof of stage 2 passed.** The air campaign over 28 days was played a thousand times with every order removed and each side given only its ends. The staff flew eleven raid days, the number in the record, on its own rhythm. Its second proof, a carrier battle and then a week of the campaign played the same way, has its pass marks written and confirmed before the run and has not been run.

**What is real and what is not.** The ships, aircraft and land forces of the battles above are historical, each rating derived by a written rule from sourced facts in `data/oob`, and the map of the South Pacific is real geography. Most of the figures that drive combat, wear, fatigue, supply and the staff's judgement are still placeholders, each marked, with a note of what it rests on. The small test scenarios use invented ships and maps and say so.

## What is left

In the order it is planned. [`docs/open-work.md`](docs/open-work.md) keeps the full list.

1. **The second proof of stage 2,** and with it a test that a reader can follow the staff's reasons, a game played by a model through intents alone, and a measurement of speed.
2. **Stage 2b: command levels.** A picture for each command, reports between them, a commander's knowledge of his own forces by report, and commanders with initiative. It changes what every doctrine reads, and some figures of step 2 will have to be fitted again.
3. **Stage 3: the operation.** Taking a place from one statement: the phases of a plan, reconnaissance as part of it, landing and what follows. With it, three rules the land fighting still needs, found when the losses at the Tenaru could not be fitted.
4. **Stage 4: the trial.** The staff estimating what a plan needs and costs by playing it forward, and offering a commander real choices.
5. **Stage 5 and step 4: the proof and the prototype.** Then the kill gate.
6. **Step 5: the full build,** if the gate is passed.

Known limits of what is built, which the later stages are meant to lift:

- The staff does not choose where a carrier force should stand, and does not move a force's home.
- A force that stands by night steams home in daylight, because orders are taken once a day.
- Nothing holds a strike to an area: a base told to deny some waters strikes what comes within its reach.
- One escort force goes with a run or none does; the record gave no rule for how many.
- There is no weather.
- The replay viewer shows the truth for an umpire, and is not the game's interface.

## Documents

- [Design pillars](docs/pillars.md): what the game is and why, and the five pillars every feature is tested against.
- [The rules](docs/rules/): every rule as the code has it. [Glossary](docs/glossary.md): one meaning for each word.
- [Decisions](docs/decisions/): each design choice and its reason, with an index.
- [Design notes](docs/design/): what is thought through and not yet planned.
- The specs of [step 1](docs/step1-spec.md), [step 2](docs/step2-spec.md) and [step 3](docs/step3-spec.md).
- [Validation](docs/validation/): the reports of each battle against the record.
- [Commands](docs/commands.md): what each command does.
- [CLAUDE.md](CLAUDE.md): the rules the code follows, and where everything else is.

## Running it

You need Rust. The toolchain version is pinned in `rust-toolchain.toml`.

```
cargo test --workspace
cargo run -p sim-cli -- run data/scenarios/01_transit.toml --events
```

The second command runs scenario 1 and prints the state hash of each day, the full event log with what caused each event, and where every ship ended up.

To run a historical battle, give it `data/scenarios/04_savo_island.toml` or `data/scenarios/05_eastern_solomons.toml`. To see a staff at work, run `data/scenarios/48_hold_a_place.toml` or `data/scenarios/50_neutralise_a_base.toml`.

`viewer/` holds a read-only replay viewer: `sim-cli export` writes a run as one file and `viewer/index.html` draws it on the map. See [viewer/README.md](viewer/README.md).
