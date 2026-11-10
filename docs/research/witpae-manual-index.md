# WitP:AE Main Manual: section index for Flag Plot step 1

Reading map of `WitPAE_Manual.txt` (332 pages) for our test scenarios 1 to 6. The manual is not in the repo; it sits in the gitignored `references/extracted_pdfs/` folder. This index cites section numbers and printed page numbers only, and every description is in our own words.

Page = printed page number (bottom of each page in the text). Line numbers for the recommended reads are in the final report, not here.

## Scenario 1: Transit (movement, fuel burn, arrival time)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 3.0 | Sequence of play | 31 | Order of the night and day resolution phases, and where movement, fuel use and repair fall inside them |
| 6.2.3 | Task force speed | 91 | How the slowest ship sets TF speed, how knots become hexes per 12-hour phase, the one-hex floor and the per-phase cap |
| 6.2.3.1 | Automatic ship separation | 92 | When a slow or crippled ship splits off into its own escort force and the parent decides to go on or abort |
| 6.2.4 | Patrol/retreat and max react range | 93 | What a force does on arrival and how easily it aborts under attack |
| 6.2.5 | Mission types and their impact on movement | 93 | Per-mission movement habits: night run-ins, react or not, retreat logic (the per-mission detail table is in an appendix that is not in this text) |
| 6.2.7 | Assigning TF destinations | 94 | Destination hex, follow, meet and patrol-zone orders; when a force ignores its destination |
| 6.1.2.1 | TF destination, routing, waypoints | 80 | Routing choices and route safety levels set from the TF screen |
| 6.2.9 | TF returning to home base | 95 | The events that automatically send a force home, and the cases that exempt a loaded transport |
| 6.2.13 | Ship endurance | 105 | What spends endurance (steaming, idling, air launches, combat) and the low-fuel state |
| 6.2.13.1 | Refuelling in port and at sea | 106 | Where and how ships refuel, the refuel options, and why low-endurance ships suit short missions only |
| 6.2.14 | Operation points | 107 | A per-phase time budget that refuelling, loading and rearming draw down, cutting movement |
| 6.5.1 | Operational damage and repairs at sea | 133 | Wear-and-tear damage while steaming, much worse at full speed |
| 4.2.1.2 to 4.2.1.3 | Hexsides and straits | 38 | Reef and mountain hexsides that block ships; strait hexsides that raise mine and coast-gun risk |

