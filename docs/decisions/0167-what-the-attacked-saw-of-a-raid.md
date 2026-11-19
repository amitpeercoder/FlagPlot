# 0167: What the attacked saw of a raid: its weapon and, where recognised, its aircraft. A plan

Status: a plan by the architect, written on Nov 19, 2026 and confirmed by the designer the same day with the answers the architect recommended; one of them fell away in writing the hand calculation (Confirmed, below). The hand calculation is written. Built on Nov 19, 2026 (As built, below).

Summary: A side whose ship or base is raided from the air is told only how many aircraft came. It is not told what they attacked with, nor what kind of aircraft they were, though the men under the attack saw both. The plan: the attacked side is always told the weapon, bombs or torpedoes; and it is told the type of aircraft where its people recognised it, which they do with a chance that is lower by night. A type not recognised is not named, and no wrong name is given in the first build. With six questions for the designer

Standing: Confirmed by the designer, Nov 19, 2026, as recommended. Built on Nov 19, 2026

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

## As built

Built on Nov 19, 2026, on the branch `raid-seen`. Every statement of the hand calculation holds (`tests/scenario_88_what_the_attacked_saw.rs`, 13 tests); over seeds 1 to 1000 at 0.75 the type is recognised on 766 seeds, within 696 to 804.

**What changed where.**

- `sim-core`, `events.rs`: the true `AirAttack` and `BaseAttacked` gain one field, `recognised: Option<AircraftTypeId>`, the type the people under the attack recognised, or none. Their words end "; recognised as aircraft_type:N" where a type was recognised and are as before where none was, so a world on the frozen figures reads as it did (see reading 12).
- `sim-core`, `side_events.rs`: `AirAttackAgainst` gains `weapon` and `recognised`; its words are "ship:N attacked by A aircraft with bombs, recognised as aircraft_type:N" or "..., type not made out". `BaseAttackedAgainst` gains `recognised`; its words begin "bombs at" as before and end "; recognised as aircraft_type:N" or "; type not made out". The attacker's `AirAttack` and `BaseAttacked` are cut of `recognised` and are as they were. The cut table test says so.
- `sim-core`, `rng.rs`: `Draw::RaidRecognised`, code 13, added at the end. No pinned number changed.
- `sim-core`, `airstrike/air_combat.rs`: the draw, made as each `AirAttack` and `BaseAttacked` is recorded (step 4 of the air combat step), in the air combat subsystem, keyed on the day, the pulse and the ship or base attacked.
- `sim-core`, `refdata.rs`, and `sim-data`, `reference.rs`: the figure `raid_recognised_chance`, required, a chance from 0 to 1; 0 in the unit tests' figures.
- Data: `data/reference/air_combat.toml` 0.75, a placeholder resting on no source; `data/reference/step1/air_combat.toml` 0, and its `README.md` says so. No scenario names a placeholder variant of the air combat file: every scenario names `data/reference/air_combat.toml` (the live figures, or the frozen copy through `LoadOptions`), so there was no other file to give the figure.
- `sim-agent`: the events tool and `explain` say the type by the name its data has ("recognised as Placeholder dive bomber"). The briefing's `attacks_on_ships` from the air are counted by ship and weapon, as salvos are, each with `weapon`, `recognised` (the names, each once) and `not_recognised` (a count); `raids_on_bases` gain `weapon` ("bombs") and `recognised` (a name or null). Its text: "Attack on ship:1 ... from the air: 27 aircraft with bombs, 2 attacks, recognised as ..." and "Raid on ...: 6 aircraft attacked with bombs, type not made out, ...". The schemas and `schemas/README.md` say so.
- `sim-cli`: the export's true events carry `recognised` where a type was recognised, and the attacked side's air attack `weapon` and `recognised`; a file where none is recognised has no new key there.
- The rulebook: chapter 5, "What the attacked saw", and the two attacks' "A side sees"; chapter 1, the tables of what a side sees and the list of `Draw` values; chapter 8, the versions. The glossary gains "Recognised".

**Versions.** The agent interface rises from 27 to 28: an air attack's `weapon` in the briefing changes meaning (it was always null) and the side's events' words change. The state's layout (`flagplot-state-v48`) and the hash do not change: no field of the state is added, and the event log enters the hash by its count only. The saved game's version (23) does not change: a saved game holds no event. The rules label (20) does not change: see reading 6.

