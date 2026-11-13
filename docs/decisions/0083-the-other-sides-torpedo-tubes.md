# 0083: The other side's torpedo tubes

Status: Decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"; "Yes, it decides"). Built together with decision 0074, which was held for it. The hand calculation below was written before any code was delegated, and the architect checked it. Built on Nov 13, 2026: see "As built".

## The gap

In the rules a ship fires every one of her tubes at once, in the first round in which she fires torpedoes, and then her torpedoes are spent (`docs/rules/04`, "Torpedo shots"). At Savo Island every Japanese ship fights the southern group first (decision 0024, 4), so every Japanese torpedo goes into that fight.

This was found while testing whether Savo Island's tuning could close (`docs/validation/tuning/savo-island-closed.md`). Over 1,000 runs on the figures of round 2, from a scratch batch:

| | The runs | The record |
| --- | --- | --- |
| Torpedo hits on Vincennes, Quincy and Astoria | none in any of the 1,000 runs | 3 to 6: Vincennes 1 to 3, Quincy 2 or 3, Astoria none (`engagement_outcomes` 61 to 63, 72, 73, 85) |
| Torpedo hits on Canberra and Chicago | a median of 2 each | 1 to 3: Canberra none to 2, Chicago 1 (23 to 25, 163, 38, 39) |
| Chicago sunk | 560 runs, 491 of them with a torpedo hit or more | afloat (768) |
| Canberra and Chicago both sunk | 471 runs | Canberra only (761, 765) |

The total agrees: torpedo hits on Allied ships are 1 to 8, median 4, against 4 to 9, and 524 runs hold it. So no judged measure shows the split, and no figure is aimed at it: it is where the torpedoes go, and that is a rule.

Decision 0074 alone does not mend it. On a scratch class file with 0074's tubes and no second side, both southern cruisers sink in 290 runs and Chicago in 411, the northern group still takes no torpedo, and torpedo hits on Allied ships fall to the edge (0 to 5, median 2, 230 runs). Its record says why: it leaves out "Firing the other side's tubes later in the fight".

## What the record says

Each row was read in the file. Times are the Allied zone's.

**Who fired which side at whom** (`engagement_events`, the 1950 analysis unless named):

| Ship | First side | Second side | Rows |
| --- | --- | --- | --- |
| Chokai | 4 to port at 01:38, at Canberra and Chicago | 4 to starboard at 01:48, at Vincennes | 1097, 1098; her own report has the same two salvos ten minutes apart, 1130, 1131 |
| Kako | 3 of her 4 starboard tubes, slightly before 01:46, at Chicago | 4 to port at 02:01, at Astoria; 2 more at 02:05 | 1100, 1101, 1102; her own report, `engagement_outcomes` 3498 to 3501 |
| Furutaka | 4 to port at Canberra and Chicago as she opened fire | 1 to starboard at Patterson at 01:48, and her remaining 3 starboard at Canberra at 01:49 | `engagement_outcomes` 3195 to 3197; `engagement_events` 1105 |
| Aoba | 3 from her starboard battery at Canberra | 1 to port, printed "about 0814" in a scan that is plainly misread | `engagement_outcomes` 3181, 3183; `engagement_events` 1099 |
| Kinugasa | 4 at Canberra at about 01:53, "which appear to have been her port torpedoes" | none on record | `engagement_events` 1103 |
| Tenryu | 6 to starboard at Quincy at 02:02, from two centreline mounts | (no second side: centreline) | `engagement_events` 1106 |
| Bagley | her starboard tubes were not primed before she swung past the bearing; 4 from her port mount at about 01:49 | none | `engagement_outcomes` 3206, 3228; `engagement_events` 1107 |

Chokai's own report counts "four firing lines" against a target and calls them too few (`engagement_outcomes` 3495). Kako's records that before the attack the tubes of both sides were trained out (3497).

**What the rows give the rule.**

