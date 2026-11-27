# 0240: The order paper reads as the order

Status: a plan, written by the architect on Nov 27, 2026 at the designer's word ("Plan the order paper"), reviewed that day by a fresh agent and amended from its findings. Needs review: the designer's word on the plan before anything is built.

Summary: The order paper is laid out again so that it reads as the order being given. Its heading is the order in words, the list of ends goes from it, what an order of its kind cannot do without stands on its face as blanks that start empty, the staff's reading stands directly under the order, and when it acts is said beside the button that signs it. For a supply order the days of supply, the ships and the source are such blanks. No rule of the game changes. Three stages; the plan takes class 3's ceremony and each build is class 2.

Standing: A plan, needs review; amends decision 0221, rule 8 and amended rule 12, decision 0222, rule 1, its as-built bullet "The order paper" and the paper's lines in `table/README.md`, decision 0232's as-built order of the parts (check P1), and decision 0237's as-built note on the floor's starting value; nothing built

## Why

The designer played rung 1 on the browser's build on Nov 27, 2026 and said of the paper:

> "I don't think the order - draft page looks very good. It's cluttered with things the user doesn't need at this point. For example, the order type (hold, supply, etc.) was already given at the previous step. I tend to thing that having it here again is confusing. As this screen is pretty much the main interface of this game now, maybe we could make it more readable, accessible with a better, clearer layout."

The architect played the same game through the page that day. What was found, in the order it hurts a new player:

1. **The end is asked twice.** The card's button says "Keep it supplied"; the paper then lists the place's three ends again as a choice (`ends()`, `openDraft`).
2. **What a supply order cannot do without is in a fold that says nothing is needed.** A supply order with no forces limit gets no order from the staff: "has no forces limit, so the staff has no ship to send and gives no order" (`crates/sim-staff/src/doctrine.rs`, `Setup::NoForces`). One that names a ship and no source is refused when it is taken in (`CommandRefusal::NoSource`, `crates/sim-core/src/command/intake.rs`); decision 0228 chooses a source for the staff only under a hold. Yet both lines are in LIMITS, which is closed and reads "The staff's choice in all." (decision 0222, rule 1), and each opens at "The staff's choice", which for this kind means nothing is done. The paper can be signed in that state, after the reading "Nothing can serve this order yet."
3. **The first number is chosen for the player, and wrongly for the game in front of him.** "Never below 30 days of supply" stands in the paper of a base that holds five days, served by a ship that carries six (as-built 0237: "starting at 30").
4. **The staff's answer is the last thing on the paper.** THE STAFF'S READING comes after RISK, LIMITS and ACTS, below what a laptop shows without scrolling, and SIGN AND SEND below that.
5. **Five labels name things he does not decide here:** "ORDER · DRAFT", "TO / The staff", "END", "ACTS", and a dashed rule under each.
6. **The number boxes are dark blocks on the cream paper,** and nothing but scrolling shows that there is more below.

## What the records say now

- **Decision 0221, rule 8:** one paper gives every order, "with the same parts in the same places": TO, the order, IN WHOSE HANDS, ACTS, the staff's reading, SIGN AND SEND and TEAR UP. Amended rule 12: "The parts are the same for every order; the controls between them are the kind's own." Rules 15 and 23: no caption explains a rule; what a player must know before signing "is said as a fact of that order and never as a rule".
- **Decision 0222, rule 1:** "The order's paper gains a part, LIMITS, folded, between the order and when it acts." Rule 13: "A line is offered only where it changes what the staff does for that kind of order." Its as-built bullet: "The risk stays among the order's own choices and is not shown on a patrol."
- **Decision 0232, rule 27:** YOUR PLAN, folded, after LIMITS, on the paper of "seize and hold an island" alone.
- **Decision 0237, as built:** `o-floor` from 1 to 99, "starting at 30"; `o-interval` starting empty, the row saying "left to the staff: 15 days"; an untouched paper sends `floor_days` 30.
- **Decision 0219:** every form offers every order at every stop, and says when it acts.

