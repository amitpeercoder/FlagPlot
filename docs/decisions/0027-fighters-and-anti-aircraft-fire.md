# 0027: Fighters, anti-aircraft fire and aircraft losses

Status: confirmed by the designer, Nov 11, 2026, with the plan for scenario 5 (stage 5c). The plan settled the order (escort against combat air patrol, then anti-aircraft fire, then the attack, as the spec has it), that fighters and guns use the shots-times-chance form of decision 0014, and that a share of fighters is held back to defend. The details below that level are the architect's, and decisions 5, 8, 9 and 13 are marked for the complexity review.

## The gap

Decision 0026 lets a strike fly and hit with nobody opposing it and nobody lost. The spec's air combat step has three parts: escort against combat air patrol, anti-aircraft fire, the attack. This decision adds the first two and the losses they cause.

## Decisions

### Fighters

1. **An air group has a fourth mission: combat air patrol.** Only a fighter type may be given it. A fighter type attacks nothing on the surface.
2. **A group on combat air patrol has a share to keep overhead,** from 0 to 1, set by order. The default is 1. Of the aircraft it can fly (decision 0026, 14), the share, rounded down, stays over its own hex as the patrol. The rest are escorts.
3. **The patrol defends the hex the group is in.** It meets every enemy raid that attacks a task force there, whoever's ships they are, in a daylight pulse.

### Raids

4. **The strikes launched in one pulse from one place at one hex are one raid.** A place is a ship or a base. The raid's bombers are those strike groups' aircraft.
5. **The escorts at a place go with one raid from it,** if it is within their radius (decision 0030). They go with the raid with the most bombers, and of those the one whose first group has the lowest number. If no raid leaves the place, its escorts fly nothing.

### The fight in the air

6. **A raid that finds an enemy task force is met by that side's patrol over the hex.** Each fighter is one shot, hitting with the fighters' chance. A hit is an aircraft lost.
   - The escorts fire at the patrol: one shot each.
   - The patrol fires as many shots at the escorts as there are escorts, or as there are patrol fighters if those are fewer. Any patrol fighters left over fire at the bombers.
   So escorts draw the patrol off the bombers one for one, and a raid with no escort takes the whole patrol.

   *Nov 14, 2026: escorts draw off a share of the patrol one for one, by decision 0080, and a share of the patrol's shots at the bombers is worked before the attack, by decision 0087.*

7. **Then every ship afloat in the task force attacked fires its anti-aircraft guns at the bombers that are left.** A class has a number of anti-aircraft shots. Each hits with the guns' chance, and a hit is a bomber lost.

   *Nov 14, 2026: a share of the anti-aircraft shots is worked before the attack and the rest after it, by decision 0079.*

8. **Then the bombers that are left attack,** by decision 0026. A raid's groups attack in order of group number, each with what it has left.

   *Nov 14, 2026: the order of fire before and after the attack is as decisions 0079 and 0087 set it.*

9. **Hits on a raid's bombers, from fighters and then from guns, are taken from its groups in order of group number,** the lowest first, each until it has none left. Hits on a patrol or on escorts are taken from their groups the same way. Hits beyond the aircraft there are wasted.
10. **Hits are the whole part of shots times chance, plus one more with a chance equal to the fraction,** as everywhere else.

### Losses

11. **An aircraft lost is taken off its group's ready aircraft,** and an event says how many and to what. Nothing else can make the count fall yet.
12. **Everything in the air combat step is worked on the state at its start** (decision 0026, 8). Every raid meets the patrol as it stood at the start of the step, and losses are applied afterwards. A patrol cannot lose more aircraft than it had.

   *Nov 14, 2026: when more than one raid attacks forces of one side in one hex in a pulse, that side's patrol is shared among them, by decision 0033, item 7.*

13. **A raid that finds nothing meets nobody,** and a patrol over a hex where no raid attacks fires nothing.

### Random numbers

14. **Each roll has its own stream in the air combat subsystem:** the escorts' fire on the patrol is keyed on the first escort group and the task force attacked; the patrol's fire on the first group of the patrol and the raid's first strike group, the roll at the escorts first and then the roll at the bombers; the guns' one roll, with the shots of every ship added together, on the lowest-numbered ship afloat in the force and the raid's first strike group. A roll with no shots draws nothing and records nothing.

### How it is recorded

15. **Escorts are counted at the start of the air combat step,** and an event says how many each fighter group sends and with which raid. For each raid, in order of its first group, the log has the escorts sent, the fights, the guns and the aircraft lost. Then come every strike's finding and attack, and then the damage and reports, as in decision 0026.
16. **A strike group with no bomber left attacks nothing** and sends no report.

## Why

- **One for one** is the smallest rule that makes an escort worth sending and a patrol worth keeping. It ignores which fighter is better, and that is the next thing a real aircraft's figures will have to say.
- **Every ship's guns** make an escort worth having round a carrier, without a rule for who stands where.
- **The lowest group first** is a bookkeeping rule. It is unfair to the first squadron and it is simple. Sharing losses in proportion needs a rounding rule that would be no fairer.
- **The patrol as it stood at the start** keeps the result free of the order raids are taken in. It is generous to a defender struck by several raids in one pulse, whose patrol would in truth be worn down by the first.

## For the complexity review

- **Results hardly vary.** Hits are the whole part of the expected number or one more, so a raid facing 1.8 expected losses always loses 1 or 2, never none and never 5. That is easy to reason about and narrower than air combat was. The legibility test's reader raised it unprompted.

## Deferred

- How good one fighter type is against another, and how much punishment an aircraft takes: stage 5e, from facts.
- Radar warning, and a patrol caught on deck.
- Fighters strafing ships. Bombers shooting back.
- Aircraft damaged and not lost; operational losses; pilots: scenario 6. *(Nov 14, 2026: built: damaged aircraft and operational losses by decision 0035, pilots by decision 0038)*
- Where aircraft land when their carrier is sunk or its deck is closed while they are away. Today they are lost only with the ship (decision 0025). *(Nov 14, 2026: built by decision 0059)*
- Anti-aircraft ammunition.

## Rule card

| | |
| --- | --- |
| Reads | Groups on combat air patrol: aircraft, share, position, their ship's flight deck; the strikes in flight; the ships afloat in the task force attacked and their classes' anti-aircraft shots; the air combat figures |
| Writes | Ready aircraft of bomber, escort and patrol groups |
| Events | `AirFight`, `AntiAircraftFire`, `AircraftLost`, between `StrikeLaunched` and `AirAttack` |
| Randomness | One stream per roll, keyed as in decision 14 |
