# 0048: The briefing and the queries

Status: confirmed by the architect, within the plan for the agent interface the designer approved on Nov 11, 2026.

## The gap

A side can now be given its own events (decision 0047) and has always had its own state. Nothing turns them into something a player can read. The spec asks for a daily situation report for a side, queries that pull detail on demand, and a way to ask why something happened, all as JSON with published, versioned schemas. Every legibility test so far used a briefing the architect wrote by hand.

## Decisions

### Where it lives and what it may read

1. **The briefing and the queries are built in `sim-agent`, from a side's view and nothing else.** They take a `SideView`. They cannot be handed the world, the full log or the umpire's view. What a side is told is therefore what decisions 0012 and 0047 allow, by construction.
2. **They read and never write.** No query changes the game, the hash or the event log.

### The briefing

3. **A briefing is the situation of one side at the start of a day.** It reports the day before and the state now. On day 1 it reports the state only.
4. **It has the same parts in the same order every day,** and says "none" where a part is empty, so that a reader learns where to look:
   1. **Heading:** the side, the day, the scenario.
   2. **Orders:** how many were accepted yesterday, and each one refused with its reason.
   3. **Contacts:** new, refreshed, dropped and found not there yesterday; then every contact held now with its place, what it is taken to be and its age.
   4. **Action yesterday:** fights at sea the side knew of, strikes it flew and what they reported, raids on its bases, and attacks on its ships.
   5. **Losses yesterday:** ships sunk and damaged; aircraft lost, by cause: shot down, destroyed on the ground, written off; pilots lost. And what the side claims of the enemy.
   6. **Task forces:** where each is, what it is doing, its readiness and fatigue, its least-fuelled ship and how far that ship can steam at cruising speed.
   7. **Air groups:** aircraft ready and damaged, pilots, fatigue, whether stood down, mission.
   8. **Bases:** supply and fuel, what was spent and destroyed yesterday, and how many days the supply lasts at yesterday's spending.
   9. **Arrivals:** aircraft due to arrive in the next three days, and the daily replacements.
   10. **Warnings:** see 6.
5. **It is given twice over:** as structured data, which is the truth of it, and as text made from that data and from nothing else. A test checks that every number in the text is in the data.
6. **A warning is a fact about the side's own state, with the figure that makes it one.** There are five: a ship that cannot reach the nearest base of its side at cruising speed; a base whose supply lasts under three days at yesterday's spending, or that has none; a base with no fuel and a force stopped at it; an air group with fewer pilots than ready aircraft; an air group stood down or above its rest threshold. They are thresholds of presentation, not rules: changing one changes no result.
7. **A briefing advises nothing.** It does not say what to do, rank threats or guess the enemy's intent. That is the staff's work, in step 3.

### The queries

8. **`list_forces`, `list_bases`, `list_contacts`** give a line for each of the side's task forces, its bases and the contacts it holds. Each takes an optional area: a hex and a radius in miles, by air distance (decision 0040).
9. **`get_entity`** gives everything the side knows of one of its own units, or of one contact. Asked for an enemy unit by the log's number, it says it knows of no such unit. It does not say that the unit exists.
10. **`explain`** takes the number of an event and gives the side's version of it and its causes, and theirs, back to the side's own orders or to where the causes are hidden. It says where they are hidden. Asked for an event the side may not see, it says it knows of no such event.

### The form

11. **Everything goes out as JSON with a schema, and the schemas are files in the repository.** They carry a format name and a version. A test checks every answer the tools give in the scenarios against its schema.
12. **Numbers of state are written as the game holds them:** whole numbers, or decimals as text, never floating point. A place is its hex, with its latitude and longitude where the map has them, and the name of a base or an anchorage in it.
13. **Units are named by kind and number, as the log names them,** with their names beside. The same handle works in every tool.

14. **An event is named to a player by the side's own number for it:** its place in the list of events that side may see, counting from 1. The log's number is never shown. The first version showed the log's number, and two games that differed only in something a side could not know gave that side briefings that differed in their event numbers. A side's list only grows: what it holds after one day is the start of what it holds after the next, which a test checks.
15. **The briefing says which rules are in play** (wear, fatigue, pilots for each group, base supply) and speaks only of those. A base is not warned of having no supply in a game where nothing uses supply. The data keeps every field.

## Why

- **Built on the view, so the fog cannot be forgotten.** A briefing written from the world, with care taken to leave things out, would be one slip from telling a side what it should not know. One written from the side's view has nothing to leave out.
- **Same parts every day.** In the legibility tests the cheap model's mistakes were about where to find a figure, not about the rules. A fixed layout is the cheapest help.
- **Data first, text from it.** A model reads the text and a program reads the data. If the text were written separately the two would drift, and the transcript of a game could not be checked against what the agent was shown.
- **No advice.** The pillar is a staff the player can read. A briefing that mixed what is known with what is recommended would make it hard to say later which of the two a player acted on.

## What it costs

- A briefing repeats the whole state every day. In a long game most of it will not have changed. A report of changes only is deferred.
- "Days of supply" is measured by yesterday's spending, which is nothing on a day nothing flew.
- A side's view builds its events from the whole log each time it is asked. That is slow in a long game and is to be measured in the stage that plays one.

## Deferred

- A report of changes only, and a summary over several days.
- Map output: areas of search covered, ranges drawn.
- Advice, threat ranking and the enemy's likely intent (the staff planner).

## The legibility test

A cheap model was given the two briefings the game itself wrote for the Allied side of scenario 6a, for the start of days 2 and 3, and the rules, and no event log. From the briefings alone it worked out the six escorts of each day from the patrol's shots, the patrol of 12 on the first morning from the 9 ready and the 3 lost, why the bombers took 3 shots on the second day and not 6, and which group lost aircraft to what. It predicted the third morning as far as a side can: 7 fighters, 6 shots at the escorts and 1 at the bombers if six escorts come again, 20 shots from the guns. It listed what it could not know, and each was something the side should not know: how many escorts will come, how many bombers set out.

It found three things unclear, and they are changed. "Own gunfire: none" stood two lines above the base's guns firing: the first now reads "Salvoes fired at sea" and the second "Anti-aircraft fire". The raid line gave aircraft without saying they were those that got through: it now says "attacked". It gave the hits on aircraft and on supply and left the rest to subtraction: it now gives the hits on nothing.

Left as they are: a loss does not name what caused it in words, only by its event number, which `explain` follows; and "ready" is not defined in the briefing itself.

## Rule card

No rule of the game is added. Reads a side's view. Writes nothing.
