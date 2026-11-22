# 0211: A strike is spread over the ships of the kind it wants

Status: a plan by the architect, written on Nov 22, 2026, on the designer's word after 1,000 games of Midway: "Do both", of spreading a strike and of fire that can destroy a carrier (decision 0212). It was read by a reviewer who had not seen it written, who built a throwaway of both rules and played 200 games of each scenario for play; the designer then ruled that the rules run in every battle, with values that may differ by side and date where doctrine differed (decision 0213), and research on that doctrine was gathered. This is the plan rewritten on all three. It is to be reviewed again and measured before anything is built.

Summary: Today every strike group that attacks a force in a pulse attacks the same ship: the first of the kind its list ranks highest, and among several of that kind the lowest-numbered. So four carriers are attacked one at a time and aircraft are spent on a ship already lost. This lets each carrier's or base's raid take the next ship of the kind it wants, leaving for last a ship that already looks lost. Whether a raid does take the next ship, or falls on the one the raid before it attacked, is a chance that differs by side, as the two navies' practice did. A raid's groups stay together. It runs in every battle.

Standing: Planned on Nov 22, 2026; reviewed and rewritten the same day; to be reviewed again and measured.

## Why

- **The rule today** (decision 0026, item 9, kept by decision 0063; rules chapter 5; `airstrike/raids.rs`): a group attacks the best-ranked ship afloat in the force its raid found, the lowest ship number on a tie. Decision 0026 said: "It wastes aircraft on a ship already sinking and never touches the escorts. Spreading a strike is deferred."
- **What it does** (decision 0198, rules 107 and 108): in every pulse with two or more groups attacking a force they attack one ship. At Midway Kaga is never attacked while Akagi is afloat; at the Coral Sea Lexington is all but spared while Yorktown is afloat, and in the record Lexington was the one lost.
- **What the rows hold** (`docs/research/carrier-doctrine-by-navy-and-date.md`):
  - **Each carrier's aircraft attacked as a unit.** American groups joined no other group (engagement events 15314 to 15322); the Japanese divided by carrier unit.
  - **The Japanese divided a strike among the carriers found, by plan.** At Pearl Harbor each attack unit had its target in the order (15331 to 15337, an order reconstructed after the war). At the Coral Sea on 8 May each carrier's aircraft took one American carrier, 26 and 27 aircraft (15342, 15343). In June 1944 the order was each squadron its own target (15359, 15360).
  - **American groups fell on what their leaders chose and as they arrived.** At the Coral Sea both groups attacked Shoho on 7 May and both Shokaku on 8 May, with Zuikaku in the same force untouched (15320 to 15322, 2509 to 2511). At Midway squadrons of two carriers fell on three (15314 to 15318). The doctrine of 1941 is known here only at second hand and says three things: the farthest target first, divide equally, or by what the groups ahead have attacked (15301 to 15303, low).
  - **That is two or three cases a side:** enough to say which way the difference runs and not enough to size it.
- **What the reviewer's 200 games showed of the first plan,** which spread by group: all the dive bombers went at one carrier and all the torpedo planes at another, which no row has; and a ship whose deck was out or that was already sinking was still the first attacked.

## The rules

1. **The rule runs in every battle,** on the figures below. In the frozen figures the chance of rule 5 is nothing for both sides, so a group there chooses its ship as today and the hand calculations stand.
2. **A raid is the unit.** A raid is the strikes of one pulse from one ship or base at one hex, as now. All the groups of a raid that want one kind of ship attack the same ship of that kind.
3. **The raids that attack one force in a pulse are taken in turn,** in the order of their first group's number.
4. **The ships of a kind are put in order, the ones that look lost last.** For the kind a raid's first group ranks best among the force's ships afloat when the pulse's air combat begins: first the ships that do not look lost, by ship number; then those that do, by ship number. A ship looks lost when her flight deck is out, where she has one; or her flotation damage is at or above `crippled_flotation`; or her fire damage is at or above `claimed_sunk_fire`. Each is a figure or a state the game has, read as the choice of ship is read today.
5. **The first raid takes the first ship of the order. Each raid after it makes one draw:** under its side's `strike_divides_chance` it takes the ship after the one the raid before it took, going round to the first after the last; otherwise it takes the same ship as the raid before it.
6. **A group of the raid that wants another kind** attacks the first ship of that kind by the same order, as it would today.
7. **A group is not divided,** and no aircraft goes at a ship of a kind its list ranks lower while one it ranks higher is afloat, as today.
8. **Nothing else of the attack changes.** The force a raid attacks is chosen as today. The fighters, the guns, the shots and their draws are as they are, keyed on the group and the force. What the ship attacked makes of the attackers is drawn for that ship, as today, so which ship's lookouts draw does follow the choice.

