# 0109: The first tuning round of the ground figures: what is settled first, and the plan

Status: a plan by the architect, written on Nov 14, 2026, from four runs of the lengthened slice (`docs/validation/full-slice-39-days-baseline.md`, `-after-0105.md`, `-after-0106.md`, `-after-stock-fix.md`) and a model's five days in the Allied seat. It follows the rules of tuning in `docs/step2-spec.md`. Decision 1 corrects a rule: the designer confirmed it on Nov 14, 2026. Decisions 2 to 4 settle data and are the architect's. The round itself is run after them. Nothing is built and no figure has moved. *(Nov 15, 2026: decisions 1 to 4 are built and a new baseline is taken; see As built. The round is not run.)* *(Nov 15, 2026: round 1 is run; see As built: round 1.)*

Summary: The first tuning round of the ground figures: before it, men who are falling back are not fired on unless the victor attacks, the Japanese runs land their own rations as cargo, and riflemen set to work make a tenth of a labourer's work; then one round on the Tenaru and the ridge, mechanism by mechanism, with the figures each may move and the direction a source supports

Standing: Decision 1 confirmed by the designer, Nov 14, 2026; the rest settled by the architect. Decisions 1 to 4 built, with a new baseline, Nov 15, 2026. Round 1 run, Nov 15, 2026: four figures moved; steps 2 and 3 stopped, the two fights wanting different values; steps 4 and 5 moved nothing. The Tenaru's two measures accepted by the designer as a named miss, Nov 15, 2026, to be mended with step 3's stage 3

## Where the slice stands

After the builds of decisions 0104 to 0106 and the stock fix, over 1,000 runs:

| Measure | The record | The slice | Verdict |
| --- | --- | --- | --- |
| Lunga and Tulagi American, by day and at the end | yes | every run | agree |
| American men lost at the Tenaru | 109 to 110 | median 23 | miss |
| Japanese men lost at the Tenaru | 800 to 900 | median 340 | miss |
| American men lost at the ridge | 134 to 253 | median 101 | reported |
| Japanese men lost at the ridge | 600 to 1,138 | median 1,900 | reported |
| Days of food at Lunga when the transports sail | 17 to 37 | 68 | reported |
| The day the airfield is built | 17 to 19 August | 17 August | reported |

Four things in those runs are not figures to tune. They are settled before the round, because a figure fitted over a fault hides the fault (the spec's rule 6: what the figures rest on is checked first; and rule 8: a scenario's orders and data are set before the first round and then left alone).

## Settled before the round

### 1. Men who are falling back are not fired on, unless the victor attacks

Decision 0104, 6, says a victor who goes on attacking fires on men who have not yet left. It follows that a victor who does not attack does not. The code does otherwise in one case. At the ridge the Kawaguchi parts are beaten off on 12 September and are marching out. On 14 September the Oka group attacks in the same hex, the Marines defend, and the losses of that fight are shared among every Japanese unit in the hex in proportion (decision 0090, 9): the men leaving lose 709 in the median run, to a fight they are not in. They also count in the limit of decision 0105, so the Marines fire as if 2,200 men were attacking and not 236.

**The correction:** a unit that is falling back, by decision 0104 or 0105, takes no share of a fight's losses and is not counted among its side's fighting men for the limit on engaged men, on a day when the other side is not on attack in the hex. On a day the other side attacks, it is fired on and counted as now. A unit marching by order is as it was.

This is a rule's wording caught up with its intent, and it changes results, so it has its own hand calculation and the designer's confirmation.

### 2. The Japanese runs land their own rations

Decision 0101 put a stock ashore in each landing hex before anyone landed, ten days and then seven for the men due there. The stock fix showed what that does: men who arrive early eat what was meant for men who arrive late, the stock is gone by 26 August, and the Oka group reaches the field in September with a fed share of 0 and fires nothing.

A row says how it was: two weeks' rations were carried with the men by the destroyers (LUF 975), and a destroyer was planned to carry 110 men and two weeks' rations (TL 170, 177, 178). So each run lands rations with its men, as cargo, and the stock stated ashore is cut to what the men who fall back there need:

- Each run of the Kawaguchi main body, the Aoba part and the Oka group carries 14 days for its men at the game's charge. The Ichiki echelon, the Yokosuka force and the force of 29 August carry 7 days, by the practice of row LUF 1044, which is all that bears on them.
- The carriers are the placeholder fast transport with cargo, as decision 0101's ruling D made for the American run of 15 August. A run's ships are as many as its men need.
- The stock stated in (2,-5) is 71 t, the seven days of amendment 8 for the garrison and construction units who fall back there. The stock stated in (3,-4) is 0: everyone who lands there brings his own.
- Each is marked a placeholder resting on a stated practice, as before.

