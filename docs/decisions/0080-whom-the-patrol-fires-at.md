# 0080: Whom the patrol fires at

Status: decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"; "Yes, it decides"). The hand calculation's cases were worked and checked by the architect before any code. Two details of the rule are the build session's, accepted by the architect: the rest is split against the escorts not yet drawn off (E − D), which is what makes s = 1 today's rule when the patrol outnumbers the escort; and the cap, so that no more shots go at the escorts than there are escorts left.

## The gap

A raid's escorts draw the patrol off its bombers one for one: "The patrol fires as many shots at the escorts as there are escorts, or as there are patrol fighters if those are fewer. Any patrol fighters left over fire at the bombers" (decision 0027, item 6). Its reason was smallness: "One for one is the smallest rule that makes an escort worth sending and a patrol worth keeping" (0027, "Why"). No record was cited.

The record contradicts it in both air battles of the slice.

- **Rabaul against Guadalcanal.**
  - Decision 0042 names the miss in its own words: "The Japanese lose three fighters for every bomber. The patrol fires at the escorts first, one for one (decision 0027) ... The Wildcats in fact dived through the escort for the bombers."
  - Over 1,000 runs on the figures of Nov 13, 2026, the patrol fires 131 shots a run at escorts and 25 at bombers. It reaches the bombers in about 3.5 raids of 11, nearly all after fresh squadrons arrive. Japanese fighters lost: 30 to 50, against a record of 6 to 10.
  - The units' own forms have bombers lost 12 and fighters 6 to 10 (`engagement_outcomes` 1048, 1060, 1020, 1024, 1095, 1101, 1034, 1109 against 1202, 1216, 1140, 1231).
  - The US fighters' claims by type for the months are 95 bombers against 68 fighters (1267: "21 bombers + 31 fighters"; 1273: "55 bombers + 22 fighters"; 1289: "19 bombers + 15 fighters"). Claims are inflated and are read here for where the fire went, not for a size.
- **The Eastern Solomons, Ryujo's raid on Henderson Field.**
  - Six attack planes and 15 fighters (1171) met the Marine patrol. The rule lets a patrol smaller than the escort fire no shot at a bomber: in the scenario 13 F4Fs against 15 escorts touch no B5N in any run.
  - The record has 4 B5Ns and 3 A6Ms lost (271, medium, preferred; 312: "2 F4-F are shot down for the loss of 4 B5N2 and 3 A6M2"). Her own form gives 5 in all (1172).
  - Today's rule makes that impossible.

## Options considered

1. **Leave it.** Rejected for the reasons above. Tuning would have to make every fighter shot weak to keep the escorts alive, and then the bombers would live too.
2. **The patrol fires at the bombers only.** Then the escorts are never fired at, and Japanese fighters at Rabaul would be lost to nothing. The record has 6 to 10.
3. **The patrol's shots split between escorts and bombers in proportion to their numbers.** No figure, but no way back to step 1's rule for the frozen copy.
4. **One figure s, the share of the patrol the escort draws off one for one, the rest split in proportion** (this decision). At s = 1 it is today's rule exactly; at s = 0 it is option 3.

## Decisions

1. **The figure.** `air_combat.toml: patrol_drawn_off_share`, s, from 0 to 1.
2. **The numbers of a raid.**
   - P is the raid's share of the patrol (decision 0033, unchanged).
   - E is the escorts sent with the raid.
   - B is its bombers at the start of the step, which is at least 1.
3. **Drawn off.** D = s × the smaller of P and E, cut to a whole number toward zero, in one fixed-point multiplication. These D patrol fighters fire at the escorts.
4. **The rest.** R = P − D. These fire at the escorts not drawn off and at the bombers, in proportion to their numbers. The shots at the escorts are X = (R × (E − D)) / (E − D + B), cut toward zero in whole numbers, and never more than E − D. The cap binds only when the raid's share is larger than the raid (P > E + B); then the extra shots go at the bombers, as today's rule sends them.
5. **The shots.** At the escorts: D + X. At the bombers: R − X.
6. **Today's rule is s = 1.**
   - D is the smaller of P and E.
   - If P ≤ E, R is 0.
   - If P > E, E − D is 0, so X is 0.
   - Either way the shots are exactly today's, in every case.
