# 0214: A force asks leave to go after what it has found

Status: a plan by the architect, written on Nov 22, 2026, on the designer's idea at the table: "If a unit finds something that it believes should be getting its attention, it could be very interesting (and realistic) if it changes its plans to the thing it finds more important. For example, a carrier task force that had an assigned order to deny water at a certain location found a carrier force in another location while searching all around. It could then have a plan proposed to the commander, and if accepted, it will delay or cancel the previous order and prioritize the new order." Asked whether to plan it he said yes. It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Today a carrier force goes after a carrier it has found only as far as its order lets it, and what lies past that it leaves alone without a word. This has the staff say so: where a force has found something it would go after but for its order, the staff tells the commander what was found, where the force would go and what it would leave, and stops the clock. On his yes the force goes after that one contact, and when the contact is lost it goes back to its order by itself. How bold the order is decides whether the staff asks, goes and reports, or stays. It is the first use of a thing the step 3 spec has had from the start and that was never built: the staff proposes, and raises an interrupt.

Standing: Planned on Nov 22, 2026; to be reviewed.

## Why

What is so today (rules chapter 8; `sim-staff`, `carrier.rs`):

- **A carrier force already closes on what it finds, inside bounds.** Each morning a force with a bomber that can fly closes on a fresh contact, three pulses old or less, of a kind its risk lets it close on (a carrier at balanced, any kind at press, nothing at preserve), if the contact is within a night's steaming at full speed and its bombers' reach.
- **Past the bounds it does nothing and says nothing:**
  - a contact too far for one night is left;
  - where the order says its waters come first, a contact in the waters is preferred to a carrier outside them, "that is what the order says";
  - a force at a station the staff chose for it is "not chosen again when a contact comes or goes, or the force would wander with every sighting" (decision 0202, rule 17);
  - at balanced a contact that is not a carrier is not closed on.
- **The commander is told nothing of any of it** but in the reason beside the day's orders. No notice of the staff's speaks of a contact found, and no interrupt stops on one.
- **The staff is asked once a day, at dawn.** A sighting of the morning waits for the next dawn (decision 0130; decision 0155, item 4, ruled out asking the staff every pulse).
- **The staff cannot ask the commander anything,** change an order of his, or form an order of its own. The nearest it comes is a plan that waits for leave to launch, and one rule that waits for leave and says each morning what it would do: a carrier force that could go and look, at preserve.
- **The spec asked for more and it was never built:** "What needs the commander: an objective, an objective's date, and cancelling. The staff proposes it and raises an interrupt. A proposal never crosses a hard limit" and "While a proposal waits, the forces in it go on with the approved plan" (`docs/step3-spec.md`, the command contract).

What the rows hold (`engagement_events.csv`):

- **Commanders on the spot said what they had found and what they meant to do, and superiors answered.** At Midway the second carrier division's admiral signalled his plan for a dusk attack and the order followed (7632, 7633); Fletcher told Spruance to "proceed southwesterly and attack enemy carriers when definitely located" (2141); on 5 June Spruance chose between two contacts, "I chose the one to the northwest" (2160).
- **They also held a strike for better word:** Fletcher at the Eastern Solomons, "pending more definite word" (155); Yorktown's group at Midway, "pending receipt of information on additional enemy carriers" (2151).
- **A change on a sighting cost time.** The Tone plane's report reached Nagumo half an hour late, and his orders changed three times in the two hours after it (7640 to 7644, 7615 to 7618).
- **And a force was turned from its task by its superior on what was found:** the convoy for Port Moresby (2503, 2518), Tanaka's at the Eastern Solomons (249, 390).

## The rules

### What a force would go after

1. **A find is a contact a carrier force would close on today but for a bound of its order.** At the staff's asking, for a carrier force that is not held, not on its way home and has a bomber that can fly, a find is a fresh contact of ships of a kind on the force's target list, as its order's limits give it, that the closing rule does not take up for one of these reasons and no other:
   - **too far:** it lies beyond a night's steaming and reach, and within `pursuit_nights` nights of it;
   - **the waters first:** the order prefers a contact in its waters and this one, of a more important kind, lies outside them;
   - **a station kept:** the force is at a station the staff chose for it under a commander's order, and the contact is one the closing rule would take from a station of the commander's own;
   - **the kind:** at balanced, a contact of a kind on the target list that is not a carrier.
2. **Where a force has several finds it takes one:** the most important kind, then the nearest.
3. **Nothing here crosses a hard limit.** A force that is held, a ship protected for another order, an order's forces limit and the fuel to go and come back at the risk's margin bind as they do today. A find the force has not the fuel for is not a find.

### Ask, go, or stay

4. **The order's risk says which,** read up the tree as risk is:
   - **preserve: stay.** Nothing changes from today.
   - **balanced: ask.** The staff asks leave (rules 6 to 9).
   - **press: go and report.** The staff takes the leave as given and tells the commander (rule 10).
5. **With command levels on, the force's commander moves it one step,** as his rating already moves the closing rule: a cautious man asks where the order says go, and stays where it says ask; a bold man goes where it says ask. With command levels off there is no such man and the order's risk stands.

### Asking leave

6. **The staff hands in a notice that it asks leave to pursue:** the force; the contact, as the side holds it, with its kind, its age and where it was reported; the hex the force would close to and when it would be there by the engine's own forecast; which bound of rule 1 it is; and the order the force would leave.
7. **That notice fires a standing interrupt,** new, which stops the clock as the others do and which the commander may switch off.
8. **While it waits, the force goes on with its order,** as the spec says.
9. **The commander answers for that force and that contact:** yes or no, by a command that is his alone. A yes is leave to pursue. A no is remembered: the staff does not ask again of that contact for that force. No answer is no leave, and the staff asks again each time it is asked while the find is still a find.

