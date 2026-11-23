# 0228: The staff chooses a supply source

Status: a plan by the architect, written on Nov 23, 2026. It is step 3 of decision 0222, which the designer confirmed that day ("staff chooses and user can change it"), planned again here because the review of decision 0224 found that the rule step 3 meant to reuse chooses badly. It is to be read by a reviewer who has not seen it written before anything is built.

Summary: Where a commander orders a place held and names no base to supply it from, the staff forms no supply and tells him to name one. Under an order to control an island the staff does choose, by the most supply above the reserve, which can pick a full base no ship lies at, or strip a forward base of its own men's food. This gives the staff one rule for choosing a source, used in both cases: a base that can spare supply after its own fifteen days, one that can cover the place's need before one that cannot, one with a transport force lying empty at it before one with none, then the most to spare. The choice is kept, shown on the order's paper as the staff's, and the commander may change it.

Standing: Planned on Nov 23, 2026; amended on review; nothing built.

## Why it is a record of its own

Decision 0222's step 3 (its rules 8 to 12, as amended) said the staff chooses "by the island's rule, unchanged". Three things have happened since:

- **The review of decision 0224 found the island's rule wrong for a rear base:** "a large rear base would be chosen as the source of 'control an island' by the rule of most supply above reserve, with no transport lying at it, and supply to Lunga would stop. The choice rule has to be settled first."
- **The architect's ruling 3 on pull request 695** left a question to this step: whether the commander may name a base another order draws on.
- **Decision 0222 is long and three of its four steps are built.** A rule that changes the island doctrine of decisions 0151, 0152 and 0163 should be found under its own number.

