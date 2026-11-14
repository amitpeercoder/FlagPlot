# 0103: An airfield damaged and mended

Status: a plan by the architect, written on Nov 14, 2026. Needs review: the three questions at the end are the designer's. Nothing is built. Every figure is a placeholder.

## The gap

Bombs and shells that hit a base destroy aircraft, supply and men, and leave the field whole (decision 0092; rulebook chapter 5, "Hits on nothing"). So a side cannot stop an airfield flying except by destroying every aircraft on it or taking it. The raids and shellings of Henderson Field were made to close it, and the men who filled the craters were why it stayed open. Decision 0099 left the place for this: "when damage comes, mending it is a fourth job".

## Decisions

### Damage

1. **An airfield has damage, from 0 to 100.** A base with no airfield has none. It starts at 0 unless the scenario says.
2. **A hit can land on the airfield.** The table that decides where a hit on a base lands (aircraft, supply, men, nothing) gains a share for the airfield, `[base_hit] airfield`. It counts only if the base has an airfield; otherwise it lands on nothing, as now. Bombs and shells use the one table, as they do today.
3. **Each such hit adds `airfield_damage_per_hit`,** up to 100.

### What damage does

4. **A damaged field flies fewer aircraft.** At the start of each pulse the share of the field that works is 100 less its damage, over 100. Of each group at the base, the aircraft that may fly in that pulse are its ready aircraft times that share, cut to a whole aircraft. The rest stay on the ground, where a hit can find them. At 100 nothing flies.
5. **Aircraft land on a damaged field as on a whole one.** Rebasing to it, landing elsewhere and arrivals are not refused. A field too damaged to launch from can still be landed on, badly; the wear of that is not modelled.
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

None rests on a source. In the frozen step 1 figures the share is `0`, so every earlier hand calculation stands. With these, fifty hits on the field close it, and a battalion of 1,000 mends 20 points a day. The record has rows to tune them on: the field was "never out of operation for more than four hours" once the construction battalion had it (`place_facts`, row 72), and the shellings of 25 August, 5 September and 12 to 14 September have rows.

## Why

- **It gives the raids and the shellings their purpose,** and gives the side that holds a field a reason to land men who mend it.
- **It is one number on a thing that exists,** read where a base out of supply is read already, and mended by a job that exists.
- **A share, and not a field that is open or shut,** so that there is no cliff a player aims for and no figure for where the cliff is.

## What it leaves out

- **A port damaged, and stocks or guns damaged apart from the hits that exist.**
- **Aircraft wrecked by landing on a damaged field.**
- **Craters that slow the building of a field not yet built.**
- **The enemy learning the state of a field.**

## What it costs

- **New state:** a base's airfield damage, in the hash and named beside it; the hash version is raised and the generated files remade.
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

1. **Decision 4:** a damaged field flies a share of its aircraft, falling to none at 100. The other choice is a field that is open until its damage passes a level and then shut.
2. **Decision 9:** nothing mends itself. The other choice is a small daily mending by the base with no unit set to it.
3. **Decision 5:** aircraft may still land on a damaged field. The other choice is to refuse rebasing and arrivals above a level of damage.