- No ship fired both sides at once. Each ship with wing mounts that fired a second time fired the other side, as a separate salvo.
- The second side went at the other group in three cases (Chokai, Kako, Aoba) and at the same group in one (Furutaka, both sides at the southern group within about five minutes).
- The interval is about five minutes for Furutaka, ten for Chokai and fifteen for Kako. The ships turned to bring the other side to bear, and the rules have no bearings.
- Tenryu, with centreline mounts, fired once.
- Bagley fired one side only. Unreadiness is the cause the rows give for the other (decision 0071's share).

**Reloads.** Kako fired ten torpedoes from eight tubes, and six were left (`engagement_outcomes` 3501). Her two at 02:05 came after both sides had fired, so they were reloads. The Takao class's row on tubes per side names a gear for reloading spare torpedoes quickly (`ship_class_facts` 1341). Reloads are left out (below).

## Options

1. **Change nothing.** The total agrees, and the split is written down as a gap. Chicago keeps sinking in more than half the runs, where the record has her afloat.
2. **Decision 0074 alone,** as written and held. It halves the Japanese heavy cruisers' torpedoes, turns a measure that agrees into one at the edge, and still sends none to the northern group (the scratch run above).
3. **Decision 0074 with the other side fired in the next round in which the ship fires.** One rule for both sides and every battle. A ship that fights twice in a pulse fires the second side into the second fight. A ship whose one fight runs on fires it into the same fight, five minutes later. This is the rule decided.
4. **The other side only in a later fight.** This fits Chokai, Kako and Aoba, and not Furutaka. It would also leave a ship that has one long fight with half her tubes never fired.
5. **The other side no sooner than ten minutes after the first.** This fits Chokai and Kako, but its size rests on two ships, and it needs a count of rounds in the state for the gain of one round.

## Decision

1. **A class has two torpedo figures.** `torpedo_tubes` is every tube aboard, as today: the whole count, call it T. `torpedo_tubes_one_side` is the tubes that bear on one side, call it B, from 0 to T, and above 0 when T is. Decision 0074 derives B by its own rule (amended below), and `torpedo_tubes` keeps the whole count.
2. **The first salvo is one side's.** A ship fires B tubes in the first round in which she has torpedo shots to fire. That is the round it is today.
3. **The second salvo is what is left.** If T is above B, the ship fires the other T − B tubes in the next round in which she has torpedo shots to fire, whether in the same fight or a later one. Then her torpedoes are spent, as now.
   - Where B equals T (centreline mounts, and every placeholder and frozen class), T − B is 0. There is one salvo, exactly as today.
   - A fit with a centreline mount and wing mounts (Mahan: 12 tubes, 8 on a side) fires 8 and then 4.
4. **Each salvo is a salvo as today.** Its shots are its tubes x the torpedo mount factor x decision 0071's share, the share being that of the force in the fight the salvo is fired in. Its target is chosen by decision 0078 at the start of its own round. Its chance to hit and its breakdown are as now. The breakdown's tube count is the salvo's: B, or T − B.
5. **Spent only when fired.** A salvo with shots above zero spends its side, however few tubes the share or a degraded mount let fire. If the mounts are out when a side's turn comes, nothing is fired and nothing is spent, and that side waits (decision 0016, 6, as now).
6. **No reloads.** Each tube fires once in a game.
7. **One rule for both sides,** by day and night, in every battle.

### The state

A ship gains one flag, `torpedoes_one_side_fired`: true once she has fired the first side of a class whose T is above B, and until she fires the second.

- `torpedoes_ready` keeps its meaning: true while the ship has any tube unfired.
- After a salvo with shots above zero:
  - if the flag is false and T is above B, the flag is set and `torpedoes_ready` stays true;
  - otherwise `torpedoes_ready` is cleared and the flag is false.
- New invariant: the flag is set only on a ship whose `torpedoes_ready` is set and whose class has T above B.
- A scenario ship starts with the flag false.
- The flag goes into `hash::hash_world` straight after `torpedoes_ready` and is named in `hash::name_every_field`. `hash::STATE_VERSION` goes from v6 to v7, and every golden file is remade in that change.

### The draws

The first side's salvo draws from the ship's fire stream, as today's salvo does.

The second side's salvo is a new rule's draws, so it has a stream of its own: a new `rng::Draw` value, `TorpedoOtherSide`, added at the end. The stream is keyed on the day, the pulse, the surface combat subsystem, the firing ship and the enemy force. It is opened when that salvo is fired and used for that salvo alone: one draw for each shot, then one system draw for each hit, as for any torpedo salvo.

- **For a class with B equal to T** no salvo of the second side is ever fired, so the new stream is never opened, and no draw on any stream moves. Every scenario on the frozen figures and every placeholder class must give its event log byte for byte as before. Only the state hash changes, because the hash now covers the flag.
- **For a class with B below T** the first salvo has fewer shots than today, so the draws after it on the ship's fire stream come sooner. That follows from the derivation, as it would under decision 0074 alone. The second salvo's draws do not touch the fire stream, so the ship's later gun salvos draw as they would without it.

### What a side sees

Nothing new. The second salvo is a `Salvo` event like any other, with the weapon and the target, cut for each side as now, and `side_events.rs` is unchanged. The firer's breakdown shows the tubes of that salvo. No event says which side fired.

## Decision 0074, amended

0074's rule is built as written, with one change: the figure it derives is written to the new column `torpedo_tubes_one_side`, and `torpedo_tubes` keeps the whole count. Everything else stands:

- the order of reading (a stated number of tubes on one side, else the mount position, else T, with the class named as not split);
- the refusals of its item 4;
- the seven exceptions of its table;
- the gaps it names (Kamikaze not split; Benham's 1942 fit unknown);
- the `[class_ids]` pins of each list.

The placeholder classes and the frozen step 1 class files carry B equal to T (its item 6).

What the Savo Island list gives:

| Class | T | B |
| --- | --- | --- |
| Aoba, Furutaka, Takao | 8 | 4 |
| Tenryu | 6 | 6 |
| Yubari | 4 | 4 |
| Kamikaze (1922) | 6 | 6, not split |
| County, Kent subclass | 8 | 4 |
| Bagley (both rows), Benham | 16 | 8 |
| New Orleans, Northampton | 0 | 0 |

## What it carries and what it does not

- **It carries:** that a ship with wing mounts fires one side at a time, and the second side at whatever she next fights.
- **It does not carry:**
  - **The bearing,** and so how long a ship takes to turn the other side on. The second side comes in the next round in which the ship fires, five minutes on, where the record has five to fifteen.
  - **Reloads.** Each tube fires once in a game. The rows have Kako firing more than her eight tubes:
    - "At 0205 Kako fired two additional torpedoes at Astoria" (`engagement_events` 1102), after her starboard and port salvos.
    - Her own table of ordnance expended reads 消耗数量 一〇 残額 六: ten expended and six remaining (`engagement_outcomes` 3501).
    - The Takao class's row on tubes per side names 次発装填装置, a gear for loading spare torpedoes quickly (`ship_class_facts` 1341).
    Torpedo reloads are on the open list of `docs/open-work.md`.
  - **A ship that holds her torpedoes for a better target.**
  - **Readiness.** That is decision 0071's share, which multiplies each salvo and is not changed here.
  - **How many ships of a force fire at all,** which no figure carries (decision 0071, "What the share carries").
  - **Fights that overlap.** At Savo Island the column was on the northern group before it had done with the southern, and its ships' second sides went there. In the rules a force meeting two enemy forces at one moment fights them one after the other. One fight of three forces would let a ship take her targets from both enemy forces in the same rounds. It is recorded as a plan for the designer at the close of step 2 and not built, by the architect's ruling of Nov 13, 2026. With it would go Mikawa's split into two columns (decision 0024, 3) and Chicago leaving the fight (decision 0078). Chicago's fate and the northern group's torpedo hits at Savo Island are named differences resting on it.

## Rule card

| Rule | Reads | Writes |
| --- | --- | --- |
| Torpedo fire (amended; decisions 0014, 0016, 0071, 0078) | The class's `torpedo_tubes` and `torpedo_tubes_one_side`; the ship's `torpedoes_ready` and `torpedoes_one_side_fired`; the torpedo mount state; the fight's readiness share | `torpedoes_ready`, `torpedoes_one_side_fired` |
| Surprise (unchanged rule) | Torpedo shots now count the side a ship has left | Nothing new |

Randomness: one new stream, `Draw::TorpedoOtherSide`, for the second side's salvo only. Events: none new.

## Hand calculation

On the frozen step 1 figures (`data/reference/step1`), with one new placeholder class added to both placeholder class files: class 8, "Placeholder cruiser with wing tubes". It is class 1 in every figure (8 guns at 3 rounds a minute, `gun_damage` 20, toughness 400, 8 tubes, `torpedo_damage` 200), except that `torpedo_tubes_one_side` is 4. Every other class has B equal to T.

Figures read:

| Figure | Value |
| --- | --- |
| `torpedo_hit_chance_night` | 0.04 |
| `torpedo_hit_chance_day` | 0.02 |
| `surprised_target_factor` | 1.5 |
| `degraded_factor` | 0.5 |
| `system_hit_chance_per_point` | 0.06 |
| `unready_torpedo_share` | 1 (0.5 where a case says so, by an altered copy of the frozen figures, as scenario 15b does) |
| Experience strengths | 0 |

A torpedo hit from class 8 or class 1 on class 1 or 8 does 100 x 200 / 400 = 50 points. Its system chance is min(1, 50 x 0.06) = 1.

| Case | Salvo | Tubes | Shots | Chance | Expected hits | After it |
| --- | --- | --- | --- | --- | --- | --- |
| A. Class 8, ready, a night fight of two rounds or more, target not surprised | round 1, first side | 4 | 4 x 1 x 1 = 4 | 0.04 | 0.16 | ready, flag set |
| | round 2, second side, target by 0078 at the start of round 2 | 8 − 4 = 4 | 4 | 0.04 | 0.16 | spent, flag clear |
| A'. As A, the target's force surprised in round 1 | round 1 | 4 | 4 | 0.04 x 1.5 = 0.06 | 0.24 | ready, flag set |
| B. Class 8, first fight ends after round 1; in the same pulse she meets a second enemy force | first fight, round 1 | 4 | 4 | 0.04 | 0.16 | ready, flag set |
| | second fight, her first firing round, at the most important kind afloat in place (i modulo their number) | 4 | 4 | 0.04 | 0.16, drawn from the new stream keyed on her and the second force | spent |
| B'. As B, the second meeting by day in the next pulse | second side | 4 | 4 | 0.02 | 0.08 | spent |
| C. Class 8, force not ready, share 0.5, a night fight of two rounds | round 1 | 4 | 4 x 1 x 0.5 = 2 | 0.04 | 0.08 | ready, flag set: the 2 unfired tubes of that side are not kept |
| | round 2 | 4 | 2 | 0.04 | 0.08 | spent. Together 4, today's 8 x 0.5 |
| D. Class 8, mounts degraded before round 1 | round 1 | 4 | 4 x 0.5 = 2 | 0.04 | 0.08 | ready, flag set |
| | round 2, still degraded | 4 | 2 | 0.04 | 0.08 | spent |
| D'. Class 8, mounts out after her first salvo | round 2 | 4 | 0 | | no salvo, no event, no draw | ready, flag set: the side waits |
| E. Class 8 sunk in round 1 by fire of that round | round 1 | 4 | 4 (a ship sunk in a round still fires in it) | 0.04 | 0.16 | sunk; no second salvo; the flag stays as it was on a sunk ship |
| F. Class 1 (B equal to T) | round 1 | 8 | 8 | 0.04 | 0.32 | spent, flag never set; round 2 has no torpedo salvo and the new stream is never opened |
| G. Class 8, a round 2 with no gun shots (ammunition gone) | round 2 | 4 | 4 | 0.04 | 0.16 | spent. She counts as having shots for the surprise test |

**The order of a round's draws, for class 8 in round 2 of case A:**

1. On her fire stream (`Draw::Legacy`, keyed on her and the enemy force): her gun salvo's 8 x 3 x 5 = 120 shot draws, then a track and a system draw for each gun hit.
2. On `Draw::TorpedoOtherSide`, keyed on the day, the pulse, her and the enemy force: 4 shot draws, then one system draw for each torpedo hit.

Damage lands at the end of the round in the order the salvos were fired, gun salvo first, as now.

**What must not move.** With class 8 in no scenario but the rule's own, every scenario on the frozen figures and every placeholder scenario gives its event log byte for byte as before; only its state hashes move, by the new field. No expected number of any existing hand calculation changes.

## The rule's own scenario

`data/scenarios/18_torpedo_sides.toml`, named for the next free number at the time of building. It is scenario 15b's set-up with the attacking force's three cruisers of class 8. Its test has the three parts of `docs/step2-spec.md`, "Tests while figures move", 7:

1. **The cases above, read from the salvos' breakdowns.** A, A', C and D are in its fight. B, B', D', E, F and G come from small variants of it, or from unit tests in `surface.rs` where a scenario cannot reach the case.
2. **Properties over many seeds.**
   - Every class 8 ship that fires twice fires B and then T − B, in different rounds.
   - No class 1 ship ever fires two torpedo salvos.
   - Torpedo hits stay within four sigma of the expected hits.
   - The sum of a class 8 ship's torpedo shots never exceeds 8 x the share.
3. **A pinned outcome for one seed,** labelled as pinned.

## As built

Built on Nov 13, 2026, in three commits on branch `torpedo-sides`, each delegated to a developer and reviewed by the architect against the hand calculation above.

**The class figure (decision 0074).** `torpedo_tubes_one_side` is a column of every class file, read by the loader as required: a file without it is refused, and that refusal is the check that no class file was missed. Every class file the scenarios and tests read is one of these:
- the live `ship_classes.csv`;
- the three derived lists, generated again by `oob-derive`;
- the four frozen copies in `data/reference/step1`.

The frozen scenarios in `data/scenarios/step2-frozen` hold no class data: they name these files. Every placeholder and frozen class has B equal to T. Placeholder class 8, "Placeholder cruiser with wing tubes", is new and appears in scenario 18 alone. It is 8 and not 7 because scenario 17a's test adds a class 7 of its own. `ShipClass` refuses B above T, and B of 0 where T is above 0. `oob-derive` writes 0074's figure by its rule, with the seven exceptions in `data/derivation/rules.toml`, and names the classes it cannot split:
- at Savo Island, Kamikaze;
- at Cape Esperance, Fubuki, Gleaves and Benson.

Every `torpedo_tubes` value is unchanged.

**The rule.**
- `Ship::torpedoes_one_side_fired` is a new field, hashed straight after `torpedoes_ready`. `hash::STATE_VERSION` is `flagplot-state-v7`.
- A new invariant covers the flag.
- `Draw::TorpedoOtherSide` is code 5.
- In `crates/sim-core/src/surface.rs`, `torpedo_salvo_tubes` gives B or T − B. `fire_ship` sets or clears the flags as above, and draws the second side from its own stream.
- Unit tests cover cases B, D', E, F and G, and a class of 12 tubes with 8 on a side.
- Scenario 18 (`data/scenarios/18_torpedo_sides.toml`, `tests/scenario_18_torpedo_sides.rs`) asserts cases A, A', C and D from the breakdowns, the properties over 1,000 seeds, and a pinned outcome for seed 1942 on the live figures. Its class 8 ships fired twice in 2,514 of 2,895 ship-runs.

**What did not move.** Every scenario, run live and on the frozen figures, was compared with its event log before the change. Every log differs only in its hashes, except the two live logs of Savo Island, the only scenario with a class whose B is below T. No expected number of any hand calculation changed.

Remade, in a commit of their own:
- every run baseline;
- the recorded games' saves: their transcripts are byte for byte as before;
- the old-reader file of scenario 6;
- the pinned hash of scenario 17a's case 8, whose log is unchanged;
- the Eastern Solomons day hash that scenario 10's test reads.

**What Savo Island shows.** 1,000 runs, seeds 1 to 1000. Before is the round 2 batch (`docs/validation/savo-island-r2.md`); after is the same figures with this decision and 0074. Each cell gives the 5th to the 95th percentile, the median, and in brackets the runs holding the record.

| Measure | Record | Before | After |
| --- | --- | --- | --- |
| torpedo_hits_on_allied_ships | 4 to 9 | 1 to 8, 4 (524) | 1 to 8, 4 (560) |
| Torpedo hits on the southern group (not a measure) | 1 to 3 | 1 to 8, 4 (418) | 1 to 6, 3 (571) |
| Torpedo hits on the northern group (not a measure) | 3 to 6 | none in any run | 0 to 3, 0 (93); 364 runs have one or more |
| allied_torpedoes_fired | 4 to 6 | 16 to 48, 34 (0), miss | 16 to 44, 26 (0), miss |
| allied_cruisers_sunk | 4 | 0 to 5, 3 (221) | 1 to 5, 3 (220) |
| southern_group_cruisers_sunk | 1 | 0 to 2, 1 (399) | 0 to 2, 1 (381) |
| northern_group_cruisers_sunk | 3 | 0 to 3, 2 (368) | 0 to 3, 2 (424) |
| allied_destroyers_sunk | 0 | 0 to 1, 0 (862) | 0 to 2, 0 (754) |
| minutes_of_action | 20 to 30 | 35 to 65, 40 (25), miss | 30 to 65, 40 (112), at the edge |
| tenryu_and_yubari_main_battery_shells_fired_in_hundreds | 1 | 2 to 5, 3 (37), miss | 2 to 4, 3 (35), miss |
| gun_hits_on_allied_ships | 153 to 219 | 35 to 158, 116 (88), at the edge | 43 to 160, 109 (86), at the edge |
| The report's head | | 19 agree, 1 at the edge, 3 miss | 18 agree, 3 at the edge, 2 miss |

Torpedo hits by ship, runs with one or more: Vincennes 212, Quincy 219, Astoria 128, where each had none. The record has Vincennes 1 to 3, Quincy 2 or 3, Astoria none. Every other measure moved by less than the runs' own spread. The Japanese main battery shells went to the edge, with 101 runs holding.

**Chicago is not mended.** She is sunk in 565 runs, against 560 before; both southern cruisers sink in 451 runs, against 471. The rule sends a second side to whatever the ship next fights, and the southern fight lasts two rounds or more in 682 runs of 1,000. So in most runs the second side goes into the southern group again.

That is not because the southern fight runs too long. The record's own southern fight lasted about two rounds:
- the Japanese opened fire about 01:43 (`engagement_events` 59);
- the group was "dissolved into independent ships by 0150" (`engagement_events` 81);
- the fight "seems to have lasted no more than 10 minutes" (`engagement_outcomes` 17).

The second sides went north in the record because the two fights overlapped. The northern group was lit at 01:50 (`engagement_events` 90). Chokai fired her starboard side at Vincennes at 01:48 (100, 1098), while Kinugasa was still firing at Canberra at about 01:53 (101). The rules fight the two groups one after the other, so they cannot give that. It is a named difference, resting on the plan for one fight of three forces below. Nothing is tuned for it.