## The plan

One paper still gives every order, and its parts are still the same for every order. What changes is which parts there are, their order, and what each says.

### The parts, top to bottom

1. **The way back,** as now: "← BACK TO THORNWICK". It is also the way to choose another order: the card it goes back to is where the orders are offered. The paper has no other way to change its end.
2. **The head line,** small capitals: "ORDER TO THE STAFF · DRAFT" for an order to the staff, "ORDER · DRAFT" for an order to a unit, "CHANGE OF LIMITS · DRAFT" and "GIVEN BACK TO THE STAFF · DRAFT" for those two papers. It takes the place of the heading "ORDER · DRAFT" and of the part TO.
3. **The heading.** For an order to the staff, the order in words, by the table below. For an order to a unit, the unit's name, which is the link to its card that TO holds now. For a change of limits and a give-back, the words of the order it is about, as YOUR ORDERS says them.
4. **The order's blanks,** by the kind's own controls: what the order needs (rule N below), then its other choices as now (the hexes of a patrol, a force's speed and course, an air group's mission). For a unit, "Nothing is changed yet." and the costs stay here, as now.
5. **IN WHOSE HANDS,** for a unit, as now.
6. **The staff's reading,** for an order to the staff, open, directly under the order: its head line, its tree and its fold THE STAFF'S WORKING IN FULL as decision 0221 has them, and what a dry run says directly under it.
7. **LIMITS,** folded, with the lines not on the face; **YOUR PLAN,** folded, where decision 0232 has it. Both now come after the reading.
8. **The foot:** when the order acts, in the words ACTS has now and with no label (nothing, where a unit's draft has nothing chosen and ACTS says "—" today); then SIGN AND SEND, DRY RUN where the game has a trial, and TEAR UP. NOT SENT with its lines, and the question "Tear up this order?", are said directly above it. The foot is held at the bottom of the paper while the rest scrolls, where the window is 760 px wide or more; narrower, it is the paper's last part, as now.

The fieldset END, its radios and the sentence "At Thornwick you are to:" go.

### The heading's words

The heading says the end as the card's button said it, with the place named where the button said "it" or "these waters". It is fixed when the paper opens and does not follow a blank.

| Kind | The card's button | The heading |
| --- | --- | --- |
| `hold` | Hold Thornwick | Hold Thornwick |
| `keep_supplied` | Keep it supplied | Keep Thornwick supplied |
| `cover` | Cover it from the air | Cover Thornwick from the air |
| `control_island` | Seize and hold Guadalcanal | Seize and hold Guadalcanal |
| `take_place` | Capture Lunga only | Capture Lunga only |
| `neutralise` | Neutralise Rabaul | Neutralise Rabaul |
| `deny` | Deny these waters, 80 miles | Deny the waters within 80 miles of Lunga |
| `patrol` | Patrol these waters with submarines | Patrol the waters round Lunga with submarines |
| `operate_carriers` | Operate Task Force 17 from here | Operate Task Force 17 from the waters round Lunga |
| `stand_at` | Station Task Force 17 here | Station Task Force 17 in the waters round Lunga |

A place is named as the card's own heading names it (a base's name, or the waters by what they are nearest, as `whereIs` says). The miles of a deny are the button's. The hexes of a patrol are in its blank and not in its heading. These are new words for a draft and are written by one new function; `sayIntent`, which says an order once signed, is not changed, and where the two differ today (the deny's distance) the record of that difference goes into `docs/rules/differences.md` with stage A.

### Rule N: what an order needs is a blank on its face, and starts empty

Where an order of a kind does nothing, or is refused, until a thing is given, that thing is a blank on the face of the paper, under the heading. It starts empty, it offers no "the staff's choice", and the order cannot be signed until it is given. Today there is one such kind:

| Kind | Needs | Why |
| --- | --- | --- |
| `keep_supplied` | the days of supply never to fall below; the ships; the source | the engine refuses a floor of nothing; the staff gives no order with no ship; the engine refuses a ship with no source |

- **The supply order's face** reads: "Never below [ ] days of supply", "Each run brings [ ] days more", "Ships" with a tick for each ship and task force the forces line offers now, and "From" with a list of the side's bases other than the place itself.
- **Ships and From are the forces limit's own lines.** What is sent is what the fold's lines send now (`set_limit`, `limit: "forces"`, `ships`, `task_forces`, `source`). On this kind they are no longer in the fold, and "The staff's choice" and "None" are not offered for them.
- **While a needed blank is empty** the reading is not asked, the reading's place is empty, SIGN AND SEND waits as it does now while a reading is awaited, and beside each empty blank the paper says so in its plain ink: "No days of supply given yet." "No ship named yet." "No source named yet."
- **One fact of the place stands above the numbers:** "Thornwick holds 5 days of supply.", from what the place's card says now. The card is put away while the paper is open, and the number cannot be chosen without it.
- **The interval is not such a blank.** It starts empty as now and its row says "left to the staff: 15 days" as now: the order works without it. On rung 1 that default fails the mission (two full ships starve the source), which is the rung's lesson and what its directive tells him to set. Question 1 puts it to the designer.
- **Nothing else moves under this rule.** A patrol opens at 2 hexes and an order at balanced risk as now: each is a default the order works with. `floor_days: 30` goes from the end in `ends()` and from `supplyFloor`, which read an empty box as 30.

The reason is its own: a number or a ship chosen for the player on a paper he can sign at once is an order he did not give. The lines beside the blanks are facts of this order, said where the blank is, and no caption of a rule (decision 0221, rule 23).

### Rule F: the fold

- LIMITS holds every limit line of the kind that is not on the face under rule N. Decision 0222, rule 13 stands: nothing is offered that changes nothing for the kind, and nothing is hidden by scenario.
- **Risk** becomes the first line of the fold, for every kind that offers it, and leaves the order's own choices. What is sent for it does not change. The fold's closed line always says it: "Risk: balanced · the rest the staff's choice", or with what is set, "Risk: press · Keep within: 200 miles of Lunga". This amends decision 0222's as-built bullet.
- A kind whose fold would hold one line has that line on the face and no fold: Boats on a patrol, Risk on `stand_at`.
- The limits model (`LIMITS_OFFERED` to its marker) is not touched. One new table says which lines of a kind are on the face, and `drawLimits` draws to two places.

### The change-of-limits paper and the give-back paper

Each keeps what it does. Each takes the head line and the heading above. Its sentence, which is in the unit's part today, stays under the heading. The change-of-limits paper draws its one line open on the face, with no fold; its reading and its foot are as every paper's. The give-back paper's reading and foot are as every paper's.

### The look

- The paper and the card keep one width, as now (both are `.order`): `min(400px, calc(100% - 32px))` in place of `min(360px, …)`, and the prompt's offset beside them moves with it.
- One rule under the heading and one above the foot. The dashed rules between parts go; a part is set off by space and by its small label.
- A blank is drawn as a blank on paper: the paper's own colour, a rule beneath it and the typewriter's face, in place of the dark box. Its focus ring is as every control's.
- Each blank has its label tied to it (`<label for>`), and a sentence with a blank in it reads aloud as the sentence.
- On opening, the focus goes to the first empty blank the order needs, else to the first control of the order, as now. Ctrl+Enter signs, Escape tears up or asks, as now.
- Type faces and sizes are as now.

### What does not change

- What any order sends, key for key, but for rule N: an untouched supply paper sent `floor_days` 30 and no limit, and now cannot be signed.
- The words of every limit line, reading and refusal, but for the fold's closed line and the lines beside empty blanks.
- The card's ORDERS YOU CAN GIVE and its "OR ONLY" parts.
- The plan paper, the stop paper, SIGNED and YOUR ORDERS.
- A unit's paper keeps its controls.
- DESTROY THIS FORCE and SHADOW IT are still signed from the contact's card and do not open the paper. That is a difference from decision 0221, rule 8, found in this reading; it goes into `docs/rules/differences.md` with stage A and is not mended here.

