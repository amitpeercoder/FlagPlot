# 0012: Air search, reports and contacts

Status: confirmed by the designer, Nov 10, 2026, as part of the scenario 3 plan. Every figure is a placeholder.

## The gap in the spec

The spec asks for air search "by arc and range", reports "generated from truth through a detection and error model", and contacts that "age and lose confidence each pulse without a fresh sighting". It gives no detection rule, no error model, no ageing rule, and no rule for when two reports are the same contact.

## Decisions

### Search

1. **An air group on a search mission searches an arc from its base:** from one bearing clockwise to another, in 10 degree steps, out to a range in nautical miles. A "from" and "to" that are equal mean the whole circle.
2. **North is the direction of decreasing r.** Hexes are pointy-topped, so the six neighbours lie on bearings 030, 090, 150, 210, 270 and 330. A hex's bearing from a base is worked out from the hex centres, in fixed point, from a constant table. This holds for every map.
3. **Distance is hex steps times 40 nm,** the same distance movement uses.

   *Nov 14, 2026: air distance is the straight line between hex centres, by decision 0040; ships are charged straight-line miles where the way is clear by decision 0058 (`docs/rules/differences.md`, row 5).*

4. **A task force can be sighted if its position is inside the arc and within the range.** The edges of the arc and the range itself are inside. A group never searches for its own side.
5. **The chance of a sighting is coverage:** the miles of sea the aircraft sweep, over the miles of arc at the target's distance.

   chance = (57.295779 x aircraft x sweep width) / (arc in degrees x distance in nm), capped at 1

   57.295779 is degrees per radian. The numerator is multiplied out first and divided once by the denominator, truncating to six decimals. A target in the base's own hex has a chance of 1.
6. **Search runs in the morning and afternoon pulses,** before movement, so it uses each task force's position at the start of the pulse. Every ready aircraft flies in both pulses.
7. **There is no search where it is dark.** Rules ask `is_dark(hex, pulse)`. For the slice it is true in the night pulse and false otherwise (decision 0001).
8. **One roll per group, per target, per pulse.** Two groups searching for the same force roll separately. See decision 0013 for the random number key.

### Reports

9. **A sighting produces one report, which arrives in the same pulse.** The report gives a hex, a ship count and the most important kind of ship seen. Importance runs carrier, battleship, cruiser, destroyer, transport, auxiliary.
10. **Each part of a report can be wrong, independently:**
    - Position: with a set chance the report names one of the six neighbouring hexes, chosen evenly. If that hex is off the map or is land, the report names the true hex.
    - Count: with set chances the report is one too many or one too few, never below one.
    - Kind: the reported kind is drawn from a table of chances for each true kind.
11. **A report starts at a fixed confidence.**

   *Nov 14, 2026: withdrawn by decision 0032, item 2: a contact has no confidence (`docs/rules/differences.md`, row 5).*

### Contacts

12. **A contact holds what the report said, an age in pulses and a confidence.** It never holds the identity of what was really seen. The event log, which only the umpire reads, records which task force each sighting was of.

   *Nov 14, 2026: the confidence is withdrawn by decision 0032, item 2 (`docs/rules/differences.md`, row 5).*

13. **A new report refreshes an existing contact** if one lies within a set number of hexes of the reported hex and is no older than a set number of pulses. The nearest such contact is used, the lowest ID on a tie. Its hex, count and kind become the report's, its age returns to zero and its confidence to the starting value. Otherwise the report makes a new contact. The test uses only what the side believes.
14. **Contacts age at the end of every pulse,** night included. A contact that was not created or refreshed in that pulse gains one pulse of age, and its confidence is multiplied by a decay factor. A contact whose confidence falls below a floor is dropped.

   *Nov 14, 2026: a contact is dropped when it reaches an age of 5 pulses; its confidence and decay factor are withdrawn by decision 0032, item 2 (`docs/rules/differences.md`, row 5).*

15. **The end-of-day intelligence step does nothing yet.** The spec lists "age contacts, merge duplicates" there and also says contacts age each pulse. Ageing per pulse and merging on arrival cover both. The step is kept for briefings.

### Knowing it was sighted

16. **A task force that is sighted may notice the aircraft.** Each sighting gives the task force one roll at a set chance. A plane can see without being seen, so it is not certain.

   *Nov 14, 2026: withdrawn by decision 0032, item 1: no notice roll exists (`docs/rules/differences.md`, row 5).*