7. **As now:**
   - The escorts fire E shots at the patrol, from their own stream, whatever s is.
   - The patrol fires from its stream, keyed on the first patrol group and the raid's first strike group: first at the escorts, then at the bombers, one draw a shot.
   - The chances, the protection factors and the pools are those of decision 0027 and 0033.
   - A raid with no escorts gives all its share to the bombers.
   - A raid whose share is 0 meets no patrol.
   - A patrol roll with no shots records nothing. At s = 1 that never happens with escorts present and a share above 0, so the log is unchanged.
8. **The breakdown.** A patrol roll's breakdown lists, as now, the whole patrol, the raid's aircraft, all the raids' aircraft and the raid's share, then the shots and chance. When s < 1 it also lists s, the fighters drawn off (D) and the rest's shots at the escorts (X).
9. **The figure's values.**
   - In the frozen step 1 figures it is 1: step 1 as it was.
   - In the live figures it is 0, a placeholder.
   - Its support for the direction: decision 0042's own sentence, the forms' losses (12 bombers against 6 to 10 fighters), the claims by type, and row 271 at Henderson Field.
   - No source sizes it. No row says what share of an interception went for the escort.
10. **One rule for both sides, over ships and over a base.**

### The words for the rulebook

`docs/rules/05`, "The patrol against the escorts and the bombers", in place of items 1 and 2 and the sentence that follows them:

> The raid's share of the patrol is split between its escorts and its bombers. With P the share, E the escorts and B the bombers, the escorts draw off D = P or E, whichever is smaller, times `air_combat.toml: patrol_drawn_off_share`, cut to a whole number. The rest, R = P − D, is split in proportion between the escorts not drawn off and the bombers: X = R × (E − D) / (E − D + B), cut to a whole number and never more than E − D, fire at the escorts, and R − X at the bombers.
>
> 1. At the escorts: D + X shots. Chance: `fighter_hit_chance` times the protection factors of the first escort group's type. Hits come off the escorts, lowest group first, no more than they have. Skipped if it has no shots.
> 2. At the bombers: R − X shots. Skipped if 0. Chance: `fighter_hit_chance` times the protection factors of the first strike group's type. Hits come off the raid's strike groups, lowest group first, no more than they have.
>
> At a share of 1 the escorts draw the patrol off the bombers one for one, and a raid with no escorts takes its whole share. [0027, 0029, 0033, 0080]

## Rule card

| | |
| --- | --- |
| Reads | The raid's share of the patrol, its escorts and its bombers at the start of the step (as now); the figure s |
| Writes | The number of the patrol's shots at the escorts and at the bombers, nothing else |

- No state, and no new version of the state hash.
- No new stream or draw. The patrol's stream draws one per shot as now; its total draws in a raid are its share whatever s is. Only the split between its two rolls moves.
- No new event or field.
- `side_events.rs`: unchanged. A side sees its own patrol's fire cut as now.

## What it carries and what it does not

- It carries whom the patrol shoots at, and not how well: `fighter_hit_chance` and the protection factors are untouched and stay placeholders.
- It does not give the bombers return fire (deferred by 0027), or the escorts a choice of whom to fire at, or the patrol any warning or height. Those are the open question of the air plan (C3), looked at again once this rule is measured.
- It does not change the patrol's division among raids (0033).

## What was seen on scratch before it was written

Nothing of this rule was run. A hand estimate on the counts of the air tuning plan (Rabaul, 1,000 runs: patrol about 14 a raid, escort about 19, bombers about 24) at s = 0 sends about 6 of the patrol's 14 shots at the escorts and 8 at the bombers. With the fighter chance at about 0.08 that would give, over 11 raids, about 11 bombers, about 5 Zeros and about 9 F4Fs lost, each inside its record. That chance is not proposed here; it is the tuning round's.

## What it costs

- `crates/sim-core/src/airstrike.rs`: the split in `work_out_raid` and the breakdown factors.
- `crates/sim-data`: the figure, required, with its loader test.
- `data/reference/air_combat.toml` at 0. `data/reference/step1/air_combat.toml` at 1, with a line in its README.
- A scenario of its own, with a hand calculation:
  - the shots and chance from the breakdown at a working value;
  - a property over many seeds;
  - a pinned outcome;
  - the cases of the split: more escorts than patrol, fewer, none, two raids sharing a patrol, the Henderson raid in small, and a share larger than the raid.
- The proof that s = 1 is today: the logs of scenarios 05c, 06a, 13a, 13b and 16 on the frozen figure, byte for byte.
- The run baselines, pins and recorded games of every live air scenario with a patrol.
- `docs/rules/05` and the glossary in the same pull request.
