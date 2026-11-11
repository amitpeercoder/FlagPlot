# Complexity review after scenario 5

Written by the architect on Nov 11, 2026, as the spec and `CLAUDE.md` require: "It is the first scenario where search, strike decisions, air combat and ship damage meet. Every rule's inputs and outputs are read together before scenario 6 starts." Anything nobody can explain is to be cut or merged.

The material was gathered from all 31 decision records, the module notes of the simulation and the four figure files. Each finding below was checked against the code or the record it names.

## Where the rules stand

- **17 records carry a rule.** 14 are scenario inputs, tooling or data.
- **83 tunable figures** in four files, every one a placeholder: 28 for search and contacts, 36 for surface combat, 12 for air combat, 7 scaling constants.
- **Every scenario from 3 on passed its legibility test.** A cheap model, given one side's log and a page of rules, explained the outcome and predicted the next step. The same few things confused it every time, and they are in the findings.

## What is working and should be left alone

- **One hit mechanism.** Guns, torpedoes, bombs, fighters and anti-aircraft fire all work as shots times a chance. One damage path serves gunfire and bombs.
- **One degraded factor.** A damaged system halves what it does, whatever it is: speed, guns, tubes, fire control, the flight deck.
- **Belief has one door.** Every change to what a side believes is a report, including a strike that finds nothing.
- **A radius is one figure doing three jobs:** how far a type searches, strikes and escorts. That it is one figure is why the Eastern Solomons scenario came out in the shape of the day.
- **Position has one answer** for a force, and a group aboard a ship takes it from the ship.

## Findings

### A. Things that do nothing: cut

| # | What | Evidence | Proposal |
| --- | --- | --- | --- |
| A1 | **A sighted force "notices" the aircraft that sighted it** (decision 0012, 17 and 18): a roll, a figure, an event and a stored time | Nothing reads the stored time. Decision 0012 said noticing "changes nothing else yet"; the reaction it was for (turn away, press on) was promised for scenario 5 and belongs to the staff, which does not exist | Cut the roll, the figure, the event and the state. Bring it back with the rule that uses it |
| A2 | **A contact's confidence** (decision 0012): three figures, a number on every report and contact | It starts at 0.8, is multiplied by 0.8 each pulse and the contact is dropped below 0.3. So it is a function of age and nothing else, and the only thing that reads it is the drop. Every legibility reader asked what "confidence 0.8" meant | Replace the three figures with one: a contact is dropped at five pulses old. Take confidence off reports and contacts until a report can be more or less sure of itself |
| A3 | **Two figures that must add to 1** for where a gun hit lands (flotation 0.75, fire 0.25) | The loader checks the sum | One figure |
| A4 | **A task force's mission** (decision 0008) | Stored and hashed; no rule reads it | Leave it. The spec lists it and the staff will read it. Recorded here so that it is not mistaken for a rule |

A1 to A3 change no outcome of any fight or search. They remove 6 figures, one roll and two fields of state. A1 and A2 are still put to the product managers: noticing was added at the designer's own suggestion, and the spec says a contact loses confidence.

### B. Two limits on one quantity: give a reason or merge

| # | What | Proposal |
| --- | --- | --- |
| B1 | **A contact's age is tested four ways:** a new report refreshes it up to 2 pulses old; it alerts a force up to 2 pulses old; it can be struck at age 0; it is dropped at 5 | Keep three, with their reasons written in decision 0012: "fresh" (2 pulses, one figure for both refreshing and alerting), "can be struck" (this pulse or the one before) and "forgotten" (5). The two figures of 2 become one |
| B2 | **A contact's distance is tested two ways:** refreshed within 2 hexes, alerts within 3 | Keep both. They answer different questions: is this the same force, and is it near enough to worry about |
| B3 | **How many aircraft a group flies has three limits:** the ship's capacity, the flight deck, the patrol share | Keep. They apply in that order and each is a different fact. Decision 0027 now says so |

### C. One word, several meanings: a glossary

The legibility readers and the gatherer of this material both tripped on the same words. No rule changes; `docs/glossary.md` fixes one meaning for each, and the records and event texts follow it.

