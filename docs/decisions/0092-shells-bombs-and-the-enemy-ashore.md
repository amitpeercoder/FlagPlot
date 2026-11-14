# 0092: Shells and bombs on a base and its men, and what a side knows of the enemy ashore

Status: a plan by the architect, written on Nov 14, 2026. It is stage 3 of the ground combat plan the designer approved (`docs/design/ground-combat.md`). The designer answered its three questions on Nov 14, 2026: shells are weighed by the gun (decision 5), the estimate is daily (decision 14), and a claim follows a bombardment (decision 17). Built on Nov 14, 2026: see "As built" at the end. Every figure is a placeholder set by the architect. The hand calculation is written before any code is delegated.

## The gap

- **The bombardment mission does nothing.** A force's guns cannot hurt a base, so the October bombardments of the airfield cannot be played (decision 0052, item 2).
- **Nothing from the sea or the air hurts men ashore.** A raid on a base destroys aircraft and supply only.
- **A side knows nothing of the enemy ashore.** After stage 2 it learns that a fight happened and what it lost. It cannot judge whether an attack will succeed, and it cannot be wrong about it.
- **A base formed by a landing can never be found** by the other side (decision 0066, 12).

## Decisions

### Shore bombardment

1. **A force bombards in the shore bombardment phase of the night pulse** if its mission is bombardment, it has a ship afloat, and its position after the night's movement is a hex that holds an enemy base on its side's map. The base is the lowest-numbered such base. A base the side has not found cannot be bombarded.
2. **Each ship fires its guns for `bombardment_minutes`.** Its shots are its class's rounds a minute, times its guns, times the minutes, times its main battery's factor, and no more than the rounds it has aboard: the rule of surface combat for a round of fire, with a longer time. The rounds are spent. Nothing but the yard gives them back, as now.
3. **Each shot hits at `bombardment_hit_chance`,** rolled, on a stream keyed on the day, the night pulse, the surface combat subsystem, a new draw `Bombardment`, the ship and the base.
4. **Each hit lands by the table that says where a bomb on a base lands** (decision 0034): on aircraft, on supply, on men (decision 6), or on nothing. All aircraft at the base are on the ground at night. A hit on aircraft destroys one, and a hit on supply destroys the fixed weight, by the rules as they are.
5. **Shells are weighed by the gun.** A ship's shots that are rolled are its shells fired times its class's gun damage over `bombardment_reference_gun_damage`, with the part of a shot rolled at that part of the chance. So a heavy shell counts for several and a light one for a part of one. The rounds spent are the shells fired. *The designer's choice, Nov 14, 2026. The architect had proposed that every shell count the same. How much a heavier shell does to a field rests on no row yet: the weight is the gun damage the game already derives from the shell's weight.*
6. **Nothing ashore fires back,** and the base's anti-aircraft guns do not fire at ships.
7. **The base's side gets a sighting report of the force,** with the errors of any sighting report, drawn on the first firing ship's stream after its hits and where they landed. Being shelled shows that ships are there.

### Men as a place for a hit to land

8. **The table gains a share for men:** `[base_hit] men`. When the base's side has men ashore in the base's hex, a hit in that share lands on men. When it has none, the share lands on nothing. One draw for each hit decides, as now, so a raid on a base with no men ashore gives the hits it gave before.
9. **A hit on men removes `men_lost_per_base_hit` men,** as many as are there, taken from the side's units in the hex in proportion by the rule of stage 1 (decision 0066, 20). It needs the ground figures; without them the share lands on nothing.
10. **This serves bombs and shells alike.** A base strike from the air reaches men by the same table.

### The estimate of the enemy ashore

11. **A side's belief holds an estimate of enemy men for a hex:** a count of men, the day it was reported, and the report's event. There is one for a hex, and a new report replaces it. It is never aged or dropped: its day says how old it is.
12. **It is written only by a ground report,** through `BeliefState::apply_report`, as every belief is.
13. **A ground report is made each day in each hex where both sides have men ashore,** in the ground combat phase, after the day's fight and losses, whether or not there was a fight. Each side gets a report of the other's men there.
14. **The report is the true count times an error,** cut to a whole man. The error is drawn evenly between `report_error_low` and `report_error_high`, on a stream keyed on the day, no pulse, the intelligence subsystem, a new draw `GroundReport`, and the reporting side's lowest-numbered unit in the hex. *Needs review: the estimate is fresh every day and each day's error is new, so a side that watches for a week can see roughly where the truth lies. The other choice is a report only on the day of a fight or a landing.*
15. **A ground report finds the enemy's bases in the hex.** A base formed by a landing that is not on the reporting side's map is put on it, with an event that side sees. This is the report decision 0066 left to this stage.
16. **A scenario may give a side an opening estimate:** a side, a hex and a count of men. It goes in through `apply_report` on day 0 with no error, as a starting contact does. For a historical scenario it is what that side believed, from a source, right or wrong.

