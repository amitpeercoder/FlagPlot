# 0240, as built: The order paper reads as the order

## Stage A: the head, the foot and the look (Nov 28, 2026)

What the pull request "order paper, stage A" makes. It changes what the table gives and no rule; what any order sends is as it was. The statements it is held to are A1 to A17 in `the_paper_reads_as_the_order` in `table/check_screen.py`, written before the build.

- **The head line and the heading.** The old heading "ORDER · DRAFT" and the part TO are one small head line, set by the new function `headLine(kind)`: "ORDER TO THE STAFF · DRAFT", "ORDER · DRAFT" for an order to a unit, "CHANGE OF LIMITS · DRAFT" and "GIVEN BACK TO THE STAFF · DRAFT". Under it is the heading. For an order to the staff the new function `draftHeading(e, target)` says the end as the card's button said it, with the place named where the button said "it" or "here" ("Keep Thornwick supplied", "Cover Lunga from the air", "Deny the waters about 9°26′S 160°02′E, within 80 miles", "Operate Task Force 17 from the waters about …"). It is set once when the paper opens and does not follow a blank. For an order to a unit the heading is the unit's link; for a change of limits and a give-back it is the order's own link, and the sentence the unit's part held stays under it. `sayIntent` is not changed.
- **The list of ends is gone.** The fieldset END, its radios and "At <place> you are to:" are removed. The end of a draft is `draft.end`, set in `openDraft` from the card's button; the six places that read the chosen radio read `draft.ends[draft.end]`. The way to another order is the way back to the card. The labels THE ORDER and ACTS are gone with TO.
- **The foot.** `p-foot` holds the line of when the order acts, with no label, then SIGN AND SEND, DRY RUN and TEAR UP. `actsText([])` is now empty rather than a dash, and the line is hidden when empty so that it takes no room. NOT SENT and the question "Tear up this order?" stand directly above the foot. From 760 px wide the foot is sticky at the paper's bottom; narrower it is the last part, as before.
- **The look.** The paper and the card are 400 px wide, and the prompt beside the paper moves with them. One rule under the heading and one above the foot; the dashed rules between parts, and the dashed rule over the question, are gone. A number blank is the paper's own colour with a rule beneath it. The three number blanks (the patrol's hexes, the supply floor and the interval) have a label tied to each by `for`, and keep their `aria-label`.
- **Focus on opening** goes to the first control of the order that is shown, no longer to a hidden one.
- **Not built, as the plan has it:** the reading's move above LIMITS, the supply order's blanks and the limits on the face (stage B) and the empty floor (stage C). The parts' order is still p-limits, p-plan, the reading.
- **Checks restated.** `one_order_paper`'s list of parts; `the_words_and_what_the_papers_say`'s width of `body.papered #prompt`; `the_submarines`'s quoted label of the patrol's hexes (now a `<label>`); `the_supply_orders_two_numbers`'s quoted line in `openDraft` (the end's change no longer calls `showSupply` and `endChanged` from the paper's listener); `your_plan_on_the_order_paper`'s P1, which named `p-acts` and now names `p-acts-t`. Nothing was loosened or removed.

### After the build: two mends by the architect

- **No dashed rule in the paper (A15a).** The rules above a fieldset and above the staff's reading come from `.order`, which the card shares. `#paper fieldset` and `#paper .pv` now set `border-top: 0`, so the paper draws only the rule under its heading and the rule above its foot.
- **The paper of a written landing.** Decision 0232's step 3 brought a new paper to `main` while this stage was built: a landing written under an order in force (`openPaper('landing', …)`). The plan did not have it. It is treated as the change-of-limits paper is: its head line is "A LANDING OF YOUR PLAN · DRAFT", its heading is the order's link, and its sentence stays under the heading. The check's A4 holds both.

Tried by hand on a local table, on rung 1: the supply order at Thornwick ("Keep Thornwick supplied") and a deny on open water ("Deny the waters about (2, 4), within 160 miles"). Not tried by hand: a unit's paper, the change-of-limits, give-back and landing papers, and the 39 days.


