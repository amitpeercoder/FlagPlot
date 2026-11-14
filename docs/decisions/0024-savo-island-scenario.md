# 0024: The Savo Island scenario: what is in it and what is left out

Status: confirmed by the designer on Nov 12, 2026. Decisions 2 and 6 change what decision 0019 said would be in the scenario.

## The gap

Decision 0019 says who was where. The facts give positions, readiness and what each side knew. Turning them into a scenario file needs choices the spec and the earlier decisions did not make, most of them because a hex is 40 nm across and the whole battle was fought inside one.

## Decisions

1. **The battle is one hex.** Savo Island, both cruiser groups, both pickets and Cape Esperance all fall in hex (2,-5) of the map. The scenario is the Japanese column entering that hex and meeting each Allied force in it, by decision 0022. Ranges, bearings and the order of events inside the hex are not modelled.
2. **Three forces fight: the Japanese column, the southern group and the northern group.** The pickets, Blue and Ralph Talbot, and Jarvis are left out. The column sighted Blue and Jarvis and chose to pass them without firing. The rules have no way for a force to pass an enemy it has sighted: a force set to engage fights, and a force that slips away avoids action for the rest of the pulse (decision 0022), which would end the battle before it began. Left in, each would be fought and sunk by eight ships, which is further from what happened than leaving them out. Ralph Talbot's brush with the retiring light cruisers is lost with them. *(Nov 14, 2026: see the notes of Nov 13, 2026 in `docs/decisions/0024-scenario-4-savo-island.md` (Yunagi in a task force of her own))*
3. **The Japanese column is one force of eight ships,** in its sailing order: Chokai, Aoba, Kako, Kinugasa, Furutaka, Tenryu, Yubari, Yunagi. Its split into two columns at 0144 is inside the hex and is not modelled. *(Nov 14, 2026: see the notes of Nov 13, 2026 in `docs/decisions/0024-scenario-4-savo-island.md` (Yunagi in a task force of her own))*
4. **The southern group fights first.** It has the lower task force ID, and decision 0022 takes meetings at the same moment in ID order. That is the order of the night.
5. **Ship order inside a force, which decides who fires at whom** (decision 0014, 8), is the steaming order: Canberra, Chicago, Patterson, Bagley; and Vincennes, Quincy, Astoria, Helm, Wilson.
6. **The column starts at 18:00 on 8 August in hex (-1,-7), 200 nm up the Slot, and steams at 24 knots.** No source gives its 18:00 position. It is worked back from the one that is given: 37 miles from Cape Esperance at midnight, making 24 to 26 knots. At 24 knots the column enters hex (2,-5) after 180 nm, at 01:30. The sources have it turning past Savo at 01:32. Its route is (2,-5) and then back to (-1,-7), so that it fights in passing and withdraws (decision 0022).
7. **Readiness is taken from the facts.** The Japanese were at battle stations from 00:45: battle readiness. Every Allied ship was in Condition II with its captain asleep: normal readiness.
8. **The Allied side starts with one contact: the Hudson's 10:26 report,** eight ships at its reported position, hex (-1,-9), two pulses old. It is seven hexes from the cruiser groups, so by decision 0021 it alerts nobody. That it was garbled and late is not modelled; that it did not put the screen on alert is.
9. **Both sides are ordered to engage and break off at half their ships sunk or crippled.** No source gives a figure. It is the placeholder level every earlier scenario used. The Japanese column is ordered to detach any ship that cannot keep up (decision 0016): it goes home to Rabaul with an escort, and a ship that cannot move is left: the sources have Mikawa set on being out of reach of carrier aircraft by daylight. The Allied groups keep theirs, having nowhere to go. *(Nov 14, 2026: see the notes of Nov 13, 2026 in `docs/decisions/0024-scenario-4-savo-island.md` (the break-off levels, read from the record))*
10. **Every ship starts with full fuel and a full outfit of ammunition and torpedoes.** No source read gives what was aboard that evening. Fuel does not limit anyone in one day.
11. **The scenario stays marked as placeholder data.** The ships and the map are real. The combat figures are not: the sighting chances, hit chances and the three scaling constants are placeholders until step 2.
12. **In step 1 the scenario is run and its spread reported, not judged.** The spec sets acceptance bands for losses in step 2. The test asserts what the rules fix (the time and place of the meeting, fuel, the sighting chances, the first salvos, the properties) and prints the spread of losses beside what happened.