## Figures

9. **`strike_divides_chance`, in the air combat figures, by side** (decision 0213). Placeholders, each with what it rests on:

| Side and date | Value | Rests on |
| --- | --- | --- |
| Plain value | 0.5 | no row; the middle |
| Japanese, from the start | 1 | a judgement from two cases and a later order: each carrier's aircraft their own carrier at the Coral Sea (15342, 15343), targets by unit in the order for Pearl Harbor (15331 to 15337), each squadron its own target in June 1944 (15359, 15360) |
| Allied, from the start | 0.5 | a judgement from two cases: both groups on one carrier at the Coral Sea on 7 and on 8 May (15320 to 15322), three carriers attacked at Midway (15314 to 15318). One fell together and one spread |

No later value is given for the Allied side: that American strikes were coordinated over several carriers from 1944 is a lead and not a row.

## What a side learns by it

Nothing new. The attacking side is told of its attack without the ship, as now.

## Rule card

Reads: the raids attacking a force in a pulse, each first group's target list, the force's ships afloat with their flight deck, flotation and fire, `crippled_flotation`, `claimed_sunk_fire`, the side's `strike_divides_chance`. Writes: nothing; the ship each raid attacks. Draws: one new value, for a raid after the first at a force, on a stream of the raid's first group and the force.

## Games on file

It runs everywhere, so games move wherever two raids attack a force with two ships of the wanted kind, or a ship that looks lost is afloat beside one that does not.

- **The frozen figures and the frozen scenarios do not move,** nor the hand calculations on them.
- **Before it is built it is measured,** on a throwaway outside the repository, with decision 0212: 1,000 games of each scenario for play and of every validation file and proof that has air attacks, before and after, each rule alone and both together. What is reported: who is ahead; ships sunk by ship; ships attacked in a pulse; hits on a ship already sunk or that looks lost; and for each validation measure and each proof's condition, its value before and after and whether its verdict changes.
- **The records of closed proofs are not altered.** A proof whose condition moves is reported to the designer as it now reads; it is his to say whether a proof is run again.
- **Recorded games and saves** are remade as at any change of a rule.

## What it does not do

- **It does not divide a group or a raid among ships,** nor send aircraft at the escorts while a ship of the wanted kind is afloat. At the Coral Sea about one Japanese aircraft in twenty went at an escort, and at Santa Cruz groups that found no carrier attacked cruisers.
- **It does not carry the turn from one pulse to the next.**
- **It does not let a commander say how a strike is to be divided.**
- **It does not change which force a raid attacks** where two forces lie in one hex.
- **It does not give the Allied side a later value.**

## What it costs

- **One figure by side, one draw, and the choice of ship made for a raid with the raids before it in view,** where today each group chooses alone.
- **Carriers of a force are attacked together,** so each is hit less and, as the rules of damage stand, sunk less often: the reviewer's 200 games of Midway had Akagi sunk in 4 where she was in 183, and the smaller carriers in her place. It is planned with decision 0212 for that reason.
- **Rulebook:** chapter 5 (the ship a group attacks, "One ship" among what is left out); chapter 1's list of draws; the glossary's "Raid".
- **A scenario of its own and its hand calculation,** written by the architect when the measuring is read and before any code is delegated.

