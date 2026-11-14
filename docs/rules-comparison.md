# Flag Plot and WitP:AE: implemented rules comparison

14 November 2026. Flag Plot baseline: `7ad171b`.

Checked against the code on 14 November 2026 at `93c69fa`, the tip of main by then. What landed since the baseline is decisions 0102 to 0104, which are records only and change no rule, so no statement below about Flag Plot changed.

## Scope and sources

This compares the rules implemented in Flag Plot at the baseline above with the supplied 2009 *War in the Pacific: Admiral's Edition* main manual. It covers simulation rules, rather than interface quality or the proposed staff planner. It does not assume that later AE patches follow every statement in this manual.

The AE source is the local, gitignored `references/extracted_pdfs/WitPAE_Manual.pdf`, read through its extracted text and checked against the PDF where needed. References below name manual sections and printed page numbers. The [manual index](research/witpae-manual-index.md) provides the reading map. This document paraphrases mechanics; it reproduces no AE tables, database values or formula constants.

Flag Plot's sources are the [current rulebook](rules/README.md), with selected implementation checks in `crates/sim-core/src/`. Chapter links accompany each comparison. The [validation close](validation/step2-close.md) establishes the limits of numerical validation. This is a rules comparison, not evidence that either game is more historically accurate overall.

**Missing** means absent from Flag Plot at this baseline, whether or not it has been discussed for later work. **Different** means Flag Plot has a counterpart with another abstraction or resolution method. Neither label is a recommendation to copy AE.

## Main finding

Flag Plot already has the operational cycle: move, search, fight, unload, consume supplies and repair. It tracks individual ships, air groups, land units and bases. AE models substantially more constraints and interactions within that cycle.

The largest differences are not just missing unit types. Shared systems such as ship damage, air combat, ports and ground combat use much simpler rules in Flag Plot. A named ship being tracked individually does not imply that its weapons, protection, machinery, crew and damage consequences are represented at AE's level of detail.

## 1. Systems absent from Flag Plot

| System in AE | Manual reference | Consequence of its absence in Flag Plot |
| --- | --- | --- |
| Submarines and anti-submarine warfare | 6.2.12, 6.4.4, 6.7; pp. 104–105, 131–132, 138 | No submerged threat to shipping, submarine reconnaissance, depth-charge combat or ASW air patrols |
| Mines and minesweeping | 6.6; pp. 134–137 | No persistent mine threat to ports, straits or landing approaches |
| Weather and climate | 12.0–12.1; pp. 223–224 | No weather cancellations or climate-related operational penalties |
| Commanders, HQ organisation and political points | 6.3.6, 9.5, 11.0–11.1; pp. 129, 216, 222–223 | No leader-dependent performance, HQ support radius or political reassignment costs |
| Industry and resource economy | 13.0; pp. 225–236 | No resource extraction, refining, industrial production, research or shipbuilding economy |
| Ship upgrades and conversions | 14.1; pp. 237–239 | Ships retain their existing capabilities |
| Signals intelligence, coastwatchers and dedicated reconnaissance | 5.4, 8.7, 10.2; pp. 74, 201, 220 | Fewer ways to discover the enemy or assess a base before attacking |
| Air transport and airborne operations | 7.2.4; p. 170 | No movement of supplies or troops by aircraft |
| Theatre and period-specific systems | 6.9, 8.5–8.9, 18.0 | No equivalent withdrawal obligations, partisan/garrison systems, atoll stacking rules or late-war special systems |

These are missing branches of the simulation. The sections below compare systems that both games already have.

## 2. Movement, time and fuel

Flag Plot source: [chapter 1](rules/01-the-day-orders-and-what-a-side-knows.md), Time; [chapter 2](rules/02-movement-fuel-and-cargo.md), Movement and Fuel.

