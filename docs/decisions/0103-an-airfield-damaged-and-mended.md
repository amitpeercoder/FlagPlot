# 0103: An airfield damaged and mended

Status: a plan by the architect, written on Nov 14, 2026. The designer answered its three questions on Nov 14, 2026: a share of the field flies (decision 4), nothing mends itself (decision 9), and landing on a badly damaged field is refused (decision 5, his choice against the architect's proposal). Confirmed. Built on Nov 14, 2026 ("As built" at the end). Every figure is a placeholder.

## The gap

Bombs and shells that hit a base destroy aircraft, supply and men, and leave the field whole (decision 0092; rulebook chapter 5, "Hits on nothing"). So a side cannot stop an airfield flying except by destroying every aircraft on it or taking it. The raids and shellings of Henderson Field were made to close it, and the men who filled the craters were why it stayed open. Decision 0099 left the place for this: "when damage comes, mending it is a fourth job".

## Decisions

### Damage

1. **An airfield has damage, from 0 to 100.** A base with no airfield has none. It starts at 0 unless the scenario says.
2. **A hit can land on the airfield.** The table that decides where a hit on a base lands (aircraft, supply, men, nothing) gains a share for the airfield, `[base_hit] airfield`. It counts only if the base has an airfield; otherwise it lands on nothing, as now. Bombs and shells use the one table, as they do today.
3. **Each such hit adds `airfield_damage_per_hit`,** up to 100.

### What damage does

4. **A damaged field flies fewer aircraft.** At the start of each pulse the share of the field that works is 100 less its damage, over 100. Of each group at the base, the aircraft that may fly in that pulse are its ready aircraft times that share, cut to a whole aircraft. The rest stay on the ground, where a hit can find them. At 100 nothing flies.
5. **A badly damaged field takes no aircraft.** While a field's damage is at or above `airfield_landing_refused_at`: a rebase order to it is refused; aircraft that cannot get home do not land there and go to the next place the rule gives (decision 0059); and an arrival due at it waits, and is delivered on the first night the field is below the level. Below the level aircraft land as on a whole field. *The designer's choice, Nov 14, 2026. The architect had proposed that landing is never refused.* An arrival waits and is not lost, the architect's ruling: the aircraft are at a base in the rear and come when the field can take them. Decision 0101, item 8, is changed to match: an arrival due at a base that is not yet its side's, or has no airfield, waits too.
6. **Damage does nothing else.** It does not touch the port, the stocks, the guns or the men.

### Mending

7. **Mending is work on the airfield.** A unit whose job is the airfield, at a base whose field is built and damaged, lowers the damage by its work over `work_per_airfield_damage_point`. A fighting unit set to it works at half, as at any job (decision 0099, 3a).
8. **A working unit with no order mends first.** Its default job is the airfield where the base has none or the field is damaged, and else the port.
9. **Nothing mends itself.** A field with no one working on it stays as damaged as it is.
10. **Work is done at the end of the day** (decision 0099, 3), so a shelling by night costs that day's flying and is mended for the next. That is the order the record has.

### What a side knows

11. **A side sees its own field's damage whole,** and the aircraft it kept on the ground. **The enemy sees none of it.** A raid's claim says nothing of the field, as it says nothing of supply today. Learning what a raid did to a field is reconnaissance, which the game does not have (decision 0102).

## Placeholder figures

| File | Key | Value | What it is |
| --- | --- | --- | --- |
| `air_combat.toml` | `[base_hit] airfield` | `"0.2"` | The share of hits on a base that land on the airfield |
| `ground.toml` | `airfield_damage_per_hit` | `2` | Damage one hit adds |
| `ground.toml` | `work_per_airfield_damage_point` | `50` | Work to mend one point |
| `ground.toml` | `airfield_landing_refused_at` | `50` | Damage at or above which a field takes no aircraft |

None rests on a source. In the frozen step 1 figures the share is `0`, so every earlier hand calculation stands. With these, fifty hits on the field close it, and a battalion of 1,000 mends 20 points a day. The record has rows to tune them on: the field was "never out of operation for more than four hours" once the construction battalion had it (`place_facts`, row 72), and the shellings of 25 August, 5 September and 12 to 14 September have rows.

## Why

- **It gives the raids and the shellings their purpose,** and gives the side that holds a field a reason to land men who mend it.
- **It is one number on a thing that exists,** read where a base out of supply is read already, and mended by a job that exists.
- **A share, and not a field that is open or shut,** so that there is no cliff a player aims for and no figure for where the cliff is.

## What it leaves out

- **A port damaged, and stocks or guns damaged apart from the hits that exist.**
- **Aircraft wrecked by landing on a damaged field** below the level.
- **Craters that slow the building of a field not yet built.**
- **The enemy learning the state of a field.**

## What it costs

- **New state:** a base's airfield damage, and whether an arrival has been delivered, in the hash and named beside it; the hash version is raised and the generated files remade.
- **The hit table gains a share,** and the raid and the bombardment each a breakdown factor, named only when the share counts.
- **The count of aircraft that may fly** is read by search, strikes, patrol and escort.
- **New events** with their rows in `side_events.rs`, a scenario key, and the agent's view of its own base.
- **The rulebook:** chapters 4, 5 and 7 and the glossary, in the same pull request.

## Rule card

| | |
| --- | --- |
| Reads | A base's airfield and its damage; the hit table; a group's ready aircraft; a unit's job and work; the three figures |
| Writes | A base's airfield damage; the events. Search, strikes, patrol and escort read the aircraft that may fly |

## Questions for the designer

Answered on Nov 14, 2026.

1. **Decision 4:** a damaged field flies a share of its aircraft, falling to none at 100. Agreed.
2. **Decision 9:** nothing mends itself. Agreed.
3. **Decision 5:** the designer chose that rebasing and arrivals are refused above a level of damage, and aircraft with no deck go elsewhere.

## As built

Built on Nov 14, 2026 (scenario 31, state version 19, rules label 2; the hand calculation is in `tests/scenario_31_airfield_damage.rs`, on the placeholder map `data/map/placeholder_airfield_damage.toml`). Every number of the hand calculation matched the run. The architect corrected one input before it ran: the bombers are the placeholder dive bomber (type 3, strike radius 250 nm) and D is at (5,4), four hexes from B, since no bomber of scenario 24 exists and type 3 could not reach (15,0); no expected number changed. Choices the record did not make:

- **State.** A base's `airfield_damage` (fixed point, 0 to 100) and an arrival's `waiting`, hashed after the base's `day_work` and after the arrival's pilots (`flagplot-state-v19`). `waiting` is the record's "whether an arrival has been delivered" turned round: an arrival is looked at on its own day as before, and on a later night only while it waits, so every arrival that does not wait behaves exactly as before. The damage of each base at the start of a pulse is kept as scratch beside the grounded bases (not hashed); the engine and `SideView` read the same scratch.
- **Where a hit lands.** The airfield's share comes after aircraft, supply and men, and counts only where the base has an airfield and the ground figures are loaded (its damage per hit is a ground figure); otherwise it lands on nothing. `[base_hit] airfield` is optional and 0 when absent, like the share for men; the shares still add to 1 at most. The existing raid and bombardment events are unchanged, so a hit on the airfield shows in them as on nothing; the new event `AirfieldDamaged` (the base, its side, the hits, the damage after) cites the raid, or the force's bombardment, and is recorded even when the damage was already 100. The raid's breakdown and each ship's bombardment fire name `hit_on_airfield_chance` only where it counts and is above 0.
- **What may fly.** `able_to_fly` takes the field's damage; the factor (100 less the damage, over 100) is multiplied with the deck and fatigue factors before the one cut to a whole aircraft, and applies only to a group at a base.
- **A field that takes no aircraft** is one helper (`landing::takes_aircraft`: an airfield, and damage below the level, or no ground figures) used by the rebase check, the landing places of aircraft with no deck, the side's landing question and arrivals. The rebase refusal `AirfieldTooDamaged` is tried after "no airfield" and before the transfer range. A group's own aircraft coming back from a sortie to its own field land as before: the record names only the three cases.
- **An arrival that waits** records `ArrivalWaiting` (the group, its base, the aircraft, the pilots, the damage) on each night it waits, with no cause, as `AircraftArrived` has none. It waits also where the group's base is not its side's or has no airfield (decision 0101 as amended), which no scenario has today. A waiting arrival whose group is lost, or aboard a ship off the map, is dropped as such arrivals were before.
- **Mending.** The `UnitWorked` event is as before (its total is the work done on the airfield); `AirfieldMended` (the unit, the base, the points, the damage after) follows it, citing it, only where there was damage to mend; the points are no more than the damage. The default job is the airfield where the base has none or its field is damaged, else the port.
- **What a side sees.** The three events are seen whole by the side whose base, unit or group it is, and by nobody else. The agent shows, for the side's own base with damage above 0, the damage, the share that flies and whether it takes aircraft; on a group's line the aircraft the field keeps on the ground; and in the briefing yesterday's hits on the field, mending and arrivals that waited. Nothing appears where the damage is 0, so every golden briefing is unchanged. An enemy base shows nothing new. The state export names a base's damage only above 0. The MCP instructions gained a paragraph, "AIRFIELD DAMAGE". The schemas stay at version 3: fields only.
- **The figures** are the four of the record, `[base_hit] airfield` 0.2 in the live figures and 0 in the frozen copy, and the three ground figures at 2, 50 and 50 in both, required in `ground.toml`.
- **What changed in existing scenarios.** Every frozen and live scenario gives the events it gave before, apart from the hashes, except scenario 24: its first bombardment of base 1 puts 5 hits on the airfield (damage 10), one new event that moves the later event numbers by one. No other live scenario loads the ground figures with a raid or bombardment on a base with an airfield, so the slice and the Rabaul scenarios do not change. The rules label rose to 2 because scenario 24 changed. The recorded games changed in their saves only (state and rules versions, the data-file hash of `air_combat.toml`, the daily hashes); the batch fixture of scenario 6 and the pinned hashes of scenarios 10 and 17a in their hashes only.