| Word | Meanings found | To be |
| --- | --- | --- |
| Alert, alerted, alarm, readiness | Four words round one flag | Readiness is the order. Alert is the state. A force is alerted by a contact, by being fired on, or by an alarm |
| Avoid | An ordered stance; what a detached force is set to; what a force does for the rest of a pulse after breaking off | Avoid is the stance. The third is "avoiding for the pulse" |
| Share | Of ships to keep when detaching stragglers; of fighters to keep overhead | "Keep share" and "patrol share", never bare |
| Sighting | An air search's; a fight's; a report | A search sights. A force in a fight sees. A report is a report |
| Range, radius | Search range, strike range, radius, range with a normal load | Radius is how far out and back. Range is how far one way, and is only ever a fact |

### D. Rules that give results nobody would defend

These change how the game plays, so they are the product managers' to decide. Each has a recommendation.

| # | What happens | Why | Recommendation |
| --- | --- | --- | --- |
| D1 | **Results hardly vary.** 1.8 expected hits is always 1 or 2 | Hits are the whole part of the expected number, plus one more with a chance equal to the fraction (decision 0014, 9). It applies to every shot in the game | **Roll each shot.** The mean is the same and the spread becomes what chance gives: 1.8 expected from six shots is anything from 0 to 6. It is the rule a player would guess, and step 2 cannot calibrate spreads against history with the present one. Cost: every hand calculation's "3 or 4 hits" becomes a band, and every pinned output changes |
| D2 | **Every raid in a pulse meets the whole patrol.** A raid of 7 bombers meets 46 fighter shots and is wiped out before it attacks | Decision 0027, 12, chosen so that the order of raids would not matter | **The patrol divides itself among the raids on its hex** in proportion to their aircraft. The order still does not matter. Cost: one rounding rule |
| D3 | **Battle readiness costs nothing,** so a player would always order it (decision 0021 says so) | The cost was to be fatigue | Keep, with a date: scenario 6 brings fatigue and must make readiness cost something. If it does not, readiness and the alerted flag are cut and every force is simply alert or surprised by events |
| D4 | **A third of strikes find nothing,** because a report names the wrong hex three times in ten and a strike looks in one hex | Decisions 0012 and 0026, 7 | Leave the rule. The 0.3 is a placeholder and step 2 sets it. A search of the neighbouring hexes would be a second mechanism for what the figure already does |
| D5 | **A whole strike attacks one ship,** and wastes itself on one already sinking | Decision 0026, 9 | Leave until the staff can give a strike its targets |
| D6 | **Aircraft are lost with their ship** even when another deck is in the same hex | Decision 0025, 5 | Leave until scenario 6, which brings bases and transfers |

### E. The three faults Savo Island exposed (decision 0024)

| Fault | Finding | Recommendation |
| --- | --- | --- |
| Both sides shoot equally well | True, and it is why Savo comes out even. The facts hold shells fired and hits for most ships at Savo | Step 2, as a figure for each navy's night gunnery set from those rows. Not a rule to invent now |
| Weight of shell matters too little: a 5-inch hit is as likely as an 8-inch one to knock out a cruiser's engines | The chance of a system hit does not depend on the damage the hit does | **Make it depend on it:** the chance is the hit's damage points over a figure, capped at the present chance. One mechanism, one figure in place of one. This is a rule change and is put with D1 and D2 |
| The column breaks off too soon | The break-off level is an order in the scenario file, a placeholder 0.5 | A scenario input, not a rule. Leave |

### F. Housekeeping

- **Rule cards are missing** for decisions 0001, 0006 and 0008. The index promised them at this review.
- **Clock times are cut, not rounded:** 10:40 prints as 10:39. Round them.
- **Three things promised "for scenario 5" were not done** and are not scenario 5's: a force's reaction to being sighted, pressing on or retiring after a fight, and passing an enemy without fighting. All three are the staff's. They are listed in the plan for the agent interface.

## What this review does

- **Done with it, as engineering:** the glossary (`docs/glossary.md`) and the three rule cards, with this review. A3, B1 and the clock follow in one small change. No fight or search comes out differently.
- **Put to the product managers:** the cuts A1 and A2, and the rule changes D1, D2 and the system-hit rule of E. D3 is a condition on scenario 6.
- **Left alone, with the reason written above:** A4, B2, B3, D4, D5, D6.