| Rule | WitP:AE | Flag Plot |
| --- | --- | --- |
| Naval movement | Rounded hex allowances in 12-hour phases, with a maximum and a minimum movement allowance | Distance and progress along legs, using speed and available hours |
| Force speed | Normally limited by the slowest ship | Also limited by the slowest ship |
| Out of fuel | Minimum movement remains available for playability | Fuel exhaustion stops movement |
| Damage and speed | Engine, system and flotation damage can reduce speed | Named propulsion-system condition determines damage-related speed reduction |
| Fuel expenditure | Movement, undocked activity, combat and aircraft operations consume endurance | Distance steamed consumes fuel; stationary ships burn nothing |
| High speed | Increased endurance expenditure and operational damage | A speed-dependent fuel multiplier, without steaming-related machinery wear |
| Refuelling | In port or at sea, with transfer limits and time costs | Automatic at friendly bases at day start, from the base's fuel stock; no handling time, berth use or port-rate limit |
| Mission behaviour | Mission types shape approach, reaction, retirement and movement | Mission currently affects bombardment; most behaviour comes from explicit orders and engagement settings |

Flag Plot has waypoints, action at intermediate waypoints, break-off rules and straggler handling. Those are not missing systems. However, it lacks AE's reactive movement, underway replenishment, operational wear and general operations-time budget.

AE's operation points charge time for refuelling, rearming and cargo work. Flag Plot accounts for cargo time directly, but refuelling has no corresponding cost and ordinary rearming is absent.

Flag Plot resolves a day as night, morning and afternoon pulses. It keeps elapsed time, but has no engine calendar date or geographic daylight calculation. AE's naval movement has night/day phases and its air operations have morning/afternoon phases; its calendar also drives period-specific rules.

**Operational effect:** Flag Plot can represent passage times and individual fuel limits, but sustained fleet activity has fewer costs beyond the distance sailed. Its zero-fuel stop is stricter than AE's minimum-movement concession.

AE references: 3.0, 6.2.3–6.2.5, 6.2.13–6.2.15, 6.3.4; pp. 31–33, 91–107, 128.

## 3. Ports, cargo and supply distribution

Flag Plot source: [chapter 2](rules/02-movement-fuel-and-cargo.md), Port size, Cargo and Refuelling; [chapter 7](rules/07-land-units.md), Men worked as cargo and Preparing a landing.

Flag Plot carries cargo and troops aboard individual ships, shares berths, distinguishes berth handling from lighters and calculates loading completion times. Loading preparation affects landing performance. The principal differences are the constraints around those mechanisms.

| Rule | WitP:AE | Flag Plot |
| --- | --- | --- |
| Capacity | Troop, cargo and liquid spaces, with compatibility and cross-loading rules | Separate troop capacity and a shared supply/fuel cargo capacity |
| Troops and equipment | Equipment load costs and loading methods affect capacity use | Men occupy troop capacity independently of cargo; equipment is abstracted |
| Docking | Limits individual ship size and total docked tonnage | Berth count, without individual ship-size or total docked-tonnage limits |
| Port throughput | Shared port ceilings, including separate fuel constraints | Per-ship berth/lighter rates, without another overall daily port ceiling |
| Port damage | Reduces handling, refuelling and repair capabilities | Absent |
| Amphibious operations | Ship type, landing craft, loading method and landing circumstances matter | A simpler handling rate modified by landing preparation |
| Supply distribution | Automatic overland redistribution along supply paths | Local stocks and explicit carriage; no equivalent overland distribution network |
| Stock management | Spoilage and limits on loading from needy bases | No equivalent spoilage or protected-stock loading limit |

At this baseline the generated South Pacific map gives every hex port size zero. Its bases therefore use lighter handling unless other scenario/map data supplies different sizes. The port mechanism exists, but the generated real-map data does not exercise its full range.

**Operational effect:** AE makes a port a constrained fleet-support facility. Flag Plot chiefly makes it a cargo-handling location, with separately configured repair capabilities. In particular, a large force can refuel without consuming the time or throughput another force needs.

AE references: 6.3, 9.3.1–9.3.3, 15.1, 15.7; pp. 109–128, 210–213, 250, 254.

## 4. Surface combat and weapons

Flag Plot source: [chapter 4](rules/04-surface-combat-and-damage.md), The fight, Damage and Breaking off.

Flag Plot has surprise, readiness, radar-assisted sighting, night experience, ammunition expenditure, fixed target priorities by kind of ship and withdrawal checks. Its engagement model nevertheless differs substantially:

- **No engagement range or manoeuvring geometry.** No opening or closing range, crossing the T, or bearing-dependent opportunity.
- **Main batteries only.** Secondary batteries do not contribute separate fire.
- **Simpler protection.** Weapon damage is scaled by ship toughness, rather than resolving AE-style penetration and hit-location effects.
- **Simpler participation.** Once a force sights, every eligible armed ship fires. Individual ships do not independently fail to participate.
- **Simpler radar.** Radar is a class-level yes/no sighting bonus. It neither improves gunnery accuracy nor creates AE's individual enemy-ship detection picture inside a fight.
- **Sequential pairs of forces.** Overlapping actions involving several forces are not one developing engagement.
- **Simpler targeting.** Ship-kind rules and stable ordering allocate fire. AE describes visibility, acquisition and tactical positioning affecting who can engage whom.
- **No torpedo dud model.** Weapon reliability and calendar-dependent improvements are absent.
- **No ordinary rearming cycle.** Gun ammunition and torpedo salvos are limited, but routine port rearming and combat torpedo reloads are absent. A yard return restores ammunition.

A Flag Plot fight does not consume movement time. After it ends, the force continues along its route with the remaining movement budget, rescaled to the speed its surviving ships can make.

**Operational effect:** Flag Plot represents surprise and firepower exchange, but leaves out many mechanisms that distinguish a confused, partial engagement from one in which every available weapon gets its opportunity. This matters directly to the recorded misses at Savo Island and Cape Esperance; it is not only a concern for a future larger campaign.

AE references: 6.3.5, 6.4.2, 10.3, 15.2, 19.2; pp. 129–130, 221, 250, 270.

## 5. Ship damage, damage control and repair

Flag Plot source: [chapter 4](rules/04-surface-combat-and-damage.md), Damage, What damage does afterwards and Repair.

Flag Plot has flotation and fire tracks, plus named systems: propulsion, fire control, main battery, torpedo mounts and flight deck. A system is working, degraded or out. This is a different abstraction from AE's system, flotation and engine damage, fire level, weapon damage and major-damage distinctions.

The largest missing interactions are:

- Fire does not spread, burn out or cause further damage. Its effects include night visibility and sinking claims.
- Flooding does not progress between attacks.
- Damage control at sea is absent.
- Flotation damage alone does not slow a ship.
- Magazine explosions and equivalent catastrophic critical-hit outcomes are absent.
- Radar cannot be damaged.

Forward repair spends a base's fixed daily allowance on stopped task forces. Serious damage and systems that are out require a yard trip: the whole force leaves the map and returns restored after a calculated delay. There is no repair-shipyard capacity competition, tender system or naval-support contribution. A ship in port outside a task force receives no repair.

AE makes repair depend on damage type and degree, port capability, repair ships, tenders, naval support and shipyard capacity. Anchored ships are an important part of that system. Normal and major damage impose different repair requirements.

**Operational effect:** AE can sustain a struggle to save a damaged ship through worsening flooding, fire and limited facilities. Flag Plot concentrates the consequence into immediate system impairment, withdrawal and time away.

AE references: 6.5–6.5.1, 14.2; pp. 132–134, 239–248.

## 6. Air operations and combat

Flag Plot source: [chapter 5](rules/05-air-operations.md).

| Rule | WitP:AE | Flag Plot |
| --- | --- | --- |
| Fighter performance | Speed, manoeuvrability, altitude, weapons and pilot skills influence engagements | Generic fighter hit chances, with aircraft protection modifiers |
| Pilots | Individual experience, skills, fatigue and replacement pools | Counts within groups; replacement pilots are equally capable |
| Missions | Separate sweeps, escort, LRCAP, recon, ASW, ground/port/airfield/city attacks, transport and training | Rest, search, naval strike, base strike and CAP |
| Allocation | Percentages can divide effort among duties | Search and strike are exclusive; CAP groups can split between patrol and escort |
| Interception | Radar warning, climb time, altitude and CAP readiness affect interception | Patrol is allocated among raids; configurable shares fire before or after an attack |
| Coordination | Formations can coordinate or become separated | Raids group by origin and target, without AE's coordination-failure system |
| Combat damage | Aircraft can return damaged or be lost subsequently from combat damage | Combat hits destroy aircraft; repairable damage comes only from operational wear |
| Bomber defence | Bomber characteristics and defensive combat matter | No bomber return fire |
| Ordnance | Carrier sortie stores and torpedo inventories constrain operations | No equivalent carrier ordnance or aviation-torpedo inventory |
| Basing | Carrier qualification, airfield size, crowding and support matter | Ship capacity and airfield existence, without equivalent qualification or base crowding rules |

