# 0033: Three changes to how hits are worked

Status: confirmed by the designer, Nov 11, 2026, from the complexity review (`docs/complexity-review-after-scenario-5.md`, findings D1, D2 and E). The figures and the rounding rule are the architect's.

## The gap

The review found three rules whose results nobody would defend: results that hardly vary, a patrol that meets every raid whole, and a light shell as likely as a heavy one to wreck a ship's engines. The last is one reason the Savo Island scenario comes out wrong (decision 0024).

## Decisions

### A system hit depends on the damage the hit does

1. **A hit also knocks a system down one level with a chance of its damage points times one figure, 1 at most.** Damage points are what the hit already does: 100 x the weapon's damage / the target's toughness. This replaces three flat chances, for a gun, a torpedo and a bomb. Which system is hit is chosen as before.

   *Nov 14, 2026: every hit makes one system draw, even when its chance is 0, so the number of draws is fixed by the hits (`docs/rules/differences.md`, row 4).*

2. **The figure is 0.06 for each point,** a placeholder. It is chosen so that the placeholder cruiser's gun hit on its like, 5 points, keeps the 0.3 it had.

   | Hit | Points | Chance before | Chance now |
   | --- | --- | --- | --- |
   | Placeholder cruiser's gun on a cruiser | 5 | 0.3 | 0.3 |
   | Placeholder destroyer's gun on a cruiser | 1.25 | 0.3 | 0.075 |
   | Placeholder cruiser's gun on a destroyer | 20 | 0.3 | 1 |
   | Placeholder torpedo on a cruiser | 50 | 0.5 | 1 |
   | Placeholder bomb on a carrier | 10 | 0.3 | 0.6 |
   | A US 5-inch hit on a Japanese heavy cruiser at Savo | 1.2 | 0.3 | 0.07 |
   | A Japanese 8-inch hit on a US heavy cruiser at Savo | 5.3 | 0.3 | 0.32 |

### Each shot is rolled

3. **Hits are no longer the whole part of the expected number, with one roll for the fraction** (decision 0014, 9). Each shot is rolled on its own at the chance to hit, and the hits are the shots that hit. The expected number is the same. The spread is what chance gives.
4. **A part of a shot is one more roll at the chance times the part.** A battery firing 67.5 shots rolls 67 at the chance and one at half of it.
5. **It applies to every roll of that form:** a ship's gun and torpedo salvoes, a strike's bombs and torpedoes, fighters' fire and anti-aircraft fire. The event still gives the shots, the chance, the expected number and the hits.
6. **The rolls of one salvo come one after another from the stream the salvo already had,** before the draws for where its hits land.

### A patrol divides among the raids on its hex

7. **When more than one raid attacks forces of one side in one hex in a pulse, that side's patrol over the hex is shared among them** in proportion to each raid's aircraft, bombers and escorts together. A raid's share is the patrol times its aircraft over all the raids' aircraft, rounded down. Fighters left over go one each to the raids in order of size, the largest first, the lower first group on a tie. This replaces decision 0027, 12, in which every raid met the whole patrol.

   *Nov 14, 2026: a raid's share of the patrol fires as decisions 0080 and 0087 set it (item 8).*

8. **Each raid then fights its share** as decision 0027 has it: its escorts fire at the patrol, its share of the patrol fires at the escorts one for one, and what is left of the share fires at the bombers.

   *Nov 14, 2026: the escorts draw off a share of the patrol one for one, by decision 0080, and a share of the patrol's shots at the bombers is worked before the attack, by decision 0087.*

9. **With one raid nothing changes,** and the order the raids are taken in still does not matter: the shares are worked from the state at the start of the step.
10. **Losses to the patrol are added up and taken from its groups, the lowest number first,** as before.

## Why

- **By damage** is one mechanism where there were three figures, and it is the mechanism the game already has: a hit's weight against the size of what it hits. A destroyer's guns stop wrecking a cruiser's engines, and a torpedo does what a torpedo did.
- **Each shot** is the rule a player would guess from "120 shots at 0.045". Step 2 compares spreads with history, and the old rule had almost no spread to compare.
- **In proportion** keeps the result free of the order of raids, which is why decision 0027 had every raid meet the whole patrol, without letting 54 fighters fight twice.

## What it costs

- A hand calculation can no longer say "3 or 4 hits". It says the shots, the chance and the expected number, which are exact, and a band over a thousand runs for the hits.
- A torpedo hit always damages a system, four times in five the engines. That is harsher than before and nearer what a torpedo did.
- Every pinned output of every scenario with a fight changes.

## What the runs show

Over seeds 1 to 1000, as each change went in. Savo Island has no aircraft, so the last change does not touch it.

| | Before | System hits by damage | Each shot rolled | Patrol divided |
| --- | --- | --- | --- | --- |
| Savo Island: Japanese ships sunk, in all | 1,694 | 1,360 | 1,685 | 1,685 |
| Savo Island: Allied ships sunk, in all | | | 2,638 | 2,638 |
| Eastern Solomons: runs in which Ryujo is sunk | 304 | | 252 | 265 |
| Eastern Solomons: American carriers sunk | 0 | | 0 | 0 |
| Eastern Solomons: runs in which Enterprise ends crippled | | | | 9 |
| Eastern Solomons: bomb hits on Enterprise a run | | | 1.28 | 2.95 |
| Eastern Solomons: aircraft lost a run, Allied and Japanese | | | 2.27 and 30.35 | 1.88 and 22.55 |

The Japanese attack in 518 runs, so in a run with an attack Enterprise takes 5.7 hits on average. On the day she took three. In all 1,000 runs every raid that found its target came with its partner from the other carrier, so the scenario never shows a raid alone; that case is covered by a unit test.

Rolling each shot showed three things the old rule had hidden. None is a new rule; each is now written in the hand calculation of the scenario where it was found.

- **A carrier can be sunk in one attack.** In scenario 5b carrier B is sunk in 13 runs of 1,000, in 5c in 1, and in 5d a carrier is sunk on the first day in 7. A later strike then attacks the lower-numbered cruiser afloat, as decision 0026 has it.
- **A raid can lose every bomber.** It then attacks nothing and records neither an attack nor a strike that found nothing, so the contact it flew at stays as it was.
- **A force can be under its break-off level in its second fight of a pulse.** The level is counted over the ships a force has when a fight begins (decision 0014). A ship sunk in the first fight has left the force, so a force that broke off after losing ships outright may not break off at once from the next enemy. It still avoids action (decision 0022), and the fight still ends with the first round in which it is fired on.

## Rule card

| | |
| --- | --- |
| Reads | A hit's damage points; the shots and chance of a salvo; the raids attacking a hex and their aircraft; the patrol over the hex |
| Writes | Nothing new: the hits of the existing rolls, and the systems they knock down |
| Events | None new |
| Randomness | One draw for each shot, in place of one for each salvo |
