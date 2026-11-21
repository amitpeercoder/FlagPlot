# 0208: Hungry men are slow to mend

Status: a plan by the architect, written on Nov 21, 2026 on the designer's question at the table: a hundred men go hungry for five days and are then given ten tons; how long until they are fit to fight again? Today the answer is one night. The designer asked for a plan, with research on how long it took in the war. It was read by a reviewer who had not seen it written, and this is the plan with that reading folded in. Its hand calculation is written next.

Summary: Today a unit fights and works by the share of its men fed last night, so a unit with no food fires nothing on its first hungry night and is whole again the first night it eats. The record says both are wrong: men weakened over weeks and were slow to mend. This gives a unit a condition that falls with each night of hunger, the count decision 0205 already keeps, and rises again a night at a time once it is fed in full. Last night's meal still counts, but for half as much: a unit fed nothing fires half its men. It runs where a scenario's ground figures give its two figures, so no game on file moves.

Standing: Planned on Nov 21, 2026; its review folded in; its hand calculation written (`tests/scenario_122_hungry_men_are_slow_to_mend.rs`).

## Why

What is so today (rules chapter 7 and decision 0205):

- **A unit's firing men are its men ashore times its fed share times its rested share** (`ground_combat::fight::firing_men`). The fed share is last night's: the men the stock fed over the men charged, one share for every unit of the side in the hex.
- **A unit's work is scaled by the same share** (`works::unit_work`), and a unit trains only on a day its fed share is one.
- **So hunger acts at once and mends at once.** A unit fed nothing last night fires no man today. A unit starved for a fortnight and fed once fires every man tomorrow.
- **The count of nights of hunger** (0205, rule 7) rises on a night fed below `hungry_below_share`, falls by one on a night fed in full, and decides only when men begin to die.
- **Nothing models ammunition for men ashore.** The fed share is the one way a want of supply weakens a unit in a fight.

What the record holds (`docs/research/hunger-and-recovery.md`; rows 14900 to 15025 of `land_unit_facts.csv`, all unconfirmed):

- **On about half a ration the fall took weeks.** On Bataan the first sign was men "listless and less eager to fight" after about a month (14907), with malaria not yet telling (14909). Combat efficiency was put "a little below 45 percent" after about 68 days (14912), by then mixed with disease. In the Minnesota experiment, healthy men on about 1,570 calories kept 44.3 percent of their capacity for severe work at 12 weeks and 20.9 percent at 24 (14977).
- **On far less it was quicker.** The South Seas Detachment, on half a ration and then a sixth, was "hardly capable of further offensive action" after about four weeks (14936 to 14939). Australians cut off for some days with little food were "very weak and emaciated" (14954, 14955). Of a battalion that had been weeks on short rations, 57 men collapsed on one day with no food (14959). None of these gives a share.
- **Short spells on a little less were borne:** men could go "for days or weeks on slightly inadequate rations provided that they were very well fed during their rest periods" (15016). Six weeks on two meals a day cost the marines no loss on record (403 to 406).
- **The rise was slower than the fall, in the one case measured.** After 24 weeks of hunger the Minnesota men had back 34.3 to 37.4 percent of what they had lost at 12 weeks of refeeding (14981), 48 percent on the two highest steps of feeding (14983, 14984), and 70 percent at 20 weeks (14987, 14988). The first three weeks gave almost nothing (14979). Recovery was faster the more they were fed (14983 to 14986, 14996).
- **That case is of very long hunger.** The record gives no rate of mending after days or a few weeks of it, and none for any field force from hunger alone: the 1st Marine Division's three to six months (15001) is put down to malaria.
- **Nothing shows one side mending faster than the other.** The Japanese ration was smaller by design (14901, 14922, 15025), which is a matter of the daily charge. The claims that Japanese troops bore hunger better (15023, 15024) have no figure behind them, and on both sides starving men held positions and none attacked with success.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give its two figures,** `hunger_weakness_per_night` and `unfed_fighting_share`. They are given both or neither, and only with the three figures of decision 0205; each is from 0 to 1. A file without them plays as today, in every rule below.
2. **The figures go into `data/reference/ground_hunger.toml`,** the file the table's copies name, and into its frozen counterpart `data/reference/step1/ground_hunger.toml`, as the test of frozen files asks; nothing on file plays the frozen one. The live `ground.toml`, its frozen copy and scenario 119's own pair are not changed.