## Scenario 2: Supply run (convoy unloads at Guadalcanal)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 6.1.1.1 | Task force missions | 76 | The TF mission list; which carry cargo (transport, cargo, tanker, fast transport, amphibious, landing craft, support) and their size limits |
| 6.2 | Creating a task force | 90 | Forming a TF at a base, size limits, ship eligibility per mission, diminishing returns in large forces |
| 6.2.2 | Disbanding task forces | 91 | Disbanding in port counts as anchoring, with no size limit |
| 6.2.8 | Transferring ships between TFs | 95 | Moving ships between forces in the same hex or port |
| 6.2.15 | Docking | 107 | Docked versus anchored forces: repair, vulnerability, endurance, forced undocking on contact |
| 6.3.1 / 6.3.1.1 | Port restrictions and docking restrictions | 109 | Port size caps the largest ship and the total tonnage that can dock; exceptions for amphibious forces; undocked work is slower |
| 6.3.2 | Task force loading | 110 | What a TF may load, the load instruction on the TF screen, one air or ground unit per ship, units split into sub-units |
| 6.3.2.1 | Load types | 110 | The six load kinds (units, air groups, supplies, resources, fuel, oil) and the equipment classes |
| 6.3.2.2 | Ship capacity types | 111 | Troop, cargo and liquid space; natural loading versus cross-loading and what may go where |
| 6.3.2.3 / 6.3.2.3.1 | Capacity usage; ship type carrying ability | 112 | Load-cost multipliers by device class and a table of which ship types may carry which loads, some only as fast transports |
| 6.3.2.4 to 6.3.2.5 | Cross-load adjustments, daily disruption, unload damage | 117 | Penalties for troops in cargo space and cargo in troop space, per-day disruption while embarked, damage while unloading, by ship type |
| 6.3.3 / 6.3.3.1 | Load procedure and load allocation | 122 | How units are assigned to ships: commercial versus combat load, "use all ships" versus "use minimum ships", fetching more ships |
| 6.3.3.2 | Task force load | 124 | The four limits on a load: item rate, ship ability, port ability, docking status; how a port runs several ships at once |
| 6.3.3.2.1 | Item load rate | 124 | Per-phase rates by cargo class, each a function of port size; troop loading also eats cargo handling |
| 6.3.3.2.2 to 6.3.3.2.4 | Ship and port load ability; rate table | 125 | Per-ship per-phase rates and daily port totals by port size, separate for liquids; fast-transport and sub-transport rates (table headings are scrambled in the text) |
| 6.3.3.2.5 | Port load rate adjustments | 126 | How port damage, naval support troops, undocked status, refineries and local production change rates |
| 6.3.3.3.1 | Normal unloading | 127 | Unloading uses the loading rates; the daily port total is shared between loading and unloading |
| 6.3.3.3.2 | Amphibious unloading | 127 | Unload by amphibious forces ignores port limits; over-the-beach and small-friendly-port variants (the latter on p. 128) |
| 8.3.4 | Transporting ground units | 193 | Which ships carry troops, unloading over a beach is slow, defenders can fire on ships unloading at an enemy base |
| 9.0 / 9.1 | Bases; base information screen | 203 | What a base is and does; the figures that matter: supply on hand versus required, fuel requested, support and engineers, facilities |
| 9.2.1.1 | Beach | 207 | A coastal hex that can become a base; relevant to a landing site with no port |
| 9.2.2 | Base composition | 208 | Port size, its natural potential size, stockpiles, overland redistribution between bases and the loss it incurs |
| 9.3 | Ports | 210 | What ports do; docked ships take no endurance loss; anchored ships in larger ports are safe from submarines |
| 9.3.1 | Port damage | 210 | Damage slows loading, unloading, refuelling and repair; engineers repair it |
| 9.3.3 / 9.3.3.1 | Port size; ships docked | 211 | Table of capabilities by port size; the docking limit is in tonnage and anchoring has no limit |
| 9.3.3.2 | Cargo and fuel handling | 212 | Daily cargo and fuel ceilings, what consumes each, rearm levels, naval support bonus |
| 9.3.3.3 | Automatic transfer of bulk cargo between adjacent ports | 212 | Free local shipping of supply and fuel between neighbouring friendly ports, capped by the smaller port |
| 9.4.2 | Base construction and repair | 214 | Engineers repair first, then build; construction burns supply and can be switched off |
| 15.0 | Logistics (intro) | 249 | Supply and fuel as the two stocks; needs estimated from recent use; effects of being undersupplied |
| 15.1 | Supply operations | 250 | Twice-daily needs calculation, overland movement, limit on how much a ship may draw from a poorly stocked base, order of daily supply steps |
| 15.2 | Naval supply | 250 | Rearming at ports from base supply by rearm points |
| 15.3 / 15.3.1 | Ground unit supply; supply and fatigue effects | 250 | Supply paths and values; how short supply and fatigue cut combat output |
| 15.4 | Air unit supply | 252 | Supply a base needs for its aircraft to fly at full strength, and spend per mission |
| 15.6 | Capturing base resources | 253 | A base only produces its daily allotment for its own nation's side |
| 15.7 | Spoilage | 254 | Stock above a size-based level is wasted; larger bases waste less |
| 6.4.2 | Surface combat | 130 | A TF stopped to load or unload is a sitting target; why escorts matter for a convoy |

## Scenario 3: Search (flying boats, detection, reports, ageing)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 10.0 | Spotting units (intro) | 217 | The ways a side learns about the enemy; fog of war meaning; reduced chance at long search range |
| 10.1 | Detection levels | 217 | Current and maximum detection level per unit; current level drives combat, maximum level drives what the map shows |
| 10.1.1 | Changing detection levels | 218 | Events that raise or lower detection by unit type (aircraft sightings, subs, coastwatchers, decay each phase) |
| 10.1.2 | How maximum detection levels change | 220 | Maximum level rises with detection and decays slower, so stale knowledge persists |
| 10.2 | Recon flights | 220 | Per-unit spotting chance tied to pilot skill and plane type; bombers give a weaker by-product |
| 10.4 | Information given regarding enemy units | 221 | What the player is told at each knowledge level: location only, class lists, rough counts; error shrinks with sustained contact |
| 7.2.1.1 | Mission types | 157 | The air mission list, including naval search and recon |
| 7.2.1.2 | Day air operations | 157 | Order of search, strike and landing steps within the morning and afternoon air phases |
| 7.2.1.8 | CAP, naval search levels | 164 | Percentage of a group assigned to search versus other duties |
| 8.7 | Coastwatchers | 201 | Allied shore observers that report enemy ships in coastal hexes and where they exist |
| 18.1.1 | Fog of war | 266 | What the option hides and randomises: unit sightings, loss figures, delayed sinking reports |
| 5.5 | Ops report | 74 | How results reach the player as messages after resolution (the model for our reports) |

