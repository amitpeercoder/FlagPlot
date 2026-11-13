# 0079: Anti-aircraft fire and the attack

Status: decided by the architect on the designer's delegation, Nov 13, 2026 ("Go ahead and decide all matters that need decisions"; "Yes, it decides"). The hand calculation's cases were worked and checked by the architect before any code. Built on Nov 13, 2026; see "As built".

## The gap

In a raid the guns of the target fire at the bombers the patrol left, and their hits come off before the attack: "Then every ship afloat in the task force attacked fires its anti-aircraft guns at the bombers that are left ... Then the bombers that are left attack" (decision 0027, items 7 and 8; `docs/rules/05`, "The order of the air combat step"). A bomber the guns shoot down never drops.

At the Eastern Solomons that is the wrong way round, and it hides a miss.

- The scenario launches all 54 D3As of Shokaku and Zuikaku at once, where the record's first wave was 27, with 10 fighters (`engagement_outcomes` 290, 1159, 1164). The second wave of 27 launched an hour later and found nothing (409).
- Over 1,000 runs the patrol and the guns kill about half the 54 before the attack. About 30 attack, so the measure of aircraft attacking (35 against 37) and the bomb hits on Enterprise (4 against 3) agree.
- On a scratch copy with only the record's first wave (the air tuning plan's scratch, variant `es_firstwave`), the attackers fall to a median of 6 and the bomb hits to 0. Both miss. Japanese losses still agree (26 against 29 to 32).
- The record has the first wave lose 22 to 25 of 37 (291, 1165, 1166) and still put three bombs into Enterprise (181 to 184).

No chance can give both of those. A defence strong enough to kill two thirds of the wave stops the bombs if it kills them first.

## The record of when the guns fired

"ev" is `data/oob/engagement_events.csv`.

- **The guns opened on the dive.** ev 222 (ONI, high, preferred): "radar plot reported at 1709 that 'the enemy planes are directly overhead now!'. The radar lost track at minimum range; the first plane was sighted after it had begun its dive, at 12,000 feet by a 20-mm battery officer."
- **The dives ran four minutes.** ev 223 (Enterprise's action report, high, preferred): "intervals of about seven seconds for a period of about four minutes ... About 30 bombers (20 to 40). Releases at 1,500 to 2,000 feet."
- **The research note's reading.** `docs/research/oob/anti-aircraft.md` (line 14) reads the same account as "Fire-control radar could not track attackers, so fire opened only after dives began." These words are the note's paraphrase, not a quotation of ONI. The row that carries the fact is ev 222.
- **The fighters met the strike before it dived.** ev 218 and 219: first contact at 33 miles, 1655; the dives began at 1711.

So the record has the patrol's work done before the attack and the guns' work done during it. A bomber hit by the guns in its dive had mostly already aimed. No row says how many of the first wave's losses fell to guns and how many to fighters (no split is found: `docs/research/oob/eastern-solomons.md`; outcomes 401 says "a fair number" to North Carolina's guns). No row gives a share of gun hits that came before release.

## Options considered

1. **Leave it.** Rejected: the tuning would then have to make the guns weak to let the bombs through, and the record has them deadly.
2. **Guns after the attack, always.** The least rule. But it is a level bomber's raid on an airfield as much as a dive bomber's on a carrier, and the guns of Henderson Field fired at G4Ms that held their course over the field (the 90 mm "kept the attacking planes at a reasonably high altitude", outcomes 497).
3. **One figure for the share of the guns' fire worked before the attack** (this decision). At 1 it is step 1's rule; at 0 every gun shot is worked after the bombs have fallen. It is one figure for ships and bases, for both sides.

## Decisions

