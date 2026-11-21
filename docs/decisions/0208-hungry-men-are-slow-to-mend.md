# 0208: Hungry men are slow to mend

Status: a plan by the architect, written on Nov 21, 2026 on the designer's question at the table: a hundred men go hungry for five days and are then given ten tons; how long until they are fit to fight again? Today the answer is one night. The designer asked for a plan, with research on how long it took in the war. It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today a unit fights, works and trains by the share of its men fed last night, so a unit with no food fires nothing on its first hungry night and is whole again the first night it eats. The record says both are wrong: men weakened over weeks, and took about as long to mend as they had been hungry. This makes a unit's fitness a matter of its nights of hunger, the count decision 0205 already keeps, which rises on a hungry night and falls on a night fed in full. A unit fires and works by that fitness in place of last night's fed share. It runs where a scenario's ground figures give its one figure, so no game on file moves.

Standing: Planned on Nov 21, 2026; to be reviewed.

## Why

What is so today (rules chapter 7 and decision 0205):

- **A unit's firing men are its men ashore times its fed share times its rested share** (`ground_combat::fight::firing_men`). The fed share is last night's: the men the stock fed over the men charged, one share for every unit of the side in the hex.
- **A unit's work is scaled by the same share** (`works::unit_work`), and a unit trains only on a day its fed share is one.
- **So hunger acts at once and mends at once.** A unit fed nothing last night fires no man today. A unit starved for a fortnight and fed once fires every man tomorrow.
- **The count of nights of hunger** (0205, rule 7) rises on a night fed below `hungry_below_share`, falls by one on a night fed in full, and decides only when men begin to die.

What the record holds (`docs/research/hunger-and-recovery.md`; rows 14900 to 15025 of `land_unit_facts.csv`, all unconfirmed):

- **The fall took weeks.** On Bataan, on about half a ration, the first sign was men "listless and less eager to fight" after about a month (14907), with malaria not yet telling (14909). Combat efficiency was put "a little below 45 percent" after about 68 days (14912), by then mixed with disease. In the Minnesota experiment, healthy men on about 1,570 calories kept 44.3 percent of their capacity for severe work at 12 weeks and 20.9 percent at 24 (14977).
- **Men on two meals of three were not disabled** (15009), and men could go "for days or weeks on slightly inadequate rations provided that they were very well fed during their rest periods" (15016).
- **The rise took about as long as the fall, or longer.** After 24 weeks of hunger the Minnesota men had back 34.3 to 37.4 percent of what they had lost at 12 weeks of refeeding (14981), 48 percent on the two highest steps of feeding (14983, 14984), and 70 percent at 20 weeks (14987, 14988). The first three weeks gave almost nothing (14979). Recovery was faster the more they were fed (14983 to 14986, 14996).
- **No field force's recovery from hunger alone is on record.** The 1st Marine Division's three to six months (15001) is put down to malaria.
- **Nothing shows one side mending faster than the other.** The Japanese ration was smaller by design (14901, 14922), which is a matter of the daily charge. The claims that Japanese troops bore hunger better (15023 to 15025) have no figure behind them, and on both sides starving men held positions and none attacked with success.

## The rules

### Where it runs

1. **The rule runs where a scenario's ground figures give `hunger_weakness_per_night`,** which may be given only together with the three figures of decision 0205. A file without it plays as today.
2. **The figure goes into `data/reference/ground_hunger.toml`,** the file the table's copies name. The live `ground.toml`, both frozen files and scenario 119's own pair are not changed. The build first confirms that no recorded game and no pinned file on file was made with `ground_hunger.toml`; if one was, it stops and asks.

### Fitness

