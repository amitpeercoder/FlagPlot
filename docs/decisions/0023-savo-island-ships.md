# 0023: The Savo Island ships: exceptions and assumptions

Status: confirmed by the designer, Nov 10, 2026. He approved the settlement sheet (`docs/research/oob/savo-island-settlement.md`) in the order-of-battle session and confirmed it to the architect the same day. How the tool carries the two things the sheet left to it, the designed-rate exception of decision 2 and the 9,999 marker of decision 3, is the architect's engineering choice.

## The gap

Decision 0020 says the derivation tool reads only `preferred` fact rows, and that an exception has to be written down. After the settlement sheet was applied to the facts, the tool still refused all 20 ships, with 28 problems of three kinds:

- A figure was chosen, but its row rests on weak sources only, so the format's own rule cannot make it `preferred`.
- A gun has no practical rate of fire, only a designed one, which decision 0020 does not read.
- Three ships have no figure for main-gun ammunition in any source read.

## Decisions

1. **The chosen weak rows are read by exception.** Each is listed in `data/derivation/rules.toml` with the sheet item it comes from:

   | Ships | Fact | Value | Sheet item |
   | --- | --- | --- | --- |
   | Bagley, Patterson, Helm, Blue, Ralph Talbot, Jarvis | Displacement | 1,624 tons | b4 |
   | Wilson | Displacement | 1,657 tons | b5 |
   | Chicago | Displacement | 9,050 tons | b3 |
   | Yunagi | Displacement | 1,270 tons | group 1 |
   | Yubari | Endurance and cruise speed | 3,310 nm at 14 knots | group 3 |
   | Yubari | Fuel | 902 tons | group 4 |
   | Chicago | Main guns | Nine 8-inch | group 7 |
   | Chokai, Aoba, Kako, Kinugasa, Furutaka | Rate of fire | 2.4 a minute | e1 |
   | Tenryu, Yubari | Rate of fire | 6 a minute | e2 |
   | Canberra | Torpedo tubes and type | Eight, Mark VII | r2 |

2. **A designed rate of fire may be read, by a named exception, where no practical rate can be.** This widens decision 0020, which read practical rates only. The exception names the weapon and the designed-rate row. The sources file says the figure is a designed rate.

   | Ships | Gun | Value | Sheet item |
   | --- | --- | --- | --- |
   | Yunagi | 12 cm | 5.5 a minute | e3 |
   | Vincennes, Quincy | 8-inch Mark 12 | 3 a minute | e4 |
   | Astoria, Chicago | 8-inch Mark 9 | 3 a minute | e5 |

   For the Mark 9 a practical figure exists (5 a minute) and is passed over: it is one ship's best quarter-hour in another battle, and it would give two cruisers a rate two-thirds higher than their sisters.

3. **Main-gun ammunition for Tenryu, Yubari and Yunagi is treated as not limiting.** No source read gives it. Tenryu fired 80 rounds that night and Yubari 96, far below any plausible outfit. This is an assumption of the scenario's ship list, like the radar assumption of decision 0019, and it applies only while the facts are silent. The tool writes 9,999 rounds per gun for these ships. That number is a marker, not an outfit: a fight of six five-minute rounds at 6 rounds a minute uses 180. The sources file says so beside the figure.

4. **Canberra is off the "unknown radar means not fitted" list.** Her Type 271 set is now a settled fact (sheet r1), so the facts are read. Vincennes and Jarvis stay on it: no source read says whether they had a search set.

## Changed on Nov 11, 2026

The Eastern Solomons settlement sheet (`docs/research/oob/eastern-solomons-settlement.md`), approved by the designer, moved three Savo Island classes to the Navy fuel tables' normal full load at 15 knots, so that endurance and fuel come from one condition: New Orleans 6,500 nm with 1,900 tons, Bagley 6,080 nm with 506 tons, Benham 4,860 nm with 491 tons. The class file was regenerated and the scenario's starting fuel follows it. No Allied ship moves at Savo Island, so only the fuel shown, and so the state hash, changed. A scenario ship that starts with more fuel than its class holds now fails to load.

## Known weaknesses

The architect's review of the sheet. None blocks the scenario; each bears on how far its result can be trusted.

- **Tenryu's displacement is probably not standard displacement.** The chosen 3,948 tons reads like normal displacement. If her standard displacement is near 3,230 tons, as the sheet suggests from memory, she is about a fifth tougher than she should be against the other ships. A sourced standard figure is wanted before the Savo result is compared with history.
- **The rates of fire are four kinds of figure:** a table of unstated kind for the Japanese 20 cm and 14 cm guns, designed rates for the US 8-inch and the Japanese 12 cm, a sustained rate in action for the British 8-inch, and a British trial for the US 5-inch. They are the best available and they are not comparable. The Savo scenario should say how much its result moves when they are changed.
- **Design speeds overstate 1942 service speeds** for every ship, and so favour none.
- **The US Mark 15 warhead (224 kg)** rests on indirect evidence. The other candidate, 374 kg, would raise the torpedo damage of seven destroyers by two thirds.

## Deferred

- A single rule for rate of fire that does not depend on which kind of figure a navy's sources happen to give.
- A service-speed allowance.
- Torpedo reloads, which the tool does not read.

## Rule card

No simulation rule. The tool reads `data/oob`, `data/derivation/rules.toml` and the scenario's ship list, and writes `data/reference/ship_classes_savo_island.csv` and the two files beside it. The simulation reads only `data/reference`.