### Going after it

10. **With leave, given or taken, the closing rule takes up that contact as if the bound were not there,** and nothing else of the rule changes: where it closes to, that it does not close when the contact is in reach, the fuel it must have. The staff's reason says it is a pursuit, by whose leave, and what order waits. Where the leave was taken (rule 4, press), the staff hands in a notice that the force pursues, which tells the commander and does not stop the clock unless he has asked that it should.
11. **The order the force leaves is not withdrawn and not changed.** It waits. The force's other tasks under it are as the staff's rules leave them when a force is elsewhere.
12. **The leave ends when the contact does:** when the side no longer holds it, or it is no longer fresh and the force has no newer word of it. The force then goes back to its hex by the rule that sends it back today, and the staff hands in a notice that the pursuit is ended. The commander may end it sooner by taking his leave back.
13. **Cancelling the old order is the commander's to do,** by withdrawing it, as now. The staff never proposes that, and never withdraws an order of his.

### When the staff is asked

14. **In a scenario whose orders may be given inside a day, the staff is asked again at each stop of the clock inside the day, for this rule alone.** It may then hand in a notice of rule 6, 10 or 12 and the orders of a pursuit that has leave, and nothing else: it plans nothing again, forms nothing and changes no other order. Decision 0155, item 4, is amended so far and no further. In a scenario with no orders inside a day the staff is asked at dawn only, as now.

## Figures

15. One, a placeholder.

| Figure | Value | Rests on |
| --- | --- | --- |
| `pursuit_nights` | 2 | a judgement: a force is not sent after a contact it could not reach in two nights, since the contact will not be where it was. On 5 June Task Force 16 steamed through the day and night after a contact of the morning (2160) |

## What is recorded and who knows

16. **A leave to pursue is new state of a side's command:** the force and the contact, and for a refusal the same with a no. It is hashed in its place and named, and the layout rises.
17. **Three notices are new** (asks leave to pursue, pursues, pursuit ended) and **one standing interrupt** (asks leave to pursue), each with its code added at the end; **one command is new,** the commander's answer. A side sees its own and nothing of the enemy's.
18. **The table** shows the staff's asking as a slip "FROM STAFF · FOR DECISION", as a plan that waits for leave is shown: what was found and where, where the force would go and when, and the order that would wait, with ALLOW and REFUSE. The chart draws the force's way to the hex it would close to. A pursuit under way is said on the force's card and in the diary; the order that waits says on its paper that its force is away and why.
19. **The doctrine's version rises,** and the agent's interface, for the new command, notices and interrupt.

## Rule card

Reads: the side's picture of contacts; each carrier force's task, the order it is under and that order's risk, targets and waters read up the tree; the force's fuel and the engine's forecasts of passage; the leaves to pursue; with command levels on, the force's commander's rating. Writes: the leaves to pursue (by the commander's command, or by the staff where rule 4 says go); the force's orders, by the closing rule. Draws: none.

## Who answers where no one is there

- **A side the engine plays** has no one to say yes. At balanced its staff asks, is not answered, and the force stays, as it does today; at press it goes. A scenario's orders say their risk, as they do now.
- **In a dry run** no seat plays. The side tried is answered by its course: a yes or a no the commander has given stands, and a course may give one; a find that arises inside the dry run at balanced is not answered and the force stays. The made-up enemy goes or stays by the risk of the orders he is believed to have.
- **A player who runs time and does not answer** gets no leave.

## Games on file

- **A side at preserve, and a side at balanced that no one answers, play as today.** Every scenario on file gives its orders at balanced but where it says otherwise, so the proofs and bands move only where an order is at press and a find arises. The build plays every scenario and recorded game before and after and names any that moves to the designer before anything merges, as decision 0202 did.
- **Recorded games are remade** for the doctrine's version and the interface's; the records of closed proofs are not altered.

## What it does not do

- **It is of carrier forces alone.** A surface force that stands at a hex, a base's bombers and a lift each have their own rules for what they find.
- **It does not let the staff weigh one order's worth against another's.** The staff does not read the directive or the objectives, and it is not shown points. The commander says what matters more, each time he answers, or once by the risk he sets.
- **It does not let the staff form an order of its own, change or cancel one of the commander's,** or pursue two contacts with one force.
- **It does not ask the staff again every pulse,** nor let it re-plan inside a day.
- **It does not change the closing rule inside its bounds.** A force that closes today still closes without asking.
- **It does not make an unanswered asking lapse into a yes.**

## What it costs

- **One figure; a command, three notices and an interrupt; state for the leaves; the staff asked inside a day for one thing; and the table's slip, card and chart.**
- **More stops of the clock.** A side with several carrier forces and a wide search may be asked often. The interrupt can be switched off, and an order set to preserve or to press does not ask.
- **Rulebook:** chapter 8 (operating a carrier force, the notices, the interrupts, the commands and who may give them, when the staff is asked, what is not built); chapter 1 (the day's order with orders inside a day, who sees what); the glossary gains "Find", "Leave to pursue" and "Pursuit".
- **A scenario of its own and its hand calculation,** written by the architect before any code is delegated.

## Questions for the designer

1. **Ask at balanced, go at press, stay at preserve.** The architect proposes it. It makes risk mean one more thing, and an order at balanced now stops the clock where it did not.
2. **The staff asked inside a day for this alone** (rule 14), which decision 0155 had left out. Without it a sighting of the morning is asked about the next dawn.
3. **Carrier forces first,** and other forces after it is played.
