# 0168: A raid recognised by the experience of the ship attacked: an amendment to decision 0167

Status: a plan by the architect, written on Nov 19, 2026 at the designer's suggestion and confirmed by him the same day as recommended. The hand calculation is written. Built on Nov 19, 2026 (As built, below), on decision 0167's build.

Summary: Decision 0167 gives every attacked ship the same chance of recognising the aircraft that raided it. Recognising aircraft was a trained skill, and the game already holds the experience of a ship's crew. The chance for a ship is now moved by its own experience, by the form the dark surface fight uses and a strength figure of its own; a crew of the standard experience recognises as before, and a base, which has no experience, keeps the plain chance

Standing: Confirmed by the designer, Nov 19, 2026, as recommended. Built on Nov 19, 2026

## Why

The designer asked whether the chance of recognising an aircraft should depend on the experience of the force attacked. It should. A ship long in the theatre had lookouts and gunners who knew a torpedo plane from a dive bomber as it came in; a crew new to it often did not, and reported "aircraft".

The game has what it needs. A ship has an experience, a whole number from 0 to 4 with 2 its service's standard, set when the scenario loads and written by no rule (decision 0064). Three chances of a dark surface fight read it by one form, `1 + strength x (experience - 2)`, each with a strength figure of its own, and a land unit's fire reads the unit's experience by the same form. Nothing new is built: one more chance reads the same field in the same way.

## The rule

1. **A ship's chance is moved by its own experience.** Where an air attack on a ship is resolved, the chance that its people recognise the aircraft is `min(1, raid_recognised_chance x experience_factor)`, in place of the plain figure of decision 0167, item 3. The one draw of that decision is made against it.
2. **The factor** is `1 + raid_recognised_experience_strength x (experience - 2)`, with the experience of the ship attacked. With the strength at 0 it is 1 and nothing is multiplied, so the chance is the plain figure to the last digit. A crew of experience 2 has a factor of 1 at any strength.
3. **The ship attacked, not its force.** Each air attack on a ship has its own event and its own draw (decision 0167, item 5), so each ship of a force is asked on its own experience. Two ships of one force under one raid may differ in what they made out, and the side is told both.
4. **A base keeps the plain chance.** A base has no experience. A raid on a base is recognised with `raid_recognised_chance` as decision 0167 has it. Taking the experience of the men ashore there is left for later (below).
5. **The strength** is one figure of the air combat figures file, `raid_recognised_experience_strength`, a placeholder marked as such: no source sizes it. It is refused below 0 or above 0.5, as the strengths of the surface fight are, so that no factor falls below 0.
6. **By day.** The rulebook says experience is read in a dark fight only. That stays true of the surface fight. This is the first chance read by day that a ship's experience moves, and the rulebook is restated to say so.
7. **Nothing else of decision 0167 changes.** The weapon is always told. A type not recognised is not named, and never named wrongly. The draw, its stream and what the true event records are as that decision has them; the attacker is told nothing new; no belief is written and no doctrine reads it.
8. **The breakdown** of the draw names the plain chance, the experience and the factor where the strength is above 0, and the plain chance alone where it is 0, as the surface fight names its factor only where it applies.
9. **Experience is not written.** No ship's experience is changed by this rule or to fit it. Where the effect is to be tuned, the strength is tuned (the standing rule of tuning).

## What it does, at the live figures

With `raid_recognised_chance` at 0.75 and the strength at 0.25:

| Experience | Factor | Chance |
| --- | --- | --- |
| 0 | 0.5 | 0.375 |
| 1 | 0.75 | 0.5625 |
| 2, the standard | 1 | 0.75 |
| 3 | 1.25 | 0.9375 |
| 4 | 1.5 | 1, cut from 1.125 |

A ship has an experience other than 2 only where a scenario gives it one, and a scenario gives one only where it is researched: today the ships of Savo Island and of Cape Esperance. Every other ship recognises as decision 0167 has it until its experience is researched.

## What it leaves for later

- **A base's men.** The experience of the land units ashore at a base could move the base's chance. It needs a ruling on whose experience counts where several units are ashore, and most bases of the 39 days hold men at the standard.
- **Experience that grows.** A ship's experience does not change with time or action (decision 0064). A crew raided daily does not learn here.
- **The fighters' pilots,** who saw the raid closer than anyone, are not asked (decision 0167, What it leaves for later), and an air group has no experience yet.

## Rule card

- **Reads:** the experience of the ship attacked; `raid_recognised_chance`; `raid_recognised_experience_strength`.
- **Writes:** nothing more than decision 0167 writes: the one field of the true event of an air attack on a ship.
- **Reuses:** the experience of a ship and the form of its factor (decision 0064); the draw of decision 0167. No new draw, event or field of the state.

## What it costs