1. **A share of the guns' fire is worked before the attack, the rest after it.** The share is `air_combat.toml: anti_aircraft_before_attack_share`, from 0 to 1.
2. **The shots.** The guns' shots are as now: the sum of the class `anti_aircraft` of the ships afloat in the force attacked, or the base's `anti_aircraft`, a whole number n. The shots before the attack are the share times n, cut to a whole number toward zero: k = floor(share × n), in one fixed-point multiplication. The shots after are n − k. At a share of 1 they are n and 0; at 0 they are 0 and n; at 0.5 with 20 guns, 10 and 10; at 0.33 with 20, 6 and 14.
3. **One roll, split.** The guns still fire only if the raid has bombers left after the patrol, as now. Their shots are rolled from the one stream they have now (keyed on the lowest ship afloat in the force, or the base, and the raid's first strike group), one draw for each whole shot, in order. The first k draws are the fire before the attack and the next n − k the fire after. So the number and order of the guns' draws are the same at every share, and on any seed the guns' total hits are the same at every share.
4. **Before the attack.** The hits of the first k draws come off the bombers left by the patrol, lowest group first, as now. What is left attacks.
5. **After the attack.** The hits of the other n − k draws come off the bombers that are left after the fire before the attack, lowest group first, each group down to 0. These bombers have attacked already: their shots, hits and damage stand. Hits beyond the bombers there are wasted. Because both parts come off one pool in group order, each group loses in all exactly what it would lose to one roll of n, at every share.
6. **Which aircraft can be lost after the attack.** Only bombers. Escorts and the patrol are not fired at by the guns, as now.
7. **The events.**
   - The fire before the attack records `AntiAircraftFire` (or `BaseAntiAircraftFire`) as now, in its place in step 3 of the air combat step, with its `AircraftLost` events and the `PilotsLost` that follow.
   - The fire after the attack is recorded in a new step between step 4 (what each strike found) and step 5 (the damage lands), raid by raid in order of the first strike group. It records the same event with a new field, `after_attack: true`, then its `AircraftLost` and `PilotsLost`.
   - A part with no shots records nothing. So at a share of 1 the log is the log of step 1, byte for byte, and at a share of 0 there is one gun event a raid, after the attack.
   - The field is false for fire before the attack and is not written when it is false, so a log, a save or a replay file made at a share of 1 is unchanged. The event's text adds "after the attack" when it is true.
8. **Causes.**
   - The fire before the attack cites the raid's launch, as now.
   - The fire after the attack cites the raid's launch and the `AirAttack` or `BaseAttacked` events of the raid's groups; where none attacked (the fire before took every bomber), it cites the launch alone.
   - The losses cite their fire.
   - An attack cites only the losses before it, as now: losses after the attack are recorded after it and are not its causes.
9. **The breakdown.** Each part's breakdown lists, besides the shots, the chance and the protection factors as now, the guns' shots in all (n) and the share. When the share is 1 the breakdown is as now.
10. **The defender's claims are unchanged at every share.** A side's false claims are drawn for each whole missed shot, in the order its firing events were noted (decision 0056). The fire before and the fire after are noted in that order and together miss exactly what the one roll missed. So the defending side's claim draws, and its claims, are the same at every share; the claim cites both events. The raiding side's claim of aircraft shot down is the same too. Its claim of hits on ships or of aircraft on the ground rests on the bombs it dropped, which the share changes, as any change in the bombers left does.
11. **Pilots, wear and the objectives.**
    - A bomber shot down after the attack takes its pilot, as any bomber lost.
    - It is removed from the sortie before wear, as any aircraft shot down.
    - It counts for the enemy's `aircraft_destroyed`, as now.
12. **The figure.** In the frozen step 1 figures (`data/reference/step1/air_combat.toml`) it is 1, which leaves step 1 as it was. In the live figures it is 0, a placeholder. Its one support is ev 222 (and ev 223), which have the guns opening on the dive, after the fighters had met the strike. No source sizes it: no row gives the share of gun hits that came before release, for ships or for a field. The live value 0 rests on one ship's account of one attack by dive bombers: the first plane was sighted after it had begun its dive. No row speaks for a torpedo attack, or for a base's guns against level bombers at height, where fire before release is the likelier case. So the figure stays a placeholder, which the anti-aircraft round may move anywhere from 0 to 1 on evidence. It is not made to differ by the kind of attack now.
13. **One rule for both sides, by ships' guns and by a base's.**

### The words for the rulebook

`docs/rules/05`, "Anti-aircraft fire", in place of its first two paragraphs' order:

> The guns of the target fire at the bombers that are left (the strike groups' aircraft after the patrol's hits). Skipped if there are no bombers left or the guns have no shots. Their shots, n, are split by `air_combat.toml: anti_aircraft_before_attack_share`: the first k = n times the share, cut to a whole number, are worked before the attack, and the other n − k after it. Both parts are rolled in order from the one stream, one draw for each shot. The hits of the fire before the attack come off the bombers before they attack; the hits of the fire after come off the bombers that are left, after their attack is worked out and whatever it did stands. A part with no shots records nothing. [0027, 0029, 0033, 0034, 0079]

And "The order of the air combat step" gains, between steps 4 and 5: "4a. For each raid in order of its first strike group, record the guns' fire after the attack, then its losses."

## Rule card

| | |
| --- | --- |
| Reads | The guns' shots of the force or base attacked (as now); the bombers after the patrol's hits (as now); the share |
| Writes | The bombers lost to the guns, in two parts: before the attack (as now) and after it, from the same pool |

- No state, and no new version of the state hash.
- No new stream and no new draw. The guns' stream and the number and order of its draws are unchanged at every share. Only the attack's own stream draws more when more bombers attack (one draw a shot, then per hit), as it does for any change in the bombers left.
- One figure, and one event field, `after_attack`, written only when true.
- `side_events.rs`: a side sees its own guns' fire, cut to the shots and without the raid's group, as now. It also sees whether the fire was after the attack: it is its own guns' fire, and it tells the side nothing of the enemy that the cut view does not already show. The attacker sees its own `AircraftLost` and `PilotsLost`, as now.

## What it carries and what it does not

- It carries the order of the guns and the bombs, and nothing of how deadly the guns are: `anti_aircraft_hit_chance` is not touched, and it stays a placeholder.
- It does not split the patrol's work. The fighters met the strike before it dived (ev 218, 219), so the patrol's hits stay before the attack.
- It does not give the guns a second target, or the escorts any exposure to the guns.
- It does not mend the doubled Japanese strike. That is decision 0081 (the two waves set from the record). This rule is what lets the record's single wave both lose two thirds of itself and hit Enterprise three times.
- At Rabaul it changes almost nothing: Henderson's guns kill about 4.4 bombers a run of 263 sorties. At a share of 0 those few bombers bomb first.

## What was seen on scratch before it was written

Nothing of this rule was run: it was written from the air tuning plan's counts (scratch of Nov 13, 2026).

- Over 1,000 runs of scenario 5, the guns over TF16 fire 123 shots at each of the two Japanese raids and kill about 7.5 bombers a run; the patrol kills about 7.9.
- With the record's first wave alone and the guns' hits after the attack, a hand estimate gives about 14 bombers attacking, about 2.8 hits and about 25 to 28 Japanese lost, against a record of 27 attacking, 3 hits and 22 to 25 lost.
- The measure of aircraft attacking still counts aircraft after the patrol. The architect has ruled on it with decision 0081.

## What it costs

- `crates/sim-core/src/airstrike.rs`: the split roll in `work_out_raid`, the new step in `resolve_strikes`, the losses after the attack, and the causes.
- `crates/sim-core/src/events.rs`: the field and the text. `side_events.rs`: the field in the cut view.
- `crates/sim-data`: the figure, required in `air_combat.toml`, with its loader test.
- `data/reference/air_combat.toml` at 0. `data/reference/step1/air_combat.toml` at 1, with a line in its README.
- A scenario of its own, with a hand calculation (`docs/step2-spec.md`, "Tests while figures move", 7):
  - the chance from the breakdown;
  - a property over many seeds, that the guns' hits and each group's losses are equal at every share while the attack's shots differ by exactly the hits before the attack;
  - a pinned outcome.
- The run baselines, pins and recorded games of every scenario with a raid that meets guns. These are all live air scenarios.
- No hand calculation on the frozen figures moves.
- `docs/rules/05` and the glossary in the same pull request.


## Note, Nov 13, 2026

The research of that day (pull request 214) corrects two statements above.

- "What it carries" says the patrol's hits stay before the attack. Simpler's study of 38 fighters at the Eastern Solomons (`engagement_outcomes` 3979 to 3984) has 4 first engaging before the push-over, 7 at it or in the dive and 7 in the retirement, so much of the patrol's fire fell during and after the attack. That is carried by a separate rule on the patrol, decision 0087, which reuses this decision's split, and not by this one.
- "No row gives a share of gun hits that came before release" still holds for kills. Events 1242 to 1245 confirm that the guns opened as the dives began (no 5-inch fire at altitude). Row 3955 (Coral Sea, low, unconfirmed) says gunfire before release turned bombers back and made them drop sooner. That is an effect on aim, not a kill before release, and it is not carried by this share.
- The live value 0 stands.

## As built

**The code.**
- `crates/sim-core/src/airstrike.rs` builds the rule in two places.
  - `work_out_raid` rolls the guns' first k shots and then the other n − k from the one stream. The second part's hits are taken from the bombers only after the attack outcomes are fixed.
  - `resolve_strikes` records the fire after the attack in step 4a, with its losses and its note in the raid's claim tally.
- `events.rs` and `side_events.rs` carry `after_attack`.
- `sim-core` has no serde, so "written only when true" is done where files are written: the export layer writes the field only when it is true (`sim-cli` export, umpire and side events alike).
- The briefing (`sim-agent`) lists a fire after the attack as a guns line of its own, unmarked. Whether it should say so is left for a wording change.
- The figure is required in `air_combat.toml`, checked to lie from 0 to 1. It is 0 live and 1 in the frozen copy.
- No state, no hash layout, no stream and no `Draw` value changed.

**The proof that a share of 1 is step 1.**
- The event logs of 05c, 06a, 13a, 13b and 16 on the frozen figures are byte for byte those of the binary before the rule. So are those of 05 and 06, run on a frozen copy with the new key taken out for the old binary.
- Every hand calculation on the frozen figures passes untouched.

**Its scenario.**
- `19a_guns_and_the_attack`, with the hand calculation of the cases above. Every number matched with no difference.
- Properties over seeds 1 to 1,000 at shares 1, 0.5, 0.33 and 0. The guns' hits, each group's losses, the defender's claims and the raider's aircraft claimed are equal at every share, and the attack-shot totals are inside their ranges.
- Pinned outcomes. The scenario's own seed, 1942, has the guns miss all 20 shots, so seed 10 is pinned beside it as the first seed on which the share moves hits across the attack.
- The defender's claims are equal at every share, and so is the raider's claim of aircraft shot down. The raider's claim of hits rests on the bombs it dropped, which differ by share; decision 10 was corrected to say so when the build found it.

**Before and after, 1,000 runs each, label `d0079`.**
- The reports are `docs/validation/eastern-solomons-d0079.md` and `rabaul-guadalcanal-air-d0079.md`. They were made on the code merged with main after decisions 0081 (the Eastern Solomons orders from the record: the first wave of 27 dive bombers and 10 fighters, the second wave on its own), 0082 (the Rabaul raid days from the forms) and 0083 (torpedo tubes by side, state version 7), against the measures as they stood then.
- "Before" is main at that merge (b82d31b), run with the same seeds.
- An earlier pair of reports, made on the scenario before 0081, was replaced by these. On the doubled strike it turned two agreements into misses, which was the scenario's fault, not this rule's.

| Eastern Solomons | Record | Before: 5th to 95th, median | After | Verdict before / after |
| --- | --- | --- | --- | --- |
| Bomb hits on Enterprise | 3 | 0 to 2, 0 (29 runs hold 3) | 0 to 5, 1 (123 runs hold 3) | miss / agrees |
| Japanese aircraft attacking TF16 and TF11 | 37 | 0 to 37, 37 | 0 to 37, 37 | agrees / agrees |
| Japanese carrier aircraft lost to enemy action | 29 to 32 | 2 to 35, 26 | 2 to 35, 26 | agrees / agrees |
| Japanese carrier aircraft lost to all causes | 35 to 54 | 4 to 58, 30 | 4 to 58, 29 | agrees / agrees |
| US aircraft lost to enemy action | 13 to 14 | 1 to 9, 4 | 1 to 9, 4 | miss / miss |

- Judged 14: before, agree 9, at the edge 1, miss 4; after, agree 10, at the edge 1, miss 3.
- The guns' hits a run are the same before and after: 7.51 on Japanese raids and 0.36 on American ones.
- The Japanese bombers that reach the ships now bomb before the guns' hits come off them. Bombs dropped at ships rise from 2.6 to 8.5 a run, and hits from 0.5 to 1.7. This is what the rule is for: the record's single wave lost two thirds of itself and still hit Enterprise three times.
- The measure of aircraft attacking counts, since decision 0081, the aircraft that take part as launched, before the patrol and the guns fire, so the rule cannot move it.

At Rabaul against Guadalcanal:
- Judged 10: before, agree 3, at the edge 4, miss 3; after, agree 3, at the edge 3, miss 4.
- The guns' hits are the same (3.64 a run), and the bombers they hit now bomb first. Bombs dropped rise from 212 to 216 a run, and aircraft destroyed on the ground from a median of 12 to 13.
- The one verdict that moves: F4Fs ready on day 21 keeps its spread and median (2 to 12, 6). Runs holding the record of 11 go from 51 to 44 of 1,000, so it goes from agreeing at the edge to a miss on the test of one run in twenty. That is the margin of the test and a few more aircraft hit on the ground, not a new cause.
