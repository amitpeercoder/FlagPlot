# 0087: The patrol's fire and the attack

Status: decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"; "Yes, it decides"). The hand calculation's cases were worked and checked by the architect before any code; its two choices (the guns' condition after the patrol's fire before the attack, and the patrol's after-roll before the guns' in step 4a) were ruled as written. Built on Nov 13, 2026; see "As built".

Summary: The patrol's fire and the attack: a share of the patrol's shots at the bombers (`patrol_before_attack_share`, 1 frozen, 0.4 live, a placeholder inside the supported 0.22 to 0.61 of Simpler's study, rows 3979 to 3981) is worked before the attack and the rest after it, by 0079's split; the after-roll is the `AirFight` with `after_attack`, in step 4a before the guns'. No figure for fighters that never engage (inside `fighter_hit_chance`).

Standing: Decided by the architect on the designer's delegation

## The gap

The patrol fires at a raid's bombers before they attack, and every bomber it shoots down drops nothing (decisions 0027 and 0033; decision 0079 left the patrol so: "The fighters met the strike before it dived (ev 218, 219), so the patrol's hits stay before the attack").

The research of Nov 13, 2026 (pull request 214) says otherwise for the Eastern Solomons. Lieutenant Commander Simpler's post-action study of 38 fighters of VF-5 and VF-6 attached to Saratoga (`engagement_outcomes` 3984, ONI, high, preferred) counts:

- "Four pilots first engaged the enemy prior to his 'push-over.'" (3979)
- "Seven pilots first engaged at the push-over point or in the dive." (3980)
- "Seven pilots first engaged during retirement." (3981)
- "Twelve pilots were engaged by enemy fighters before they could attack his dive bombers or torpedo planes." (3982)
- "Eight pilots never engaged at all." (3983)

The research note says the counts are not reconciled by the source. They sum to 38 if the twelve met by fighters are read as a class apart from the three classes of first engagement with the bombers; that is a reading, and nothing below rests on the total.

Of the 18 whose first engagement was with the bombers, 4 engaged before the push-over and 7 in the retirement. The bombs were released at 1,500 to 2,000 feet (`engagement_events` 223). So much of the patrol's fire fell during the dive or after the bombs had gone.

At Santa Cruz the same fighter director found the same, in words: "most enemy aircraft were intercepted too late, often not until they had completed their attack" (ev 1282, ONI). That is another battle and is not counted here.

## Options considered

1. **Leave it.** Rejected: tuning would have to weaken the patrol to let the bombs through, where the record has it deadly to the bombers and late.
2. **A rule of its own for the patrol.** Rejected: decision 0079 already splits a roll into a part before the attack and a part after it, on one stream, and the rules say a new rule reuses an existing mechanism where it can.
3. **One figure for the share of the patrol's fire at the bombers that is worked before the attack, by 0079's split** (this decision).

## Decisions

1. **A share of the patrol's shots at the bombers is worked before the attack, the rest after it.** The share is `air_combat.toml: patrol_before_attack_share`, from 0 to 1.
2. **The shots.** The patrol's roll at the bombers has n shots, as decisions 0027, 0033 and 0080 give them (R − X). The first k = floor(share × n), one fixed-point multiplication cut to a whole number, are worked before the attack and the other n − k after it.
3. **One roll, split, as in decision 0079.** The patrol's shots are rolled from the one stream they have now, in order. That is the roll at the escorts first, then the first k shots at the bombers, then the other n − k. So the number and order of the patrol's draws are the same at every share, and on any seed its hits at the bombers in all are the same.
4. **Not split:** the patrol's roll at the escorts, and the escorts' fire at the patrol. Both stay before the attack: the twelve pilots met by escorts were met before they reached the bombers (3982).
5. **Before the attack.**
   - The patrol's before-hits come off the bombers, lowest group first.
   - The guns then fire if any bomber is left, with their own split (decision 0079). Their condition reads the bombers left after the patrol's fire before the attack, which at a share of 1 is all of the patrol's fire, as now. At a share below 1 the guns can therefore fire in a raid where, at 1, the patrol would have taken every bomber first. That is the one case in which a group's losses can differ across the share; otherwise both parts of each roll come off one pool in group order and each group loses what it would at 1.
   - What is left attacks.
6. **After the attack.** In step 4a of the air combat step, raid by raid in order of the first strike group:
   - the patrol's after-roll comes first, its hits coming off the bombers that are left, lowest group first;
   - then the guns' after-roll of decision 0079.
   The attack stands. Hits beyond the bombers there are wasted. Only bombers are hit after the attack.
7. **The events.**
   - The after-roll is the same `AirFight` (firer: the first patrol group; at the bombers) with a new field, `after_attack: true`, written only when true. The text adds "after the attack" when it is true.
   - It cites the raid's launch and the raid's `AirAttack` or `BaseAttacked` events (the launch alone if none attacked).
   - Its `AircraftLost` cite it.
   - A part with no shots records nothing, so at a share of 1 the log is step 1's, byte for byte.
8. **The breakdown.** The patrol's roll at the bombers lists, as now, the whole patrol, the raid's aircraft, all the raids' aircraft and the raid's share, and 0080's factors when that share is below 1. When this share is below 1 it also lists the shots at the bombers in all (n) and the share. Then come the part's shots and chance.
9. **Claims.** The defender's false claims are drawn for each whole missed shot in the order its firing events were noted (decision 0056).
   - With no guns in the raid, the patrol's two parts miss what its one roll missed, in the same order, so the claims are the same at every share.
   - With guns, the guns' misses are noted between the patrol's two parts at a chance of their own, so the claims may differ.
10. **The fighters that never engaged.** Eight of the 38 never engaged (3983). There is no figure for them: in expectation a fighter that never engages is a shot that never hits, so the share sits inside `fighter_hit_chance`, which the next tuning round sets on the Eastern Solomons. A fighter that never engaged claims nothing, where a shot that missed may be claimed. That is why the claim chances are tuned after the fighter chance.
11. **Whom the patrol fires at is unchanged (decision 0080).** These rows also bear on that decision's figure.
    - Of the 30 who engaged, 12 met the escort first (0.40); of all 38, 0.32.
    - The first wave was 10 escorts and 27 bombers (290). The proportional split, `patrol_drawn_off_share` 0, sends 10/37 = 0.27 of the patrol at the escorts. One for one, at 1, sends 10/53 = 0.19.
    - The rows sit at or above the proportional end. So 0 is now supported by rows as an end of the figure's range, though it is not sized by them.
    - No rule is made for escorts that draw off more than one defender each.
    - *Note, Nov 13, 2026 (decision 0089, ruling 1):* the comparison above leaves out the cap of decision 0080, the shots at the escorts no more than the escorts. With 53 fighters, 10 escorts and 27 bombers, a share of 0 gives 53 x 10 / 37 = 14, capped to 10, and a share of 1 gives 10: both ends send 10 of 53 at the escorts. The rows do not tell the two ends apart here, and 0 is not supported by them as an end. `patrol_drawn_off_share` is fitted, not checked, as decision 0084 had it.
12. **The figure.**
    - In the frozen step 1 figures it is 1, which leaves step 1 as it was.
    - In the live figures it is 0.4. The supported range is 0.22 (before the push-over only, 4 of 18) to 0.61 (the whole dive, 11 of 18). No row divides the seven who engaged at the push-over or in the dive, so 0.4 is a placeholder inside the range, chosen now before any result is read.
    - A later round may move it only inside 0.22 to 0.61.
    - It is one figure for ships and bases alike.
    - *Note, Nov 13, 2026 (decision 0089, ruling 2):* the range 0.22 to 0.61 is the architect's reading of pilot counts as shares of shots, not a bound from a source. The rows count when each pilot first engaged; the rule splits shots, and the study covers the fighters of one carrier. The figure stays 0.4, a placeholder, and the range stays the limit a later round may move within, named as an interpretation. The rows support the direction only.
13. **One rule for both sides.**

### The words for the rulebook

`docs/rules/05`, "The patrol against the escorts and the bombers", after item 2:

> The shots at the bombers are split by `air_combat.toml: patrol_before_attack_share`: the first k = the shots times the share, cut to a whole number, are worked before the attack, and the others after it, in order from the one stream, so the draws and the patrol's hits are the same at every share. The hits before the attack come off the bombers before they attack. The hits after come off the bombers that are left, after their attack is worked out, and whatever the attack did stands. The roll at the escorts is not split. A part with no shots records nothing. The fire after the attack is the same `AirFight` with `after_attack` set, recorded in step 4a before the guns' fire after the attack. [0087]

## Rule card

| | |
| --- | --- |
| Reads | The patrol's shots at the bombers (as now); the bombers after the escorts' and patrol's fire; the share |
| Writes | The bombers lost to the patrol, in two parts: before the attack (as now) and after it, from the same pool |

- No state, and no new version of the state hash.
- No new stream and no new draw: the patrol's stream and its draws are unchanged at every share.
- One figure, and one event field, `after_attack` on `AirFight`, written only when true.
- `side_events.rs`: a side sees its own patrol's fire, cut as now, and whether it was after the attack. The attacker sees its own `AircraftLost`.

## What it carries and what it does not

- It carries the timing of the patrol's fire at the bombers, and nothing of how deadly it is: `fighter_hit_chance` is not touched.
- It does not split the escorts' fight.
- It gives no figure for fighters that never engage (decision 10).
- It gives no warning or height rule. C3 of the air plan is ruled: no such rule in step 2, since no row sizes it; the fighter chance is tuned on the Eastern Solomons and checked on Rabaul, where bombers lost is expected to be written as a named miss.

## What it costs

- `crates/sim-core/src/airstrike.rs`: the split of the patrol's roll at the bombers, its place in step 4a, and the causes.
- `events.rs`, `side_events.rs` and the export: the field.
- `sim-data`: the figure, required, from 0 to 1.
- The live and frozen `air_combat.toml`, and the frozen README.
- Scenario 19c with the hand calculation, properties and pins. The byte-for-byte proof at a share of 1.
- `docs/rules/05` and the glossary in the same pull request.
- A dated note in decision 0079.
- The run baselines, recorded games and pins of the live air scenarios.
- No hand calculation on the frozen figures moves.

## As built

**The code.**
- `crates/sim-core/src/airstrike.rs`, in `work_out_raid`: the patrol's roll at the bombers rolls the first k shots and then the other n − k on the patrol's stream. The second part is carried, and its hits are taken from the bombers after the attack outcomes and before the guns' after-roll.
- The guns' condition reads the bombers left after the patrol's fire before the attack.
- `resolve_strikes` records the patrol's after-roll in step 4a before the guns', with its losses, its pilots and its note in the claim tally.
- `AirFight` carries `after_attack`. The export layer writes it only when true.
- The figure is required, checked from 0 to 1: live 0.4, frozen 1.
- No state, hash layout, stream or `Draw` value changed.

**The proof that a share of 1 is step 1.**
- The event logs on the frozen figures of 05, 05c, 06, 06a, 12b, 13a, 13b, 16, 19a and 19b, and case 1 of 19c for seeds 1 to 100, are byte for byte those of the binary before the rule.
- No expected number of any hand calculation moved.

**Its scenario.**
- `19c_patrol_after_the_attack`: the architect's four cases, the properties over seeds 1 to 1,000, and pinned outcomes.
  - The pins are seed 1942 and seed 1 in each case and at each share. Seed 1942 already moves patrol hits across the attack.
  - The property tests add four-standard-deviation ranges for cases 2 to 4, worked by the developer and reviewed.
- Every number of the hand calculation matched.

**Before and after, 1,000 runs each, label `d0087`.**
- The reports are `docs/validation/archive/eastern-solomons-d0087.md` and `docs/validation/archive/rabaul-guadalcanal-air-d0087.md`, made on the code with decisions 0079, 0080 and 0081 to 0084.
- "Before" is the `d0080` pair, the same code without this rule.

| Eastern Solomons | Record | Before: 5th to 95th, median (runs holding) | After |
| --- | --- | --- | --- |
| Bomb hits on Enterprise | 3 | 0 to 5, 1 (123): agrees | 0 to 7, 3 (105): agrees |
| Japanese bombers left to attack TF16 and TF11 (reported) | 20 to 27 | 0 to 18, 11 | 0 to 24, 20 |
| US aircraft left to attack the Ryujo detachment | 36 | 0 to 38, 0 (130): agrees | 0 to 38, 0 (34): miss |
| Japanese carrier aircraft lost to enemy action | 29 to 32 | 2 to 35, 26 | 2 to 35, 26 |

- Judged 15: before, agree 11, at the edge 1, miss 3; after, agree 10, at the edge 1, miss 4.
- The patrol's hits at bombers are the same a run (9.18 on Japanese raids, 0.65 on American ones). The Japanese bombs dropped at ships rise from 8.5 to 13.4 a run, and their hits from 1.7 to 2.7.
- The middle run now puts three bombs into Enterprise, as the record has. The bombers left to attack come into the record's 20 to 27.
- **The new miss is a question for the measure.** The record's 36 US aircraft left to attack the Ryujo detachment are the 38 Saratoga launched less 2 forced back by mechanical trouble (row 333). None was shot down before the attack (rows 237, 354 to 357). The rules have no abort. So a run held 36 only when Ryujo's patrol shot down two before the attack, and with less of that fire before the attack fewer runs do. The runs give 38 in 273, 37 in 181 and 36 in 34. The agreement before rested on that substitution, not on the record's cause.

- *Nov 13, 2026, after this result:* the architect changed the measure's record from 36 to 38, taking the aborts, which are outside the rules, out of it (the measures file's head and decision 0084 have dated notes). Remade on that record, the Eastern Solomons report reads judged 15, agree 11, at the edge 1, miss 3, and the measure agrees, held by 273 runs; `eastern-solomons-baseline.md` was remade with it.

At Rabaul against Guadalcanal:
- Judged 10: before, agree 4, at the edge 3, miss 3; after, agree 4, at the edge 2, miss 4.
- Bombers lost (26 against 27), fighters lost (12) and US fighters lost (23) do not move: the patrol's hits are the same.
- The bombers it hits after the attack now bomb. Bomber sorties that reach the island go from 201 to 214, against a record of 211 to 213. Aircraft destroyed on the ground go from 12 to 13 in the middle run.
- F4Fs ready on day 21 keeps its spread and median (2 to 12, 6). Runs holding 11 go from 50 to 40, so it falls from the edge to a miss on the one-in-twenty test, as it did under decision 0079 for the same cause: a few more aircraft hit on the ground.