## Why

- **Leaving the pickets out** costs the chance of a warning. That chance was real, and on the night it came to nothing: Blue saw nothing at all. A rule for passing an enemy by belongs with the staff and rules of engagement in scenario 5, and the pickets can come back with it.
- **One Japanese force** keeps to what the rules can say. Two columns inside one hex would be two forces that meet the same enemies at the same moment, which adds nothing.
- **24 knots** is the speed the sources give for the approach, and it puts the column at Savo within two minutes of the recorded time.

## What the first runs show

Over seeds 1 to 1000 (`sim-cli batch`), with the placeholder combat figures:

| | In the runs, on average | On the night |
| --- | --- | --- |
| Japanese ships sunk | 1.7, and none in 293 runs of 1,000 | None |
| Canberra, Vincennes, Quincy, Astoria sunk | 0.19 of the four | All four |
| Allied destroyers sunk | 2.0 of the four | None |

**This is not a plausible spread, and it is not presented as one.** The scenario runs, and everything the rules fix is as calculated. The losses are the wrong shape, for reasons that are in the rules and their placeholder figures and not in the ships:

- **The two sides shoot equally well.** A gun shot hits with the same chance whoever fires it, and a force fired on returns full fire from the next round. On the night the Japanese hit with something like one shell in eight and the Allied cruisers with one in fifty, and three Allied cruisers fired almost nothing. The only advantage the rules give the Japanese is the better chance of seeing first.
- **Weight of shell does not matter enough.** Three 8-inch hits sink a destroyer. A 5-inch hit is as likely as an 8-inch hit to knock out a cruiser's engines.
- **The column breaks off too soon.** With half its ships crippled after a round or two against the southern group, it avoids the northern group, which is then hardly touched.

Changing one doubtful figure at a time moves the averages a little and the shape not at all:

| Changed figure | Japanese sunk | Allied sunk |
| --- | --- | --- |
| As derived | 1.70 | 2.25 |
| US 5-inch at 12 rounds a minute, not 15 | 1.57 | 2.33 |
| Japanese 20 cm at 3 rounds a minute, not 2.4 | 1.61 | 2.54 |
| US 8-inch at 5 rounds a minute, not 3 | 2.29 | 2.19 |
| Mark 15 warhead 374 kg, not 224 | 1.91 | 2.23 |

So the doubts recorded in decision 0023 are not what stands between this scenario and history. Step 2 sets the hit chances from the recorded shells fired and hits, which the facts hold for most ships. The three points above go to the complexity review after scenario 5.

## Deferred

- A force passing by an enemy it has sighted without fighting. With it, the pickets and Jarvis.
- Illumination by flares and searchlights, which decided how fast the northern group was hit.
- A warning that fails to arrive. The alarm roll of decision 0021 is the only stand-in.
- Command gaps: a group that fights as single ships.

## Rule card

No rule. These are scenario inputs.

## Amended by decision 0058 (Nov 12, 2026)

Item 6 worked the column's start back with 40 miles a hex. Ships are now charged straight-line miles (decision 0058), and from hex (-1,-7) the column entered hex (2,-5) at 00:25. The start is now hex (-1,-8) and the speed 25 knots, settled by the architect: it is 208 miles from (2,-5), the column enters that hex after 188 miles at 01:31, and at midnight it is 38 miles short of it, against the source's 37 miles from Cape Esperance. 25 knots is inside the 24 to 26 the sources give. Nothing else of the scenario changes.