So this record replaces rules 8 to 12 of decision 0222 as amended. Rule 11 there (the table's own choosing goes) and the designer's answer 1 (shown on the paper, with CHANGE) are carried here unchanged.

## What is so today

Rules chapter 8, Holding a place and Controlling an island; `island.rs` (`island_source`, `source_test`, `pool_at`, `island_pools`) and `holding.rs` (`no_source_reason`, `hold_supply`):

- **A commander's hold with men at its base and no source on its chain forms no supply.** Its reason names the bases that could be a source, or says the place lives on what it has (decision 0222, rule 1).
- **Under "control an island" the staff chooses:** the side's base in no named place's hex, that it may take, with the most supply above its reserve, the lower number on a tie; no transport force need lie there. It is chosen on the first morning and kept. The pool is the transport forces stopped there with nothing aboard, and only grows.
- **A lift's source is its home port,** and is not chosen.
- **The table chooses a source of its own** (`supplyForces`) for an order given from a base's card, and shows it as "WE CHOSE IT FOR YOU".
- **The supply doctrine cuts a run's load to the source's stock above its reserve.** No play scenario sets a reserve, so a source gives everything it has.

## The rules

### One rule for choosing

1. **A base may be chosen** where it is the side's, is not in the hex of the place supplied (under "control an island", in no named place's hex), the staff may take it (decision 0149, 5), and it has supply to spare.
2. **Supply to spare** is the base's supply less what it keeps, not below 0. What it keeps is the greater of its reserve and fifteen days of the planned use of its own hex today (Days of supply). The fifteen days are the supply doctrine's own interval where the commander set none, a placeholder of the staff's.
3. **The place's need** is fifteen days of the planned use of the place's hex today. Under an island it is that of the island's first place. Where the side uses nothing there yet, it is 0.
4. **Among the bases that may be chosen, the staff takes the first by this order:**
   - one that can spare the place's need before one that cannot;
   - then one where a transport force the staff may take lies stopped with nothing aboard (the island's pool, as it is) before one with none;
   - then the most to spare;
   - then the nearer to the place by the engine's sea path;
   - then the lower number.
5. **The choice is kept,** as decision 0152, 10 has it: on later mornings the source the staff set is the source while the base is the side's, whatever the stocks have become. The staff does not choose again.
6. **With no base that may be chosen, the staff forms no supply** and says the place lives on what it has, with its days of supply, as decision 0222, rule 1 built it. The words change in one thing: a base passed over because it keeps its own fifteen days is named with the two figures.

Why this order. A base that cannot cover the need is a stopgap, and a kept stopgap is worse than a full base that waits for a ship. Among bases that can cover it, the one with a ship can start tomorrow. Distance only breaks a tie: a near base with a small stock is spent in a run or two and the choice is kept. Why "to spare" and not "above its reserve": two limits on one quantity need a reason (`docs/engineering/rule-complexity.md`), and it is that the reserve is the commander's word and most bases have none, while a staff that chooses for itself must not feed one place by starving another.

### What a chosen source gives

7. **A run from a source the staff chose is cut to what the source can spare** on the morning it is ordered, where the supply doctrine today cuts it to the stock above the reserve. A source the commander named gives down to its reserve, as now: it is his.
8. **A source is the staff's choice** where no intent of the commander's on the supply's chain has a forces limit that names a source.

### Under a commander's hold

9. **Where a hold of the commander's has men at its base, no "keep supplied" for a base in its hex is in force and no source is named on its chain, the staff chooses by rules 1 to 4** and forms "keep supplied" under the hold, as it does where a source is named: the same floor, the same interval.
10. **The staff sets the forces limit of the supply it formed:** the source, and the transport forces stopped there with nothing aboard that the hold's chain lets it use. Each morning it adds a force newly found there, as the island's pool grows (decision 0152, 11). It sets nothing on the commander's hold.
11. **That supply has a plan of its own,** drafted when it is formed and made ready and released by the supply doctrine on the leaves, as under an island (decision 0151, 9), so that a supply that fails does not end the hold.
12. **With no ship at the source the supply is formed all the same,** is served, says that no ship of its pool is afloat and sends nothing, as under an island (decision 0163, 1 to 3). This replaces decision 0222's rule 10, "with no ship to carry it the staff forms none": the pool can then grow on the morning a ship comes, by one rule and not two.
13. **Under "control an island" the source is chosen by rules 1 to 4** in place of the most supply above the reserve. Everything else of the island's source and pool is as it was.

### The commander's source

14. **A source the commander names is used, and none is chosen,** whatever it holds and whatever else draws on it. Two supplies that draw on one base share it as the supply doctrine already shares: in rank order, the tons given to the first committed against the second. This settles ruling 3 on pull request 695. The island's test today refuses a named base that is the source of an order higher in priority; that refusal goes for a named base, and stays for a base before its day or not the side's.
15. **A change of source is taken up when no run is unfinished.** Where the commander names a source on his order after the staff has set one, the staff sets the forces limit of the supply it formed (under an island, of the holds it formed) to his source and the pool there, made anew, on the first morning no run of that supply's plan is loading or under way. Until then the old source stands and the reason names the run it waits for.
16. **A source put back to the staff's choice stays where it is.** The staff keeps a source, as rule 5 says, and does not choose again.
17. **Where the kept source has nothing to spare and the supply cannot be met,** the reason of the day names each other base that may be chosen, with what it can spare, and records that it speaks of the source (decision 0222, rule 5), so that the paper's link leads to "supplied from".

### What the commander sees, and how he changes it

18. **The tools say where an order is supplied from.** `command_state` gives each hold and each island in force `supplied_from`: the base, and `chosen_by`, `staff` or `commander`. It is worked from the command state and stored nowhere.
19. **On the plan's paper** of an order to hold a place or to control an island, the line "Supplied from" shows the base by name with "our choice" where the staff chose it, and CHANGE. CHANGE opens the order paper for that limit, as decision 0222's step 4 built it; BACK TO THE STAFF'S CHOICE is offered where he named one. The table says nothing of why a base was chosen there: the reason is in the task's WHY.
20. **Before signing,** THE STAFF'S READING of an order to hold a place shows the supply the staff would form and from where, since the preview runs the staff's morning.
21. **The table's own choosing goes** (`supplyForces`, and "WE CHOSE IT FOR YOU"). An order given from a base's card is sent with no source unless he sets one.

## What a side learns

Nothing. The staff reads its own side's bases, forces, men and orders, and the map. Nothing of the enemy is read: the danger of the way is not weighed here.

## Rule card

Reads: the side's bases through `SideView` (hex, supply, reserve); `SideView::days_of_supply` for the hex of each base weighed and of the place (its planned use); the side's transport forces (mission, where stopped, what is aboard, the day each may be used); the engine's sea path between two hexes; the command state (intents in force and who made each, the forces limits on a chain, plans and their runs). Writes: nothing new. By commands that exist: `form_intent` of a "keep supplied" under a hold, `set_limit` of the forces limit of an intent the staff formed (source and task forces), reasons; by the engine, a plan drafted for a "keep supplied" formed under a commander's hold with no source on its chain. Draws: none.

## Determinism, and truth and belief

- **No new state, so no change to the hash or the layout.** The choice is remembered where the island's is today: in the forces limit of the intent the staff formed.
- **Every quantity is fixed-point,** bases are weighed in base order, and the last key of rule 4 is the base's number, so the choice never rests on the order of a collection.
- **No draw.**
- **The staff takes a `SideView` and nothing else.** `supplied_from` is worked from the side's own command state. No new kind of event: the supply formed, its plan and its reasons are events a side already sees of its own staff and the enemy never does, and `side_events.rs` is not touched.
- **The engine changes in one place:** which formed intents are given a plan of their own (rule 11). It is believed that the staff may already set a forces limit on an intent it formed under a commander's order, inside what his limits bind (decision 0222, rule 2). If intake refuses it, or refuses the commander's source on a hold whose child names another (rule 15), the build stops and that comes back to the architect.

## The scenario and the statements

Scenario 132, "a source the staff chooses" (the next free number when this was written; checked again at the build). Its test is `tests/scenario_132_a_source_the_staff_chooses.rs`, its files in `data/scenarios/a-source-the-staff-chooses/`, its map `data/map/placeholder_a_source_the_staff_chooses.toml`. Everything in it is a placeholder and named so.

### The setup

- **The map.** Open sea, `q` from -3 to 9 and `r` from -1 to 9, with four coast hexes: H at (0,0), port size 0; N at (-2,0), F at (0,8) and G at (8,0), each port size 5. Each lies on an axis from H, where a leg between centres is charged 40 nm (decision 0058): N is 2 hexes, 80 nm; F and G are 8 hexes, 320 nm each.
- **The figures.** A ground file with `daily_supply_t_per_man = "0.005"` and no figures of the air, so the only planned use is the men ashore.
- **The Allied side.** Base 1 at H with 20 t of supply and land unit 1, 1,000 men ashore. Base 2 at N with 200 t and land unit 2, 2,000 men ashore; task force 2 stopped there, one placeholder troop transport (class 10), nothing aboard. Base 3 at F with 2,000 t and no men; task force 1 stopped there, two such transports, nothing aboard. Base 4 at G with 5,000 t, no men and no force. No base has a reserve.
- **The order.** On day 1 the commander states "hold base 1" with no limit, and gives both leaves. Ten days. No enemy acts.

### The hand calculation

On the first morning, before anything is charged:

| Base | Supply | Own use a day | Keeps (15 days) | To spare | Covers the place's 75 t | A transport force empty there | Sea miles to H |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 1, H, the place | 20 t | 1,000 x 0.005 = 5 t | | | | | |
| 2, N | 200 t | 2,000 x 0.005 = 10 t | 150 t | 50 t | no | task force 2 | 80 |
| 3, F | 2,000 t | 0 | 0 | 2,000 t | yes | task force 1 | 320 |
| 4, G | 5,000 t | 0 | 0 | 5,000 t | yes | none | 320 |

- **The place:** 20 t over 5 t a day is 4 days of supply. Its need is 15 x 5 = 75 t.
- **File a, the choice:** bases 3 and 4 cover the need and base 2 does not; of those two base 3 has a force and base 4 none. The source is base 3, with task force 1. The rule as it was (most above the reserve) gives base 4, which no ship lies at.
- **File c, the load.** Bases 1 and 2 alone: base 2 is the only base that may be chosen and is the source. The supply is formed on the morning of day 1 and served from the morning of day 2 (Holding a place, The supply that follows). By then base 2's men have eaten one day, 10 t: it holds 190 t, keeps 150 t and can spare 40 t. Base 1 holds 15 t that morning against a floor of 5 days at balanced, 25 t, so a run is needed at once, and its load before any cut is at least the 75 t of the interval. Cut to what the source can spare, the first run's load is **40 t**. This number rests on two readings: that the first run is ordered on the morning of day 2, and that one day's use, 10 t, has been charged at base 2 by then. If the build finds either otherwise it stops and says which; the number is the architect's to restate.
- **File d, nothing to spare.** Bases 1 and 2 alone, base 2 with 150 t: it keeps 150 t and can spare 0. No base may be chosen. The place lives on what it has: **4 days**.
- **File i, a base in the place's own hex.** Bases 1 and 2 as in file d, and base 5 at H with 900 t. Base 5 is not weighed. No base may be chosen. The hex holds 920 t against 5 t a day: **184 days**.

### The files and the statements

Each statement is tested as written. Nobody but the architect changes one or an expected number.

File a, the setup as above:

1. On the morning of day 1 the staff forms "keep supplied" for base 1 under the hold, and sets its forces limit to source base 3 and task force 1.
2. The reason of the choice carries, as evidence, each base weighed with its figures: base 2, 50 t to spare, less than the place's 75 t, task force 2 there, 80 nm; base 3, 2,000 t, task force 1 there, 320 nm; base 4, 5,000 t, no transport force there, 320 nm. It says the source is base 3.
3. The supply has a plan of its own. The hold's plan is released on day 1 and is not failed on any day.
4. A run of task force 1 sails from base 3 and unloads supply at H within the ten days. Base 4 holds 5,000 t at the end. Task force 2 is given no order.
5. On every morning from day 2 the source is base 3, and `command_state` gives the hold `supplied_from` base 3, `chosen_by` `staff`.

File b, file a without base 3 and task force 1, and with task force 3 (one such transport, nothing aboard) at base 4 from day 3:

6. On day 1 the source is base 4, though task force 2 lies at base 2: base 2 cannot spare the place's 75 t. The supply's forces limit names base 4 and no force.
7. On day 2 the supply is served, says that no ship of its pool is afloat and sends nothing. Neither its plan nor the hold's ends.
8. On day 3 the staff adds task force 3 to the supply's forces limit, and the first run is ordered on day 4 (Controlling an island, The source kept, and its pool: a force joins by a command of the morning it is found and serves from the morning after).

File c, bases 1 and 2 alone:

9. The source is base 2 with task force 2. The first run is ordered on day 2 with a load of 40 t, and its reason says the load is cut to what the source can spare.
10. No second run is ordered. From the morning base 2 has nothing to spare, the reason of each day the supply cannot be met says so and names no other base.

File d, file c with base 2 at 150 t:

11. No supply is formed. The hold's reason says no other base of the side can spare supply, names base 2 with 150 t held and 150 t kept, and says base 1 lives on what it has: 4 days of supply today.

File e, file a with the commander's forces limit on the hold, on day 1, naming source base 4:

12. The staff chooses nothing. The supply reads base 4 as its source, and `supplied_from` says base 4, `chosen_by` `commander`.

File f, file a played for twenty days, in which on day 3 the commander names source base 2 on the hold, and on day 16 clears it:

13. While a run from base 3 is loading or under way the source is base 3, and the reason of each such morning names the run the change waits for.
14. On the first morning no run is unfinished, the staff sets the supply's forces limit to base 2 and task force 2, and task force 1 is no longer in it.
15. The change is taken up before day 16. From day 17 the source is still base 2: the staff does not choose again.

File g, file a with task force 3 at base 4 from day 3:

16. On every morning the source is base 3, though from day 3 base 4 would be the first by rule 4. The reason says the source was chosen on an earlier morning and kept.

File h, file a with island 1 of one place, base 1, and "control island 1" stated in place of the hold:

17. The island's hold of base 1 is given a forces limit of source base 3 and task force 1.

File i, file d with base 5 at H holding 900 t:

18. No supply is formed. The reason names base 2 as in statement 11 and does not name base 5. It says base 1 lives on what it has: 184 days of supply today.

### What must always hold

Property tests beside the files, each over the setup with its figures varied:

- **More supply at a base never puts it later in the order** of rule 4, and a transport force added at a base never does.
- **A base in the place's hex is never chosen,** whatever it holds.
- **A source the commander names is always the source.**
- **A run from a source the staff chose never leaves the source, on the morning it is ordered, below fifteen days of its own use or below its reserve.**
- **Supply is conserved** with no enemy: what leaves a source is aboard a ship, ashore at the place or eaten there.
- **Asked twice on the same view, the staff chooses the same base.**

### Games on file

The staffed games on file are run before and after (`scripts/compare-scenarios.sh`), with the hashes cut, and each that moves is listed with what moved.

- **Expected to move in words alone:** every game with "control an island", whose reason of the source now carries the figures of rule 4. By the architect's reading of the files of stage 3's proof, the choice itself does not move there: the Japanese side has Rabaul against bases with nothing to spare and no force on the first morning, and the Allied side has Espiritu Santo alone. That reading is a lead to be checked by the comparison, not a result.
- **Expected to move in substance:** a game where a commander's hold has men and no source, and a base may be chosen. Scenario 69's second case and scenario 79's hold are two; their statements say the staff forms no supply, which this record makes untrue. Each such statement comes to the architect to restate before the build goes on.
- **If a file of a closed proof moves in an event of substance,** the build stops on that file and it comes back to the architect. The recorded games of closed proofs are not altered.

## What it leaves out

- **The staff does not choose again** when the kept source runs dry. It says so and leads the commander to "supplied from" (rule 17).
- **The staff brings no ship to a source.** A source with no transport force waits for one to lie there: a lift that has landed and goes on to it (decision 0163), or a force the commander sends. A row of open work.
- **A lift's source stays its home port** (decision 0152, 1). The choice here is for supply to a place held.
- **The need of a place not yet the side's is 0,** so before a landing every base that can spare anything covers it, and the order falls to the transport force and then the most to spare. A row of open work with the take's own reckoning of what its men will eat.
- **Distance is the fourth key only.** When a rear base stands behind a rear base (Nouméa behind Espiritu Santo, decision 0224), the order is looked at again.
- **The danger of the way** to the place is not weighed in the choice. The run by night, the escort and the cover are the rules for that, as they are.
- **Fuel.** The staff chooses a source of supply; nothing here ships fuel to a place.
- **Ships in port in no force** are not counted as a transport lying at a base, as the island's pool does not count them.
- **Decision 0224's rulings** on the 39 days are data and need none of this.

## The versions it raises

Corrected on review: the record as first written was one behind main. Each is one above what main has when the build's pull request is opened; as main stood on Nov 23, 2026:

- **The doctrine:** `supply-58` to `supply-59`.
- **The state layout:** `flagplot-state-v58` to v59, and **the saved game version** 26 to 27, for the scenario's key (see Amended on review, A).
- **The agent interface:** 51 to 52. If decision 0226's build, which also takes 51 to 52, has merged first, 52 to 53. The rule for the builder: merge main in before the pull request is opened and take the next number after main's.
- **Not raised: the rules label,** which stays 32. No game on file plays differently, and the label rises only when one would.

## How it is built

Reviewed first by a fresh reader given this record, rules chapter 8, `island.rs`, `holding.rs` and the supply doctrine. Then two pull requests:

1. **The staff, the engine's one change and the tools** (rules 1 to 18), with scenario 132. The rulebook changes in the same pull request: chapter 8 (Holding a place, The supply that follows; Controlling an island, The source of supply and The source kept; the doctrine's A new run; The tools; the doctrine's version), the glossary ("supply to spare", "the place's need", and "source" where its meaning moved), and the rows of `docs/rules/differences.md` it settles or finds.
2. **The table** (rules 19 to 21), with the page's checks: the line and its CHANGE on a hold's and an island's plan, "our choice" where the staff chose, nothing chosen by the page.

## Questions for the designer

None. He has said the staff chooses and the commander may change it; the rest is the architect's inside that.

## Amended on review, Nov 23, 2026

A fresh reader was given this record and the code. It found today's code described truly, no breach of determinism or of truth and belief, and the hand calculation holding. It found eight things to mend. The architect checked each against the code and rules as below. Where this section and the rules above differ, this section holds.

### A. The staff chooses only where the scenario says so

**Found.** The recorded game `second_proof_week_sonnet`, of stage 2's closed second proof, has an Allied hold with no source while Espiritu Santo "could be the source". Under rule 9 the staff would form a supply there: a change of substance in a game that must replay as recorded, whose calls may not be edited. Scenarios 69 (its third property, "a base that could be one", not the second as the record said), 79, 76, 86 (which pins the island's words of the source), 130 (part D, statement 18) and 106 (statement 12's hashes of the five-day file) all assert today's behaviour.

**Ruled.** As command levels are (decision 0132), this is on only where a scenario says, and with it off every game plays as before.

- **A scenario states, for a side, that its staff chooses its sources of supply.** The key is the side's, beside the side's other doctrine values (decision 0223, stage 0), and is left out by every file on file today. It is state: it goes into `hash::hash_world` in its place, is named in `hash::name_every_field`, and the layout version is raised and the golden files remade (decision 0057). It never changes in a game.
- **For a side without it, nothing in this record applies:** rules 1 to 17 are not read, the island chooses by the most supply above the reserve, a commander's hold with no source forms no supply, and the orders and reasons are those `supply-58` gave. `supplied_from` (rule 18) is given for every side, since it only reports.
- **No test on file is restated and no pin is remade for this record** but for the layout version. The comparison of the staffed games before and after must show none moved; one that moves comes back to the architect. No file of a closed proof states the key, and the recorded games of closed proofs replay as they do at every rise of the doctrine's version.
- **The files the table plays state it for both sides,** by the scripts that make them. The table's files of the 39 days under decision 0224 are measured for that record's rulings first without it, then with it, so that the two are not confused.
- **Why two rules are kept.** One mechanism is the rule (`docs/engineering/rule-complexity.md`); the reason for two is that a closed proof's agent-played record cannot be said again in a new language. Open work: when those records are next marked as of an older engine, the old choice goes and the key with it.

"Games on file" above is replaced by this. Its two expectations (island games moving in words, scenarios 69 and 79 restated) no longer arise.

### B. No plan of its own; a plan is not failed for want of the staff's ships

**Found.** Rule 11 cannot be built as worded. `CommandState::forms_own_plan` knows only an intent's kind and its parent's kind; intake, `plan_owner` and the invariants rest on it; nothing in the state says who chose a source, and that can change in a game.

**Ruled.** Rule 11 is withdrawn. The supply the staff forms under a commander's hold shares the hold's plan, as it does today where he names a source. What rule 11 was for is done in the doctrine instead:

- **Where the ships of a supply are the staff's to find** (no limit of the commander's on its chain binds ships), an empty pool does not mark the plan failed. The supply is served, says no ship of its pool is afloat, and sends nothing. Where his limit names the ships and none is afloat, the plan is marked failed, as today.
- **Where the source is the staff's choice** (rule 8) and the base is no longer the side's, the plan is not marked failed either; the staff chooses again that morning by rules 1 to 4. This is the one case in which it chooses again, and it is the island's today.