## Stage B: the face, the reading and the fold (Nov 28, 2026)

What the pull request "order paper, stage B" makes. It changes what the table gives and no rule; what any order sends is as it was. The statements it is held to are B1 to B13 in `the_face_and_the_fold` in `table/check_screen.py`, written before the build.

- **The reading under the order.** The staff's reading and the dry run's line stand directly under the order and its in-whose-hands part; LIMITS and YOUR PLAN, folded, come after them.
- **The face.** A new part `p-face` in the order's part holds the lines that stand on the face of a kind: for a supply order the ships and the source, for a patrol its boats, for a station order its risk. Five pure functions in the limits model say it: `faceKeys`, `hasFold`, `missing`, `sourceChoices` and `foldLine`. `LIMITS_OFFERED` is not changed.
- **A supply order's blanks.** Ships are ticks for each ship and task force the forces line offers, headed "Ships", and the source a list of the side's other bases headed "From", with an empty first choice. Neither offers the staff's choice, none or a way back to it; the ships are "use only these" from the start. Beside an empty one the paper says "No ship named yet." or "No source named yet." While either is empty the staff is not asked, the reading's place is empty and SIGN AND SEND and DRY RUN wait. What is sent is the forces limit it was. The floor still starts at 30 (stage C).
- **The fold.** The fieldset RISK is gone; the risk of a draft is `lim.risk`, the first line of the fold for every kind that offers it. The fold's closed line is "LIMITS · " and `foldLine`: the risk, then what is set in the fold, or "the rest the staff's choice"; what is on the face is not said again. A patrol and a station order show no fold.
- **The change-of-limits paper** draws its lines on the face and shows no fold. A limit refused is marked at its line, on the face or in the fold, and the fold is opened only where the line is in it.
- **Checks restated.** `the_paper_reads_as_the_order`'s A1 (the order of the reading and the folds is B1's) and `the_supply_orders_two_numbers`'s O1 (its row is cut at `p-face`, not at the risk's fieldset). Nothing was loosened or removed.

Not tried by hand in a browser; the check reads the page's text and runs its functions in node.

### Stage B, tried by hand

On a local table, on rung 1, by the architect: the supply order at Thornwick opens with "Ships" and "From" on its face, each with its sentence beside it, the reading's place empty and SIGN AND SEND disabled; with Merrowdale ticked and Saltmere chosen the reading comes directly under them; the fold reads "LIMITS · Risk: balanced · the rest the staff's choice" and opens with the risk first; the foot stays in view while the paper scrolls. The change-of-limits paper was then tried too, on day 2 with the order in force: "CHANGE OF LIMITS · DRAFT", the order's link as the heading, the risk's line open on the face and no fold. Not tried by hand: a patrol, a station order and the 39 days.

Seen and not mended in this stage: the reading names a force by its handle ("Task_force:1 · load 30 t of supply"), which is older than this work; "This reading is worked with the limits you set." still follows the reading of a supply order whose only limits are its ships and its source; and the list for "From" is still drawn as a dark box. On the change-of-limits paper the reading says "Nothing can serve this order yet." on an evening when the staff has no new order to give under the order, before and after the risk is changed; stage B did not touch that reading, and the words are wrong for that paper.


## The reading's mends (Nov 28, 2026)

Four mends to the reading and one to the look, asked for by the designer after stages A and B were tried by hand. They change what the table says and no rule; what any order sends is as it was. The statements are M1 to M5 in `the_readings_mends` in `table/check_screen.py`, written before the build.