Further differences:

- A group sends its available strike aircraft together; every aircraft of that group attacks one ship.
- A naval strike goes to the reported hex and does not search neighbouring hexes.
- There are no night air missions, altitude orders, reduced-load range trade-offs or drop-tank orders.
- A base can host unlimited aircraft. Airfield size has no operational effect.
- Any aircraft type can be based on a ship with aircraft capacity; there is no carrier-capable/carrier-trained distinction.
- Emergency diversion after losing a flight deck is implemented. Diverted aircraft join a group of their type at the destination, or form one there; AE also models attempts to return to the parent carrier.
- Aircraft replacement streams are daily scenario allowances. Unused replacements disappear, rather than accumulating in national pools.
- Rebasing moves a whole land-based group. It has no interception or flight losses; damaged aircraft are written off. Routine carrier-to-base and base-to-carrier transfers are absent.

**Operational effect:** Flag Plot represents reach, numbers, protection, fatigue and attrition, but aircraft types lack much of AE's tactical performance differentiation. A base strike can destroy aircraft, supply and men, but cannot suppress flying by damaging a runway or its service facilities. *Note, Nov 14, 2026: decision 0103 is built. A share of the hits on a base now lands on its airfield and damages it, and a damaged field flies fewer aircraft. A port and service facilities are still not damaged.*

AE references: 7.0.1–7.4, 9.4, 15.4, 16.3; particularly pp. 141–142, 156–175, 213–214, 252, 259.

## 7. Ground combat, preparation and engineering

Flag Plot source: [chapter 7](rules/07-land-units.md).

Land units, sea transport, supply, marching, combat, capture, fatigue, training, experience, unit skills, landing rehearsal and working units are implemented. Ground combat is therefore not a wholly missing system.

| Rule | WitP:AE | Flag Plot |
| --- | --- | --- |
| Composition | Squads, guns, vehicles, tanks and support devices | Men, with unit skills and weight abstracting differences |
| Fire | Weapon acquisition, range, soft/armoured targets and penetration | Shots derived from effective manpower and generic hit chances |
| Sequence | Bombardment, defensive fire, attacking fire and assault | Both sides calculate fire from starting strengths, then apply losses |
| Attack choices | Bombardment, deliberate and shock attacks | Attack or defend |
| Terrain | Changes movement, supply and combat | Changes marching through movement skills; no combat terrain effect |
| Defeat | Adjusted assault odds, forts, retreat and surrender | Surviving fighting-man ratio determines defeat and base capture |
| Losses | Disabled and destroyed devices, recovery and replacements | Permanent manpower losses; no scheduled land replacements |
| Movement | Routes, roads, railways, rivers and controlled hexsides | One adjacent land link per order; no comparable transport network or retreat-path system |

Other consequential differences:

- Defeated troops remain in the hex after a base falls; they do not automatically retreat or surrender.
- Troops aboard ships consume no supply and gain no passage fatigue. AE models transport-related fatigue and disruption.
- Aircraft and men share a base's supply stock, with aircraft upkeep charged first; no order changes this priority.
- Preparation applies to a landing place. It does not provide a general operation-preparation system for bombardments, strikes or convoys.
- Experience can grow through training and combat, but losses do not dilute it. At this baseline real units have standard experience and skills pending research.

Engineering is narrower too. Flag Plot can build an airfield, accumulate defences, and assign work that improves next-day handling or aircraft repair. Airfields are present or absent. *Note, Nov 14, 2026: an airfield now also has damage from 0 to 100, which work on it mends (decision 0103).* Port work provides a temporary handling boost, not a permanent increase in port size.

