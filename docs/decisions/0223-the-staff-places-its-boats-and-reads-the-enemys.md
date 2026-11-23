# 0223: The staff places its boats its navy's way, and reads where the enemy's are

Status: a plan by the architect, written on Nov 23, 2026, on the designer's question of Nov 22: "Did we run a research around how submarines were employed by both navies, so that the staff could place their submarines and also try to assess where the other side employed his?" The research is `docs/research/submarine-employment-1942.md` (rows 16430 to 16915 of `data/oob`).

Summary: Since decision 0218 a side has boats, a commander can order a patrol of waters, and the staff spreads the boats evenly over those waters. Both navies' staffs do it the same way, no staff sends a boat out unless the commander orders a patrol, and what a side believes of enemy boats is only what a scenario tells it and the reports it holds. This plan gives each navy its own way of placing boats, lets the staff form a patrol under the orders that call for one, and gives the staff an estimate of the enemy's boats made from its own reports, the attacks it has suffered and, for the navy whose record shows it, habit.

Standing: Planned on Nov 23, 2026; amended on review; stages 0 and A built.

## Why

What is so today (rules chapters 8 and 10; decision 0218):

- **A patrol is a root order** with a hex and a distance. The staff gives its boats stations: the centre first, then each next station the navigable hex of the waters farthest from the stations already given.
- **No doctrine differs by navy.** The same spread serves the American and the Japanese side.
- **No order but a patrol uses a boat.** A side given "hold Midway" or "deny these waters" leaves its boats where the scenario put them.
- **Suspected waters are a scenario's:** told on a day, right or wrong, never changed in play. Reports of submarines are a belief list of their own. The staff's regard (0218, rule 55) keeps a standing force's chosen hex out of suspected waters and away from a report by one hex.