## Scenario 4: Night surface action (Savo Island)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 6.1.1.1 | Task force missions | 76 | Surface combat and bombardment forces, size limits; transport forces as targets |
| 6.2.4 / 6.2.5 | Patrol/retreat; mission types and movement | 93 | Night approach habits, react-or-not rules, when forces decline combat against stronger enemies |
| 6.4 / 6.4.2 | Naval combat; surface combat | 129 | Which phases allow ship fights, when contact triggers combat, a second round if still in the hex, stopped forces at a disadvantage |
| 6.4.2.1 | Torpedo duds | 130 | A date-driven reduction of Allied torpedo failure (and its option override) |
| 10.3 | Radar detection levels and surface combat | 221 | Per-ship detection inside a fight, radar raising enemy visibility, burning ships standing out at night |
| 10.1.1 | Changing detection levels | 218 | How a force's visibility moves with its actions |
| 6.5 | Damage to ships | 132 | The four damage tracks plus weapon loss, critical hits, flooding and fire progression, speed effects |
| 6.3.5 | Naval ammunition and retreat | 129 | A low-ammunition test that sets a force to withdraw |
| 6.3.6 | TF officers | 129 | Commander traits that affect combat and fuel use |
| 19.2 | Notes on combat (naval) | 270 | Qualitative description of how surface battles are staged, including the effect of darkness and fog |
| 15.2 / 9.3.3.2 | Naval supply; rearm levels | 250 | How ammunition is restored and which ports can rearm which weapons |
| 6.2.3.1 | Automatic ship separation | 92 | Crippled ships leave the line and run for home |

## Scenario 5: Carrier duel (Eastern Solomons)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 6.1.1.1 | Task force missions | 76 | Air combat and carrier-escort forces |
| 6.2.5 | Mission types and movement | 93 | Carrier forces seek enemy carriers and avoid enemy surface forces |
| 6.3.4 | Special aircraft carrier movement | 128 | Automatic closing move of a carrier force toward an enemy carrier force after search |
| 7.0.1.1.1 | Basing carrier, patrol and float aircraft | 141 | Which aircraft can operate from carriers, training versus capability, air-unit limits per carrier |
| 7.0.1.1.1.1 | Emergency landings | 142 | Where aircraft go when their carrier is disabled and what overflow does to losses |
| 7.2 | Air sequence of play | 155 | Day and night air phases; air units stay put unless transferred |
| 7.2.1 | Assigning aircraft missions | 156 | The components of an air unit's orders (mission, target, level, altitude, day or night); "commander discretion" when no target is set |
| 7.2.1.2 / 7.2.1.3 | Day and night air operations | 157 | Step order within air phases and what may fly at night |
| 7.2.1.8 | CAP and search levels | 164 | Splitting a group between CAP, search and strike |
| 7.2.1.9 | Altitude selection | 164 | Trade-offs of altitude for flak, accuracy and fighter performance |
| 7.2.1.10 | Impact of ordnance | 165 | Carrier sortie limits and torpedo supply, with fallback to bombs |
| 7.2.1.11 | Coordinating strikes | 166 | How strikes from several sources merge, split up or arrive piecemeal; larger carrier air wings coordinate worse |
| 7.2.1.12 | Weather and aborted missions | 167 | Bad-weather hexes block launch and attack |
| 7.2.1.13.2 | Carriers in base hexes | 167 | Reduced air operations from a carrier inside a base hex |
| 7.2.1.14 | Operational losses | 168 | Non-combat losses by mission range, pilot skill and carrier basing; write-offs |
| 7.4 | Air combat | 171 | Raids as the unit of resolution, break-up into smaller raids, sweeps before bombers |
| 7.4.1 / 7.4.1.1 | Combat air patrol; CAP and radar | 172 | CAP readiness tiers, reacting to nearby targets, how radar warning time governs interception |
| 7.4.2 | Air-to-air combat | 173 | What decides fighter engagements: skill, manoeuvre, speed, altitude |
| 7.4.2.2 | Bombers in combat | 174 | Survivability by bomber type, unescorted loss patterns |
| 7.4.2.5 to 7.4.2.7 | Range, incidental combat, damage | 175 | Fatigue from long flights, return-trip losses, incidental engagements, damage carrying over |
| 6.5 / 14.2.1 | Damage to ships; damage and repairs | 132 | Damage tracks and the major versus normal damage split that drives repair needs |
| 15.4.1.2 | Aircraft status on ships | 253 | Conditions that stop carrier aircraft flying, capacity overload limits |
| 10.1.1 / 10.2 | Detection changes; recon | 218 | How strikes and searches raise a carrier force's visibility |
| 19.3 | Notes on combat (air to air) | 271 | Qualitative account of how a fighter engagement unfolds, useful as a design checklist |