### What a side knows

17. **The side that bombards gets a claim, as after a raid from the air.** One claim for a force for a night: the aircraft it believes it destroyed on the ground. It is the aircraft really destroyed plus false ones, rolled from the force's weighed shots less the aircraft really destroyed, at `bombardment_false_claim_chance`, on a stream keyed on the day, the night pulse, the intelligence subsystem, the draw `Claim`, the first firing ship and the base. It is never below the truth. It goes into belief by `apply_report` and is never corrected. There is no claim for supply or for men. The side is also told its own shells fired. *The designer's choice, Nov 14, 2026. The architect had proposed no claim.*
18. **The base's side sees the bombardment whole but for who fired:** the shots, the hits and where each landed, its aircraft and supply destroyed, and its men lost.
19. **A side sees its own estimates and the reports that made them.** It never sees the enemy's estimate of its own men. The error drawn is not shown to anyone but the umpire.
20. **Men lost to a bomb or a shell are seen whole by their own side.** The raider is told nothing of men hit. They count in the `men_lost` tally.

## Placeholder figures

| File | Key | Value | What it is |
| --- | --- | --- | --- |
| `surface_combat.toml` | `bombardment_minutes` | `30` | Minutes a ship fires in a night's bombardment |
| `surface_combat.toml` | `bombardment_hit_chance` | `"0.05"` | The chance a weighed shot hits something the table names |
| `surface_combat.toml` | `bombardment_reference_gun_damage` | `20` | The gun damage of a shell that counts as one shot |
| `surface_combat.toml` | `bombardment_false_claim_chance` | `"0.01"` | The chance a weighed shot that destroyed no aircraft is claimed as one |
| `air_combat.toml` | `[base_hit] men` | `"0.2"` | The share of hits on a base that land on men |
| `ground.toml` | `men_lost_per_base_hit` | `10` | Men a bomb or shell hit removes |
| `ground.toml` | `report_error_low` | `"0.5"` | The least a ground report can be, as a share of the truth |
| `ground.toml` | `report_error_high` | `"1.5"` | The most |

Each goes into the frozen step 1 figures with the same first value. None rests on a source. A cruiser of 6 guns at 1.13 rounds a minute, with a gun damage near 20, fires 203 shells in a night and expects about 10 hits: with the table as it stands, under 1 on aircraft, 3 on supply, 2 on men and the rest on nothing. A destroyer's shell, with a gun damage near 4, counts as a fifth of that.

The table is read from `air_combat.toml`, so a bombardment needs the air combat figures as well as the surface combat ones. Without either the phase does nothing.

## The test

Scenario 24, of made-up ships, units and bases marked placeholder, in two parts: a night bombardment of a base with aircraft, supply and men; and an attack ordered on an opening estimate that is far too low, which fails. The hand calculation gives each ship's shots and chance, what follows from the hits the run rolls, and the bounds every ground report must lie within. Beside it, property tests:

- **A side's view is the same in two worlds that differ only in enemy men it has no report of.**
- **A ground report lies between the low and high share of the truth,** always.
- **A bombardment never destroys more than is there:** aircraft, supply or men.
- **A raid on a base with no men ashore gives the hits it gave before** the table gained its share for men.
- **Rounds are conserved:** a ship's rounds before a bombardment equal its rounds after plus its shots.
- **A scenario with no bombardment mission, no men and no estimate is unchanged,** apart from the hash version.

## Why

- **One table for what a hit on a base does.** Shells, bombs and any later weapon land by it, and men are one more row.
- **The bombardment reuses the gun rule,** so a damaged main battery and an empty magazine count as they do in a fight.
- **The estimate is belief like any other:** one writer, the report, and a side can be wrong. An opening estimate from the record is how a side's hopefulness gets into the game without a rule for it.
- **A daily report keeps the rule small.** One rule covers a fight, a landing beside the enemy and a long siege.

## What it leaves out

- **Runway damage.** A bombardment destroys aircraft and supply and does not close the field.
- **Bombardment by day, and by a force that is passing.** The force must end the night's movement in the hex. Note of Nov 14, 2026: a force may now act at a marked waypoint and steam on (decision 0096).
- **Fire from the shore on ships.**
- **A claim of supply destroyed or of enemy men lost.**
- **Patrols and scouts ashore.** A side with no men in a hex gets no ground report of it, and aircraft report nothing of men.
- **The ground support mission** (the plan defers it).

## What it costs

- **New state:** a side's ground estimates, in its belief. They go into the hash and are named beside it; the hash version is raised and the golden files remade (decision 0057). A base's place on the map can now change by a report.
- **Two new draws,** `Bombardment` and `GroundReport`, added at the end.
- **The mission is read by a rule for the first time.** Every existing scenario's forces keep their missions; none has bombardment.
- **A new kind of report,** new events with their rows in `side_events.rs`, the scenario's opening estimate, and the agent interface: the estimates in the briefing and queries, the bombardment in the briefing of both sides.
- **The rulebook:** chapter 4 gains shore bombardment, chapter 5 the share for men, chapter 3 the ground report and the estimate, chapter 7 what a side knows of the enemy ashore and men lost to fire, and chapter 1 the phase, the draws and the events, with the glossary, in the same pull request as the code.