17. **A task force that notices records the hour.** The record is the hour of the latest sighting it noticed. Its own side sees it, as it sees everything about its own units. An event records each notice and cites the sighting.

   *Nov 14, 2026: withdrawn by decision 0032, item 1: no notice roll exists (`docs/rules/differences.md`, row 5).*

18. **Noticing changes nothing else yet.** How a force reacts (press on, turn away until dark, abort) is a standing instruction from its commander, and arrives with rules of engagement in scenario 5. The player cannot react inside a day, so the reaction is the local commander's, as strike decisions are.

   *Nov 14, 2026: withdrawn by decision 0032, item 1: no notice roll exists (`docs/rules/differences.md`, row 5).*

19. **The notice roll has its own random stream:** the task force is the entity and the air group is the second entity, the reverse of the sighting roll. Adding it changes no other result.

   *Nov 14, 2026: withdrawn by decision 0032, item 1: no notice roll exists (`docs/rules/differences.md`, row 5).*

## Why

- **Coverage is one mechanism.** More aircraft raise the chance. A wider arc or a more distant target lowers it. "A concentrated arc beats an all-round search" needs no special rule. Our research notes name sector width, number of aircraft and range as the factors that most decided real searches.
- **No detection level.** WitP:AE gives each force a detection level from 0 to 10 that resets every phase, and a second "maximum" level that carries the memory (manual section 10.1). One number stands for how fresh a sighting is and for how good it is. A contact with an age and a confidence keeps the two apart, and can be wrong in ways a level cannot.
- **A linear rule with a cap** can be worked by hand and explained in a sentence. Search theory would use a curve that flattens before 1. Step 2 can replace the rule if history disagrees.
- **No true identity in a contact,** because a side that could tell two sightings were the same force would be reading the truth.

## What the event log shows

- Every target inside the arc and range gets a search event with its chance, sighted or not.
- A target outside the arc or beyond the range gets no event. The log cannot yet answer "why was that force not looked for". That belongs to the "explain" query, with briefings.

## Legibility test

Run on Nov 10, 2026, as CLAUDE.md requires from scenario 3 on. A Haiku model was given this record and the event log of one run (seed 7), and nothing else. It answered ten questions correctly: why a force at 680 nm was not searched for, why a report gave the wrong count, why a 76% chance missed, why a report made a second contact, why a contact was dropped, what the sighted force knew, and three predictions of chances and confidences for changed inputs.

It found three things hard:

- The figures (error chances, decay, merge distance) are in `data/reference/search.toml`, not in this record. That is deliberate: they are placeholders and will change.
- Targets outside the arc or range leave no trace in the log, as above.
- It could not check from the log alone that a hex was inside the arc. Decision 2 gives the convention, and the search event gives the arc, but no event gives a target's bearing. Add the bearing to the search event's breakdown when briefings are built.

## Deferred

- Report delay, and reports that never arrive. Our notes rate this as a major real factor. It arrives with scenario 5, where strike decisions hang on it.
- Night search, radar, weather and coastwatchers.
- Ships sighting ships, and aircraft as contacts. A task force that notices a search plane records the fact, but its side gets no contact for the aircraft.
- Any reaction to being sighted.
- A share of a group on search with the rest on another mission. WitP:AE has the player set this per squadron in steps of ten percent (section 7.2.1.8). The staff will set it.
- One patrol a day for long-range aircraft, with fuel, supply, fatigue and losses.
- Course and speed in a contact, and moving a contact along its estimated course as it ages.
- Event logs filtered for one side. Events stay umpire-only until briefings exist. *(Nov 14, 2026: built: decisions 0047 and 0048)*
- Real figures. Sweep widths, error chances and decay come from sources in step 2.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Air search (air search phase) | Air group mission, arc, range, ready aircraft, base; aircraft type sweep width; enemy task force position; enemy ship classes (for the true count and kind); map; search figures; darkness | Nothing in truth. Produces reports |
| Report into belief | The report; the side's contacts; search figures | The side's contacts |
| Contact ageing (end of each pulse) | The side's contacts; search figures | The side's contacts |
| Noticing a sighting (air search phase) | The sighting; search figures | Task force hour last noticed sighted |
| Air group order intake | Air group side; aircraft type search range | Air group mission, arc, range |

Search reads the position that movement writes. Nothing reads contacts or the hour last noticed yet. Strike decisions and reactions will, in scenario 5.