## Scenario 6: Air attrition (Rabaul against Guadalcanal)

| Section | Title | Page | What it covers |
| --- | --- | --- | --- |
| 7.1 | Air unit information screen | 143 | Group state: ready, damaged, reserve; pilots with fatigue and experience; training; stand-down |
| 7.2.1.4 | Morale and air missions | 159 | Morale and tests that cut the number of aircraft that actually fly |
| 7.2.1.5 | Air HQs and air missions | 159 | Distance from an air HQ cuts sortie strength |
| 7.2.1.13.1 | Size 1 airfields | 167 | Mission types barred from the smallest fields |
| 7.2.1.14 | Operational losses | 168 | Loss sources and the pilot outcomes |
| 7.2.3 | Transfers | 169 | Rebasing rules: one move per orders phase, overland ferry arrives damaged |
| 7.4.2.5 | Range | 175 | Long missions raise fatigue and loss (flight from a far base is worse) |
| 7.4.2.7 / 7.4.2.7.1 | Damage; maintenance | 176 | Damage accumulates, fatigue forces maintenance, standing down recovers airframes |
| 9.4 / 9.4.1 | Airfields; airfield damage | 213 | Field size effects on losses, range and bomber suitability; stacking limits; runway and service damage limits |
| 9.4.2 | Base construction and repair | 214 | Repair order and supply cost of fixing and growing airfields |
| 15.4 / 15.4.1 | Air unit supply; aircraft status | 252 | Supply and aviation support needed for readiness, daily status changes, supply spent per sortie |
| 16.2 | Air units (replacements) | 255 | Conditions under which a group receives replacement aircraft and how often |
| 16.3 | Pilot replacements | 259 | Three pilot pools, quality of replacements, order in which pilots are picked |
| 15.1 | Supply operations | 250 | Daily order of repair, morale, replacement and upgrade steps |
| 7.4.1 | Combat air patrol | 172 | Defender strength at the target airfield |
| 12.1 | Weather | 224 | Weather forecast effect on missions |

## Cross-cutting

| Topic | Section | Page | What it covers |
| --- | --- | --- | --- |
| Sequence of play | 3.0 | 31 | Orders then paired night and day resolution; the full phase list with cross-references |
| Sequence (air detail) | 7.2.1.2, 7.2.1.3 | 157 | Air phases inside each resolution phase |
| Fog of war, detection | 10.1 to 10.4; 18.1.1 | 217 | See scenario 3 |
| Weather | 12.0, 12.1; 18.1.2; 7.2.1.12 | 223 | Climate zones with fatigue effects, a daily forecast per region, and bad weather blocking air missions |
| Leaders | 11.1, 11.1.1; 6.3.6; 11.0 | 222 | Replacing leaders costs a political currency; TF commanders and their traits |
| Ship damage | 6.5; 14.2.1 | 132 | Damage tracks, major damage, how damage picks the repair order |
| Ship repair | 14.2, 14.2.2, 14.2.3 | 239 | Repair modes (own crew, pierside, repair ship, yard), capacity by port size, per-ship assignment |
| Ship repair detail | 14.2.4; 9.3 | 248 | Weapon repair and port role in repair |
| Endurance, refuelling | 6.2.13; 6.2.13.1; 6.2.14; 9.3.3.2; 15.0 | 105 | See scenario 1; port fuel ceilings; fuel as a stock |
| Ship type reference | 20.1.2; 20.1.2.1; 20.1.2.2 | 273 | Ship type codes, which TF type takes which ship, rearm requirements |

## Out of scope for step 1