Rule 12 stands with those words. The engine does not change but for the key of A, so the rules label is not raised.

### C. A commander's source acts at once, as it does today

**Found.** Rule 15 made a named source wait for runs to finish, where an island's is taken up the same morning today (`island_pools`); rule 14 dropped a refusal the island makes today; neither was listed as a change. File e was not covered by rule 10. What `supplied_from` shows while a change waits was not said.

**Ruled.** Rule 15 is withdrawn. Rule 10 is restated:

- **Under a commander's hold, for a side with the key, the staff sets the forces limit of the supply in every case,** each morning it differs: the source is the commander's where an intent of his on the chain names one, else the staff's, chosen and kept; the task forces are his where his limits bind ships, else the transport forces stopped at the source with nothing aboard, a pool that only grows while the source is the same and is made anew when it changes.
- **So a source he names is taken up on the staff's next morning,** under a hold as under an island, and nothing waits. What becomes of a run still loading at the old source is the supply doctrine's reading as it is today when an island's source is renamed; the architect has not traced it, the build reports it, and it is a row of open work if it is wrong.
- **`supplied_from` is the source the supply reads that morning** (the nearest forces limit on its chain that names one), with `chosen_by` `commander` where an intent of his on the chain names a source and `staff` otherwise. After he puts it back to the staff's choice it shows the same base, `staff`.
- **Rule 14's change is a change, for a side with the key:** a base the commander names is not refused because an order higher in priority draws on it. It is listed with the rulebook's changes.

