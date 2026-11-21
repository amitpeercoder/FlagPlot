# 0205: Hunger takes time

Status: a plan by the architect, written on Nov 21, 2026 on the designer's question at the table: what "lost 187 men for want of food" means, on the first night his marines were ashore, and his word to plan a gentler rule. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in: it is simpler than the first. Its hand calculation is written next.

Summary: Today a twentieth of the men a base cannot feed are lost every night, from the first night ashore. The record says otherwise: men landed carrying food, lived for weeks on short rations with no such loss, and died of hunger only after weeks on almost nothing. This puts the rations men land with into the base their landing forms, counts for each unit the nights it goes hungry, and takes men only from a unit that has been hungry for many nights. It runs where a scenario's ground figures give its figures, so no game on file moves; the table plays copies of its scenarios that have them.

Standing: Planned on Nov 21, 2026; its review folded in; its hand calculation written; built on Nov 21, 2026 (As built, below), for the architect's review.

## Why

What is so today (rules chapter 7, The men's supply):

- **Each night the men ashore in a hex are charged a daily supply** from the stock of their side's bases there. The men the stock covers are fed and the rest are unfed. Every unit of the side in the hex has the one fed share, the fed over the charged.
- **A share of the unfed is lost that night,** one man at least: `unfed_loss_share_per_day`, 0.05, a placeholder that "rests on no source". It applies from the first night.
- **The fed share already weakens a hungry unit:** it cuts the men who fire and the work done, and stops training.
- **A landing forms a base and puts no food in it.** Men who land where no supply has yet been landed are unfed that night.
- **Men who march by order already carry food:** `march_supply_days`, 5, put down where they arrive. It rests on no source.

What the record holds (`data/oob/land_unit_facts.csv`):

- **Men landed with rations.** A company of the 5th Marines "landed with three days' rations" at Tulagi (row 538). It is the one row of men landing.
- **Men setting out on a march carried three to five days.** The 2d Division "carrying only five days' rations" (33, 34), the 2d Raider Battalion "four days of canned rations" (877), and a force taken to be the Kawaguchi Detachment "only three day's ration" (867, of low confidence). These bear on the march's figure, not on a landing.
- **Weeks on two meals a day cost no such loss.** The 1st Marine Division went to two meals a day in mid August, on the 12th or the 15th by the source, and stayed on them about six weeks (403 to 406). No source read gives men lost to hunger in that time.
- **Half rations weakened men.** The 2d Division's troops were "weakened on their half ration of rice" in October (31).
- **Men died of hunger after about three weeks on almost nothing.** On 5 and 6 October marines found in a Japanese bivouac "many who had died from hunger" (188, low confidence, no count), about three weeks after the fight on the ridge, which its men had begun with three days' rations. In December the Japanese were on "less than one-third the regular daily allowance" (137). The counts of Japanese dead that include hunger (104, 178, 179) mix it with disease and give no rate.

So a rule that takes a twentieth of the unfed on the first night is wrong in kind, not only in size.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give its three figures** (rule 9), as the men a hit removes and the errors of a report already do. A figures file without them plays as today.
2. **The live `ground.toml` does not gain them.** Its bytes are in the saves of recorded games, which refuse to load when it changes. The figures go in a ground file of their own, which a scenario names.
3. **The table plays copies of its scenarios that name that file.** The copies are made by a script from the files on file, changed in that one line, and checked as the generated scenario files are; `table/scenarios.json` points at them. The files the proofs play are not touched.

### Rations landed

4. **The first landing of men at a base puts their rations into it:** the men aboard the ships of the force, at the moment its first man lands there, times `rations_landed_nights`, times the daily charge for a man, as supply in the base's stock. However many pulses the men take to go ashore, the rations are put down once, then. It is the mechanism of the supply a marching unit puts down where it arrives, used at a landing. It is so whether the landing forms the base or the base stands already, as the Allied base at Lunga stands before the landing in the 39 days.
5. **A base is given rations once.** That it has had them is new state of the base. A second force landing in the same pulse, and men landed there on any later day, bring none; that base feeds them or it does not. So men cannot be taken off and landed again to make food.
6. **Nothing else of the feeding changes.** The fed share stays one for every unit of the side in the hex, and `MenFed` is as it is.

### Nights of hunger

7. **A unit's nights of hunger are a count, new state of the unit.** On a night its fed share is below `hungry_below_share` the count rises by one. On a night it is fed in full the count falls by one, not below nothing. On any other night it stays. A unit with no man ashore keeps its count; a unit made by dividing another takes the count of the unit it came from. A scenario may give a unit the count it starts with; without it the count is nothing.
8. **Men are lost to hunger only from units whose count is above `hunger_nights_before_loss`.** The counts are brought up for the night first, so with the figure at 14 the first men are lost on a unit's fifteenth hungry night. The loss in a hex is worked as today, on the men of those units alone: their men times one less the fed share, cut, are their unfed; the loss is their unfed times `unfed_loss_share_per_day`, cut, and one man at least where any of them is unfed; it is taken from those units in proportion by today's rule. With no unit past the count nobody is lost.

## Figures

9. All are placeholders in the game's sense, each with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `rations_landed_nights` | 3 | one row: three days' rations at a landing (538) |
| `hungry_below_share` | 0.5 | a judgement: half rations weakened men (31); two meals of three are above it and cost no loss (403 to 406) |
| `hunger_nights_before_loss` | 14 | a judgement between two rows: no loss in six weeks on two meals; "many" dead about three weeks after a force with three days' rations broke (188) |
| `unfed_loss_share_per_day` | 0.05, unchanged | rests on no source, as today |

The march's `march_supply_days` stays 5; the rows of men setting out with three, four and five days are noted against it.

## What is recorded and who knows

10. **The rations a landing brings are recorded** with the base formed: the men, the nights and the tons, citing the landing. **A unit's count changing is recorded** in an event for the unit and the night, citing that night's `MenFed`, and only when the count changes. `MenLostUnfed` gains the factors of rule 8. Each side sees its own and nothing of the enemy's.
11. **One field is new state of a land unit and one of a base,** each hashed in its place and named, with the layout raised and the golden files remade. A side sees it for its own units; the table's card says "Hungry 5 nights" where it is above nothing. The rules label does not rise, since no game on file plays differently.

## Rule card

Reads: the men ashore, the bases' stock, each unit's fed share and its nights of hunger. Writes: the stock of a base formed by a landing, the count, the men lost. Draws no random number.

## Games on file

None moves: the figures are in a file no scenario on file names, and the live figures are not changed. The layout rises, so the golden files and the daily hashes are remade for the layout alone, as at every new field. The 39 days and the proofs keep the old rule until the designer says otherwise; that is his call, since their measures count men lost unfed.

## What it does not do

- It does not model sickness. The record's sick lists are far larger than any loss to hunger and are a rule of their own.
- It does not let a unit forage or eat captured stores, which the record says fed the marines in August (405, 707). A base taken with its stock already gives that stock to the taker.
- It does not change the daily supply a man costs, nor the fed share's effect on a fight, on work or on training.
- It does not give rations to a unit that starts ashore or that enters ashore on its day, nor to men landed at a base that has had its rations.

## What it costs

- **One field of state for each land unit and one for each base, two events, and a landing that creates supply.** The supply is small: 3,000 men bring 45 tons at the live charge.
- **A table that plays by a rule the proofs do not,** from copies of the scenarios. A game at the table and the same scenario in a closed proof will differ ashore until the proofs take the rule.
- **Rulebook:** chapter 7 (Landing men and a base formed by a landing, The men's supply, The unfed are lost, A unit divided); the glossary's "Fed, unfed"; chapter 1's list of who sees what.

## Questions for the designer

1. **Should the 39 days and the other proofs take this rule?** It would move their measures of men lost. The architect proposes to build it, play it at the table, and bring what would move before any proof is run again.

## How the plan changed on review

A reviewer who had not seen the plan read it against the feeding's code, the bridge, the rows and the engineering pages on Nov 21, 2026.

- **The first plan gave each unit rations of its own and a fed share of its own.** The fight, training, the entity answer and the briefing all read one fed share for a hex and cite the hex's feeding; a share for each unit would have broken each of them. The march already puts carried food down as supply where men arrive. Rations at a landing now go the same way, into the base the landing forms, and the fed share stays one for the hex (rules 4 to 6).
- **The table cannot give a scenario a key:** it passes a scenario's path and nothing more, and its 39-day games are the proof's own files. The rule had also been gated twice, by figures and by a key, where the house has one or the other. It is now gated by figures in a ground file of its own, and the table plays copies (rules 1 to 3).
- **The count could be held at one for ever** by a full meal every other night on a quarter ration. It now falls only on a night fed in full (rule 7).
- **The loss was not defined for a unit, and the one man at least was dropped without a word.** Rule 8 defines it and keeps him.
- **The rows said less than the plan claimed.** One row is of a landing; three are of marches. One row does date deaths from hunger, which the plan had said none did. The figures' reasons are rewritten from the rows as they are.

## As built

Built on Nov 21, 2026, on the branch `hunger-build`, from the hand calculation of `tests/scenario_119_hunger_takes_time.rs`. Every statement and property of it holds as written; none is ignored.

### What was built, and where

- **The three figures and the gate (rule 1).** `sim_core::refdata::HungerFigures` (`rations_landed_nights`, `hungry_below_share`, `hunger_nights_before_loss`) is `GroundFigures::hunger`, an `Option`, last in the struct. The loader (`sim-data/src/reference.rs`) reads the three keys all together or none; one or two alone is refused, and the checks are `rations_landed_nights` 0 or more and `hungry_below_share` from 0 to 1. Every rule below runs only where it is `Some`.
- **The files of figures (rules 2 and 3).** The live `data/reference/ground.toml` and its frozen copy are not changed. `data/reference/ground_hunger.toml` is the live `ground.toml` with the three figures added at the end and a header saying so; its frozen counterpart `data/reference/step1/ground_hunger.toml` is the frozen `ground.toml` with the same three. Scenario 119 has its own pair, `ground_placeholder_scenario_119.toml`, the frozen `ground.toml` with the three figures and no other change.
- **Rations landed (rules 4, 5 and 10).** `World::land_rations` in `sim-core/src/cargo.rs`, called by `prepare_landing` right after `BaseFormed`, before any man lands: the men aboard the ships of the force that formed the base, their day's charge by `ground::daily_charge_t` (the one formula), times the nights, into the new base's stock. `EventKind::RationsLanded { base, side, task_force, men, nights, supply_t }` records it, citing the `BaseFormed`, with a breakdown of `men_aboard`, `rations_landed_nights`, `daily_supply_t_per_man` and, only where a unit's weight acts, `weighted_men`. A base that stands forms nothing and so brings nothing; a second force in the same pulse finds the base and forms none.
- **The count (rule 7).** `LandUnit::hunger_nights: u32`, last in the struct, 0 from `LandUnit::new`. `World::count_nights_of_hunger` in `sim-core/src/ground.rs` runs in `feed_men_ashore` after each hex's `MenFed`, for every unit of the side with men ashore there, and records `EventKind::HungerNightsChanged { unit, hex, before, nights }` only when the count changes, citing the `MenFed`, with a breakdown of `nights_before`, `fed_share` and `hungry_below_share`. A unit divided at a landing takes its parent's count (`cargo.rs`, with the experience and fieldworks it already took). A scenario's `[[land_unit]]` may give `hunger_nights`, a whole number.
- **The loss (rule 8).** In `feed_men_ashore`, with the figures, the loss is worked on the units past the count alone; without them, on every unit of the side in the hex, by the code as it was. `MenLostUnfed` keeps its fields; its breakdown gains `fed_share`, `hunger_nights`, `hunger_nights_before_loss`, `men_past_count` and `unfed_past_count`.
- **The hash (rule 11).** `hunger_nights` is named in `hash::name_every_field` and hashed after the formation; `STATE_VERSION` is `flagplot-state-v53`. The rules label is not raised.
- **Who sees what (rule 10).** `side_events.rs`: `RationsLanded` whole to the side that formed the base, `HungerNightsChanged` whole to the unit's side, nothing to the enemy; both cite only the side's own events, so neither may have hidden causes. The test table has a row for each (159 kinds).
- **The agent and the table.** A land unit line (`sim-agent/src/lines.rs`) gains `hunger_nights`, left out when 0, so no answer of a game without the figures changes; the text adds "hungry N nights" to the unit's line. The schema (`defs.schema.json`) and its README gain the field; the version is not raised, a field added only. The briefing lists neither new event; they are in the side's events and its explain. The table's card of a unit (`table/index.html`) shows "Hungry" with "N nights" where the count is above 0, and its diary words the two events plainly ("landed 45 t of rations at ...", "... hungry 5 nights"), the count's changes among the routine lines. `sim-cli`'s export names them `rations_landed` and `hunger_nights_changed`; the batch record counts neither.
- **The table's copies (rule 3).** `python3 scripts/table-copies/make_copies.py [--check]` writes each scenario of `table/scenarios.json` that names `../../reference/ground.toml` to `data/scenarios/table-hunger/` under its own name, with that one line changed to `ground_hunger.toml`; `--check` writes nothing and fails where a copy is not the one it makes or the folder holds anything else. `table/scenarios.json` points at the nine copies. Scenario 119's test runs the check.
- **Scenario 119.** `data/scenarios/hunger-takes-time/119_hunger_takes_time.toml`, its ground figures above, and `tests/scenario_119_hunger_takes_time.rs`, 22 tests.

### The readings

1. **The daily charge for a man** in rule 4 is read as each unit's own, by the one formula, so a unit whose weight is not the standard brings the nights of its own charge, as a march carries. Every unit of scenario 119 is standard.
2. **The record of the rations is an event of its own,** `RationsLanded`, citing `BaseFormed`, and not new fields of `BaseFormed`: so no `BaseFormed` of a game on file changes. A landing that would put down nothing (no man aboard, or 0 nights) records nothing.
3. **"The men aboard the ships of the force"** is every man of every unit aboard the force's ships at that moment, after any division, which moves no man.
4. **The unfed of the units past the count** are their men times 1 less the fed share as the state holds it (the men fed over the men charged, cut at six decimals), cut to a whole man, and never above their men. Where every unit of the hex is past the count this is the hex's unfed exactly, since the share is cut down and the excess is below a man.
5. **The breakdown of `MenLostUnfed`** keeps `unfed_in_hex` and `men_ashore_in_hex` as the hex's, and works `in_proportion` over the men of the units past the count, which is what the loss is shared over.
6. **Without the figures the count is never moved,** and a scenario may still give a unit one: it is state, shown on the unit's line, and read by no rule.
7. **The table plays the copies of the files that name the live ground figures only.** The Eastern Solomons in one day (`second-proof/05c` and `05e`) has no land unit and names no ground file, so it has no line to change; the table plays the file on file. The copies are in a folder two deep under `data/`, as each source's folder is, so every `../../` path in them resolves to the file the source names; no source names a file in its own folder. The table's bridge passes the scenario's path to `sim-cli` and reads nothing else of it, so nothing else needed a change.
8. **Force T of statement 10 is five transports, not two.** The one placeholder class with berths, class 10 of `ship_classes_placeholder_transports.csv`, carries 600 men, so 3,000 men need five; no class file was made for the test. Nothing in the statements depends on the number of ships.
9. **Statement 9's division** is set so that V is still at 5 when it is divided: Q holds 3 t at the start and a supply line gives 1.5 t on day 2, so V's fed share stays between a half and 1 and its count stays 5; a transport takes 600 of V on night 1 and lands them at P on night 2.

### What did not hold

Of scenario 119's hand calculation, nothing: no statement or property needed an `_as_built` test.

Of another: scenario 104's statement 23 says that scenarios 103's and 105's eight files have the key `for_play` and "no other scenario file has it". The table's copies of six of them (files b, c and d of each, in `data/scenarios/table-hunger/`) carry it too, since they are those files but for the ground line. Its test is left as written and ignored with that reason, and `statement_23_the_key_as_built` beside it asserts the eight files and the six copies.

### Pins remade for the layout

Besides what `regenerate.sh` remakes: the ten export pins of `crates/sim-cli/tests/export_pictures.rs` (each export checked against the export of the commit before with the state version and the day hashes taken out: the same in every one, and the old export hashes as pinned before), and scenario 110's `PRE_RULE` and day-7 hash in `tests/scenario_110_the_last_day_ashore.rs`, daily hashes of scenario 109's file on the frozen figures, remade at version 53 as they were at version 52.

### Games on file

Every one of the 174 scenario files under `data/scenarios` (all but scenario 119's folder and the table's copies) was run with `run --events` before the change and after it: the events are the same in every one, and only the daily hashes differ, by the layout. The generated files were remade by `sh scripts/regenerate.sh` for the layout alone.