AE supports progressive facility sizes, runway/service/port damage and repair, and fortifications reduced by attacks and engineers. Flag Plot's defences increase defensive fire accuracy; they do not wear down and transfer intact with a captured base. Working units never fire, even in close combat.

**Operational effect:** Landing preparation and logistics have consequences, but terrain, equipment, disruption, recovery, retreat and replacement do not create AE's land-campaign dynamics.

AE references: 8.2.2–8.4.5, 9.4.2, 15.3; pp. 188–198, 214, 250–251.

## 8. Search, intelligence and claims

Flag Plot source: [chapter 3](rules/03-search-contacts-and-claims.md).

AE uses current detection level and maximum detection level: recent information affects combat, while longer-lived awareness controls what the player sees. Continued observation can improve identification and estimates.

Flag Plot uses contacts with estimated position, count, kind and age. Its reports can be wrong and a strike can fly to an empty reported position. The main differences are:

- A new report replaces a nearby fresh contact's estimate; evidence is not combined into confidence.
- Contacts have no confidence, course or speed, and their positions are not projected as they age.
- There is no progressive recognition of individual enemy ships.
- No dedicated reconnaissance assesses aircraft or facility damage at enemy bases.
- Ground estimates require friendly men in the same hex. AE includes reconnaissance and adjacent ground spotting.
- Search reports have no transmission delay or chance of never arriving.
- Claims never age or get reconciled against later evidence.
- Current hit and loss claims contain the actual results plus possible overclaims; those reported quantities cannot underclaim the actual results.

The last point makes this part of Flag Plot's uncertainty systematically optimistic. AE describes missed or delayed sinking information as well as erroneous combat reports. Flag Plot's strict separation of truth and belief is useful infrastructure, but it is not the same thing as having AE's range of collection and assessment mechanisms.

AE references: 5.4, 10.0–10.4, 18.1.1; pp. 74, 217–221, 266.

## 9. Objectives and victory

Flag Plot source: [chapter 6](rules/06-objectives-and-results.md).

Flag Plot can score cargo landed, days a base stays operational, bases held at the end, casualties and ships' time out of action. The scenario assigns the objectives and values. The true result is separate from a side's claim-based knowledge of its progress.

AE mainly combines losses, base values, industrial damage and campaign victory conditions. Base value depends on facilities and supply; losses distinguish more categories of equipment and ship value.

This is an alternative design, not just an incomplete AE score. Flag Plot explicitly rewards logistical accomplishment and operational availability. However, its current point values remain placeholders, so its results do not establish historical success on an agreed historical scale.

AE reference: 17.0; pp. 263–265.

## 10. Most consequential gaps

This ranking is an assessment of operational consequences, not an approved implementation sequence.

1. **Airfield suppression.** Bombing and shelling cannot close a field through runway or service damage. *Note, Nov 14, 2026: built by decision 0103. Hits on a base can damage its airfield, which cuts the aircraft it flies and, at a level, takes no aircraft. Service and port damage are not built.*
2. **Aircraft and crew differentiation.** Pilot quality, altitude, speed and tactical performance lack AE's influence on air combat.
3. **Sustained fleet operations.** Replenishment, rearming, machinery wear and repair-facility constraints are much thinner.
4. **Threats to shipping.** Submarines and mines are absent.
5. **Developing battle damage.** Fires and flooding do not evolve after an attack.
6. **Land-campaign dynamics.** Terrain, equipment, disruption, retreat and replacements are largely absent.
7. **Weather.** Plans face no weather-driven loss of air cover or cancellation of offensive missions.

Importance depends on the scenario. Missing industry barely affects a one-night Savo scenario; missing engagement range and participation rules directly affect it. Across Guadalcanal, airfield suppression, replenishment, weather and accumulated damage become much harder to omit without changing the commander's decisions.

Flag Plot does not need every AE rule to achieve its design. A missing mechanism needs attention when it changes which operational choices are sensible, makes a historical constraint disappear, or lets the planner exploit a simplification. That is a more useful test than counting matching features.