What the record states often and clearly enough (the research's last section):

- **American boats worked off enemy bases, straits and focal points, not on lines,** one or two to a station, attack first and information second from the spring of 1942. One or two boats stood as a picket off the side's own outlying base and were drawn in when a threat was read.
- **Japanese boats were put on the expected track of an American force,** in one or two lines across it, before the force was seen. A report moved them by order the same day; they reached the new line in two to three days, "usually too late".
- **American estimates of enemy boats in 1942 were counts in an area,** given with "at least" and "about", made mostly from sightings and attacks, with an expectation by habit of boats off the side's own bases and on its supply line. The one check against the record ran high (twelve against eight).
- **The Japanese estimate of American boats in 1942 rests on single sightings.** The plot and the prediction are later.
- **A force that stayed in one area was found.** The remedy was to move, to clear the area after an attack and to route round reported boats. No source gives a time or a distance for 1942.

By the designer's rule of Nov 22 (one rule set, doctrine values by side and date), each difference above is a value of the doctrine by navy and date, not a rule for one battle.

## The rules

### A. How boats are placed in the waters of a patrol

1. **The doctrine gains `patrol_pattern`,** by navy and date: `stations_off_bases` (United States, 1942) or `lines_across_the_track` (Japan, 1942). A side whose doctrine names neither keeps the spread of decision 0218, and every game on file plays as it did until its doctrine names one.
2. **Stations off bases.** The staff lists the enemy bases on the side's map whose hex is in the waters or within one hex of them, nearest the centre first, lowest number on a tie. For each in turn it gives one station: the navigable hex next to the base that is in the waters, nearest the centre, lowest hex on a tie. When each such base has one boat, a second round gives each a second station by the same test among the hexes not taken, to `boats_per_station` (2: the rows show one or two). Boats left after that are spread as decision 0218 spreads them. Waters with no such base are spread as decision 0218 spreads them.
3. **Lines across the track.** The track is the straight line from the nearest enemy base on the side's map (by hex distance to the centre, lowest number on a tie) to the centre of the waters. The first line passes through the centre, across the track; its stations are the centre and then, by turns to one hand and the other, the navigable hexes `line_spacing_miles` apart along it (30 miles: the Solomons lines of September 1942, and the same figure in 1944), while they lie in the waters. Boats left when the first line is full go on a second line `line_gap_miles` (200 miles: one source, one battle, marked so in the figures file) farther along the track from the enemy, by the same test. Boats left after that are spread as decision 0218 spreads them. With no enemy base on the side's map there is no track, and the boats are spread.
4. **Miles are turned into whole hexes** by the map's hex size, rounded to the nearest, at least one.
5. **The reason of each station order says which of these gave it:** "off Rabaul", "on the first line across the track from Rabaul", or the spread.
6. **Everything else of decision 0218's patrol stands:** the boats of the order, a boat that cannot serve, a boat that keeps her station, the one decision to give no order.

### B. A patrol the staff forms itself

7. **Under three orders the staff forms a patrol task of its own,** a sub-intent of kind patrol, made and ended as any staff task is:
   - under **hold a place** and under **control an island**, where `picket_boats` is above 0 (United States: 2; Japan: 0): a picket of that many boats in the waters within `picket_miles` of the place (see question 1 for the figure);
   - under **deny waters**: a patrol of the same waters as the order;
   - under **take a place**, for a side whose pattern is lines across the track: a patrol of the waters within `line_gap_miles` of the place, so that the lines lie across the enemy's way to it.
8. **It uses only boats that are free:** not named by a forces limit of another order, not held, not given a station by a patrol the commander ordered. The commander's own patrol orders are served first. A forces limit on the order binds the task as it binds any (decision 0222): boats named are the only ones used, and "none of submarines" forms no patrol.
9. **Leaves are read up the tree** as for any staff task: a patrol task stations boats on leave to prepare and needs no leave to launch, since a boat on station attacks by the rule and not by an order.
10. **With no free boat the task is not formed** and one reason says so under the order.

### C. Boats moved on a report

11. **For a side whose pattern is lines across the track,** a patrol task the staff formed (rule 7) is laid again when the commander's picture gains a fresh contact of a carrier or a battleship within `line_gap_miles` of its waters: the track is then from that contact to the centre, and the stations are chosen again by rule 3. A boat whose new station is her old one is given no order. The boats steam to the new line at their own speed, by the passage rule; nothing here hastens them.
12. **It is laid again at most once a morning,** on the freshest such contact, lowest number on a tie, and not again for the same contact.
13. **A patrol the commander ordered is never moved by the staff.** Its waters are his.
14. **A side whose pattern is stations off bases moves no boat on a report.** The picket of rule 7 is ended, and its boats freed, on the morning the place it guards has an enemy contact of any kind within `picket_miles`; it is formed again when no such contact is held (the boats were drawn in on a threat read, not left to be run over).

### D. The staff's estimate of the enemy's boats

15. **Each morning the staff makes an estimate of enemy boats** for the commander it serves, from that commander's picture and the side's own events only. It is a reading, kept nowhere in the state, made again each morning. It has areas, each with a count and a basis:
    - **Reported:** the reports of submarines the picture holds are gathered into areas, two reports within `estimate_join_hexes` of each other joining one area. The count is the number of reports in the area that are of different days or more than one hex apart, said as "at least N".
    - **Attacked:** a hex where a ship of the side was attacked by a submarine in the last `estimate_days` days is an area of its own, or joins a reported area within `estimate_join_hexes`, and adds one to its count where no report of that day lies within a hex of it.
    - **By habit** (a side whose doctrine has `expects_boats_off_own_bases`: United States, 1942): the waters within one hex of each of the side's own bases on the map that has had a ship of the side at it in the last `estimate_days` days, with no count: "boats may be expected off Lunga".
16. **Suspected waters a scenario gave stand beside it,** unchanged, named as told to the side.
17. **The regard of decision 0218, rule 55 reads the reported and attacked areas** as it reads a report today. It does not read the habit areas: a force may not be kept from every base of its own.
18. **A standing force is moved after an attack.** Where a hex of a station or stand the staff chose lies within `estimate_join_hexes` of a hex attacked the day before, the staff chooses the place again that morning with the attacked area regarded, and the reason says so ("clearing the waters where Saratoga was attacked"). A place the commander named is never moved; the staff says the caution of decision 0218 instead.
19. **The estimate is told:** in `list_contacts` beside the reports, as areas with a count, a basis and the days of the reports; in the evening report in a line of its own; at the table on the chart as hatched waters with the count, under the PIECES control with the reports.

## What a side learns by it

Nothing new of the truth. Rule 15 reads only what the side's picture and events already hold. The enemy learns of a boat only by the sightings and attacks decision 0218 already makes. A patrol task and its stations are the side's own and are seen by no one else.

## What is recorded

- A patrol task is formed, changed and ended by the events any staff task has; each station order cites the task and, under rule 11, the contact that moved the line.
- The estimate is recorded in no event: it changes no state. The staff's decision of rule 18 cites the attack.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| 1 to 6 | the doctrine; the side's map of enemy bases; the waters; the boats of the order | station orders, their reasons |
| 7 to 10 | the orders in force and their limits; the side's boats and holds | a staff intent of kind patrol; station orders |
| 11 to 14 | the commander's contacts; the task's waters | station orders; the task ended and formed |
| 15 to 19 | the commander's reports of submarines; the side's events of attacks by submarine; the side's bases and ships | nothing in the state; staff reasons; the answers of `list_contacts` and the evening report |

No draw is made. No new state field: the doctrine's new values are figures. A new staff intent kind is not needed: the patrol kind stands.

## What it does not do

- **No strait or focal point of shipping** is known to the map, so the American pattern is "off enemy bases" only. Straits wait for the map to name them.
- **No spacing or line for American boats, and no line for a side with no enemy base on its map.**
- **No direction finding and no radio intelligence.** The estimate is sightings, attacks and habit.
- **No Japanese expectation by habit:** the 1942 record gives none.
- **No speed, zigzag or time limit in submarine waters:** the record gives no 1942 figure.
- **No boats on supply runs, no midget carriers, no reconnaissance by a boat's floatplane.**

## What it costs

- A doctrine version for each of A to D as it is built, an agent interface number for rule 19, and the rulebook's chapters 8 and 10.
- The games on file do not move until a doctrine names a pattern. The scenarios for play have no boats yet (open work): this is built and proved on test games, and the play scenarios are measured before and after when boats are put in them.

## Before it is built

1. **Reviewed** by a fresh reader given this record, the research and the code of `patrol.rs`, `waters.rs`, `submarine.rs` and `suspected_waters.rs`.
2. **The architect's scenario and statements** for each of A to D, in four stages, each its own pull request.

## Questions for the designer

1. **The picket's distance.** The only figures are Midway's arcs of 200, 150 and 50 miles, one battle. The plan uses 150 miles as `picket_miles`, marked in the figures file as resting on one battle. Say if a picket should wait for a better figure.
2. **The staff sends boats out without a patrol order** (rule 7). It does so only under an order to hold, control, deny or take, never with no order at all. Say if a boat should move only on the commander's own patrol order.

## Amended on review, Nov 23, 2026

A fresh reader was given this record, the research and the code, and found eight things that had to be mended before a build and nine that should be. The architect accepts the findings. The plan above is cut to what can be built on what stands, and the rest is set aside with what each part needs. Where this section and the rules above differ, this section holds.

### What the review found

- **A patrol is a root nothing stands under, served only when its plan is released, by every boat of the side in number order with stations that do not move up** (`patrol.rs`, chapter 8). Rules 7 to 10 needed four new parent pairs, a boat list of free boats with a count, stations that move up, an exception to the launch leave, and a doctrine at the area's level reading the holds of bases below it. None was in the plan.
- **The picket is against the rows.** Two boats at a base is the plan of July 1941. By January 1942 Wake and Midway were cut to one boat and then none, and at Midway in May the boats were raised and moved towards the approach, not drawn in. Rule 14 had it backwards.
- **Rule 3's line was not defined on a hex grid,** had no direction where the nearest enemy base is the centre, and its "nearest enemy base" is often not where the enemy comes from.
- **The staff's figures are constants in the code,** values by date were withdrawn (decision 0213), and a value by side for Japan would move scenario 129 at once.
- **Rules 17 and 18 assumed the staff can move a station it chose.** The carrier station is chosen on the morning its task is formed and kept; moving it is a task ended and formed again, which is new work.
- **The estimate's count was not an algorithm,** "at least" claimed a lower bound the reports cannot give, the belief keeps one report a hex and drops it after six pulses, and `sim-agent` cannot call `sim-staff`.
- **Two citations were wrong:** "usually too late" (16776) is a judgement of 1944; the rows begin at 16436.

### What is built: three stages

**Stage 0. A scenario says how each side's staff uses its boats.**

- A scenario may give a side `submarine_pattern` (`off_bases` or `lines`) and `expects_boats_off_own_bases` (true or false). A side given neither plays as it does today, so no game on file moves, scenario 129 among them. This is the scenario stating its side's doctrine for its navy and date; a doctrine file by navy and date waits for the campaign that needs one.
- The two values are in the state (hashed in their place, the layout version raised) and in a side's own picture. Neither side sees the other's.

**Stage A. The pattern of a patrol the commander orders** (rules 1, 2, 5 and 6 above stand, with these changes).

- **Off bases** (rule 2): a base with no navigable neighbour in the waters is passed over. The second round and the spread count the stations already given. The list of bases is read each morning, and a boat whose station is unchanged is given no order, as now. `boats_per_station` is 2, a judgement from rows that show one or two, marked so.
- **Lines** (rule 3 is replaced):
  - **The approach** is the enemy base on the side's map nearest the centre among those more than `line_gap_hexes` (5) from it; lowest number on a tie. With none there is no line and the boats are spread.
  - **The line** runs through the centre at right angles to the approach. For the approach's vector (q, r) from the centre in axial hexes, the vector across it is (-(q + 2r), 2q + r), which is exact. The hexes of the line are those the hex line-drawing rule gives from the centre along that vector and along its opposite (each point rounded to the nearest hex, the lowest hex on a tie), while they lie in the waters.
  - **Stations** are the centre, then the next hex along the first hand, then the next along the other, and so by turns, each `line_spacing_hexes` (1: 30 miles is less than a hex of 40) farther. A hex ships cannot enter is stepped over and the line goes on beyond it.
  - **The second line** is the same line drawn through the hex `line_gap_hexes` from the centre along the approach, away from the enemy, where that hex is in the waters; boats left when the first line is full go there. Boats left after that are spread.
  - The 5 hexes rest on one source and one battle, and the 1 hex on one source with a repeat in 1944; both are marked so where they are written.
- "Allied" here is the American pattern for every Allied boat; the rows are American.

**Stage D1. The estimate of the enemy's boats, as a reading.**

- It is a function in `sim-core` of one command's picture and nothing else, so an agent's tools and the table can ask it. It reads the reports of submarines the picture holds, and no event: what a command has not been told it does not estimate from.
- **Areas:** the reports are taken in hex order; a report within `estimate_join_hexes` (2) of the first report of an area joins that area, else it begins one. No area joins another, so none chains across the map.
- **The count** of an area is its reports that are more than one hex from every report counted before them in the area, said as "about N" and never "at least".
- **By habit,** for a side whose scenario says so: each of the side's own bases is named as waters where boats may be expected, with no count.
- **It is told** in `list_contacts` beside the reports and on the chart with them. It drives nothing: the regard of decision 0218 reads the reports as it does now.
- `estimate_join_hexes` is a judgement, marked so.

### What is set aside, and what each needs

- **A patrol the staff forms itself** (rules 7 to 10). It needs a patrol that can stand under another order, a boat list of free boats with a count and stations that move up, a ruling on the launch leave, and a pass at the area's level. A boat the commander stationed by his own order must be shut out of it. Until then a boat moves only on a patrol order of the commander's, which answers the designer's second question for now, and a scenario gives the side that plays by its staff alone its patrol orders.
- **The picket** (rules 7 and 14) is withdrawn: the rows of 1942 are against it.
- **Boats moved on a report** (rules 11 to 13). It needs the staff-formed patrol, a memory of the contact that moved the line, and a delay between the order and the boats; without the delay the boats arrive in under a day, where the record has two to three.
- **A station moved after an attack, and the regard reading the estimate** (rules 17 and 18). They need a carrier task ended and formed again at a new station. Decision 0220 needs the same for a force that follows a contact; it is built once, there, and used here after.

### The designer's questions, as they stand

1. The picket's distance is no longer asked: the picket is withdrawn.
2. Whether the staff may send boats out under other orders is set aside with rules 7 to 10, and is asked again when that work is planned.

### Before it is built

The architect's scenario and statements for stages 0 and A together, then D1. The reviewer's order is kept: 0, A, D1.

## As built: stages 0 and A (Nov 23, 2026)

Built to the architect's statements of scenario 131 (`tests/scenario_131_the_pattern_of_a_patrol.rs`), every hex as worked by hand; no statement or hex was changed. State layout `flagplot-state-v58`, saved game version 26, rules label 32, doctrine `supply-58`, agent interface version 51.

**Stage 0.** A scenario gives a side `[side.allied]` or `[side.japanese]` with `submarine_pattern` (`off_bases` or `lines`) and `expects_boats_off_own_bases`, each optional; another word for the pattern is refused when the scenario is read, naming the side and the word. The two values are a new field at the end of the side's `CommandState`, `submarine_doctrine` (`SubmarineDoctrine`: the pattern, `SubmarinePattern` with the stable codes 1 and 2, 0 for none, and the expectation), set by `World::set_submarine_doctrine` before day 1 and never changed after. They are hashed after the commander's refusals in the command state and named in `hash::name_every_field`. Being in the command state, they go with the side's own half of a picture and not with the enemy's, and a world made up by a dry run gives the enemy half an empty command state. `SideView::submarine_doctrine` gives the asking side's own values; a seat at a command of the side is given them too. The briefing's `submarines` part shows them to the side's own seat as `doctrine`; the export writes them in the side's command state. `CommandState::is_empty` is false for a side with a doctrine, so the export writes that side's command state. `expects_boats_off_own_bases` is read by nothing.

**Stage A.** In `crates/sim-staff/src/patrol.rs`, a patrol whose side has a pattern is given its stations by `patterned`; a side with none goes through `stations` as before, unchanged. The three figures are constants at the head of the file with what each rests on: `LINE_GAP_HEXES` 5, `LINE_SPACING_HEXES` 1, `BOATS_PER_STATION` 2. The nearest hex of a point of the line is found in whole numbers: the point's coordinates times the segment's length N are whole, and the squared straight distance to a hex's centre times N squared is dq² + dr² + dq·dr of the whole differences; the hexes within one of the point's own q and r rounded down are tried, the lowest (q, then r) kept on a tie. Each station order's reason says which rule gave the station, naming the base, and carries the staff's reading of the pattern (the approach, the vector across, each line's stations and whether the second lies in the waters; or each base with its stations and why one has fewer), and records the forces limit it speaks of. The reading is a line of a preview's and a proposal's words too, and their `stations` are the stations given.

**Choices the plan did not make.**

- The scenario's key is a table per side, `[side.<side>]`, and the values are held in the side's command state rather than in a new field of the world, so that a picture and a dry run carry them as the side's own without new work.
- "Nearest hex" is by the straight distance between hex centres, the measure an air distance uses; every tie the statements work agrees with it.
- A line's spacing is read as each station at least `line_spacing_hexes` farther from the line's centre than the last station on its hand; with 1 it is every hex of the drawn line that ships can enter.
- The second line's hex is the first hex the line drawn from the centre along the opposite of the approach's vector, divided by the greatest common divisor of its parts, reaches at `line_gap_hexes` from the centre. A hex of the second line already a station is stepped over.
- Off bases: the first round, as the second, takes only hexes not yet a station, so no two bases share one (statement 17); a base whose turn finds no hex is not tried in the second round. A base not reached because the boats ran out is said so ("the boats being fewer"), not passed over. A station of the spread is not kept off a base's own hex (a coast hex ships can enter); no game on file puts one there.
- The games of the statements with land are on two copies of the map, `placeholder_patterns_l3.toml` and `placeholder_patterns_b2.toml`, each with its game's one land hex, so that each game is a file that runs as it stands.
- The briefing's part shows the doctrine only where the scenario states one, so every game on file is briefed as before.

**What the comparison showed.** `scripts/compare-scenarios.sh` against main's commit 4dc0b4da: 254 runs, live and frozen, the same with the hashes cut. The recorded games differ only in the interface version and the hashes; `legibility_*` and `stage_2b_seat_sonnet` are untouched.

### Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Stage 0: the side's doctrine | the scenario's `[side.<side>]` | `CommandState::submarine_doctrine`, once, before day 1 |
| Stage A: the pattern of a patrol | the side's own doctrine; the enemy bases on the side's map; the waters; the boats of the order | station orders, their reasons with the staff's reading; a preview's and a proposal's `stations` |

No draw is made and no event is added.

## The architect's rulings on pull request 712 (Nov 23, 2026)

I read `crates/sim-staff/src/patrol.rs` in full as changed. All 17 statements of scenario 131 pass as I wrote them; no statement and no hex is restated.

1. **Differences row 76 is closed: the code stands.** In the first round as in the second a base takes only a hex that is not yet a station. A base whose turn found no hex is not tried again, and a base the boats did not reach is said so and not "passed over".
2. **Differences row 77 is closed: the code stands.** A station is the next hex of the drawn line that ships can enter, not taken, and at least `line_spacing_hexes` farther from the line's centre than the last on its hand. No game can tell this from another reading while the figure is 1.
3. **The values are the side's own command state,** not a field of the world. That is right: a picture and a dry run carry them, and the enemy's half has none.
4. **Two phrases are taken out of the reasons, by me, after the build.** A reason no longer gives the line's step in axial numbers, and a spread station no longer names decision 0218. A player reads these reasons, and neither says anything he can use. No test read either phrase. Scenario 131 was run again after the change and passes.
5. **The briefing changes shape, so the agent interface is 51.**

Owed, and not proved here:

- The direction of a line is not said in the reason at all now. It should be given as a bearing when the reasons carry fields.
- The table doubles a base's name in these reasons ("Allied base 2 at Allied base 2").
- The table's screen check was not run by the team on this branch; I run it before the tables are restarted.
- `expects_boats_off_own_bases` is read by nothing until stage D1.
- No game on file has two enemy bases that share a neighbouring hex, so ruling 1 is tested by no game.