## Rule card

| | |
| --- | --- |
| Reads | A force's mission, position and ships; a ship's guns, rounds and main battery; the bases in the hex, their side, stock and whether they are on the map; the air groups at a base; the land units ashore in a hex; a side's estimates; the figures |
| Writes | A ship's rounds; an air group's aircraft; a base's supply and whether it is on the map; a land unit's men; a side's contacts and ground estimates; the events |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 5:** shells are weighed by the gun.
2. **Decision 14:** a fresh estimate each day, wrong by up to half either way. Agreed.
3. **Decision 17:** a claim follows a bombardment, as after a raid from the air.

## As built

Built on Nov 14, 2026, to the sections above and the architect's hand calculation in `tests/scenario_24_shells_and_estimates.rs`. Nothing above this section was changed. Every choice the build made that the record did not is listed here. The rulebook's chapters 1, 3, 4, 5, 6 and 7 state the rules as built.

1. **Missing figures.** The four bombardment keys of `surface_combat.toml` are given all or none; with none, no force bombards. `[base_hit] men` missing gives 0, and `men_lost_per_base_hit` missing gives 0. `report_error_low` and `report_error_high` are given both or neither; with neither, no ground report is made. A partial set is refused by the loader. World set-up refuses the report error figures without the search figures, because belief is written with them.
2. **Scenario keys.** A base takes `on_map` (`true` by default): `false` puts it off the enemy's map at the start. An opening estimate is `[[estimate]]` with `side`, `q`, `r` and `men`. It is refused off the map, as a second for the same side and hex, without the search figures, and after day 1 has begun. The loader refuses unknown keys and floats, as everywhere.
3. **A ship with no shell to fire** (no rounds, or its main battery out) does not fire and records nothing. A force none of whose ships fires records nothing: no bombardment, no sighting report and no claim. The first firing ship is the lowest-numbered ship that fires. A force away at the yard does not bombard. The share of the guns a force that is not ready fires (decisions 0068 and 0071) is not applied: a bombardment is not a meeting.
4. **The events.** Each ship's `BombardmentFire`, then the force's `ShoreBombardment` citing them. The aircraft, supply and men destroyed, and the sighting report, cite the force's event; the claim cites the ships' fire. A ship's fire cites its force's route order and its arrival. The base's side is given the force's event cut of the force (shells, weighed shots, hits and where they landed) and not the ships' events, which would tell it how many ships fired.
5. **The share for men** applies only when the ground figures are loaded with `men_lost_per_base_hit` above 0 and the base's side has men ashore in its hex at the start of the attack. A raid's breakdown names the factor `hit_on_men_chance`, and its text says "on men", only then, so a raid with no men ashore reads exactly as before.
6. **A ground report's error** is drawn as a whole number of millionths from the low bound to the high, both included. The reports of a hex come at the end of its part of the ground combat phase, after the fight, the losses, fatigue and any base that changed side, the Allied side's first. A report cites the day's fight there if there was one, and otherwise nothing. Its breakdown (the true count, the bounds and the error drawn) is the umpire's: a side is given the report with no breakdown.
7. **A ground report puts on the reporting side's map every enemy base in the hex that is not on it,** a scenario's hidden base as well as one formed by a landing.
8. **A claim gains `from_bombardment`.** It reads "claim from the sea at (q,r): N aircraft destroyed on the ground". Its misses are the force's weighed shots less the aircraft really destroyed, never below 0.
9. **Scenario 23,** the only scenario before this one with both sides ashore in a hex, now has ground reports each day at X and on days 1 and 2 at Y: its truth is unchanged (the same fights, losses and bases) and its log gains the reports, so its baseline was remade. Every other scenario gives the same events and results and differs only in the state hash (version 13).
10. **Men lost to fire** count in the `men_lost` objective of decision 0090, which reads `MenLostToFire` beside `MenLostWithShip`, `MenLostUnfed` and `MenLostInFight`, and in the batch record's `men_lost` tally, under the cause `fire`.
11. **The agent interface.** The briefing gains a part for the estimates of the enemy ashore and the bases found (`enemy_ashore`), and in its action part a side's bombardments, its ships' shell fire and the bombardments of its own bases; a raid line names hits on men only when above 0; the men lost in `lost_yesterday` have the cause `fire`; `list_contacts` answers `ground_estimates`. The briefing's schema version stays 3 (an added field does not raise it, by the schemas README); the batch record gains `men_lost` and `bombardment_shells` and stays version 5. `sim-cli run` prints each side's estimates at the end when it holds any. The viewer shows a bombardment in the event log and not on the map.
12. **The state hash is version 13.**