## The paper of rung 1, as it opens

With rung 1's names as decision 0241 has them.

```
← BACK TO THORNWICK
ORDER TO THE STAFF · DRAFT
Keep Thornwick supplied

Thornwick holds 5 days of supply.
Never below [    ] days of supply     No days of supply given yet.
Each run brings [    ] days more      left to the staff: 15 days
Ships   [ ] Merrowdale, at Saltmere   No ship named yet.
From    [               v ]           No source named yet.

THE STAFF'S READING

▸ LIMITS · Risk: balanced · the rest the staff's choice
--------------------------------------------------------------
Acts at 1800, with tonight's orders
[ SIGN AND SEND ]   [ TEAR UP ]
```

Filled in (2, 5, Merrowdale, Saltmere), the lines beside the blanks are gone and the reading says what the staff would do and, as now, "It would wait for your leave to launch."

## Stages

The plan takes class 3's ceremony under decision 0233: it amends rules the designer confirmed in decisions 0221 and 0222, so it had an independent review and waits on his word. Each build is class 2: it changes what the table gives and no rule. For each stage the architect writes the check's statements before the build is briefed, a Sonnet developer builds, and the architect is shown the restated checks, the page tried by hand in a browser on rung 1 and on the 39 days (the check reads the page's text and does not draw it), and the pull request.

- **Stage A, the head and the foot.** Parts 1 to 3 and 8, the heading's words, the two smaller papers' heads, and the look. Checks restated: `one_order_paper` (the parts and their order), `the_words_and_what_the_papers_say` (ACTS, and the width it pins in `body.papered #prompt`), `the_fold_and_the_keys`, and the `ends()` lines that `the_submarines` and `the_island_of_two_places` quote, where the list's drawing moves them. `docs/manual.md` and `table/README.md` where they describe TO, END and ACTS.
- **Stage B, the face, the reading and the fold.** Rule N for the ships and the source, parts 4, 6 and 7, and rule F. Checks restated: `the_limits_on_the_paper`, `limits_in_force`, `your_plan_on_the_order_paper` (P1: the order of `p-limits`, `p-plan`, `p-acts`), `the_staffs_reading`, `the_supply_orders_two_numbers` (O1 cuts its row at the risk's fieldset), and the risk's place wherever a check names it. `docs/manual.md` where it says to set the risk.
- **Stage C, the empty floor.** Rule N for the days of supply, and the fact of the place. Checks restated: `the_supply_orders_two_numbers`, statements O1 to O6, with the end's own `floor_days: 30`; and `the_supply_order_signed`, which is in pull request 774 and not yet on `main`, if it rests on the default. `docs/manual.md`, which still says "Keep it supplied, 30 days".

The as-built file for this record is written with each stage.

## What it reads and writes

No rule card: the plan adds no rule, state, draw, event or tool. It reads what the paper reads now (the seat's `preview`, `command_state`, `list_forces`, `list_bases`) and sends what it sends now (`preview`, `approve`, `issue_orders`, `hold`).

## Questions for the designer

1. **Rule N.** A new player meets a supply paper that cannot be signed until he gives the days, the ship and the source, in place of one he can sign at once to no effect. The plan takes the first. Within it: the interval keeps "left to the staff: 15 days", which fails rung 1's mission if he leaves it. The other course is for the interval's empty row to say nothing of a number.
2. **Risk in the fold.** Risk is a choice of some weight in the 39 days and of none in rung 1. The plan puts it first in the fold for every kind, with the fold's closed line always saying it. The other course keeps it on the face wherever the side holds a contact of the enemy, which is a control that comes and goes.