### 3. Riflemen set to work make a tenth of a labourer's work

`fighting_unit_work_share` is 0.5. A model playing the Allied side put two combat groups of 4,300 men each to the airfield: each made about 2,150 of work a day against the pioneers' 660, and the field was finished on 11 August. The record has it finished on 17 to 19 August by the engineers and pioneers, with captured Japanese machines (PF 56, 198), and no row has riflemen building it. The figure becomes `"0.1"`: a regiment digs as well as a few hundred labourers with tools, not as well as two thousand. It is a placeholder moved toward the one thing the record supports, that the field was the engineers' work. The hand calculations run on the frozen figure and stand.

### 4. The Oka group's day

Decision 0101 has the Oka group march on day 37 and attack on day 39, 14 September, the day two rows give (LUF 1007, 949). That stands: it is set from rows. With correction 1 its attack no longer costs the Kawaguchi men anything, and with decision 2 it arrives fed.

## The round

It is run by a team, on the slice, after decisions 1 to 4 are built and a new baseline is taken. One mechanism at a time, in the order the rules run (the spec's rule 2), each step recorded with the report before and after in `docs/validation/tuning/` (rule 3). One set of figures serves both fights and every placeholder scenario (rule 1). The hand-calculation tests stay on the frozen figures.

| Step | What is wrong | Figures that may move, and the direction | What supports the direction |
| --- | --- | --- | --- |
| 1. Who can fire | Ichiki's 916 men reach the field with 412 able to fire | `unprepared_landing_fatigue`, down; `fatigue_per_march_day`, down | The echelon landed unopposed at night and marched two days (LUF 1049, 994, 1020); the record has it attack in strength |
| 2. The limit | Fire counted for three times the attackers | `engagement_ratio`, either way | A reinforced battalion with artillery met about 900 men at the Tenaru; two battalions and the artillery met about 2,000 at the ridge (EO 1924, 2013) |
| 3. What a hit does | Both sides lose too few at the Tenaru: 23 and 340 against 110 and 800 to 900 | `men_per_hit`, up; then `hit_chance` and `defender_hit_chance` for the balance between the sides | The losses of both fights on both sides (the measures' rows) |
| 4. When an attack stops | The Tenaru was one night; the ridge was two nights | `failed_attack_fatigue`, `attack_fatigue_limit` | The hours of the attacks (LUF 1017 to 1024, 1001 to 1014) |
| 5. Cover | Shallow fieldworks save about half of a raid's men, not the share the figure states, because small shares are cut to whole men | Not a figure: the team reports what the cut does at each level of cover, and the architect rules whether the rule is changed | The rule's own table |
| 6. The landing | 68 days of food ashore when the transports sail, against 17 to 37 | `unprepared_landing_share`, down; `handling_t_per_man`, down, so that men still land in a day | Less than half the supply was landed in 26 hours of unloading, and the men were ashore on the first day (LUF 465, 773, 688) |

Rules of the round, beside the spec's:

- **The two fights are fitted together and nothing checks them.** Every figure the round moves is recorded as fitted on two fights and not checked, as the spec has for the surface figures. The Tenaru's two measures are judged; the ridge's are reported, since the game cannot say which part of each side was engaged.
- **A step stops when its measure's record is inside the middle half of the runs,** or when moving its figures further would take another measure out. A measure left at the edge is said to be, with the reason.
- **No figure moves for one scenario,** and none moves to make up for something another step owns. If the Tenaru and the ridge want different values of one figure, the round stops and says so: the rule is wrong, not the figure.
- **The placeholder scenarios are rerun at the end** on the live figures, and each whose result changes is listed in plain words.
- **The round ends with a report** in the shape of the battle reports of step 2: every measure before and after, each figure moved with its old and new value and what it rests on, and what the round could not reach.

## What the round will not mend

- **The landings meet no enemy at sea or in the air.** The slice has no covering force, no night action off Savo Island and no raid on the transports, so the transports' sailing is by order. That is data and rules for a later round.
- **No ship shells Lunga,** so strain and wear under bombardment are barely exercised.
- **Real units are all alike.** Experience and skills are 2 for every unit until the rule that reads the research is written.
- **What a side is told.** The model in the Allied seat could not tell that its own attack was stopping work on the airfield, that the squadrons were due later, or that the enemy was leaving. Those are for a record of their own on the briefing.

## Rule card, for correction 1

| | |
| --- | --- |
| Reads | Whether a unit is falling back; whether the other side has a unit on attack in the hex |
| Writes | The units that share a fight's losses; the fighting men counted for the limit on engaged men |

## Question for the designer

Answered on Nov 14, 2026.

1. **Correction 1:** men who are falling back are fired on only when the side that beat them is on attack. Agreed: a victor chooses to pursue by ordering it.

## As built

Built on Nov 15, 2026, in one pull request, before the round. No figure moved but the one decision 3 names.

**Correction 1, with the architect's ruling.** A unit is falling back while it is on a march whose cause is an event: the falling back of a beaten side (decision 0104) or of a spent attacker (decision 0105). A march by order, including one that replaces a falling back, is not. On a day when the other side is not on attack in the hex, a unit falling back is out of the day's fight both ways (the architect's ruling of Nov 15, 2026): it does not fire, it is not counted among its side's fighting men for the limit on engaged men, and it takes no share of the fight's losses. Men marching away from the enemy do not join a fight their own side starts behind them. On a day the other side attacks, it is in the fight as before. Two things are not changed: whether a hex is contested, and the test of a side beaten, which is made only where the other side attacked, and on such a day the unit is in the fight. Code: `falling_back` and `in_fight` in `crates/sim-core/src/ground_combat/mod.rs`, and the units in the fight in `fight.rs`, which the fire, the limit's count and the shares of the losses all read. A unit out of the fight has no fire event; nothing new is recorded, so no event and no row of `side_events.rs` is added, and the enemy is told nothing new. No field of the state is added, so the state version is unchanged (23, raised by decision 0111, built alongside); the rules label is 7, after decision 0111's 6.

- **Scenario 37** (`data/scenarios/37_men_falling_back.toml`, with its own placeholder map `data/map/placeholder_men_falling_back.toml`, since no map had the hand calculation's hexes and links) proves it on the frozen figures with `engagement_ratio` at 3. Every number of the hand calculation matched. The dice of the scenario's seed: on day 1 at P unit 2 hit 8 times and unit 1 5 times; at T unit 5 11 times and unit 4 7 times; on day 3 at P unit 2 hit 8 times (c = 8: unit 3 lost 160) and unit 3 once; on day 4 unit 2 hit 4 times and unit 3 not at all; at T unit 5 hit 9 times on day 2 and 5 on day 3. The doc comment gained the architect's one sentence that a unit out of the fight would not fire if it could. The properties are tests: units not falling back share losses and count for the limit; the enemy's view is the same in two worlds that differ only in whether a unit is falling back (a unit test in sim-core, since an integration test cannot set a march; it runs the ground combat phase only, so the ground reports, which count the true men in the hex, are outside it); and a fed unit falling back, in a hex where its own side attacks and the other defends, fires nothing and loses nothing, while with the other side on attack it fires and is fired on.
- **Every other scenario.** `scripts/compare-scenarios.sh` against the merge of pull request 315: on the frozen figures no scenario changed; on the live figures only the full slice did, at the Oka group's fight on day 39.

**Decision 2.** Each Japanese run of the full slice lands its rations as supply cargo with its men, at men x 0.005 x days, the weight and the row it rests on in a comment by each run. Each run's ships are those it had, the placeholder fast transports changed from class 11 to class 12 (the same 200 men, 30 t of cargo) and named "Placeholder fast transport with cargo"; a run's rations are shared equally among its class 12 ships (the smallest choice: the decision says nothing of how they are split, and the unload order lands them all).

| Task force, run | Men | Days | Rations | Ships of class 12 |
| --- | --- | --- | --- | --- |
| 30, Yokosuka force, 16 August | 200 | 7 | 7 t | 1, of 7 t |
| 31, Ichiki echelon, 18 August | 916 | 7 | 32.06 t | 5, of 6.412 t |
| 32, force of 29 August | 900 | 7 | 31.5 t | 3, of 10.5 t, beside Fubuki and Shirayuki |
| 33, Kawaguchi part 1, 31 August | 1,200 | 14 | 84 t | 5, of 16.8 t, beside Hatsuyuki and Amagiri |
| 34, Kawaguchi part 2 | 150 | 14 | 10.5 t | 1, of 10.5 t |
| 35, Kawaguchi part 3 | 1,000 | 14 | 70 t | 5, of 14 t |
| 36, Kawaguchi part 4 | 370 | 14 | 25.9 t | 2, of 12.95 t |
| 37, Kawaguchi part 5 | 370 | 14 | 25.9 t | 2, of 12.95 t |
| 38, Aoba part, 11 September | 637 | 14 | 44.59 t | 4, of 11.1475 t |
| 39, Oka group | 333 | 14 | 23.31 t | 2, of 11.655 t |

- **The derived destroyers** (the architect's ruling of Nov 15, 2026). A derived destroyer has no cargo figure, and none is given to one: Fubuki and Shirayuki (task force 32) and Hatsuyuki and Amagiri (task force 33) carry men only, and the run's rations sail in the placeholders beside them. Decision 0101's ruling D has a line that class 12 is now used for these runs as well.
- **The stocks stated ashore** are 0 in (3,-4) and 71 t in (2,-5). Base 5 is kept, with no stock: a base of the side with no stock is still a base, so where a beaten unit falls back to does not change (decision 0104, 2, looks for a base of the side, not a stock), and the rations of the Taivu runs are unloaded into it. Every run unloads its rations in the night pulse of its landing, as before, and steams on in the morning.
- **Decision 4** changes nothing in the file.

**Decision 3.** `fighting_unit_work_share` is `"0.1"` in `data/reference/ground.toml`, with the comment the decision gives; the frozen figure is 0.5. It changes the live scenarios 30 and 31, where fighting units are set to work, and the recorded game `full_slice_five_days_sonnet`, in which the model put both combat groups to the airfield: the field is no longer finished on 11 August in that game.

Decision 0101 has dated notes where these make a line of it untrue: its items 17 and 22, amendments 5 and 8 on the stocks, ruling D, and its two "As built" sections. The rulebook (chapter 7: "A hex is contested", "Firing men", "Shots and chance" and the limit on engaged men, "Losses", "A side beaten" and "A beaten side's men", and the table of the work figures) and the glossary ("Falling back", "Limit on engaged men", "Engaged men") say the correction.

**The new baseline** is `docs/validation/full-slice-39-days-before-tuning.md`: 1,000 runs, beside the four earlier reports, with the figures the round will need. Of 6 judged measures 4 agree and 2 miss, the Tenaru's, which none of the three changes touches. The ridge's Japanese losses fall from a median of 1,900 to 1,220 (the record, 600 to 1,138, is now inside the middle 90 percent); the Kawaguchi men lose no man on day 39 in any run, where they lost 709; the Oka group attacks fed, with 188 firing men; every Japanese attacker has a fed share of 1 on its day of attack. The airfield is still built on day 11 and the food at Lunga is still 68 days: what the Americans do is the slice's scripted orders, which set no fighting unit to work.


## As built: round 1

Run on Nov 15, 2026, by a team, on the slice after decisions 1 to 4, in the order of the table. Each step has its record under `docs/validation/tuning/` (`ground-r1-1-who-can-fire.md` to `ground-r1-6-landing.md`). The report after the round is `docs/validation/full-slice-39-days-ground-r1.md`: every measure before and after, a typical run, the scenarios that changed, and what the round could not reach. No code, order, unit, stock or rule changed; four figures of `data/reference/ground.toml` moved. The frozen figures and every hand calculation stand. Each figure moved is a placeholder, fitted on the two fights of the slice and not checked.

| Step | Figure | Before | After | Why |
| --- | --- | --- | --- | --- |
| 1. Who can fire | `unprepared_landing_fatigue` | 40 | 10 | The Ichiki echelon landed unopposed at night, marched two days and attacked in strength (land_unit_facts 1049, 994, 1020): it fires with 778 of 916 men, not 412. Not 0: a hurried landing still costs something |
| 1. Who can fire | `fatigue_per_march_day` | 10 | 5 | The same rows. Not lower: a march must tire, and the ridge's week inland is longer than the game's two days |
| 2. The limit | `engagement_ratio` | 3 | 3, **stopped** | The rows support down at both fights. The Tenaru's Japanese losses want about 8, the ridge's about 1.5 to 2.5 |
| 3. What a hit does | `men_per_hit` | 20 | 20, **stopped** | At 50 both Tenaru measures are in the middle half and the ridge loses 3,000 Japanese against 600 to 1,138. The chances to hit scale both fights alike and cannot part them |
| 4. When an attack stops | `attack_fatigue_limit`, `failed_attack_fatigue` | 60, 30 | the same, **left** | 65, or 25, gives one day at the Tenaru and two at the ridge, on a margin of five points of fatigue, and takes the ridge's Japanese losses to 1,920 |
| 5. Cover | none | | | A report: the cut of each piece to a whole man saves 0.517 of a raid's men at cover 30 against 0.27 stated; cover 10 and 20 save the same, full cover saves all. For the architect's ruling on the rule |
| 6. The landing | `unprepared_landing_share` | 0.5 | 0.13 | Less than half the supply landed (land_unit_facts 465, 773): 36 days of food at Lunga when the transports sail, against 17 to 37 (68 before). Every lower value tried leaves Americans unfed later, first at Tulagi |
| 6. The landing | `handling_t_per_man` | 0.25 | 0.02 | So the men still land in a day: 10,880 at Lunga at the end of day 1 (10,000 by 2000 on 7 August, 688) |

**The measures.** Of 6 judged measures 4 agree and 2 miss, as before: the Tenaru's two. The Americans there lose 46 (median) against 109 to 110, where they lost 23. The Japanese lose 340 against 800 to 900, unchanged. The ridge's American losses move into the middle half of their record (148 against 134 to 253); its Japanese losses are where they were (1,220 against 600 to 1,138). The food at Lunga is 36 days. Every run is a decisive Allied victory; the median margin is 92.15, where it was 101.64.

**The two stops have one cause, the rule and not a figure.** While the limit on engaged men binds, which it does at both fights for a ratio up to 4, the defender's fire is the ratio times every man the attacker has ashore in the hex. So the ridge's Japanese losses are 3,090 / 916 = 3.37 times the Tenaru's whatever the figures are, and the American 3.6 times by the firing men; the record has both near 1 to 2. The architect confirmed both stops on Nov 15, 2026.

**What the round could not reach**, set out in full with every row in the round report, for the designer:

- **A defender's counter-attack that destroys a spent attacker.** At the Tenaru the 1st Battalion, 1st Marines was taken out of reserve, Company G counter-attacked, the howitzers fired and light tanks crossed the bar at 1500 (engagement_outcomes 1935). The Japanese record has the counter-attack of the afternoon killing most of the Advance Unit (1921). No row says where the 1st Battalion went. The slice's Americans stand on the defence from days 3 and 4 with no later order. In the rules the fight is once a day, a unit cannot be behind another in a hex, and a beaten side falls back to any linked hex with a base of its side.
- **An attacker who commits part of his force and holds the rest.** At the ridge one battalion probed on the first night (land_unit_facts 859), two attacked on the second (1004, 1018), about 2,000 of 3,090 (engagement_outcomes 2013). Others struck the Ilu flank (land_unit_facts 1006, 1014, 1019), sat on the Tenaru line and fired nothing (988), or attacked in the west a day later (1007). In the slice all five parts attack together. In the rules the perimeter is one hex, every unit of a side in it fights as one body, and the limit counts every man ashore.
- **Two nights at the ridge,** which needs the above, not a figure (step 4).
- **Cover that grades,** which is the rule's arithmetic (step 5).
- **The food at Lunga at the middle of its record,** which would take the slice's later supply, which is data (step 6).

**Every live scenario whose result changed** (`sh scripts/compare-scenarios.sh` against pull request 318's merge: 118 runs, 7 differ, every frozen run the same): 07, the full slice, as above; 21 and 27, whose men land sooner; 25, 29, 32 and 37, whose marches tire less. Only in 07, 25 and 37 does a fight fire with more men. No other base, loss or outcome moved, and none would have changed by more than the four figures do. The recorded game `full_slice_five_days_sonnet` lands its men faster and its supply more slowly; its orders are answered as before.

## The Tenaru, a named miss

Decided by the designer on Nov 15, 2026. The two measures of the Tenaru, American and Japanese men lost, are accepted as a named miss for now, as the eleven of step 2 were (`docs/validation/step2-close.md`). No figure moves for them and the ground figures are not tuned further on that fight until the rules below exist.

They are mended with step 3's stage 3, the work on landings, which needs the same three things for its own sake:

- **Units held back are out of the fight.** An attacker commits part of his force and the limit on engaged men counts only what is committed.
- **Kawaguchi's force re-cut into battalions,** so that a part can be committed and a part held.
- **A defender's counter-attack that cuts off a spent attacker.**

Each gets its decision record, rule card and hand calculation before it is built. The two measures are judged again on the slice after them; until then every report of the slice lists them as the named miss and not as a fault of the figures.
