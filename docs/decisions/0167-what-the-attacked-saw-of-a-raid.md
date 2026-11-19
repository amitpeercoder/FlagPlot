# 0167: What the attacked saw of a raid: its weapon and, where recognised, its aircraft. A plan

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the answers the architect recommended; one of them fell away in writing the hand calculation (Confirmed, below). The hand calculation is written. To be built.

Summary: A side whose ship or base is raided from the air is told only how many aircraft came. It is not told what they attacked with, nor what kind of aircraft they were, though the men under the attack saw both. The plan: the attacked side is always told the weapon, bombs or torpedoes; and it is told the type of aircraft where its people recognised it, which they do with a chance that is lower by night. A type not recognised is not named, and no wrong name is given in the first build. With six questions for the designer

Standing: Confirmed by the designer, Nov 19, 2026, as recommended. The hand calculation is written; the build is next

## Why

The designer, reading the war diary, saw "attacked by 9 aircraft" and asked for the type where it was identified. The table cannot say it: the game does not tell the side.

What the game gives today (`side_events.rs`): of an air attack on a side's ship, the ship, its force and the number of aircraft, "not the chance or the hits, nor the air group that attacked"; of a raid on a side's base, everything the log has but the air group. The weapon is left out of an air attack, although a salvo fired at a side's ship does tell it "that it was shot at, and with what" (decision 0065).

What kind of aircraft came is more than colour. Carrier aircraft over a force say that a carrier is within their reach; twin-engined bombers say a base is. Staffs of 1942 acted on exactly this. It is also the enemy's truth, so it may reach a side only as a report of what its own people saw, which may fall short.

## The rule

1. **The weapon is always told.** The side's event of an air attack on its ship says whether the aircraft attacked with bombs or with torpedoes, as its event of a salvo says guns or torpedoes. The men under it know what was dropped at them. A raid on a base is with bombs, and says so.
2. **The type is told where it was recognised.** When an air attack on a ship, or a raid on a base, is resolved, one draw decides whether the attacked side's people recognised the aircraft. Where they did, the side's event names the type, by the name its data has. Where they did not, the event names no type: the side has the number and the weapon.
3. **The chance** is one figure of the air combat figures file, a placeholder marked as such: no source sizes it. (First written with a second, lower figure for the night; no raid flies by night, so it was dropped on confirmation.)
4. **No wrong name, in the first build.** An unrecognised type is not named; it is not named as another. A staff that is told nothing is misled less than one told the wrong thing, and a rule for which type is mistaken for which needs figures nothing supports.
5. **It is recorded at the time.** The true event of the attack gains the outcome of the draw, so that what a side is told of a raid is the same whenever it is asked and a replay shows the same. The draw has its own value of `rng::Draw`, added at the end, and its stream is keyed on the day, the pulse, the air subsystem and the ship or base attacked.
6. **It is a report, not a belief.** Nothing is added to the side's belief state, and no doctrine reads it in this build. The side's events, the briefing's account of yesterday's raids and `explain` carry it.
7. **The attacker is told nothing new.** His own events are as they were.

## What it leaves for later

- **A wrong recognition,** as a contact at sea may be taken for another kind of ship.
- **A staff that reads it:** an enemy carrier believed near because its aircraft were seen. It belongs with the staff's estimate of the enemy's approaches (decision 0161, the tenth step).
- **Recognition by those who fought the raid in the air.** A side's fighters already know whether they fired at bombers or at escorts; they are not asked for the type.

## Rule card

- **Reads:** the attacking group's aircraft type and its weapon; the pulse; the two figures.
- **Writes:** one field of the true event of an air attack on a ship and of a raid on a base. No truth state of a unit, no belief.
- **Reuses:** the side's event of an attack against it, which gains the weapon and the type where recognised; the rule that a side learns of the enemy only by report.

## What it costs

- **A draw and two figures.** The air combat figures file gains two placeholders; the frozen step 1 figures gain them at values that change no hand calculation (the build says how: a figure of 1 by day and by night names every type and draws nothing that alters a fight).
- **Versions.** The true event has a new field and the side's event a new shape: the saved game's version, the agent interface version and the rules label rise, and whichever of the state's layout and the hash the event log enters. The build reports each.
- **Every recorded game and transcript with an air attack** changes in the words of those events and nowhere else: no fight is resolved differently, since the draw's stream is its own.
- **The rulebook:** the chapter on air attack and the one on what a side knows; the glossary gains "recognised".
- **The table:** the plot and the war diary say "attacked by 9 torpedo aircraft, recognised as …" or "by 9 aircraft with bombs, type not made out".

## The proof of it

A scenario of its own, written with the hand calculation after confirmation: one raid on a ship and one on a base, played with the figures at 1 (the type is named), at 0 (it is not, and the weapon still is), and by night. Properties: the attacker's events are unchanged; two worlds that differ only in what the attacked side cannot see give it the same events; the fights are resolved as they were before the rule.

## Questions for the designer

1. **Named rightly or not named, and never wrongly, in this build.** The architect recommends it (item 4).
2. **The weapon is always told.** The architect recommends it: it is what was dropped at them, and the game already tells a ship what a surface salvo was fired with.
3. **A raid on a base is treated as an attack on a ship is.** The architect recommends it.
4. **Night lowers the chance.** The architect recommends two figures, both placeholders, over one.
5. **The staff does not act on it yet.** The architect recommends it: the estimate of the enemy's approaches is where it is read.
6. **The type is said by its own name, not by an Allied code name.** "Betty" and "Val" were only coming into use in the second half of 1942, and the data holds the makers' and services' designations. The architect recommends the name the data has, shortened by the table as it shortens the side's own types.

## Confirmed on Nov 19, 2026

The designer confirmed the plan: "Yes, as recommended."

1. Named rightly or not named, and never wrongly, in this build.
2. The weapon is always told.
3. A raid on a base is treated as an attack on a ship is.
4. **Night: fallen away.** The designer confirmed two figures, one for the night. Writing the hand calculation, the architect found that no raid flies in the night pulse: a day's air step is worked in the morning and the afternoon only. A figure for the night would be read by nothing, so the rule has one figure, `raid_recognised_chance`. A night figure is added with the rule that first flies a raid by night.
5. The staff does not act on it yet.
6. The type is said by the name its data has, not by an Allied code name.

## The figure

`raid_recognised_chance` in the air combat figures file: 0.75 in the live figures, a placeholder that rests on no source; 0 in the frozen step 1 figures, where a side was told no type, so that every hand calculation on them holds.

## The hand calculation

It is in `tests/scenario_88_what_the_attacked_saw.rs`, written before the code. It has no scenario file of its own: it plays the morning raid of scenario 19a, an attack on a ship that is certain to be made, and the first raid of scenario 6a, on a base, with the figure at 1, at 0 and at 0.75.