### D. What a base keeps, where bases share a hex; and loads still to be taken

**Ruled.** Rule 2 is restated. Days of supply are a hex's, so the keep is worked for the hex: the greater of the summed reserves of the side's bases there and fifteen days of the hex's planned use. The hex's supply less that, not below 0, is what the hex can spare. A base can spare the lesser of its own supply above its own reserve and what its hex can spare. Rule 7 is restated: a run is cut to what the source can spare, less what runs still loading there have yet to take, those added earlier the same morning included, as the doctrine reckons the stock above the reserve today (`source_available`).

### E. The hand calculation's unstated figures

The numbers hold on these, now stated in the setup: every skill of land units 1 and 2 is 2, so each skill's factor is 1 and a man's charge is 0.005 t a day whatever the ground file's strengths; the placeholder troop transport (class 10) has room for 3,000 t, so no load in the files is cut by the ships' room; no base has a reserve and no air figures are loaded.

### The statements, restated

Every file states the key for the Allied side, but files j and k. Statements 1, 2, 4 to 6, 8 to 11 and 16 to 18 stand as written. Restated by the architect, Nov 23, 2026:

- **3.** The supply serves the hold's plan and has none of its own. The hold's plan is released on day 1 and is not marked failed on any day.
- **7.** On day 2 the supply is served, says that no ship of its pool is afloat and sends nothing. The hold's plan is not marked failed.
- **12.** The staff chooses nothing. It sets the supply's forces limit to source base 4 and no force, since none lies there; `supplied_from` says base 4, `chosen_by` `commander`. The supply sends nothing, and the hold's plan is not marked failed.
- **13.** Withdrawn.
- **14.** On the morning after the commander names base 2, the staff sets the supply's forces limit to source base 2 and task force 2, and task force 1 is no longer in it; `supplied_from` says base 2, `chosen_by` `commander`.
- **15.** From day 17 the source is still base 2 and `chosen_by` is `staff`: the staff does not choose again.