- **A force the order would form** is said "The new force" in the reading's lines, not by its handle: the new function `readingUnit(h)`, used by `unitsView`, answers so for a task force the side does not have. Any other handle is left for the page's links.
- **A force formed** is said "Form a force of 1 ship" or "Form a force of 3 ships" by `planOrderLine`, which had no case for it.
- **The note under a reading**, "This reading is worked with the limits you set.", is said by the new `limitsNote(kind, sent)` in the limits model, and only where a limit outside the kind's face is set. A supply order whose only limits are its ships and its source has no note. The sentence is held in one constant.
- **A change of limits that brings no new order** reads "The staff would give no new order under it now." and not in the colour of a fault: `readingTop` takes a third argument, `own`, which `readingView` passes as `!!own`. A bare order still reads "Nothing can serve this order yet."
- **A list in the paper** (`#paper select`) is drawn as a blank, as a number is: transparent, no border, a rule beneath.
- **A supply order that needs no run yet** (found when stage C was tried by hand): where the staff gives no order because the place holds more than the floor, the reading's head line is no longer "Nothing can serve this order yet." in warning ink. The new `runsLine(w, dayWords)` says it from the reading's `next_run`: "No run is needed yet. The first, of 25 t, would load on day 3 and be due on day 4." `readingView` uses it for a draft only, with a date and the day (as "Not before" says it) where the game has dates, else "day N". Statements M6 and M7.
- **One check restated:** `the_staffs_reading`'s quoted line for the note (it names the sentence in the page and `limitsNote(kind, sent)` in `askPreview`, not the sentence in `askPreview`). Nothing was loosened or removed.

## Stage C: the empty floor (Nov 28, 2026)

What the pull request "order paper, stage C" makes. It changes what the table gives and no rule; a supply order signed with its days, a ship and a source sends what it sent before. The statements are C1 to C8 in `the_empty_floor` in `table/check_screen.py`, written before the build.

- **The blank starts empty.** `o-floor` has no value, `openDraft` empties it, and the end "Keep it supplied" carries no `floor_days`. `supplyFloor()` gives none for an empty box or one that is no number, 1 for 0 and 99 for 150; `addSupplyNumbers` adds `floor_days` only where there is one.
- **The need.** `missing(kind, lim, floor)` takes the floor as its third argument and, for a supply order, names it first: floor, ships, source. Every caller passes `supplyFloor()`. Beside the empty blank the paper says "No days of supply given yet." (`need-floor`), in plain ink; while it is missing the staff is not asked, the reading's place is empty and SIGN AND SEND and DRY RUN wait. A paper left untouched can no longer be signed, where it sent 30 days.
- **The fact of the place.** `holdsLine(name, dos)` says "Thornwick holds 5 days of supply." from the same `days_of_supply` of the base that the place's card reads for its "Days of supply" row; it is the first thing in `o-supply`, in `o-holds`, filled when the paper opens and hidden while empty.
- **Focus on opening** goes to `o-floor` for a supply order.
- **Documents.** The manual's "Keep it supplied, 30 days" and the table README's "30 days where it is left alone" are changed.
- **Checks restated.** `the_supply_orders_two_numbers`'s O1 to O4 (the box's value, the empty floor, the untouched paper's arguments, the reset in `openDraft`), and stage B's B5, B11 and the B10 render's stand-in for `supplyFloor` (B10 draws `needLine`, which now passes `supplyFloor()` to `missing`; the stand-in answers 30 and the statement is unchanged). Nothing was loosened or removed.

Not tried by hand in a browser.

### The mends and stage C, tried by hand

On a local table, on rung 1, by the architect on Nov 28, 2026. The supply order at Thornwick opens with "Thornwick holds 5 days of supply.", the days blank empty with the focus in it, a sentence beside each of the three empty blanks, no reading and SIGN AND SEND disabled. With 2 and 5, Merrowdale and Saltmere, the reading is "No run is needed yet. The first, of 25 t, would load on day 3 and be due on day 4." in plain ink, then "It would wait for your leave to launch.", and no note about limits. With a floor of 30 it is "The staff would give these orders:", "Saltmere · form a force of 1 ship" and "The new force · load 30 t of supply". Emptying the days blank again clears the reading and disables SIGN AND SEND. The list for "From" is drawn as a blank. Not tried by hand: the mend to the change-of-limits reading (M4), and the 39 days.