**Generated and pinned files.** Remade by `sh scripts/regenerate.sh`: 20 run baselines (`*.events.txt` of scenarios 5, 5b, 5c, 5d, 6, 6a to 6d2, 7, 12a, 12b, 12c, 13b, 16, 31, 50, 73 and 83), in 86 attack lines only, each gaining "; recognised as aircraft_type:N", no hash line; 3 agent goldens, in the words of the raid and attack lines only; 5 recorded games, in the answers' version and, in three, the words of what the attacked side is told (their README has the paragraph), and 4 of their `save.json` in the hash of `air_combat.toml` only; the batch fixture of scenario 6, in the hash of `air_combat.toml` only. No pinned hash changed. By hand, wording and versions only: `tests/scenario_06b_wear_and_repair.rs` and ten other scenario tests (`recognised: None` in an expected true event, or `..` in a pattern; no number moved), `tests/scenario_09a_side_events.rs` (see reading 13), `crates/sim-agent/tests/legibility.rs` and `amendments.rs` (the new words), `play.rs`, `validator.rs`, `holds_and_preview.rs` and `interrupts.rs` (version 28), and the four export digests of `crates/sim-cli/tests/export_pictures.rs` for scenarios 5, 31, 50 and 58, checked by taking the new words, fields and file hash back out, after which each export hashes as pinned before.

**Readings made that the record did not, for the architect to rule on.**

1. **One field carries the outcome.** The rule card's "one field" is `recognised: Option<AircraftTypeId>` on each true event: the type where recognised, none where not. The type is the attacking group's own, read when the attack is recorded.
2. **A raid on a base gains no weapon field.** Its side's words already begin "bombs at", which says it; the briefing's raid gains `weapon`, always "bombs".
3. **Two attacks on one ship in a pulse share its stream.** Item 5 keys the stream on the ship or base alone. Each ship or base attacked opens one stream for the pulse, and each attack on it draws the next number, in the order the attacks are recorded (group order). A stream opened afresh for each attack would give every attack on one ship in one pulse the same draw; keying it on the attacking group too would make what the side is told move with the enemy's group numbers.
4. **The draw is taken at 0 and at 1 too.** `chance` always takes a number. It is a stream of its own, so nothing else moves.
5. **The air subsystem is `Subsystem::AirCombat`.** The claims of a raid draw in the intelligence subsystem; this draw is in the air combat one, as item 5 says.
6. **The rules label is not raised.** The record's "What it costs" expected the saved game's version and the rules label to rise. By the project's rule the label rises when a change makes an existing game do something else; this one changes no fight, order, staff choice or daily hash, and a replay of a saved game gives the same hashes. A game saved before it is refused anyway, since `air_combat.toml` changed. The saved game holds no event, so its version stays too.
7. **The briefing counts air attacks on a ship by weapon.** One entry per ship and weapon, as salvos are, so that each entry has one weapon. The types recognised are listed once each, and the attacks not made out are counted.
8. **"A game saved and replayed gives the same"** is tested in scenario 88 as the same scenario, seed and orders played again from a fresh load, which is what a replay does; the save and replay of `sim-cli` are exercised on the recorded games, whose daily hashes did not move.
9. **"Nothing more is told"** is tested with the enemy's task force and air groups renamed: the Allied side's events are the same, field and word.
10. **The record's summary** still speaks of a chance "lower by night", which the confirmation dropped. It is the architect's line and is left as it is.
11. **The figures' list.** `docs/validation/tuning/air-figures-closed.md`, which lists every air figure, does not yet name `raid_recognised_chance`; it is left to the architect, who keeps the tuning records.
12. **The true event says nothing where no type was recognised.** It names the type where one was recognised ("; recognised as aircraft_type:N") and is word for word as before where none was. A first build that wrote "; type not made out" in the true event too broke the word-for-word checks of the step 1 hand calculations (scenarios 5, 5b, 5c, 5d, 6a and 11 compare the log's words), whose worlds are on the frozen figures and recognise nothing; the attacked side's event still says "type not made out", since that is what the side is told.
13. **Scenario 9a's property is amended to the new rule.** Its check that a side is given no truth of the enemy asserted that an air attack against the side carries no `weapon`, decision 0065's cut; decision 0167, item 1, tells the weapon. The check now asserts that the event carries no chance and no hits. It is a code comment and an assertion, not a statement of a hand calculation; the architect may prefer another form.