Added:

- **19.** File j, file a without the key: no supply is formed on any day, and the hold's reason on day 1 is the one `supply-58` gives, naming bases 2, 3 and 4 with 200 t, 2,000 t and 5,000 t of supply above their reserves.
- **20.** File k, file h without the key: the island's hold is given source base 4 and no force, as today.

Added to what must always hold: two bases of a side in one hex can never together spare more than their hex can; and a side without the key gets, on any setup, the orders `supply-58` gave.

### What the build changes in the rulebook

Chapter 8: the key and what a side without it gets; Holding a place, The supply that follows; Controlling an island, The source of supply and The source kept; the doctrine's step 1 (an empty pool) and A new run (the cut); The tools (`supplied_from`); the doctrine's version. Chapter 1 or 8, wherever a scenario's keys for a side are listed. The glossary: "supply to spare", "the place's need".


## The architect's ruling on the build, Nov 23, 2026

**Found, before any code.** This record believed the staff may already set a forces limit on an intent it formed under a commander's hold. It may not. Intake checks that a limit on an intent with a parent narrows (`check_command`, `SetLimit`, `forces_narrow` in `crates/sim-core/src/command/intake.rs`). Off a take and an island, with no limit above that binds ships the limit may name no ship and no task force, and it may name a source only where a limit above names that source. So under a hold with no limit, the staff's limit of source base 3 and task force 1 is refused as wider than its parent; under a hold that names only a source, its task forces are. The architect read the function and its one caller, and it is so. Only a chain under a take or an island is checked by `forces_narrow_under_take`, where a limit above binds only what it names.