3. **A unit's fitness is one less its nights of hunger times `hunger_weakness_per_night`, not below nothing.** It is worked when asked for and is no new state: the count is the state.
4. **A unit's firing men are its men ashore times its fitness times its rested share,** cut as today. Its fitness takes the place of its fed share there.
5. **A unit's work is scaled by its fitness** in place of its fed share.
6. **A unit trains only on a day its fed share is one, as today, and its count is nothing.** Men who are mending do not train.
7. **The fed share is kept and shown as it is.** It still decides the count, the men lost to hunger and what the briefing says of a unit not fed.
8. **Nothing of decision 0205 changes:** when the count rises, stays and falls, the rations a landing brings, and the loss past fourteen nights.

So a unit hungry for N nights and then fed in full is weaker by N times the figure, gains the figure back each night, and is whole after N nights. A unit on short rations that are half or more is neither weakened nor mended.

## Figures

9. One figure, a placeholder with what it rests on.

| Figure | Value | Rests on |
| --- | --- | --- |
| `hunger_weakness_per_night` | 0.01 | a judgement from two cases on about half a ration: a little below 45 percent after about 68 days on Bataan (14912; 0.0081 a night, mixed with disease) and 44.3 percent at 84 days in the Minnesota experiment (14977; 0.0066 a night). A night of hunger in the game is a night on less than half, so the figure is set a little above both |

The count's fall of one for each night fed in full is 0205's and is not changed. It gives a rise as fast as the fall. The record's best-fed men had back 48 percent after half the time they had been hungry and 70 percent after five sixths of it, so the rise is at the quick end of what the record holds, and is the same for both sides. If play wants it slower, the count's fall can be given a figure of its own later.

## What is recorded and who knows

10. **The breakdown of a unit's firing gains `hunger_nights` and `fitness`** where the figure is given, and keeps `fed_share`. The breakdown of work done gains the same.
11. **A side sees the fitness of its own units,** as it sees their count: the unit's line for an agent gains `fitness`, left out when it is one. Nothing of the enemy's is shown. An estimate of the enemy ashore is not changed by it.
12. **The table's card** says "Hungry" and the nights, as now, on a unit not fed in full last night; on a unit fed in full whose count is above nothing it says "Mending" and the nights left, which are its count. Where fitness is below one it says "Fit to fight" and the share as a whole percentage.
13. **The staff's count of a day's work** (`sim-staff`, `ready.rs`) reads the same function the engine does, so its days agree with what happens.
14. **No new state, so no new field in the hash and no new layout.** The rules label does not rise, since no game on file plays differently.

## Rule card

Reads: a unit's nights of hunger, its fed share, its men ashore and fatigue. Writes: nothing new; the hits a fight scores and the work done follow from the firing men and the work as today. Draws no random number of its own.

## Games on file

None moves: the figure is in a file no proof names. The table's nine copies play by it from the next restart; a game kept at the table from before the change will not load, as at any change to its figures.

## What it does not do

- **It does not make the rise slower than the fall,** nor give the first nights of feeding nothing, though the one measured case shows both after very long hunger.
- **It does not give the sides different figures.** The record supports none. A smaller Japanese ration would be a daily charge by side, which is a rule of its own.
- **It does not weaken men on short rations of half or more,** however long.
- **It does not stop a weak unit attacking.** Fatigue still does that, and hunger is not fatigue.
- **It does not model ammunition ashore.** Today the fed share stands in, roughly, for every want of supply in a fight; with this rule a unit cut off from supply fights on at nearly full strength for its first nights. Ammunition for men ashore would be a rule of its own.
- **It does not model sickness,** which in the record took longer to mend than hunger.

## What it costs

- **One figure, one small function, and three readers changed** (the fight, the work, training), with the staff's count of work and the agent's line.
- **A cut-off unit is far stronger in its first fortnight than today.** At 0.01 a unit unfed for five nights fires 95 men in 100 where today it fires none. That is what the record says of hunger, and it is a large change to how supply bites at the table.
- **Rulebook:** chapter 7 (the firing men, the work, training, Nights of hunger, the list of figures, the list of what is left out); the glossary gains "Fitness".

## Questions for the designer

None. The figure is a placeholder to be tuned at the table.