### Condition

3. **A unit's condition is one less its nights of hunger times `hunger_weakness_per_night`, not below nothing.** It is worked when asked for and is no new state: the count is the state.
4. **A unit's share from last night's meal is `unfed_fighting_share` plus one less that figure times its fed share.** With the figure at a half, a unit fed in full has a share of one, a unit fed half has three quarters, and a unit fed nothing has a half.
5. **A unit's fighting share is its condition times its share from last night's meal.** One function of `sim-core` gives it from a unit and the figures, and gives the fed share where the figures are not given. Everything below calls it.
6. **A unit's firing men are its men ashore times its fighting share times its rested share,** cut once at the end as today. The fighting share takes the place of the fed share.
7. **A unit's work is scaled by its fighting share** in place of its fed share.
8. **A unit trains only on a day its fed share is one, as today, and its count is nothing.** Men who are mending do not train.
9. **The fed share is kept and shown as it is,** and still decides the count and the men lost to hunger.
10. **Nothing of decision 0205's rules changes:** when the count rises, stays and falls, the rations a landing brings, and the loss past fourteen nights.

So a unit hungry for N nights and then fed in full is weaker by N times the figure, gains the figure back each night, and is whole after N nights.

### What follows, said so that nobody has to guess

11. **A count past the point where condition is nothing keeps rising,** and each night fed in full takes one from it, so such a unit gains nothing until its count is back below that point. With the figure at 0.01 that point is a hundred nights.
12. **Men aboard ship do not mend.** A unit keeps its count while no man of it is ashore (0205, rule 7), so a starved unit taken off by sea mends only once it is landed and fed in full.
13. **Men who join a unit take its condition,** as they take its fatigue: the count is the unit's.

## Figures

14. Both are placeholders, each with what it rests on. One figure of 0205 moves in the table's file.

| Figure | Value | Rests on |
| --- | --- | --- |
| `hunger_weakness_per_night` | 0.01 | a judgement from two cases on about half a ration: a little below 45 percent after about 68 days on Bataan (14912; 0.0081 a night, mixed with disease) and 44.3 percent at 84 days in the Minnesota experiment (14977; 0.0066 a night), set a little above both |
| `unfed_fighting_share` | 0.5 | rests on no source. It is what a night on nothing does at once: the record has men very weak after days on little (14954, 14955, 14959) and gives no share. It also stands, roughly, for the want of ammunition, which nothing else models |
| `hungry_below_share` | 0.6, from 0.5, in `ground_hunger.toml` only | half a ration is the record's case of men weakening (14907, and row 31), and two meals of three cost no loss (403 to 406); at 0.5 a night on exactly half was not a night of hunger. Scenario 119's own file keeps 0.5 |

The count's fall of one for each night fed in full is 0205's and is not changed, so the rise is as fast as the fall. That is quicker than the one measured case, whose best-fed men had back 48 percent after half the time they had been hungry, where this gives 50, and 70 percent after five sixths of it, where this gives 83, with nothing in the first three weeks, where this gives the first nights in full. It is kept for its simplicity and because that case is of 24 weeks of hunger and a game's hunger is of days or weeks. If play wants it slower, the count's fall can be given a figure of its own.

## What is recorded and who knows

15. **The breakdown of a unit's firing gains `hunger_nights`, `condition` and `fighting_share`** where the figures are given, and keeps `fed_share`. A side sees them for its own units: `side_events.rs` names them among the factors of `GroundFire` a side may see of its own unit, with a test. The breakdown of work done gains the same.
16. **A side sees the condition of its own units,** as it sees their count. `SideView::firing_men` reads the same function, so "would fire today" agrees with the fight. The unit's line for an agent gains `condition`, left out when it is one; the schema and its README gain the field. Nothing of the enemy's is shown, and an estimate of the enemy ashore is not changed.
17. **The briefing's reasons a unit did nothing follow the rule.** A working unit is told "not fed" only where its fighting share is nothing. A unit that did not train because its count is above nothing, though fed in full, is given a reason of its own, "mending".
18. **The table's card** has three states for a unit whose count is above nothing: "Hungry" and the nights where last night raised the count; "Mending" and the nights left, which are its count, where last night was fed in full; "Short of food" and the nights where it was neither. Where the fighting share is below one the card says "Condition" and the share as a whole percentage. It explains nothing.
19. **The staff's count of a day's work** (`sim-staff`, `ready.rs`) reads the same function. Its days are worked on today's share, as now, so they are a little long for a unit that is mending.
20. **The staff's count of fit men for taking a place** (`take.rs`) does not read the condition, as today it does not read the fed share.
21. **No new state, so no new field in the hash and no new layout.** The rules label does not rise, since no game on file plays differently.