**Ruled: a second engine change, beside the key.** For a side whose scenario says its staff chooses its sources of supply, a forces limit set on a "keep supplied" whose parent is a "hold a place" is checked as under a take (`forces_narrow_under_take`): each limit above binds only what it names of the task forces and ships, the land units and the source, and what none names is the staff's to name. A source the commander names anywhere above still binds: a limit that names another is refused.

- **It is keyed on the side's key, the intent's kind and its parent's kind, and on nothing else.** Not on who gives the command and not on who made the intent: a "keep supplied" under a hold is always the staff's, since a hold is a root under which only the staff forms (and the invariants hold a child to the staff's making). So `forces_narrow` is given the intent the limit is set on, and needs neither the issuer nor the maker. The developer's suggestion is taken with that one simplification.
- **For a side without the key the check is what it is today,** on every chain. A take and an island are untouched for every side.
- **Rule card, added.** Reads: the side's key; the kind of the intent a forces limit is set on and of its parent; the forces limits above it as they were set. Writes: nothing; it decides whether `set_limit` is taken or refused as wider than its parent. Draws: none.
- **The rules label stays 32.** No file on file states the key, so no game on file meets the changed check. No other version moves for this: the doctrine, the layout, the saved game and the agent interface are as "The versions it raises" has them.
- **Statements.** 1, 6, 8, 9, 12 and 14 rest on the change and none is restated. Added, as tests of intake in `sim-core` beside those of `forces_narrow`:
  - **21.** For a side without the key, a forces limit naming a source and a task force, set on a "keep supplied" under a hold with no forces limit, is refused as wider than its parent, as today.
  - **22.** For a side with the key, the same limit is taken.
  - **23.** For a side with the key, under a hold whose forces limit names source base 4, a limit on the "keep supplied" naming source base 3 is refused as wider than its parent, and one naming base 4 and a task force is taken.
  - **24.** For a side with the key, a forces limit set on a cover formed under the same hold is checked as today.
- **Rulebook:** chapter 8, The intent tree (a limit must narrow) and The forces limit on a staff's orders, say the keyed case.

"The engine changes in one place" above is replaced by: the engine changes in two places, the key and this check.

### The second stop: a run of the supply under a hold, and rule 17's bases

**Found, by play of file a.** The choice, the forming and the limit work on day 1 (source base 3, task force 1). Every later `add_run` and `commit_ships` is refused as having no forces limit. Intake's `forces_of` (`crates/sim-core/src/command/intake.rs`) reads the forces limit of the plan's owner, and, where the owner has none, the nearest limit up the tree only for a "keep supplied" under an island. Section B of the review made the supply share the hold's plan, so the owner is the hold, and the limit the staff sets on the supply beneath it is never read. The architect read the closure and its two callers, and it is so. Section B did not foresee it.

**Ruled: a third engine change.** For a side with the key, where the owner of a plan is a "hold a place" and a "keep supplied" in force formed directly under it has a forces limit of its own, the forces limit of a run of that plan is that supply's: its source and its pool. With more than one such supply, the lowest-numbered. With none, the owner's limit is read as today. That limit was taken by the check of the first ruling, so it is inside whatever the commander named above. For a side without the key nothing is read differently.

- **Rule card, added.** Reads: the side's key; the kind of a plan's owner; the "keep supplied" intents in force directly under it and their forces limits. Writes: nothing; it decides which forces limit `add_run` and `commit_ships` are checked against. Draws: none.
- **Versions:** none moves. The rules label stays 32, since no file on file states the key.
- **Statements:** 4, 8, 9 and 14 rest on it and none is restated. Added, a test of intake: **25.** For a side with the key, a run of a hold's plan is checked against the forces limit of the "keep supplied" in force under the hold (taken where its ships are in that limit, refused where they are not); for a side without the key, with no limit on the hold, it is refused as having no forces limit, as today.
- **Rulebook:** chapter 8, Runs and Holding a place (Its plan) say which limit a run of a hold's plan draws on.

"The engine changes in two places" is replaced by: three, the key, the narrowing check and this.

**Rule 17's bases: confirmed as the developer built it.** The other bases the day's reason names are weighed by rules 1 and 2 without the test of "the source of an order higher in priority", and with the test of a base before its day. The list is there to lead the commander to name a source, and by rule 14 a base he names is not refused because another order draws on it; a base before its day he cannot use at all. So the list is exactly the bases he could name with something to spare. Rule 17 is read so: "each other base of the side, not in the place's hex and not before its day, that has supply to spare".