- **One figure.** The air combat figures file gains `raid_recognised_experience_strength`: 0.25 in the live figures, a placeholder; 0 in the frozen step 1 figures, where the chance is the plain figure and decision 0167's hand calculation holds as written.
- **Versions.** No field of the state and no event changes shape. The rules label rises, since a game recorded at the live figures with a ship of experience other than 2 under a raid is told differently. Whatever else the build finds it must raise, it reports.
- **Earlier games.** No fight is resolved differently: the draw's stream is its own. A recorded game changes only where a ship whose experience is not 2 is attacked from the air on the live figures, and there only in whether the type is named. The build lists each.
- **The rulebook:** chapter 5's passage on what the attacked saw gains the factor; chapter 4's "Known limit" on experience is restated (item 6); the glossary's entry for experience names the new reader.
- **The table:** nothing. It says what the side's event says.

## Confirmed on Nov 19, 2026

The designer suggested the rule, the architect recommended its form, and the designer confirmed: "yes, please write that amendment as per your recommendations".

1. The chance of a ship is moved by the ship's own experience, by the form the game already uses.
2. The strength is 0.25, a placeholder.
3. A base keeps the plain chance in this build.
4. It is built after decision 0167's build is merged, as a change of its own with its own hand calculation.

## The hand calculation

It is in `tests/scenario_89_a_raid_recognised_by_experience.rs`, written before the code. It has no scenario file of its own: it plays the morning raid of scenario 19a with the attacked ship given each experience from 0 to 4 by the scenario's own key, and the first raid of scenario 6a, on a base, on the frozen step 1 figures with the two figures altered.

## As built

Built on Nov 19, 2026, on the branch `raid-experience-build`, from the head of decision 0167's build (`raid-seen`). Every statement of the hand calculation holds but one, the breakdown (reading 1), which is not built (`tests/scenario_89_a_raid_recognised_by_experience.rs`, 9 tests). Over seeds 1 to 1000 at the figure 0.75 and the strength 0.25 the type is recognised on 395 seeds at experience 0 (allowed 314 to 436), 587 at 1 (500 to 625), 766 at 2 (696 to 804, and the very seeds of decision 0167's 766), 941 at 3 (907 to 968) and 1000 at 4, where the attack is made on all 1000.

**What changed where.**

- `sim-core`, `refdata.rs`: `AirCombatFigures` gains `raid_recognised_experience_strength`, refused below 0 or above 0.5 (`MAX_EXPERIENCE_STRENGTH`) by `problem`, 0 in the unit tests' figures; and two methods, `ship_recognised_factor` (`experience_factor`, the dark fight's form, with this strength) and `ship_recognised_chance` (`min(1, chance x factor)`, and the plain figure itself, with nothing multiplied, where the strength is 0).
- `sim-core`, `airstrike/air_combat.rs`: an air attack on a ship draws against `ship_recognised_chance` of the ship's own experience, read from the truth when the attack is recorded; a raid on a base draws against the plain figure as before. The draw, its stream (`Draw::RaidRecognised`) and its order are decision 0167's: one draw at every chance, 0 and 1 included. No new `Draw`, event or field of the state; experience is read and never written.
- `sim-data`, `reference.rs`: the figure is required and read as a decimal string; a float, a missing key or a value outside 0 to 0.5 is refused.
- Data: `data/reference/air_combat.toml` 0.25, a placeholder set by the architect resting on no source; `data/reference/step1/air_combat.toml` 0, and its `README.md` says so.
- The rulebook: chapter 5, "What the attacked saw", gains item 6 (the chance, with the factor) and the figure; chapter 4's "Known limit" on experience is restated (by day a ship's experience moves this one chance, which changes what its side is told and nothing that happens); chapter 8's versions say decision 0168 changes none. Chapter 1's table of figures files names topics, not figures, and is unchanged. The glossary's "Experience" names the new reader, and "Recognised" says a ship's chance is moved by its experience.
- `docs/validation/tuning/air-figures-closed.md` lists `raid_recognised_chance` and `raid_recognised_experience_strength` as placeholders resting on no source (decision 0167, ruling 7), and its count goes from 12 placeholders and 26 figures to 14 and 28; `surface-and-search-figures-closed.md`'s table of counts and `CLAUDE.md`'s Status go from 101 figures to 103.

**Versions.** None is raised, by the test of decision 0167's ruling 2. The rules label stays 20: no fight, order, staff choice or daily hash moves, only what a side is told of a raid on a ship whose experience is not 2, and a game saved before is refused anyway since `air_combat.toml` changed. This is against "What it costs", which expected the label to rise (reading 2). The agent interface stays 28: no event, answer or briefing changes shape, only which value `recognised` takes. The state's layout (`flagplot-state-v48`), the hash, the saved game's version (23) and the doctrine version (`supply-41`) do not change: no field of the state is added and no doctrine reads it.