## Rule card

Reads: a unit's nights of hunger, its fed share, its men ashore and fatigue. Writes: nothing new; the hits a fight scores and the work done follow from the firing men and the work as today. Draws no random number of its own.

## Games on file

None moves: the figures are in a file no proof names, and no recorded game or pinned file was made with it. The table's nine copies play by it from the next restart; a game kept at the table from before the change will not load, as at any change to its figures. The build runs the three 39-day copies before the change and after it and reports what moves: the men lost in fights and to hunger on each side, and who holds what at the end.

## Why not fatigue

Fatigue already cuts the firing men and mends by a figure, and hunger could have added to it. It was not used because fatigue mends five points a day whether or not a unit is fed, so a starving unit would rest its hunger away; and because the count is already kept, and already falls only on a night fed in full.

## What it does not do

- **It does not make the rise slower than the fall,** nor give the first nights of feeding nothing.
- **It does not weigh a night by how little was eaten** beyond rule 4: a night on nothing and a night on just under the mark add the same to the count.
- **It does not give the sides different figures.** The record supports none. A smaller Japanese ration would be a daily charge by side, which is a rule of its own.
- **It does not weaken men on short rations above the mark,** however long, beyond the share of rule 4 on the day.
- **It does not stop a weak unit attacking.** Fatigue still does that.
- **It does not model ammunition ashore.** `unfed_fighting_share` stands in for it until it is a rule of its own.
- **It does not model sickness,** which in the record took longer to mend than hunger.

## What it costs

- **Two figures, one function, and the readers of the fed share changed** (the fight, the work, training, the side's own firing men, the staff's count of work, the agent's line, two reasons in the briefing).
- **A cut-off unit is stronger in its first fortnight than today.** With these figures a unit fed nothing for five nights fires 47 men in 100 where today it fires none, and a unit on half its food for five nights fires 71 where today it fires 50. A unit then fed in full fires 95 at once and all its men after five nights, where today it fires all at once.
- **Rulebook:** chapter 7 (the firing men, the work, training, Nights of hunger, the list of figures, the list of what is left out); chapter 8 where it says the fed share of the last night; chapter 1's list of who sees what; the glossary gains "Condition" and "Fighting share", and "Fit men" is told apart from them.

## Questions for the designer

1. **How hard should a night on nothing bite at once?** The plan says a half, which rests on no source. Put to the designer with the plan on Nov 21, 2026; the build goes on with a half, and the figure is his to move at the table.

## How the plan changed on review

A reviewer who had not seen the plan read it against the code, the rows and the engineering pages on Nov 21, 2026.

- **The first plan put the condition in the fed share's place and nothing else.** A unit fed nothing would have fired 86 men in 100 on its fourteenth night, with nothing else in the game to hold it back, since ammunition ashore is not modelled; and the rate had been set from men on half a ration, while the research's rows of men on far less had been left out. Last night's meal now counts for half (rules 4 and 5), and the rows are named.
- **A night on exactly half a ration was not a night of hunger,** though half a ration is the record's own case of men weakening. The mark moves to 0.6 in the table's file.
- **The frozen counterpart of the file must gain the figures,** or a test on file fails. Rule 2 now says so.
- **Five readers of the fed share were missed:** the side's own count of firing men, the list of factors a side may see of its own fire, two reasons in the briefing, and the staff's count of fit men. Rules 15 to 20 take each.
- **"Fitness" collided with the glossary's "Fit men".** The word is now "condition".
- **A row was misread:** 15009 speaks of disease from food, not of fighting power, and is no longer cited. Row 15025 is of the ration, not of bearing hunger, and is cited as that.
- **The rise was called "at the quick end of what the record holds". It is quicker than anything measured,** and the plan now says so with the figures.
- **Rules were added for what was left to guess:** the range of the figures, the gate on training, a count past a hundred, men aboard, men who join, and the card's three states.