| Section(s) | Pages | Why not yet |
| --- | --- | --- |
| 1.0, 2.1 to 2.3, 2.5 to 2.7, 4.1, 4.2.4 to 4.2.7, 5.1 to 5.4 | 13 to 74 | Interface, menus, hotkeys, display screens: no UI in step 1 |
| 2.4, 18.1 (except 18.1.1 and 18.1.2) | 20 to 24; 266 to 269 | Optional rules switches, scenario start flags |
| 6.2.10 | 96 | Off-map movement |
| 6.2.11, 15.5 | 104; 253 | Computer-run convoys; belongs to the staff planner, later |
| 6.2.12, 6.4.4, 6.7 | 104; 131; 138 | Submarines and anti-submarine warfare |
| 6.2.16 | 107 | Building PT boats, barges, midget subs |
| 6.1.3.1, 14.1 | 88; 237 | Ship upgrades and conversions |
| 6.3.3.4, 6.3.7 | 128; 129 | Evacuations, shakedown cruises |
| 6.6 | 134 | Mines and sweeping |
| 6.8, 6.9 | 139 | Coast-gun fire on passing forces (maybe later), withdrawals |
| 7.2.1.13.3, 7.4.2.3 | 167; 175 | Atomic bombs, kamikazes |
| 7.2.4, 7.3 | 170; 171 | Air transport, group resizing |
| 8.0 to 8.11 (keep 8.3.4) | 176 to 203 | Ground units and ground combat; we keep it simplified |
| 9.5, 11.0 | 216; 222 | Command organisation and political points: no equivalent in step 1 |
| 13.0 | 225 | Production system |
| 15.8, 16.1, 16.2.3, 16.4 to 16.6 | 254 to 262 | Burma Road, reinforcements, upgrades, ground and base replacements |
| 17.0 | 263 | Victory conditions |
| 18.2, 18.3 | 269 | Survivor rescue, starting factory damage |
| 20.2 to 20.5, 21, 23, 24 | 288 to 323 | Island sizes, straits list, PBEM, designer notes, credits, index |

## Pain points (clerical work the staff planner should absorb)

1. 6.3.2: the player must tell each loading TF which cargo class to load (troops, supplies, fuel, oil, resources).
2. 6.3.3.1: land units must first be put into the right movement mode, then the player picks a load method and asks for more ships if the first allocation falls short.
3. 6.3.1.1: when a TF exceeds the port's dock limit, the player must split it by tonnage into dockable groups by hand.
4. 6.3.2.3.1: the player must know which ship types carry which load and which only as fast transports, when forming each TF.
5. 6.2.13.1: choosing the refuel option per TF and keeping low-range ships out of long fast runs, with fuel pre-positioned at forward ports.
6. 6.2.9: after combat or an abort, forces head home on their own; the player must check and re-order them next turn.
7. 6.2.7 and 6.1.2.1.4: routes are set by hand through destinations, waypoints and a safety level.
8. 6.2.3.1 and 6.2.8: damaged ships split off into new escort forces that need fresh orders; transfers between forces are manual.
9. 14.2.2: each damaged ship is assigned a repair mode and priority one at a time (with a bulk option in 14.2.2.2).
10. 9.4.2: engineers' construction is switched on and off per base and per facility so it does not burn scarce supply.
11. 7.2.1: every air group carries six order fields, set group by group, with "set all" shortcuts that skip groups on the other day/night setting.
12. 7.2.1.8: CAP and search shares are set per group in steps.
13. 7.2.3 and 16.2: rebasing is one manual move per group per turn, and replacements need the group to be set to accept them.
14. 7.1 and 16.3: new pilots can be added by hand to groups short of pilots.
15. 11.1: replacing a leader or reattaching a unit to a different headquarters is a manual political-point purchase.

## Notes on the other two manuals in the folder

**WitpStaff_Manual.txt (862 lines).** A fan-made Java tool by a third party that reads a saved game into a database and shows stock levels, industry, aircraft classes, leaders, map overviews (naval transport, search, supply) and inconsistency warnings. It exists because the game gives players hundreds of knobs but little summarised information. Useful to us as evidence of which summaries a staff needs (supply flow, search coverage, spoilage, leader choice), not for mechanics. Skim chapters 4 and 5 for ideas; the rest is install and configuration.

**WitPAE_Editor_Manual.txt (2023 lines).** The scenario and database editor guide: field-by-field descriptions of devices, ship classes, ships, aircraft, air groups, locations (bases, task forces, land units), pilots, leaders and scenario settings, plus AI data lists. Useful as a checklist of what attributes each entity carries (for example ship endurance, fuel and cargo capacity; location fuel and daily arrivals), and for field meanings when we write our reference data. It describes data entry, not rules, so it adds little to the mechanics.