**Generated and pinned files.** Remade by `sh scripts/regenerate.sh`: four recorded games' `save.json` (`eastern_solomons_haiku`, `eastern_solomons_sonnet`, `full_slice_five_days_sonnet`, `second_proof_week_sonnet`), and the batch fixture of scenario 6, each in one line, the hash of `air_combat.toml`; their README has the paragraph. By hand: the four export digests of `crates/sim-cli/tests/export_pictures.rs` for scenarios 5, 31, 50 and 58, each export changing in the hash of `air_combat.toml` alone (with the old hash put back each hashes as pinned before). No run baseline, agent golden, transcript, pinned hash or daily hash changed: no scenario with a ship of an experience other than 2 (Savo Island and Cape Esperance) has an attack from the air.

**Readings made that the record did not, for the architect to rule on.**

1. **No breakdown of the draw is recorded** (item 8, and the hand calculation's line "The breakdown of the draw, with breakdowns on, gives ..."). The draw has no event of its own: its outcome is a field of the true `AirAttack` and `BaseAttacked`, and each event carries one breakdown, that of its expected hits (`aircraft`, `hit_chance`, and for a base the shares where a hit lands). Adding the draw's factors to it was weighed and not done: (a) the step 1 hand calculations of scenarios 5, 5c, 5d, 6 and 6a assert those factor lists exactly on the frozen figures, so naming the plain chance where the strength is 0, as item 8 says, would change their expected lists, which the build may not do; (b) the base's side is given a raid's breakdown whole (decision 0065), so the factor would be told to it as well, a change to what a side is told that no record makes; (c) a factor that is not part of the result's product reads, in `sim-cli events --why` and `explain`, as if it were. A faithful breakdown needs a home the record did not give (a breakdown of its own on a new event, or a second breakdown on the attack), which the build was told not to invent. Decision 0167 recorded none either. The rulebook says so (chapter 5, What the attacked saw, item 6).
2. **The rules label is not raised,** against "What it costs", by the test of decision 0167's ruling 2 (Versions, above).
3. **At the strength 0 nothing is multiplied.** `ship_recognised_chance` returns the plain figure itself where the strength is 0, so the chance is the figure to the last digit by construction, not by the arithmetic of `x 1`. At a strength above 0 the factor and the product are in the fixed arithmetic; at the live figures every row is exact (0.375, 0.5625, 0.75, 0.9375, 1).
4. **The experience is the ship's as it is when the attack is recorded,** in step 4 of the air combat step, before the attack's damage lands. A ship is never taken out of the truth; were one missing it would be read at 2.
5. **"No fight is resolved differently"** is tested, for each experience from 0 to 4, as the ships, air groups and bases at the end of each day and the daily hashes with the strength at 0.25 against the same with it at 0, on seeds 1 to 50, and ship 1 ending with the experience given. Across experiences the state differs by construction in the experience itself.
6. **"A game saved and replayed gives the same"** is tested as in scenario 88 (decision 0167, reading 8): the same seed twice, and played again from a fresh load.
7. **"The frozen figures are as they were"** is tested by the frozen file holding both figures at 0, by the chance on those figures being 0 at experiences 0 and 4 with no type named on seeds 1 to 50, and by scenario 88's own tests, which run unchanged.
8. **The comments of `data/scenarios/04_savo_island.toml`** and its two copies (`stage-2b-proof/`, `step2-frozen/`) say a ship's experience is "read only in a dark fight". It is still true there, where no raid flies, and the files are left as they are: the frozen copy may not change, and the scenario's hash is in its saved games and batch files.
9. **The figures page.** The two figures are sorted as placeholders, as the brief and ruling 7 say, though no tuned measure reads either (the page's "No battle reads it" is for a figure no battle can move; these move only what a side is told). The page lists the figures it listed on Nov 13 and two more; `[strike_aircraft_per_ship]` (decision 0118) is still not on it, and its count is not corrected for it.

## The architect's rulings

Made on Nov 19, 2026, on reading the build.

1. **The breakdown of the draw is not built, and is owed** (reading 1). Item 8 asked for it. The only breakdown an attack's event carries is that of its expected hits: putting the factors of recognition there would change factor lists that step 1's hand calculations assert, would tell a raided base's side a factor of a rule it is told nothing else of, and would read as part of a product it is no part of. A faithful breakdown needs a home of its own. Until it has one the draw is a chance worked with no breakdown recorded, which the rule of traceability does not allow: it is a named gap, written in `docs/open-work.md`, and is closed with the next change to the events of an air attack. The statement of the hand calculation that asserts the breakdown is struck by the architect, in the test file.
2. **The rules label is not raised** (reading 2), against "What it costs", by the test of ruling 2 on decision 0167: no fight, order, staff choice or daily hash moves.
3. **The other readings stand as made.** At a strength of 0 the plain figure is returned unmultiplied; experience is read when the attack is recorded; the stale comment in scenario 4's files stays, since its frozen copy may not change.