## Questions for the designer

None before the measuring. What it shows of each battle is put to him before anything is built.

## How the plan changed

- **It spread by group, in the order of the groups' numbers.** A carrier's dive bombers and torpedo planes would never have attacked the same ship. It now spreads by raid, which is a carrier's or a base's aircraft together, as the rows have both navies attack.
- **"Looks lost" was fire alone,** which today is rare: a carrier with her deck out or sinking was still attacked first. It now takes the deck and flotation too.
- **It was the same for both sides.** The Japanese divided by plan and the Americans did not always; the chance of taking the next ship is now a figure by side.
- **It ran only in the scenarios for play.** By the designer's ruling it runs in every battle.
- **It said the same shots follow whatever ship is chosen and nothing else moves.** The ship attacked draws what she makes of the attackers on her own stream; rule 8 says so.
- **It had no scenario of its own.** It will.

## Measured and reviewed again, Nov 22, 2026

The plan above was read by a second reviewer and both rules were played on a throwaway, 1,000 games of each scenario for play under each rule alone and both, and the validation batches and the proofs' batches before and with both. What follows is the plan as it now stands; where it differs from the rules above, this holds.

### What the games showed of this rule alone

| | Before | This rule alone | Both rules |
| --- | --- | --- | --- |
| Midway, who is ahead of 1,000 | Japanese 867 | Japanese 947 | Japanese 521, Allied 479 |
| Coral Sea | Allied 574 | Allied 654 | Allied 622 |
| Five days | Allied 615 | Allied 682 | Allied 812 |

- **It does what it is for.** Where two raids or more attack the Japanese force at Midway, one ship was attacked in every such pulse before; with the rule, two or more in 3,070 of 3,842. Hits on a ship that already looked lost fall from 7.6 a game to 3.8 at Midway and from 3.4 to 1.3 at the Coral Sea.
- **Alone it sinks fewer carriers,** as the plan said it would: Akagi from 891 games to 135. It is not built without decision 0212.
- **No validation verdict and no proof condition read from a batch changes** with both rules.

### What the review found, and the rules as they now stand

- **The figure by side is withdrawn.** It rested on one Japanese case of dividing among carriers found and one American case each way, Pearl Harbor being ships at anchor and the order of 1944 not 1942. One rule serves both sides, with no figure and no chance of falling on the same ship.
- **The first raid's ship was an accident of numbering.** At Midway the same carrier drew the first raid of every pulse.

1. **The raids that attack one force in a pulse and want one kind of ship are taken in turn,** in the order of their first group's number. Raids that want another kind take their own turns apart.
2. **The first of them draws its ship:** one draw, each ship of the kind that does not look lost as likely as another. Where every ship of the kind looks lost, among those.
3. **Each raid after it takes the next ship of the kind that does not look lost,** by ship number, going round among those that do not look lost. A ship that looks lost is attacked only where none of the kind is otherwise.
4. **"Looks lost" is read once, at the start of the pulse's air combat,** as every attack is worked on the state at its start. A ship sunk earlier in the same pulse is therefore still attacked; that is not mended here.
5. **The rule is off where its switch is absent.** The live air combat figures gain `strike_is_spread`, and the frozen copy does not: there a group chooses its ship as today, and every frozen test stands byte for byte. The key is named in the frozen-figures test's list of keys the frozen copy is without.
6. **The draw is recorded:** the raid's attack names the ships it could have drawn and the one drawn, in its breakdown, and the ship a later raid took by turn. A side sees of it what it sees of its own attack today.
7. **Rules 2, 6, 7 and 8 above stand.** Rules 1, 3, 4, 5 and 9 above are replaced by these.

Rule card: reads the raids at a force, each first group's target list, the force's ships afloat with deck, flotation and fire, `crippled_flotation`, `claimed_sunk_fire`. Writes nothing. Draws: one new value, for the first raid of a kind at a force, on a stream of that raid's first group and the force.

It is measured again with decision 0212 as amended before anything is built.
