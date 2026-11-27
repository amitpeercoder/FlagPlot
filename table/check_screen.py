#!/usr/bin/env python3
"""A check, by script, of the table's bridge started with no scenario: the new
game screen and the games it begins (decision 0185).

It starts `table/serve.py` with no scenario, reads the list of scenarios,
begins every game the list offers and plays a little of each, plays a person
against an agent through several days with the person's page calls and the
agent's door both made by script, checks that the sides are kept apart, that
an agent's door holds its address across a new game and that a call held when
a game ends is told so, runs `seat_mcp.py` against a door, and then runs
`table/check_watch.py` for the bridge started with a scenario. It needs
`sim-cli` built (`cargo build --release -p sim-cli`) and is run from the top
of the repository:

    python3 table/check_screen.py

It uses ports 8780 to 8785 (`--port` for the first of six). It prints one line
for each thing checked and stops at the first that does not hold. Every
process it starts it stops. Standard library only. It is run by CI,
with `check_watch.py`.
"""

import argparse
import json
import os
import signal
import subprocess
import sys
import tempfile
import threading
import time
import urllib.error
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
SIDES = ("allied", "japanese")
COUNT = {"held": 0}
# What the page calls through /call when it loads and on every refresh (index.html, refresh()).
PAGE_LOAD_CALLS = [
    ("list_forces", {}),
    ("list_bases", {}),
    ("list_contacts", {}),
    ("command_state", {}),
    ("get_briefing", {"text": False}),
    # On every refresh, not in a watched game: what the seat has queued, for SIGNED (showSigned).
    ("queued", {}),
    # Once at load, not on refresh: the day's reports for the stop paper's SINCE (loadSince). Its day is the
    # day just fought at dusk, or the day in hand inside one.
    ("events", {"day": 1}),
]


# What the page asks at load that a seat whose call is held is not answered (not a watcher's read, decision 0182).
NOT_WHILE_HELD = {"queued"}


def check(holds, words):
    if not holds:
        raise AssertionError(words)
    COUNT["held"] += 1
    print("ok  " + words, flush=True)


def page_scripts_parse():
    """Every inline <script> of the page parses: each one's text is written to a temporary file and
    `node --check` is run on it. A syntax error once passed every other check."""
    import re
    import shutil
    node = shutil.which("node")
    check(node is not None, "node is found, to check that the page's scripts parse")
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    scripts = [body for attrs, body in re.findall(r"<script([^>]*)>(.*?)</script>", text, re.S)
               if "src=" not in attrs and body.strip()]
    check(scripts, "the page has inline scripts")
    with tempfile.TemporaryDirectory() as folder:
        for number, body in enumerate(scripts, 1):
            path = os.path.join(folder, f"script{number}.js")
            with open(path, "w", encoding="utf-8") as out:
                out.write(body)
            done = subprocess.run([node, "--check", path], capture_output=True, text=True)
            check(done.returncode == 0, f"inline script {number} of the page parses" + ("" if done.returncode == 0 else ": " + done.stderr.strip()))


def outcomes_said_one_way():
    """Every outcome the page says goes through one function with its kind (decision 0221, stage 0):
    nothing writes the status line but `say`, and every call names a kind the page knows."""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check("function say(kind, words, home)" in text and "setStatus" not in text,
          "the page says every outcome through say(kind, words), and nothing else writes the status line")
    kinds = set(re.findall(r"\b(\w+): 1", re.search(r"var SAY_KINDS = \{([^}]*)\}", text).group(1)))
    used = set(re.findall(r"\bsay\('(\w+)'", text))
    check(used and used <= kinds, f"every say names a kind of SAY_KINDS ({', '.join(sorted(used))})")


def the_stop_paper():
    """The stop paper (decision 0221, stage 1): one column down the left with a head and a foot that do not
    scroll, its parts in order, the outcomes' line and the controls that run time in its foot; cards and
    forms at the right; the chart's controls at the chart's foot; the tray, the reports panel and the bar
    gone. The page is drawn in a browser and is not run here: these read the page's text."""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    start = text.index('<aside id="stop"')
    column = text[start:text.index("</aside>", start)]
    parts = ['id="sp-head"', 'id="clock"', 'id="clock-state"', 'id="sp-body"', 'id="sp-seats"', "WHY THE CLOCK STOPPED",
             "FOR YOUR DECISION", 'id="sp-since"', 'id="sp-orders"', "YOUR ORDERS", 'id="sp-signed"', "SIGNED", 'id="sp-foot"']
    at = [column.find(p) for p in parts]
    check(all(a >= 0 for a in at) and at == sorted(at),
          "the stop paper holds, in order: the head with the clock and its state, the seats, WHY THE CLOCK STOPPED, FOR YOUR DECISION, SINCE, YOUR ORDERS with SIGNED, the foot")
    foot = column[column.index('id="sp-foot"'):]
    check(all(f'id="{b}"' in foot for b in ("btn-day", "btn-go", "btn-end")) and not any(f'id="{g}"' in column for g in ("status", "btn-intent")),
          "its foot holds the step, Carry on and Read how it ended, and no outcomes' line and no Give an order")
    check(">Next day<" in foot and "'Next pulse'" in text, "the step is named for the game: Next day, or Next pulse where the game is played by pulses")
    css = lambda selector: (re.search(re.escape(selector) + r" \{([^}]*)\}", text) or [None, ""])[1]
    check("flex: none" in css(".sp-head") and "flex: none" in css(".sp-foot") and "overflow-y: auto" in css(".sp-body"),
          "the head and the foot do not scroll; what lies between scrolls as one")
    check("--sp-w: 360px" in text and re.search(r"@media \(min-width: 1440px\) \{[^}]*--sp-w: 400px", text)
          and "@media (max-width: 1099px)" in text and "drawer-open" in text,
          "it is 360 wide, 400 from 1440, and a drawer under 1100")
    check(not any(old in text for old in ('id="plot"', 'class="tray"', 'class="bar"', 'id="btn-in"', "--bar-h", 'id="btn-intent"', 'id="btn-orders"', 'id="pop-orders"',
                                          'id="planchip"', 'id="status"', "$('status')", "#planchip")),
          "the reports panel, the signals tray, the bar at the foot, the in-tray button, Give an order, the Orders menu, the plan chip and the outcomes' line are gone")
    view = text[text.index('<div class="view"'):]
    view = view[:view.index("</div>\n</div>") if "</div>\n</div>" in view else 2000]
    check(all(f'id="{b}"' in view for b in ("btn-plans", "btn-pieces", "btn-labels", "btn-hexes", "btn-view", "btn-lean", "btn-back")) and 'id="btn-above"' not in text
          and "$('btn-view').addEventListener('click', function () { setView(!chartView); });" in text and "'VIEW \\u00b7 ' + (chartView ? '2D' : '3D')" in text
          and "bottom: 16px" in css(".view"),
          "the chart's controls are at its foot: plans, pieces, names, hexes, one button that turns between the table (3D) and the chart from above (2D), and the zoom")
    menus = text[text.index('<div class="menus"'):]
    menus = menus[:menus.index("</div>")]
    check(all(f'id="{b}"' in menus for b in ("btn-staff", "btn-game")) and 'id="btn-orders"' not in menus and 'id="btn-agent"' in text[text.index('id="pop-game"'):],
          "the menus at the top right are Staff and Game, and the agent's seat is under Game")
    check("right: 16px" in css(".order") and "setViewOffset" in text,
          "cards and order forms open at the right, and the chart turns to the middle of what is not covered")
    check("pressed[p.number]" in text and text.count("press('APPROVE PLAN '") == 1 and text.count("press('CANCEL PLAN '") == 1
          and text.index("doc.appendChild(first)") < text.index("doc.appendChild(shown)") and "needSaid[" in text and "taskWords('strong'" in text,
          "the plan's paper gives each plan's APPROVE and CANCEL once, under the plan's line at the top and before the tree, says a need once, and names a task's places")


def orders_and_signed():
    """YOUR ORDERS and SIGNED (decision 0221, stage 2), by the page's text: the orders in force with their empty fact
    and the leave to launch; SIGNED read from `queued` with TAKE BACK through `take_back`; each outcome in its place; the
    drawer open at load, with its chevron and its count, and the two controls that run time in sight with it closed."""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check("YOUR ORDERS" in text and "'No order in force.'" in text and "i.in_force && !i.parent" in text,
          "YOUR ORDERS lists the orders in force, each a link to its plan's paper, and says \"No order in force.\" when there is none")
    check("'Plans launch: ' + (given ? 'when ready' : 'on my word')" in text and "act('set_leave', { leave: 'launch', given: !given }" in text,
          "it says \"Plans launch: when ready\" or \"on my word\", with the switch that gives and withholds the leave")
    check("api('queued').catch(function () { return null; })" in text and "api('take_back', { queue: one.place.queue, number: one.place.number, as_of: q.as_of })" in text
          and "'TAKE BACK'" in text and "NOT TAKEN BACK" in text,
          "SIGNED is read from queued at each refresh, TAKE BACK withdraws one entry by its place and as_of, and a refusal is said beside the entry")
    check("function canSign() { return !WATCH && !M.over && !playing; }" in text and "if (canSign()) {" in text and "(e.goes_with || [])" in text,
          "TAKE BACK is offered only while the clock is stopped and the seat may sign; what goes with an entry is not a line of its own")
    check("function actsWords(e)" in text and "clock.at === 'dusk' ? ACTS_TONIGHT : a.at === 'pulse' ? ACTS_NOW : ACTS_DUSK" in text,
          "each entry says when it acts in the words of ACTS: at dusk \"Acts at 1800, with tonight's orders\", inside a day \"Acts at once\" or \"Acts at 1800, for the coming day\"")
    check("refusedIn(home, words)" in text and "'NOT SENT'" in text and "act(tool, args, done, home)" in text,
          "a refusal is said as NOT SENT in the form, paper or entry that was signed")
    check("else if (kind === 'confirm') $('game-says')" in text and "else if (kind === 'running')" in text and "'clock-doing'" in text
          and "showPrompt(kind, words)" in text and "else if (kind === 'draft') $('sr')" in text,
          "a confirm is said in the GAME menu, what the clock is doing under the head's state, a pick and the first hint in a prompt by the chart's controls, a draft to the live region")
    check("act(tool, { plan: p.number }, done, row)" in text and "tallied[word]" not in text and "el('h3', '', 'The brief')" not in text,
          "the plan's paper has no tally and no brief: the plan's line is one, the tree is the rest")
    paper = text[text.index("// ---- the plan's paper as one tree ----"):text.index("// ---- the dry run (decisions")]
    check("function paperNode(t, bare)" in paper and "t.kids.forEach(function (k) { n.appendChild(paperNode(k)); })" in paper
          and "function proposedTree(top, said, pr)" in paper and "pr.tree" in paper and "tr.sub_intents" in paper and "s.parent" in paper
          and "st.intents.filter(function (c) { return c.parent === i.number; })" in paper,
          "the plan's paper is one tree: a plan that waits from propose's tree (each task under its parent), a plan in force from command_state's intents under the root")
    css = lambda selector: (re.search(re.escape(selector) + r" \{([^}]*)\}", text) or [None, ""])[1]
    check("border-left: 2px solid" in css(".sheet .doc .node") and "node ptask" in paper,
          "each task is a node with the rule down its left, the brief's own look, nested in its parent's")
    check("'order ' + i.number" not in paper and "'Order ' + top.number" in paper and "PAPER_TASKS[n]" in text,
          "a task is named by what it is, never by its number; the root keeps the player's own order number")
    check(paper.count("sm.textContent = 'WHY'") == 1 and "'WEIGHED'" in paper and "'CHOSE'" in paper and "'HOLDS IT BACK'" in paper and "paperArcsDraw()" in paper and "mouseenter" in paper,
          "each task has one fold, WHY, with what was weighed, what was chosen and what holds it back, and its search arc is drawn on the chart while the fold is open or the task is hovered")
    last = text.index("if (working) doc.appendChild(working);")
    check("fold('THE STAFF\\'S WORKING IN FULL')" in text and last > text.index("rk = fold('RISK AND TIMING')") and last < text.index("'WITHDRAW THIS ORDER'", last),
          "the staff's working in full is the last fold, closed, before the withdrawal")
    column = text[text.index('<aside id="stop"'):text.index("</aside>", text.index('<aside id="stop"'))]
    check('id="sp-chev"' in column and "The stop paper is closed: open it" in text and "The stop paper is open: close it" in text
          and "' for your decision'" in text and "role=\"img\"" in column,
          "the head has a chevron whose label says the paper is open or closed, and the count of what waits for a decision")
    check("function atStop() { setDrawer(true); focusStop(); }" in text and text.count("atStop();") >= 4 and "function startRun() { setDrawer(false);" in text,
          "the drawer opens at load and at every stop of the clock; the clock's starting closes it")
    check(re.search(r"\.stoppaper \.sp-body, \.stoppaper \.fold \{ display: none; \}", text) and not re.search(r"\.stoppaper \.sp-foot \{ display: none", text)
          and 'id="sp-foot"' in column and 'id="btn-day"' in column and 'id="btn-go"' in column,
          "with the drawer closed the two controls that run time stay in sight under the head: the same buttons of the foot")


def one_order_paper():
    """The one order paper (decision 0221, stage 3), by the page's text: the three forms and the forms inside the cards
    are folded into one paper with the same parts in the same places; it opens in the card's place with a way back, asks before
    Escape tears up a draft with something chosen, lets the chart draw what is drafted while it is open, says NOT SENT line by
    line, takes the hold back where nothing of the order went, and gives an air group's mission and arc as one entry under
    SIGNED. (Tried in a browser, not here.)"""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check('<form class="order gives" id="paper"' in text and not any(f'id="{old}"' in text for old in ("order", "force", "course"))
          and "'airorder'" not in text and "'landorder'" not in text and "function orderForce(" not in text and "function closeForce(" not in text and "function closeOrder(" not in text,
          "one order paper: the forms #order, #force and #course and the forms inside the air group's and the land unit's cards are gone")
    check(text.count("openPaper('force'") == 1 and text.count("openPaper('staff'") == 1 and text.count("openPaper('air'") == 1 and text.count("openPaper('land'") == 1,
          "every way to an order opens the one paper: a force's card, an air group's, a land unit's, and a card's orders you can give")
    start = text.index('<form class="order gives" id="paper"')
    form = text[start:text.index("</form>", start)]
    parts = ['id="p-head"', 'id="p-title"', 'id="p-the"', "IN WHOSE HANDS", 'id="o-pv"', "THE STAFF'S READING", 'id="p-says"', 'id="p-acts-t"', "SIGN AND SEND", "TEAR UP"]
    at = [form.find(p) for p in parts]
    check(all(a >= 0 for a in at) and at == sorted(at), "the paper's parts are in order: the heading, the order, IN WHOSE HANDS, the staff's reading, what was sent, when it acts, SIGN AND SEND, TEAR UP")
    check('id="o-pv" open' in form and "$('o-pv').open = true" in text and 'id="p-dry"' in form and "OFFERED.trial" in text,
          "the staff's reading is shown open, with DRY RUN where the game has a trial")
    check("function actsText(orders)" in text and "return ACTS_NOW;" in text and "return ACTS_DUSK;" in text and "return ACTS_TONIGHT;" in text and "ACTS_NOW + ': ' + names(now)" in text,
          "ACTS says before signing when the order acts: at dusk with tonight's orders, inside a day at once or at 1800 for the coming day, or each part by itself")
    check("function handsText(d)" in text and "STAFF_HANDS = 'you keep it out of the staff\u2019s plans'" in text and "'The staff\u2019s now. Yours once signed: '" in text,
          "IN WHOSE HANDS says, for a unit, whose it is now and whose it will be once signed, in decision 0216's words")
    check('id="p-back"' in form and "'\u2190 BACK TO '" in text and "from.go()" in text and "from: from" in text,
          "the paper has a way back to the card it was opened from")
    check("function paperAsk(words, yes)" in text and "'KEEP WRITING'" in text and "function tearUpAsking(then, words, back)" in text and "if (!paperChosen())" in text
          and "else if (paper) { if ($('p-says').querySelector('.ask'))" in text and "tearUpAsking(null, null, true)" in text,
          "Escape tears up a draft with nothing chosen, and asks in the paper where something is chosen")
    check("arcDraft && (!$('card').hidden || paper)" in text and "marchDraft && (!$('card').hidden || paper)" in text and "$('card').hidden && !paper &&" in text
          and "else if (!picking && !paper && !playing" in text and "if (draft) ring(draft.v" in text and "force && force.route" in text,
          "the chart draws what is drafted while the paper is open (a course, an arc, a march, the deny circle) and a touch on it opens no card")
    check("function paperLines(lines)" in text and "el('b', '', 'NOT SENT')" in text and "' \\u2014 '" in text and "ORDER_LABEL" in text and "createTextNode('sent.')" in text,
          "a refusal keeps the paper open and says NOT SENT line by line: \"Speed: sent.\" and \"Course: NOT SENT\" with the engine's reason")
    check("function takeBackHold(p)" in text and "e.command.kind === 'hold'" in text and "api('take_back', { queue: mine.place.queue" in text and "nothing && p.holdSent ? takeBackHold(p)" in text,
          "where nothing of a unit's order went, the hold sent before it is taken back with take_back")
    check("also[arc.place.queue" in text and "x.order.kind === 'set_search_arc' && x.unit === e.unit" in text and "var group = [e].concat(also[" in text,
          "an air group's mission and its search arc, signed together, are one entry under SIGNED, taken back together")
    check(not any(w in text for w in ("BEFORE YOU SIGN", "f-when", "your direct order takes it out of the staff")),
          "the captions before signing are gone: what a unit's order does to whose hands it is in is said as a fact, in IN WHOSE HANDS")


def stops_speak_as_the_staff():
    """The stops of the clock speak as the commander's own staff (decision 0221, stage 2): "we" and "our" for the side, "you"
    only for what is the commander's to do; an aircraft type is said as the cards say it (the maker left off, a Japanese
    type cut after its designation), by the page's text."""
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    told = text[text.index("var TOLD = {"):text.index("\n", text.index("var TOLD = {"))]
    check("'We are attacked'" in told and "'Our strike is away'" in told and "'Our men are ashore, or fighting'" in told
          and not any(w in told for w in ("You ", "Your ")), "what the clock was stopped for is said as \"we\" and \"our\", never \"you\" for the side")
    check("'We ask your leave.'" in text and "'A plan waits for your leave.'" in text and "'our reinforcements '" in text and "the commander's/g, 'your'" in text,
          "a leave asked or waited for is the commander's own: \"your\"; the side's reinforcements are \"our\"")
    check("function toldHead(t)" in text and "'We have lost '" in text and "' is on fire among her aircraft'" in text and "'Our men are ashore at '" in text
          and "'Our men are fighting at '" in text and "'We have a contact: '" in text and "' bombed'" in text and "events: told.map(" in text,
          "where a slip's own events say which it is, its heading says so (a ship lost or out of action, men ashore or fighting, the contact, the base bombed); the general heading stays for a mix")
    check("function shortType(name)" in text and "' by ' + shortType(seen[1]) + ' aircraft'" in text,
          "an aircraft type in a stop's line is the short designation, not the data's long name")


def cards_show_no_handle():
    """A contact's HOW WE KNOW is said in the table's words, not the engine's raw lines (decision 0221, stage 2), by the
    page's text: its rows are a when and the words, units by name, places linked, and a handle of the form word:number is
    never put on a card (the page is drawn in a browser; the same was read there for every kind of card)."""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check("howWeKnow(how, " in text and "listOf(how, 'HOW WE KNOW'" not in text and ".replace(/\\bcontact:\\d+/g, 'it')" in text,
          "a contact's HOW WE KNOW is built by howWeKnow, which has no handle of its own: a contact is \"it\"")
    css = (re.search(r"#card \.how \.hrow \.words \{([^}]*)\}", text) or [None, ""])[1]
    check("text-align: left" in css and "#card .how .hrow .when { display: block;" in text,
          "each row is the when on its own small line and the words beneath it, left-aligned, with no figure column")


def the_plan_paper_has_no_rule_caption():
    """The plan's paper (the plan as one tree): no caption at its foot explains how a plan is changed (agreed Nov 19, 2026);
    inside a day it says only when it was worked."""
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check("You change a plan by instruction" not in text
          and "if (!WATCH && a[2].inside_day) doc.appendChild(el('div', 'foot', 'Worked now, inside the day; the staff works it again at 1800.'));" in text,
          "the plan's paper has no caption explaining how a plan is changed; inside a day its foot says when it was worked")


def the_trail_holds_no_duplicate():
    """The trail of cards (decision 0221, stage 2), by the page's text: a card is the thing it shows by its handle; opening the
    card of the thing already shown changes nothing in the trail; the trail never holds one thing twice running; stepping
    replaces the card; a link from the stop paper starts a new trail. (Tried in a browser, not here.)"""
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    check("function remember(f, keyOf)" in text and "same = !!key && key === shownKey;" in text
          and "trail[trail.length - 1].key === shownKey" in text,
          "opening the card of the thing whose card is open leaves the trail as it is, and the trail never holds one thing twice running")
    check("while (key && trail.length && trail[trail.length - 1].key === key) trail.pop();" in text,
          "BACK never leads to the thing about to be shown, as after a form closed its card and the thing is touched again")
    check("stepping = true; try { stepStackIn(by); }" in text and "if (!stepping && viaStop && !(same && !$('card').hidden)) trail = [];" in text and "closest('#stop')" in text,
          "stepping through a stack replaces the card without growing the trail, and a link from the stop paper starts a new trail")
    check(all("remember(" + c + ", function" in text for c in ("cardContact", "cardForce", "cardBase", "cardPlace", "cardHex", "cardUnit", "cardEntity", "cardGone")),
          "every way a card opens names the thing it shows")


def the_paper_reads_as_the_order():
    """Decision 0240, stage A: the order paper's head and foot, by the page's text and by the page's own functions run in node.
    Written by the architect before the build; the expected words here are his and are changed by nobody else. (The page as drawn is
    tried in a browser, not here.)

    The parts.
    A1. Inside `<form class="order gives" id="paper"`, in this order: `id="p-back"`, `id="p-head"`, `id="p-title"`, `id="p-the"`,
        `id="p-hands"`, `id="p-limits"`, `id="p-plan"`, `id="o-pv"`, `id="o-dry-says"`, `id="p-says"`, `id="p-foot"`. Inside
        `p-foot`, in this order: `id="p-acts-t"`, `id="p-sign"`, `id="p-dry"`, `id="p-cancel"`. (The reading moves above LIMITS in
        stage B, not here.)
    A2. Gone from the page's text: `id="p-to"`, `id="p-to-name"`, `id="o-ends"`, `id="o-place"`, `name = 'end'`, `chosen('end')`,
        "you are to:", `<legend>END</legend>`, the labels `<span class="pl">TO</span>`, `<span class="pl">THE ORDER</span>` and
        `<span class="pl">ACTS</span>`, and the heading `<h2>ORDER &middot;`. `id="p-acts"` is gone; `id="p-acts-t"` stays, in the foot.
    A3. The end of a draft is kept in the draft itself: the page's text holds `draft.end = first || 0` in `openDraft`, and every
        place that read the chosen radio reads `draft.ends[draft.end]`.

    The head line: the page's own `headLine(kind)`, cut and run in node.
    A4. headLine('staff') is "ORDER TO THE STAFF \u00b7 DRAFT"; headLine('force'), ('air'), ('land') and ('boat') are each
        "ORDER \u00b7 DRAFT"; headLine('limit') is "CHANGE OF LIMITS \u00b7 DRAFT"; headLine('giveback') is "GIVEN BACK TO THE STAFF
        \u00b7 DRAFT". headLine('landing'), the paper of a landing written under an order in force (decision 0232, step 3, which came
        to main while this stage was built), is "A LANDING OF YOUR PLAN \u00b7 DRAFT", and `openLanding` puts the order's link in its
        heading. `openPaper` sets `$('p-head').textContent = headLine(kind)`.

    The heading: the page's own `draftHeading(e, target)`, cut and run in node with `round` and `cleanName` cut from the page beside
    it, `showHexes` false and `hexMiles` standing in as a function that returns 40. `e` is an entry of `ends(target)`:
    [words, the order's arguments, ...]. `target` is {kind, name, ...}.
    A5. e = ['Hold Placeholder base T', {intent_kind: 'hold', base: 2}], target named "Placeholder base T": "Hold Base T".
    A6. e = ['Keep it supplied', {intent_kind: 'keep_supplied', base: 2}], the same target: "Keep Base T supplied". With a target named
        "Thornwick": "Keep Thornwick supplied".
    A7. e = ['Cover it from the air', {intent_kind: 'cover', base: 2}], target named "Lunga": "Cover Lunga from the air".
    A8. e = ['Seize and hold Guadalcanal', {intent_kind: 'control_island', island: 1}]: "Seize and hold Guadalcanal".
        e = ['Capture Lunga only', {intent_kind: 'take_place', base: 3}]: "Capture Lunga only".
        e = ['Neutralise Rabaul', {intent_kind: 'neutralise', target: 7}]: "Neutralise Rabaul". (Each is the end's own words, cleaned.)
    A9. target = {kind: 'waters', name: 'the waters about 9\u00b026\u2032S 160\u00b002\u2032E'}:
        e = ['Deny these waters, within 80 miles', {intent_kind: 'deny', distance: 2}]: "Deny the waters about 9\u00b026\u2032S
        160\u00b002\u2032E, within 80 miles"; with distance 4: "..., within 160 miles".
        e = ['Patrol these waters with submarines', {intent_kind: 'patrol'}]: "Patrol the waters about 9\u00b026\u2032S
        160\u00b002\u2032E with submarines" (no distance: it is the paper's blank).
        e = ['Operate Task Force 17 from here', {intent_kind: 'operate_carriers', task_force: 17}]: "Operate Task Force 17 from the
        waters about 9\u00b026\u2032S 160\u00b002\u2032E".
        e = ['Station Task Force 44 here', {intent_kind: 'stand_at', task_force: 44}]: "Station Task Force 44 in the waters about
        9\u00b026\u2032S 160\u00b002\u2032E".
    A10. No heading holds the word "Placeholder", "it" as a word by itself, "these waters" or "here".
    A11. In the page's text: `openDraft` sets `$('p-title').textContent = draftHeading(` once; for a unit's paper `openPaper` puts the
        unit's link in the heading, `$('p-title').appendChild(refTo(o.unit, o.name))`; for a change of limits and a give-back the
        heading is the order's own link, `orderLink(`, and "'Change the limits of '" is still in the page.

    The foot.
    A12. `actsText([])` is "" (it was "\u2014"), by the page's own `actsText` run in node as `the_submarines` runs it; the other
        answers of `actsText` are as they were. The page's text holds `$('p-acts-t').hidden = !` so that an empty line takes no room.
    A13. The style sheet holds a rule for `#paper .pfoot` with `position: sticky` and `bottom: 0` and a `background`, inside
        `@media (min-width: 760px)`; outside it `.pfoot` is not sticky.

    The look.
    A14. `.order`'s width is `min(400px, calc(100% - var(--sp-w) - 48px))`, and `body.papered #prompt` has
        `right: calc(min(400px, 100% - var(--sp-w) - 48px) + 32px)`. Neither holds 360px.
    A15. `#paper .pp` has no `border-top`. `#paper .phead` has a `border-bottom` or `#paper #p-title` has one (one rule under the
        heading), and `#paper .pfoot` has a `border-top` (one rule above the foot). No other rule of `#paper` holds "dashed".
        A15a. `#paper fieldset` and `#paper .pv` each have `border-top: 0`: the dashed rule those take from `.order`, which the
        card shares, is not drawn in the paper.
    A16. A number blank of the paper (`#paper input[type=number]`) has `background: transparent`, `border: 0` and a `border-bottom`.
    A17. Each of `o-floor`, `o-interval` and `o-dist` has a `<label for="...">` in the paper, and keeps its `aria-label`.

    What does not change: every statement of `one_order_paper` but its list of parts (restated by A1 and A2), the ACTS words of
    `the_words_and_what_the_papers_say`, and what any order sends. `the_supply_orders_two_numbers`, `the_submarines` and
    `the_island_of_two_places` hold as they are but for the page lines they quote that this stage moves."""
    import re
    print("-- the order paper reads as the order: its head and its foot", flush=True)
    text = page_text()
    start = text.index('<form class="order gives" id="paper"')
    form = text[start:text.index("</form>", start)]
    # A1
    parts = ['id="p-back"', 'id="p-head"', 'id="p-title"', 'id="p-the"', 'id="p-hands"', 'id="p-limits"', 'id="p-plan"', 'id="o-pv"', 'id="o-dry-says"', 'id="p-says"', 'id="p-foot"']
    at = [form.find(p) for p in parts]
    check(all(a >= 0 for a in at) and at == sorted(at), "A1: the paper's parts are in order: the way back, the head line, the heading, the order, in whose hands, LIMITS, YOUR PLAN, the reading, the dry run's line, what was sent, the foot")
    foot = form[form.index('id="p-foot"'):]
    fat = [foot.find(p) for p in ('id="p-acts-t"', 'id="p-sign"', 'id="p-dry"', 'id="p-cancel"')]
    check(all(a >= 0 for a in fat) and fat == sorted(fat), "A1: the foot holds, in order, when the order acts, SIGN AND SEND, DRY RUN and TEAR UP")
    # A2
    gone = ['id="p-to"', 'id="p-to-name"', 'id="o-ends"', 'id="o-place"', "name = 'end'", "chosen('end')", "you are to:", "<legend>END</legend>",
            '<span class="pl">TO</span>', '<span class="pl">THE ORDER</span>', '<span class="pl">ACTS</span>', "<h2>ORDER &middot;", 'id="p-acts"']
    check(not [g for g in gone if g in text] and 'id="p-acts-t"' in foot,
          "A2: the part TO, the list of ends with its radios, the sentence \"you are to:\", the labels TO, THE ORDER and ACTS and the old heading are gone; the line that says when it acts stays, in the foot")
    # A3
    open_draft = cut_function(text, "  function openDraft(best, first) {")
    check("draft.end = first || 0" in open_draft and "draft.ends[draft.end]" in text and "draft.ends[parseInt(" not in text,
          "A3: the end of a draft is kept in the draft, and everything that read the chosen radio reads it there")
    # A4
    head = run_in_node(cut_function(text, "  function headLine(kind) {")
                       + "\nfunction main(ks) { return ks.map(headLine); }", ["staff", "force", "air", "land", "boat", "limit", "giveback", "landing"])
    check(head == ["ORDER TO THE STAFF \u00b7 DRAFT", "ORDER \u00b7 DRAFT", "ORDER \u00b7 DRAFT", "ORDER \u00b7 DRAFT", "ORDER \u00b7 DRAFT",
                   "CHANGE OF LIMITS \u00b7 DRAFT", "GIVEN BACK TO THE STAFF \u00b7 DRAFT", "A LANDING OF YOUR PLAN \u00b7 DRAFT"]
          and "$('p-title').appendChild(orderLink(order))" in cut_function(text, "  function openLanding(order, hex) {"), f"A4: the head line says what the paper is ({head})")
    check("$('p-head').textContent = headLine(kind)" in cut_function(text, "  function openPaper(kind, o) {"), "A4: opening the paper sets the head line")
    # A5 to A10
    waters = "the waters about 9\u00b026\u2032S 160\u00b002\u2032E"
    base_t, thorn, lunga, wt = {"kind": "own", "name": "Placeholder base T"}, {"kind": "own", "name": "Thornwick"}, {"kind": "own", "name": "Lunga"}, {"kind": "waters", "name": waters}
    cases = [
        (["Hold Placeholder base T", {"intent_kind": "hold", "base": 2}], base_t, "Hold Base T"),
        (["Keep it supplied", {"intent_kind": "keep_supplied", "base": 2}], base_t, "Keep Base T supplied"),
        (["Keep it supplied", {"intent_kind": "keep_supplied", "base": 2}], thorn, "Keep Thornwick supplied"),
        (["Cover it from the air", {"intent_kind": "cover", "base": 2}], lunga, "Cover Lunga from the air"),
        (["Seize and hold Guadalcanal", {"intent_kind": "control_island", "island": 1}], lunga, "Seize and hold Guadalcanal"),
        (["Capture Lunga only", {"intent_kind": "take_place", "base": 3}], lunga, "Capture Lunga only"),
        (["Neutralise Rabaul", {"intent_kind": "neutralise", "target": 7}], lunga, "Neutralise Rabaul"),
        (["Deny these waters, within 80 miles", {"intent_kind": "deny", "distance": 2}], wt, "Deny " + waters + ", within 80 miles"),
        (["Deny these waters, within 160 miles", {"intent_kind": "deny", "distance": 4}], wt, "Deny " + waters + ", within 160 miles"),
        (["Patrol these waters with submarines", {"intent_kind": "patrol"}], wt, "Patrol " + waters + " with submarines"),
        (["Operate Task Force 17 from here", {"intent_kind": "operate_carriers", "task_force": 17}], wt, "Operate Task Force 17 from " + waters),
        (["Station Task Force 44 here", {"intent_kind": "stand_at", "task_force": 44}], wt, "Station Task Force 44 in " + waters),
    ]
    code = (cut_function(text, "  function cleanName(s)") + "\n" + cut_function(text, "  function round(n)") + "\n" + cut_function(text, "  function draftHeading(e, target) {")
            + "\nfunction main(cs) { return cs.map(function (c) { return draftHeading(c[0], c[1]); }); }")
    said = run_in_node(code, [[c[0], c[1]] for c in cases])
    for (e, tgt, want), got in zip(cases, said):
        check(got == want, f"A5-A9: {e[1]['intent_kind']} at {tgt['name']} is headed \"{want}\" ({got})")
    check(not [s for s in said if "Placeholder" in s or "these waters" in s or "here" in s or re.search(r"\bit\b", s)],
          "A10: no heading holds \"Placeholder\", \"it\" by itself, \"these waters\" or \"here\"")
    # A11
    check(text.count("$('p-title').textContent = draftHeading(") == 1 and "$('p-title').textContent = draftHeading(" in open_draft,
          "A11: openDraft sets the heading to the end's words once")
    open_paper = cut_function(text, "  function openPaper(kind, o) {")
    check("$('p-title').appendChild(refTo(o.unit, o.name))" in open_paper and "$('p-title').appendChild(orderLink(" in cut_function(text, "  function openLimitPaper(root, key, back) {")
          and "$('p-title').appendChild(orderLink(" in cut_function(text, "  function openGiveBack(i) {") and "'Change the limits of '" in text,
          "A11: a unit's paper is headed by the unit's link, and a change of limits and a give-back by the order's link")
    # A12
    acts = run_in_node(text[text.index("  var ACTS_TONIGHT"):text.index("\n", text.index("  var ACTS_TONIGHT"))] + "\n" + text[text.index("  var INSIDE_KINDS"):text.index("\n", text.index("  var INSIDE_KINDS"))]
                       + "\nvar ORDER_LABEL = { station: 'Station' }; var clock = { at: 'dusk' };\n" + cut_function(text, "function labelOf(o)") + "\n" + cut_function(text, "function actsText(orders)")
                       + "\nfunction main(x) { const o = [{ unit: 'submarine:1', order: { kind: 'station', hex: [1, 1] } }]; const out = [actsText([]), actsText(o)]; clock = { at: '1200' }; out.push(actsText(o)); return out; }", None)
    check(acts == ["", "Acts at 1800, with tonight\u2019s orders", "Acts at 1800, for the coming day"] and "$('p-acts-t').hidden = !" in text,
          f"A12: with nothing chosen the line of when it acts is empty and takes no room, and says what it said otherwise ({acts})")
    # A13 to A16, from the style sheet
    css = re.sub(r"/\*.*?\*/", "", text[text.index("<style"):text.index("</style>")], flags=re.S)
    rules = re.findall(r"([^{}]+)\{([^{}]*)\}", css)
    def decl(sel_has):
        return [(s.strip(), b) for s, b in rules if sel_has(s)]
    media = css[css.index("@media (min-width: 760px)"):]
    media = media[:media.index("}", media.index("#paper .pfoot") + 20) + 1]
    check("#paper .pfoot" in media and "position: sticky" in media and "bottom: 0" in media and "background:" in media, "A13: from 760 px the foot is held at the paper's bottom")
    outside = css.replace(media, "")
    check(not any("sticky" in b for s, b in re.findall(r"([^{}]+)\{([^{}]*)\}", outside) if ".pfoot" in s), "A13: below 760 px the foot is not sticky")
    check(re.search(r"\.order \{[^}]*width: min\(400px, calc\(100% - var\(--sp-w\) - 48px\)\)", css) is not None
          and "body.papered #prompt { right: calc(min(400px, 100% - var(--sp-w) - 48px) + 32px); }" in css
          and not any("360px" in b for s, b in rules if s.strip() == ".order") and not any("360px" in b for s, b in rules if "#prompt" in s),
          "A14: the paper and the card are 400 px wide and the prompt lies beside them")
    pp = [b for s, b in rules if s.strip() == "#paper .pp"]
    check(pp and not any("border-top" in b for b in pp), "A15: a part of the paper has no rule above it")
    check(any("border-bottom" in b for s, b in rules if s.strip() in ("#paper .phead", "#paper #p-title")) and any("border-top" in b for s, b in rules if s.strip() == "#paper .pfoot"),
          "A15: one rule under the heading and one above the foot")
    check(not any("dashed" in b for s, b in rules if "#paper" in s), "A15: no rule of the paper is dashed")
    check(all(any("border-top: 0" in b for s, b in rules if s.strip() == sel) for sel in ("#paper fieldset", "#paper .pv")),
          "A15a: the paper draws no rule above a fieldset or above the staff's reading")
    num = [b for s, b in rules if s.strip() == "#paper input[type=number]"]
    check(num and "background: transparent" in num[0] and "border: 0" in num[0] and "border-bottom" in num[0], "A16: a number blank is drawn as a blank, transparent with a rule beneath it")
    for i in ("o-floor", "o-interval", "o-dist"):
        tag = re.search(r'<input type="number" id="%s"[^>]*>' % i, form).group(0)
        check(f'<label class="fl" for="{i}">' in form and "aria-label=" in tag, f"A17: {i} has a label tied to it and keeps its aria-label")


def the_fold_and_the_keys():
    """What stops the clock and the keys (decision 0221, stage 4), by the page's text: the fold in the foot with the
    table's ticks and the staff's standing interrupts; the ticks kept in the browser for the game, keyed as the directives
    are, and forgotten by New game and Restart; why the clock stopped kept across a reload; one Escape, in its order;
    the focus at a stop; SINCE's count while the clock runs; a card that is not a live region."""
    import re
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    foot = text[text.index('id="sp-foot"'):text.index("</footer>", text.index('id="sp-foot"'))]
    check('<details id="stopfold"' in foot and "WHAT STOPS THE CLOCK" in foot and foot.index('id="stopfold"') < foot.index('id="btn-day"'),
          "the fold WHAT STOPS THE CLOCK is in the stop paper's foot, above the controls that run time")
    for tick in ("contact", "attack", "submarine", "loss", "strike", "landing", "arrive", "evening"):
        check(f'id="stop-{tick}"' in foot, f"the fold holds the table's tick for {tick}")
    check("each evening" in foot and re.search(r"evening: false", text) and re.search(r"TICK_DEFAULT = \{[^}]*contact: true, attack: true, submarine: true, loss: true, strike: false, landing: false, arrive: false, evening: false", text),
          "each evening is in the fold and off by default; a contact, an attack, a submarine's attack and a loss are ticked, and the rest not")
    check("eveningStop()" in text and text.count("eveningStop()") >= 4 and "stopEvening" in text, "Carry on stops each evening where that is ticked, in the pulse, day and watched loops alike")
    check('id="stops-staff"' in foot and all(k in text for k in ("cannot_meet", "waits_for_leave", "ship_lost", "cannot_bring_enough", "lift_turned_back", "asks_leave_to_pursue")) and "var STAFF_STOP" in text,
          "the staff's six standing interrupts are listed in the fold")
    check("interrupts.switched_off" in text and "act('set_interrupt', { standing: k, on: c.checked }" in text,
          "each staff tick is read from command_state's switched_off and cleared and set by set_interrupt")
    check("'from 1800'" in text and "M.queue.at !== 'dusk'" in text, "a staff tick changed inside a day says from 1800")
    check("btn-stops" not in text and "id=\"stops\" hidden" not in text and "THE CLOCK STOPS WHEN I AM TOLD OF" not in text, "the old panel and its button are gone")
    check("button(a, 'DO NOT STOP ME FOR THIS'" in text and 'id="btn-leave' not in foot and "leave" not in foot.split('id="stops-staff"')[0].lower().replace("my leave", ""),
          "a slip's DO NOT STOP ME FOR THIS stays, and the leave to launch is not in the fold")
    check("function keptKey(name)" in text and "KEPT_ROOT.slice('flagplot-directives'.length)" in text and "localStorage.setItem(keptKey('stops')" in text,
          "the table's ticks are kept in the browser, keyed as the directives are")
    check(text.count("forgetKept();") >= 2 and "localStorage.removeItem(keptKey('stops'))" in text and "localStorage.removeItem(keptKey('why'))" in text,
          "New game and Restart forget the ticks and the kept WHY")
    check(all(re.search(fn + r"[^\n]*try \{", text) or "try {" in text[text.index("function " + fn):text.index("function " + fn) + 400] for fn in ("recallTicks", "keepTicks", "forgetKept", "keepWhy", "forgetWhy", "recallWhy")),
          "every access to the browser's storage for them is in a try, so a page that cannot keep them plays as before")
    check("function keepWhy()" in text and "function recallWhy()" in text and "k.day !== clock.day || k.at !== clock.at" in text and text.count("keepWhy();") >= 4 and "forgetWhy();" in text.split("function startRun()")[1][:300],
          "WHY is kept at each stop and read again at a reload at the same stop, and is dropped when the clock starts or has moved")
    escapes = re.findall(r"e\.key\s*[!=]==\s*'Escape'", text)
    check(len(escapes) == 1 and text.count("addEventListener('keydown', escapeOnce)") == 1, "exactly one listener answers Escape")
    body = text[text.index("function escapeOnce"):text.index("document.addEventListener('keydown', escapeOnce)")]
    order = ["picking === 'route'", "$('sheet').hidden", "else if (paper)", "$('card').hidden", "MENUS.some", "$('stopfold').open", "drawer-open"]
    at = [body.find(o) for o in order]
    check(all(a >= 0 for a in at) and at == sorted(at), "Escape answers, in order: a place being picked, a paper for reading, a draft, a card, a menu, the fold, the drawer")
    check("tearUpAsking(null, null, true)" in body and "wantCard = true" in text, "an empty draft goes back to the card it came from")
    check("function focusStop()" in text and "function atStop() { setDrawer(true); focusStop(); }" in text and "if (paper || playing" in text,
          "at a stop the focus goes to what waits or to WHY, and is never taken from a draft")
    check("addEventListener('focusin'" in text and "openedBy[id]" in text, "a card or paper that closes gives the focus back to what opened it")
    check('id="since-count" class="sr" aria-live="polite"' in text and "function sayCount()" in text and "sayCount(); if (!calm)" in text and 'id="since" aria-live="off"' in text,
          "while the clock runs SINCE says a count, once for each pulse, and not each line")
    check('id="clock-doing" aria-live' not in text and "function sayState()" in text and "stateSaid" in text, "the clock's state is said once at each change")
    card = re.search(r'<aside[^>]*id="card"[^>]*>', text).group(0)
    check("aria-live" not in card and "setAttribute('aria-label', String(title))" in text, "a card is not one live region: its title is its label, said once")
    check("e.ctrlKey || e.metaKey" in text and "p-sign').click()" in text, "an order paper is signed from the keyboard from anywhere in it")


# What the page's script is given to run its own wording in node: the page's own functions cut from its text, with the few
# things they read from the rest of the page stood in (the names the table holds, no map, a document that only collects text).
NODE_PRELUDE = r"""
const M = { names: { 'ship:1': 'USS Enterprise', 'base:1': 'Lunga' }, hexNames: {} }, hexAt = {}, showHexes = false;
function hexMiles() { return 40; } function whereIs() { return ''; }
class Node { constructor(tag, cls) { this.tag = tag; this.className = cls || ''; this.kids = []; this.own = ''; this.attrs = {}; }
  appendChild(k) { this.kids.push(k); return k; } setAttribute(k, v) { this.attrs[k] = v; }
  set textContent(v) { this.kids = []; this.own = v; } get textContent() { return this.own + this.kids.map(k => k.textContent).join(''); } }
const made = {}; const $ = id => made[id] || (made[id] = new Node('div'));
function el(tag, cls, text) { const n = new Node(tag, cls); if (text != null) n.own = text; return n; }
const document = { createTextNode: t => { const n = new Node('#text'); n.own = t; return n; } };
"""


def run_in_node(code, data):
    """Runs `code` (after the prelude) in node with `data` as its JSON argument and returns what it printed as JSON."""
    import shutil
    node = shutil.which("node")
    check(node is not None, "node is found, to run the page's own wording")
    with tempfile.TemporaryDirectory() as folder:
        path = os.path.join(folder, "run.js")
        with open(path, "w", encoding="utf-8") as out:
            out.write(NODE_PRELUDE + code + "\nconst input = JSON.parse(require('fs').readFileSync(process.argv[2], 'utf8')); console.log(JSON.stringify(main(input)));\n")
        data_path = os.path.join(folder, "input.json")
        with open(data_path, "w", encoding="utf-8") as out:
            json.dump(data, out)
        done = subprocess.run([node, path, data_path], capture_output=True, text=True)
        check(done.returncode == 0, "the page's own wording runs in node" + ("" if done.returncode == 0 else ": " + done.stderr.strip()[-300:]))
        return json.loads(done.stdout)


def page_text():
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        return page.read()


def the_words_and_what_the_papers_say():
    """The words (decision 0221, stage 4, the second half), by the page's text and by its own wording run in node: one word for
    each thing in every place; ACTS and SIGNED in one wording; no caption that explains a rule; the engine's text worded before it
    is shown; SINCE with a type short and a place with its hex; a place said once not said in full again in a list; the prompt at
    the chart's top centre; SIGNED never silently stale; an interrupt's reasons opened as a paper."""
    import re
    text = page_text()
    # 1. one word for each thing
    check("var HELD_BY_YOU = 'Yours: ' + STAFF_HANDS;" in text and "function handsHeld(hold)" in text and text.count("handsHeld(") >= 5,
          "a unit the player holds is \"Yours: you keep it out of the staff's plans\" on every card, in the paper and in the staff's menu")
    check(not any(w in text for w in ("kept out of the staff\\'s plans by you", "held by your direct order", "'Held from the staff", "': kept from the staff'", "'Released. ", "'Release'", "Yours: you hold it", "' from the staff.'")),
          "no other word names a unit that is held: not \"kept from the staff\", \"held from the staff\", \"released\" or \"you hold it\"")
    check("var RETURNED = 'Returned to the staff." in text and text.count("RETURNED") >= 5 and "'Return to the staff'" in text,
          "a unit given back is \"returned to the staff\", in the button, the line and the status")
    check("': withdrawn'" in text and "'The order is withdrawn from 1800.'" in text and "'The draft is torn up. Nothing was ordered.'" in text and "'TAKE BACK'" in text and "NOT TAKEN BACK" in text,
          "an order to the staff is withdrawn, a draft is torn up, a signed entry is taken back")
    # 2. ACTS and SIGNED say one wording
    check("var ACTS_TONIGHT = 'Acts at 1800, with tonight\\u2019s orders', ACTS_NOW = 'Acts at once', ACTS_DUSK = 'Acts at 1800, for the coming day'" in text
          and "'Waits for 1800'" not in text and "'Acts at once, at '" not in text and "hourNow" not in text,
          "ACTS and SIGNED say it in the same three sentences, and the old \"Waits for 1800\" and \"Acts at once, at 1800\" are gone")
    check("staff ? (clock.at === 'dusk' ? ACTS_TONIGHT : ACTS_DUSK)" in text and "was.acts" in text and "' Not yet in effect.'" not in text,
          "an order to the staff and the card's \"Ordered\" line say it in the same words")
    # 3. captions that explain a rule are gone
    gone = ["Capture its places, garrison and supply them", "Land men and capture the place. Nothing is ordered for afterwards", "Air raids on its airfield until it can fly no aircraft",
            "Its men defend it; it is kept supplied", "Supply runs keep 30 days of stock there. Nothing else.", "Fighters over it and searches round it. Nothing else.",
            "Search them and strike any enemy ship that enters", "The same, over a wider circle.", "The carrier force goes there and works from it", "The force goes there and waits, to meet enemy ships",
            "A course ends the fuelling", "your risk goes with the order", "The draft is not signed. Sign it on the order", "The game is as it was.", "Giving it frees every plan of yours that waits on it",
            "play it again with", "What was already ordered stands", "the units it would take lie idle in between", "A claim is what they believe", "Nothing is ordered until you sign"]
    check(not [g for g in gone if g in text] and "el('small', '', e[2])" not in text and "b.title = e[2]" not in text,
          "no caption explains a rule: the notes under each end of an order to the staff, the dry run's \"sign it on the order\", its foot and its sheet lines, and the preview's risk note are gone")
    # 4. the situation's text goes through the table's words
    check("function situationDoc(doc, text)" in text and "situationDoc($('sheet-doc')" in text and "$('sheet-text').textContent = b.text" not in text,
          "the situation is shown as a paper of its sections with each line through the table's words, not as the engine's raw text")
    # 5. SINCE: a type short, a place with its hex
    check("shortTypes(list.replace(" in text and "shortType(m[4])" in text and "function hexPlace(q, r)" in text and text.count("hexPlace(") >= 3,
          "SINCE, the war diary and the evening report say an aircraft type short and a position with its hex in brackets, linked")
    said = run_in_node(text[text.index("  function cleanName"):text.index("  function sayIntent")] + text[text.index("  // A list of \"12 Aichi"):text.index("  // ---- a force asks leave")]
                       + "\nfunction main(x) { return [shortTypes(x[0]), shortType(x[1])]; }",
                       ["12 Aichi D3A1 Type 99 Carrier Bomber Model 11, 3 Mitsubishi A6M2 Type 0 Carrier Fighter Model 21, 2 Douglas SBD-3 Dauntless, 1 unidentified", "Nakajima B5N2 Type 97 Carrier Attack Bomber Model 12"])
    check(said == ["12 D3A1, 3 A6M2, 2 SBD-3 Dauntless, 1 unidentified", "B5N2"], f"an aircraft type is said short as the slips say it ({said[0]})")
    # 6. a place said once is not said in full again
    check("function taskWords(tag, text, said)" in text and "(again ? 'there'" in text and "said = {}" in text and "taskWords('span', paperText(it, t.n), said)" in text and "placeSpan(a.hex, said)" in text
          and "typeof it === 'function' ? it()" in text,
          "in one task's WHY a place is said in full once, with its latitude, longitude and hex, and \"there\" afterwards, in the order the lists are drawn")
    # 7. the prompt at the chart's top centre
    css = re.search(r"#prompt \{([^}]*)\}", text).group(1)
    check("top: 12px" in css and "bottom: 76px" not in css and "margin: 0 auto" in css and "right: 200px" in css and "left: calc(var(--sp-w) + 16px)" in css
          and "body.papered #prompt { right: calc(min(400px, 100% - var(--sp-w) - 48px) + 32px); }" in text
          and "top: calc(var(--sp-h, 190px) + 8px)" in text and "setProperty('--sp-h'" in text and "body.prompting .order" in text,
          "the prompt lies at the chart's top centre between the stop paper and the order paper, and under the head and its strip in the drawer layout, clear of the order paper and the controls")
    # 8. SIGNED is never silently stale
    check("function staleSigned()" in text and "Could not read what is signed. " in text and "'READ AGAIN'" in text and "refresh().catch(function () { staleSigned(); })" in text
          and text.count("signedRefresh()") >= 8 and "return refresh();\n      })" not in text,
          "a failed read after a signing or a taking back says \"Could not read what is signed. READ AGAIN\" where SIGNED is, and READ AGAIN reads again")
    check("function signedRefresh() { return refresh().catch(function () { staleSigned(); }); }" in text and "}, function (e) { say('refused', e.message, home); return false; }); }" in text
          and "function (err) { b.disabled = false; backRefused" in text,
          "a read that fails is not taken for the order's refusal: a sign or a take back that went is not reported as NOT SENT or NOT TAKEN BACK")
    # 10. an interrupt's reasons opened as a paper
    check("function readFired(f)" in text and "api('why', { number: n })" in text and "button(a, 'WHY', true, function () { readFired(f); })" in text and "function firedWords(w)" in text,
          "a firing of the staff's, with reasons, opens them as a paper from its WHY, and its words are the table's")
    said = run_in_node(text[text.index("  function cleanName"):text.index("  function sayIntent")] + text[text.index("  // What the engine says of a firing"):text.index("  function showWhy() {")]
                       + "\nfunction main(x) { return x.map(firedWords); }",
                       ["standing ship_lost fired: ship:1 was lost", "standing cannot_meet fired: plan:2 cannot meet its intent", "interrupt:3 fired, base:1 at 8.3 days of supply, below 10",
                        "standing waits_for_leave fired: plan:1 waits for the commander's leave"])
    check(said == ["USS Enterprise was lost", "Plan 2 cannot meet its intent", "Lunga has 8.3 days of supply, below your 10", "Plan 1 waits for your leave"],
          f"what a firing says has no handle in it ({said})")


SUBMARINE_GAMES = "data/scenarios/table-submarines/"


def cut_function(text, header):
    """The text of one function of the page, from its header to its closing brace: a one-line function to the end of its line, any
    other to the next line that is a lone two-space brace (the page's functions are all laid out so)."""
    start = text.index(header)
    line_end = text.index("\n", start)
    if text[start:line_end].rstrip().endswith("}"):
        return text[start:line_end]
    return text[start:text.index("\n  }\n", start) + 4]


INTEL_SAMPLE = {
    "format": "flagplot-agent", "version": 52, "kind": "intelligence",
    "parts": ["seen", "lost_sight_of", "claimed", "enemy_has_done", "ashore", "enemy_bases", "enemy_submarines", "before_the_game"],
    "seen": [{"contact": {"handle": "contact:4", "name": None}, "place": {"q": 2, "r": 1}, "ship_count": 3, "taken_for": "cruiser", "age_pulses": 2},
             {"contact": {"handle": "contact:2", "name": None}, "place": {"q": 3, "r": 1}, "ship_count": 1, "taken_for": "carrier", "age_pulses": 0}],
    "lost_sight_of": [
        {"contact": {"handle": "contact:3", "name": None}, "last_report": {"day": 2, "pulse": "night", "place": {"q": 5, "r": 5}, "ship_count": 8, "taken_for": "cruiser", "event": 11},
         "dropped": {"day": 3, "pulse": "afternoon", "event": 40, "why": "no_report_since", "age_pulses": 5}},
        {"contact": {"handle": "contact:1", "name": None}, "last_report": {"day": 0, "pulse": None, "place": {"q": 6, "r": 6}, "ship_count": 2, "taken_for": "destroyer", "event": 1},
         "dropped": {"day": 1, "pulse": "afternoon", "event": 20, "why": "strike_found_nothing", "place": {"q": 6, "r": 6}}}],
    "claimed": {"days": [{"day": 2, "claims": [{"pulse": "morning", "place": {"q": 4, "r": 4}, "claimed_sunk": 1, "claimed_hits": 3, "taken_for": "cruiser", "claimed_shot_down": 0, "claimed_on_ground": 0, "from_air": True, "event": 9}]},
                         {"day": 1, "claims": [{"pulse": "night", "place": {"q": 0, "r": 0}, "claimed_sunk": 0, "claimed_hits": 0, "taken_for": None, "claimed_shot_down": 0, "claimed_on_ground": 29, "from_air": False, "from_bombardment": True, "event": 3}]}],
                "totals": {"sunk": 1, "hits": 3, "shot_down": 0, "on_ground": 29}},
    "enemy_has_done": [{"day": 2, "raids": [{"base": {"handle": "base:1", "name": "Lunga"}, "aircraft": "18", "weapon": "bombs", "recognised": "Mitsubishi G4M1 Type 1", "hits": 4, "event": 5}],
                        "attacks_on_ships": [{"by": "aircraft", "against": {"handle": "ship:1", "name": "USS Enterprise"}, "weapon": "torpedoes", "attacks": 2, "aircraft": "9", "recognised": [], "not_recognised": 2, "events": [7]}],
                        "submarine_attacks": [{"against": {"handle": "ship:1", "name": "USS Enterprise"}, "place": {"q": 5, "r": 0}, "pulse": "morning", "event": 8}],
                        "fights": [{"force": {"handle": "task_force:1", "name": "Task Group Able"}, "place": {"q": 1, "r": 2}, "rounds": 3, "ended": "slipped_away", "event": 6}]},
                       {"day": 1, "bombardments": [{"base": {"handle": "base:1", "name": "Lunga"}, "place": {"q": 0, "r": 0}, "shells": "1320", "hits": 37, "event": 4}]}],
    "ashore": {"estimates": [{"place": {"q": 8, "r": 0}, "men": 330, "came_from": "men in contact", "day": 1, "age_days": 1, "event": 16}],
               "found_empty": [{"day": 3, "place": {"q": 9, "r": 0}, "men": 39, "reported_day": 2, "event": 45}]},
    "enemy_bases": [{"base": {"handle": "base:3", "name": "Tulagi"}, "place": {"q": 8, "r": 0}, "held_by": "japanese",
                     "estimate": {"place": {"q": 8, "r": 0}, "men": 330, "came_from": "men in contact", "day": 1, "age_days": 1, "event": 16},
                     "claimed_on_ground_from_air": 12, "claims": [{"day": 2, "number": 12}]}],
    "enemy_submarines": {"reports": [{"place": {"q": 5, "r": -1}, "day": 1, "age_pulses": 4, "event": 6}],
                         "suspected_waters": [{"place": {"q": 5, "r": 0}, "distance_hexes": 3, "words": "Placeholder: the staff believes enemy submarines work in these waters.", "told_day": 1, "event": 1}]},
    "before_the_game": {"contacts": [{"contact": {"handle": "contact:1", "name": None}, "place": {"q": 6, "r": 6}, "ship_count": 2, "taken_for": "destroyer", "age_pulses": 2, "event": 1}],
                        "estimates": [{"place": {"q": 8, "r": 0}, "men": 300, "came_from": "told", "event": 1}]},
}

INTEL_HEADS = ["SEEN", "LOST SIGHT OF", "CLAIMED", "WHAT THE ENEMY HAS DONE", "ASHORE", "ENEMY BASES", "ENEMY SUBMARINES", "BEFORE THE GAME"]


def intelligence_code(text):
    """The paper's own functions and the few of the page's they use, with stubs for what draws, run in node."""
    start = text.index("  // ---- INTELLIGENCE SUMMARY")
    paper = text[start:text.index("  // What is known of how a contact was last reported", start)]
    helpers = "\n".join(cut_function(text, h) for h in ("function pulsesOld(n)", "function daysOld(e)", "function commas(n)", "function fig(x)", "function shortType(name)",
                                                           "function cleanName(s)", "function round(n)"))
    return """
Node.prototype.addEventListener = function (k, f) { (this.listeners = this.listeners || []).push(k); };
Object.defineProperty(Node.prototype, 'classList', { get() { const n = this; return { add(c) { n.className += ' ' + c; } }; } });
var config = { start: '1942-08-24' };
function dateOf(n) { return 'D' + (n + 1); } function num(h) { return parseInt(String(h).split(':')[1], 10); }
function refTo(h, given) { const b = el('button', 'ref', given || (/^contact:/.test(h) ? 'contact ' + num(h) : 'unnamed')); b.type = 'button'; return b; }
function placeLink(pl) { const s = el('span', ''); s.appendChild(el('button', 'ref', '<' + pl.q + ',' + pl.r + '>')); return s; }
""" + helpers + "\n" + paper + """
function all(node, f) { f(node); (node.kids || []).forEach(k => all(k, f)); }
function main(a) {
  const doc = el('div', ''); let hows = 0, directives = 0;
  drawIntelligence(a, doc, function (row, event) { hows += 1; row.how = event; }, function () { directives += 1; });
  const heads = [], text = [], refs = [], rows = [], buttons = [];
  doc.kids.forEach(k => { if (k.className === 'sub') heads.push(k.textContent); });
  all(doc, n => { if (n.className === 'ref') refs.push(n.textContent); if (n.className.indexOf('dline') >= 0) rows.push({ text: n.textContent, how: n.how === undefined ? null : n.how, cls: n.className }); if (n.tag === 'button') buttons.push(n.textContent); });
  return { heads, text: doc.textContent, refs, rows, buttons, hows };
}
"""


def the_intelligence_paper():
    """The paper of what is known of the enemy (decision 0226, the page's statements 1 to 13): by the page's text and by its own
    functions run in node on an answer. The page is drawn in a browser and tried by hand."""
    import re
    print("-- the intelligence paper: the page's text and its own functions in node", flush=True)
    text = page_text()
    # 1. where it is, and that it asks once, when it is opened
    check("staffPaper('Intelligence', readIntelligence, true)" in text and "sheetAs('INTELLIGENCE SUMMARY')" in text,
          "the Staff menu has Intelligence, which opens a sheet titled INTELLIGENCE SUMMARY")
    opened = cut_function(text, "function readIntelligence()")
    check(opened.count("api('intelligence'") == 1 and text.count("api('intelligence'") == 1 and "refresh" not in opened,
          "the paper is drawn from one call of the tool intelligence, made when it is opened, and not at load or on a refresh")
    paper = text[text.index("  // ---- INTELLIGENCE SUMMARY"):text.index("  // What is known of how a contact was last reported")]
    check("api(" not in paper and "events" not in paper.replace("enemy_has_done", "") and not re.search(r"\.(test|exec|match|search)\(|\.replace\(/|\.split\(/", paper)
          and not re.search(r"[(=,] ?/[^/*\s][^/\n]*/[gimsu]*[.),;]", paper),
          "the functions that draw it call no other tool, read no events and match no pattern against words")
    # 13. a watched game
    check("if (WATCH) body.side = WATCH.side" in text and "staffPaper('Intelligence'" in text and "WATCH" not in text[text.index("staffPaper('Intelligence'") - 80:text.index("staffPaper('Intelligence'")],
          "the paper opens for the seat watched, by the same call, which carries the side watched")
    code = intelligence_code(text)
    said = run_in_node(code, INTEL_SAMPLE)
    flat = said["text"]
    # 2. order
    check(said["heads"] == INTEL_HEADS, f"given every part, the headings are in the paper's order, each once ({said['heads']})")
    some = dict(INTEL_SAMPLE, parts=["claimed", "ashore"])
    check(run_in_node(code, some)["heads"] == ["CLAIMED", "ASHORE"], "a part the answer does not name has no heading, and the order is the answer's own")
    reordered = dict(INTEL_SAMPLE, parts=["ashore", "seen"])
    check(run_in_node(code, reordered)["heads"] == ["ASHORE", "SEEN"], "the page takes the order from the answer's parts")
    none = run_in_node(code, {"parts": []})
    check(not none["heads"] and "We hold nothing of the enemy yet." in none["text"], "an answer with no part says We hold nothing of the enemy yet. and draws no heading")
    # 3. how known and how old
    check("3 ships" in flat and "heaviest seen cruiser" in flat and "Report 2 pulses old" in flat and "1 ship" in flat and "taken for carrier" in flat and "Reported this pulse" in flat,
          "a contact's line gives its ships, what the heaviest was taken for, its place and its age, in the card's words")
    check("About 330 men" in flat and "from men in contact" in flat and "Report 1 day old" in flat,
          "an estimate's line gives the men, the place, where it came from and its age in days, in the place's card's words")
    check("Report of a submarine" in flat and "Report 4 pulses old" in flat and "Placeholder: the staff believes enemy submarines work in these waters." in flat and "within 120 miles" in flat,
          "a report of a submarine gives its place and age, and suspected waters their place, their distance and the words as told")
    # 4. links
    check(not re.search(r"\b[a-z_]+:\d+\b", flat), "no handle of the form word:number is shown")
    check(all(n in said["refs"] for n in ("contact 4", "contact 2", "Lunga", "Tulagi", "USS Enterprise", "Task Group Able")) and any(r.startswith("<") for r in said["refs"]),
          "every contact held, base, own unit and place on the paper is a link made by the functions the cards use")
    # 5. lost sight of
    lost = [r for r in said["rows"] if "Last reported" in r["text"]]
    check(len(lost) == 2 and "D2, night" in lost[0]["text"] and "We have had no report since." in lost[0]["text"] and "Our strike found nothing there." in lost[1]["text"]
          and "before the game" in lost[1]["text"] and "8 ships" in lost[0]["text"],
          "each lost line says the day and part of the day of the last report, what it said, and one of two things")
    check(said["hows"] == 2 and [r["how"] for r in lost] == [11, 1], "a lost line is not a link to a contact: pressing it opens how it was last reported, by the event of that report")
    check("chart.hidden = !lost.length" in opened and "lostMarks = lost" in opened and "(lostMarks || []).forEach" in text and text.count("lostMarks = null") >= 2,
          "SHOW ON CHART is shown while the answer has a lost contact, marks the last place of each with a mark of its own, and is hidden with none")
    check("M.contacts.forEach(function (c) { var v = placeOf(c.place); if (!v) return; var a = Math.max(0.25" in text and "g.moveTo(c[0] - k, c[1] - k)" in text,
          "the mark of a lost contact is a cross, not a held contact's ring")
    # 6. claimed
    check(flat.index("Claimed 1 ship sunk, 3 hits on a cruiser") < flat.index("Claimed 29 aircraft destroyed on the ground by our guns") and "Claimed, not confirmed: 1 sunk, 3 hits, 29 aircraft on the ground." in flat,
          "claims by day, the latest first, a claim of ships says what the target was taken for, and one line of totals with the 0 left out")
    check(flat.count("Claimed, not confirmed") == 1 and not re.search(r"on (a|the) (carrier|cruiser|destroyer|battleship): \d", flat) and "contact" not in flat[flat.index("CLAIMED"):flat.index("WHAT THE ENEMY")].lower().replace("claimed, not confirmed", ""),
          "no total by kind of ship, and no claim is linked to a contact")
    # 7. what the enemy has done
    check("Raid on Lunga by 18 aircraft, recognised as G4M1, 4 hits." in flat and "type not made out" in flat and "attacked by a submarine at <5,0>, morning." in flat
          and "shelled from the sea" in flat and "Action at <1,2>: Task Group Able met the enemy, 3 rounds, the enemy slipped away." in flat
          and flat.index("D2", flat.index("WHAT THE ENEMY")) < flat.index("D1", flat.index("WHAT THE ENEMY")),
          "the enemy's raids, shellings, attacks on our ships, a submarine's among them, and the actions our forces met, by day, with the type where recognised")
    # 8. enemy bases
    check("Tulagi · About 330 men · reported D1 · 12 aircraft claimed destroyed on the ground from the air, D2" in flat, "an enemy base shows the estimate of men there and the aircraft claimed on the ground from the air, with the day of each")
    # 9. before the game
    check("The directives" in said["buttons"] and "told at the start" in flat and "given 2 pulses old" in flat and "directive" not in paper.lower().replace("opendirectives", "").replace("ondirectives", "").replace("directives", "").replace("the directives", ""),
          "BEFORE THE GAME shows the contacts and the estimates as given and one link that opens the directives, and reads no sentence of one")
    # 10. not kept
    kept = run_in_node(code, {"parts": ["seen", "ashore"], "seen": INTEL_SAMPLE["seen"], "ashore": INTEL_SAMPLE["ashore"], "not_kept": ["lost_sight_of", "enemy_has_done", "before_the_game"]})
    check(kept["heads"] == ["SEEN", "LOST SIGHT OF", "WHAT THE ENEMY HAS DONE", "ASHORE", "BEFORE THE GAME"] and kept["text"].count("Not kept at this command.") == 3,
          "each part named in not_kept has its heading and the one line Not kept at this command.")
    # 11 and 12. words, and no points or result
    low = flat.lower()
    check("report" in low and "contact" in low and "force" not in low and "intent" not in low and not re.search(r"\bhis\b", low) and "the enemy" in low and "we " in low,
          "the words: report or contact, never force of a contact, the enemy and never his, we for the staff, and no intent")
    check(not any(w in low for w in ("points", "victory", "margin", "score")) and "M.over" not in paper and "result" not in paper,
          "no points, victory, margin or score on the paper, and nothing of the game's result is read")
    check(not any(w in low for w in ("probably", "perhaps", "possibly", "might", "unconfirmed", "believed", "usually", "because", "can be wrong", "means")),
          "no sentence explains a rule and claimed, not confirmed is the only qualifier on the paper")


def the_intelligence_through_a_bridge(sim_cli, port):
    """The page's statement 14: on the table's game of scenario 128, the Allied seat, three days played, the call the page makes
    answers and the paper drawn from it has ENEMY SUBMARINES (a report and the waters in the words told) and WHAT THE ENEMY HAS DONE
    (the submarine's attacks on the carrier, one on each of days 1 and 2)."""
    print("-- the intelligence paper on a real game: the table's copy of scenario 128", flush=True)
    repo = os.path.dirname(HERE)
    with open(os.path.join(repo, SUBMARINE_GAMES, "128_a_submarine_in_the_way.toml"), encoding="utf-8") as f:
        original = f.read()
    with tempfile.TemporaryDirectory() as folder:
        path = os.path.join(folder, "game.toml")
        with open(path, "w", encoding="utf-8") as out:
            out.write(original.replace('"../../', '"' + os.path.join(repo, "data") + "/"))
        command = [sys.executable, os.path.join(HERE, "serve.py"), path, "--seat", "allied", "--start", "1942-08-24", "--port", str(port), "--sim-cli", sim_cli]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
        try:
            check(process.stdout.readline().startswith("The table is at "), "the bridge is up on the table's copy of scenario 128, the Allied seat")
            root = f"http://127.0.0.1:{port}"
            call = lambda tool, arguments=None: post(root + "/call", {"tool": tool, "arguments": arguments or {}})["ok"]
            for _ in range(3):
                call("end_day")
            answer = call("intelligence")
            check(answer["kind"] == "intelligence" and "enemy_submarines" in answer["parts"] and "enemy_has_done" in answer["parts"], f"the page's call answers: {answer['parts']}")
            said = run_in_node(intelligence_code(page_text()), answer)
            flat = said["text"]
            check("ENEMY SUBMARINES" in said["heads"] and "WHAT THE ENEMY HAS DONE" in said["heads"], f"the paper has ENEMY SUBMARINES and WHAT THE ENEMY HAS DONE ({said['heads']})")
            check("Report of a submarine" in flat and "the staff believes enemy submarines work in these waters" in flat, "ENEMY SUBMARINES gives a report and the suspected waters in the words told")
            check(flat.count("attacked by a submarine") == 2 and "D2" in flat and "D1" in flat, "WHAT THE ENEMY HAS DONE gives the submarine's attacks on the carrier, one on each of days 1 and 2")
        finally:
            process.send_signal(signal.SIGTERM)
            process.wait(30)


def the_intelligence_of_the_long_game(bridge):
    """The page's statement 14, second half: on the 39 days at the first evening the answer has BEFORE THE GAME, and the paper has the link
    to the directives."""
    print("-- the intelligence paper on the 39 days, at the first evening", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    answer = person.call("intelligence")["ok"]
    check("before_the_game" in answer["parts"], f"the answer has BEFORE THE GAME ({answer['parts']})")
    said = run_in_node(intelligence_code(page_text()), answer)
    check("BEFORE THE GAME" in said["heads"] and "The directives" in said["buttons"], "the paper has BEFORE THE GAME with the link to the directives")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_submarines():
    """Submarines at the table (decision 0218, stage 3), by the page's text: a boat of the side's own as a piece and a card with
    the direct order through the one paper and the hold; a report of a submarine as a mark of its own with its card and HOW WE KNOW;
    suspected waters drawn faintly with no boat and a card; the patrol order with its distance, its stations drawn while drafted and
    in the plan's paper; the stop and its slips; the diary's and SINCE's words. (The page is drawn in a browser, tried by hand.)"""
    import re
    text = page_text()
    # 1. a boat of the player's own
    check("function boatTo(b, hex, i)" in text and "if (i.kind === 'patrol' && i.hex)" in text and "label: 'PATROL'" in text and "if (/^submarine:/.test(o.unit) && o.order.kind === 'station' && o.order.hex) boatTo(boatOf(o.unit), o.order.hex, i);" in text
          and "sub: 2.4 })[m.kind]" in text and "e.order.kind === 'station' && e.order.hex) signed[e.unit]" in text and "stationMark(cpx(s), u, open ? '' : boatName(b), open || !!signed[h]);" in text,
          "a patrol is drawn with the plans (its waters, each boat to her station, the staff's station orders), and every boat's station and a signed station order are drawn always")
    check("function cardBoat(t)" in text and "boat: cardBoat, subreport: cardSubReport, waters: cardWaters" in text and "tile('own', 'submarine', true, 0.15)" in text
          and "kind === 'submarine') { g.beginPath(); g.ellipse(" in text and "{ kind: 'boat', boat: b }" in text and "kind === 'submarine') { found = M.boats.filter(" in text,
          "a boat of the player's own has a piece of her own (a hull on the side's own rectangle) and a card, reached by touching the piece, stepping the pieces at her hex and a link to her")
    card = cut_function(text, "function cardBoat(t)")
    rows = ["['Class',", "['Where',", "['Her station',", "['Salvoes left',", "['Doing',", "['Condition',", "['Held down',", "['In whose hands', whose]"]
    at = [card.find(r) for r in rows]
    check(all(a >= 0 for a in at) and at == sorted(at), "her card says, in order: class, where, her station, salvoes left, what she is doing, sound or damaged, held down, in whose hands")
    cls = run_in_node(cut_function(text, "function cleanName(s)") + "\n" + cut_function(text, "function shipClass(c)")
                      + "\nfunction main(x) { return shipClass(x); }",
                      "I-168 to I-175, fleet submarines built 1934 on (the Technical Mission's block); the Japanese type code is not stated by the rows")
    check("['Class', shipClass(b.class)]" in card and cls == "I-168 to I-175",
          f"her class is shown as a ship's is, by the page's own short form of the class file's name ({cls})")
    check("'The staff\u2019s, under '" in card and "handsHeld(held)" in card and "'Free'" in card and "placeLink(b.place)" in card and "placeLink(b.station)" in card,
          "in whose hands is the staff's under the order named (linked to its paper), the player's as every held unit says it, or free; where she is and her station are places, linked, with the hex in brackets")
    check("'GIVE A DIRECT ORDER'" in card and "openPaper('boat', { unit: h, name: boatName(b), hold: { submarine: n }" in card
          and "'KEEP THIS BOAT OUT OF THE STAFF\\'S PLANS'" in card and "act('hold', { submarine: n }" in card and "'RETURN IT TO THE STAFF'" in card and "act('release', { submarine: n }, RETURNED" in card,
          "her card offers the direct order through the one order paper, keeping her out of the staff's plans, and returning her, as a force's card does")
    check("function boatOrderable(b) { return (b.state === 'on station' || b.state === 'on passage') && !b.damaged; }" in text and "if (boatOrderable(b))" in card,
          "the order is offered for a boat on station or on passage and sound, which the engine takes, and not for one going home, sunk or gone home")
    controls = cut_function(text, "function boatControls(form, b)")
    check("kind: 'station', hex: [stationDraft.place.q, stationDraft.place.r]" in controls and "'Take station at '" in controls and "startStation();" in controls
          and "picking === 'station'" in controls and "if (hh[5] !== 0) { say('picking', 'Touch open water.'); return; }" in text and "({ route: pickRoute, station: pickStation, area: pickArea }[picking] || pickMarch)(p)" in text,
          "the paper's order is a station, touched on the chart and drawn there (open water only), with the way to choose again")
    check(re.search(r"var INSIDE_KINDS = \{[^}]*\}", text) and "station" not in re.search(r"var INSIDE_KINDS = \{[^}]*\}", text).group(0) and "station: 'Station'" in text,
          "a station is not an order that acts inside a day: the paper says when it acts as every such order does")
    acts = run_in_node(text[text.index("  var ACTS_TONIGHT"):text.index("\n", text.index("  var ACTS_TONIGHT"))] + "\n" + text[text.index("  var INSIDE_KINDS"):text.index("\n", text.index("  var INSIDE_KINDS"))]
                       + "\nvar ORDER_LABEL = { station: 'Station' }; var clock = { at: 'dusk' };\n" + cut_function(text, "function labelOf(o)") + "\n" + cut_function(text, "function actsText(orders)")
                       + "\nfunction main(x) { const o = [{ unit: 'submarine:1', order: { kind: 'station', hex: [1, 1] } }]; const out = [actsText(o)]; clock = { at: '1200' }; out.push(actsText(o)); return out; }", None)
    check(acts == ["Acts at 1800, with tonight’s orders", "Acts at 1800, for the coming day"],
          f"a station order says it acts at 1800: with tonight's orders at dusk, for the coming day inside a day ({acts})")
    # 2. a report of a submarine
    check("function subMark(c, u, alpha)" in text and "M.subReports || []).forEach(function (r) { var v = placeOf(r.place); if (v) subMark(" in text
          and "ring(" not in cut_function(text, "function subMark(c, u, alpha)") and "tile('enemy', 'submarine', false, 0.15" in text and "'SUBMARINE REPORTED'" in text,
          "a report of a submarine is a mark of its own (a dashed diamond with a hull, fainter as it ages) and an unsure enemy tile, not a ship contact's ring, with its card")
    how = cut_function(text, "function howWeKnowBoat(into, nodes)")
    check("'Reported after a salvo on '" in how and "'Sighted from the air by '" in how and "'Sighted from the air.'" in how and "rep.hidden_causes" in how and "api('explain', { event: r.event })" in text,
          "a report's HOW WE KNOW says it came after a salvo on a ship, or was sighted from the air, from the events that made it (`explain` of the report's event)")
    # 3. the suspected waters
    check("red: true }, true, u)" in text and "(w.distance_hexes + 0.5) * hexMiles()" in text and "g.fillStyle = a.red ? 'rgba(163,33,26,0.07)'" in text and "'SUSPECTED WATERS'" in text
          and "'Told '" in text and "kind: 'waters', waters: w" in text,
          "suspected waters are drawn faintly (a fill and a dashed edge in the enemy's red, no boat), and have a card with what is believed, round where, how far and the day told")
    sit = run_in_node(cut_function(text, "function situationLine(line, part, over)") + "\nfunction dateOf(n) { return 'D' + (n + 1); }\nfunction noPoints(l) { return l; }\n"
                      + "function main(x) { return x.map(l => situationLine(l, 'SUBMARINES', false)); }", [
        "- Report of an enemy submarine: at (5,-1), made day 1, 1 pulse old [event 6]",
        "- Waters where enemy submarines are thought to work: within 3 hexes of (5,0), told on day 1: Placeholder words. [event 1]"])
    check(sit == ["- A submarine is reported at (5,-1), made D1, 1 pulse old", "- Waters where enemy submarines are suspected: within 3 hexes of (5,0), told D1: Placeholder words."],
          f"the situation says a report and the suspected waters in words, with the day as a date and no event number ({sit})")
    # 4. the patrol order
    ends = cut_function(text, "function ends(target)")
    check("if (target.kind === 'waters' && (M.boats || []).some(function (b) { return b.state !== 'sunk' && b.state !== 'home'; })) list.push(['Patrol these waters with submarines', { intent_kind: 'patrol', hex: [target.q, target.r], distance: 2 }, null]);" in ends,
          "\"Patrol these waters with submarines\" is among the orders on waters, and only where the side has a boat on the map or coming")
    check('id="o-dist" min="1" max="6" step="1"' in text and "function patrolDistance()" in text and "if (e[1].intent_kind === 'patrol') args.distance = patrolDistance();" in text
          and "e.target.id === 'o-dist'" in text and "function showPatrol()" in text and "round(d)" in text,
          "the distance is a small whole-number control (1 to 6 hexes, said in miles) that asks the staff again when it changes")
    check("patrolStations = w.stations || p.stations || null;" in text and "patrolStations.forEach(function (s) { var v = placeOf(s.hex); if (v) stationMark(cpx(v), u, cleanName(s.name || s.submarine), true); })" in text
          and "stationDraft = null; patrolStations = null; stationPicked = null;" in text and "pd + 0.5) * hexMiles()" in text,
          "while a patrol is drafted the waters and the stations the staff would give are drawn, each named, from the preview, and are cleared when the paper closes")
    check("if (top.kind === 'patrol') (pr.stations || []).forEach(" in text and "if (i.kind === 'patrol') (M.boats || []).filter(function (b) { return b.hands && b.hands.by === 'staff' && b.hands.intent === i.number" in text
          and "if (k === 'station')" in text and "case 'station': text('take station at ')" in text and "patrol: 'PATROL WITH SUBMARINES'" in text and "'Patrol the waters within '" in text,
          "a patrol signed is in the plan's paper as the tree shows any task, each boat under it with her station, in YOUR ORDERS and SIGNED and on the chart as an order")
    check('<label class="fl" for="o-dist">Patrol these waters,</label><input type="number" id="o-dist"' in text
          and "' hex' : ' hexes') + ' of ' + where + ' with submarines'" in text,
          "the patrol paper labels its distance as the deny order's choices are labelled (\"these waters\"), and the signed line says it: within 2 hexes of <place>, 1 hex for 1")
    # A link to a boat, a force, a base, a contact, a hex or any other `#see=` kind goes through see(); at a narrow width the stop
    # paper is a drawer over the chart, and a card opened from a link in it lay under the drawer, so the first click seemed to do nothing.
    check("function see(h) {" in text and "if (document.body.classList.contains('drawer-open')) setDrawer(false);\n    if (paper)" in text
          and "refTo(h, given) { var b = el('button'" in text and "see(h); }); return b; }" in text and "see('hex:' + q + ',' + r)" in text
          and "var m = /^#see=(.+)$/.exec(location.hash); if (m) see(decodeURIComponent(m[1]))" in text,
          "every link to a card (a boat, a force, a base, a contact, a hex) goes through see(), which shuts the narrow layout's drawer so that the card opened is in sight")
    # 5. the stop and its slips
    check('id="stop-submarine"' in text and "a submarine attacks us" in text and "submarine: 'We are attacked by a submarine', boat: 'We have lost a boat'" in text,
          "the fold WHAT STOPS THE CLOCK has the tick \"a submarine attacks us\", and the general headings say \"we\"")
    toldhead = cut_function(text, "function toldHead(t)")
    check("'We are torpedoed: '" in toldhead and "', by a submarine'" in toldhead and "' was fired on by a submarine'" in toldhead and "'Lost: '" in toldhead,
          "a slip says \"We are torpedoed: <ship>, by a submarine\" for a hit, that a ship was fired on by a submarine and where for a salvo that missed, and \"Lost: <boat>\" for a boat lost")
    check("var subT = submarineTold(subEv); lastTold = lastTold.concat(subT);" in text and "evening || subT.length)" in text and "lastTold = subT;" in text and "|| subT.length) {" in text
          and "lastTold = toldOf(evs, stopOn()).concat(submarineTold(evs));" in text and "if (lost.length && ticks.loss)" in text and "if (salvos.length && ticks.submarine)" in text,
          "the clock stops on a submarine's attack in the pulse, day and watched loops alike, by a stop of the table's own (a boat lost rides the stop for a loss), and sends the engine no kind of stop it does not have")
    check("stop_on: stopOn()" in text and "'submarine'" not in text[text.index("function stopOn()"):text.index("\n", text.index("function stopOn()"))],
          "the stops sent to the engine are still its own five: nothing of the table's is asked of it")
    # 6. the diary's and SINCE's words
    kinds = ["submarine_entered", "submarine_on_station", "submarine_left_for_home", "submarine_reached_home", "own_submarine_fired", "own_submarine_hunted", "submarine_hunted_by",
             "own_submarine_attacked_from_air", "attacked_submarine_from_air", "submarine_sunk", "salvo_from_submarine", "submarine_reported", "submarine_report_dropped", "suspected_waters_told"]
    sayevent = cut_function(text, "function sayEvent(e, linked)")
    check(all(f"k === '{k}'" in sayevent for k in kinds) and "claim from a submarine at " in sayevent,
          "the war diary and SINCE have words for each event of the submarines: " + ", ".join(kinds))
    check("submarine_steamed: 1, submarine_ordered_to_station: 1 };" in text and text.count("submarine_steamed: 1") == 2,
          "a boat's steaming and being ordered to her station are routine: not plotted, and folded in the diary")


def the_submarines_through_the_page(sim_cli, port, hold):
    """The page's own wording on real events (decision 0218, stage 3): a bridge on the table's copies of scenarios 128 and 129
    (`data/scenarios/table-submarines/`), each side's events and contacts read through `/call` as the page reads them, and the page's
    own functions (`sayEvent`, `submarineTold`, `toldHead`) run in node on them. The seeds: 6 (the boat fires on the carrier on the
    first morning and is hunted), 8 (it fires and misses), 13 (it fires and is sunk)."""
    print("-- submarines through the page's own wording: the table's copies of scenarios 128 and 129", flush=True)
    import re
    text = page_text()
    repo = os.path.dirname(HERE)
    base = os.path.join(repo, SUBMARINE_GAMES, "128_a_submarine_in_the_way.toml")
    with open(base, encoding="utf-8") as f:
        original = f.read()
    check('seed = 6' in original and 'distance = 3' in original, "the table's copy of scenario 128 has seed 6 and the waters the Allied side is told of")
    code = cut_function(text, "function cleanName(s)") + "\n" + cut_function(text, "function sayEvent(e, linked)") + "\n" \
        + cut_function(text, "function salvoHits(salvo, evs)") + "\n" + cut_function(text, "function submarineTold(evs)") + "\n" + cut_function(text, "function toldHead(t)") + "\n" \
        + cut_function(text, "function kindWords(k, n)") + "\n" + cut_function(text, "function orderWords(t)") + """
var config = { seat: 'allied' }; var ticks = { submarine: true, loss: true };
function whereIs(pl) { return '<' + pl.q + ',' + pl.r + '>'; } function dateOf(n) { return 'D' + (n + 1); } function round(n) { return n + ' hexes round'; }
function fig(x) { return x; } function nameOf(h) { return h; }
function main(evs) { const told = submarineTold(evs); return { lines: evs.map(e => sayEvent(e, false)), told: told.map(t => ({ thing: t.thing, head: toldHead(t), n: t.lines.length })) }; }
"""
    def play(seed, seat, scenario_text=None):
        body = (scenario_text or original).replace("seed = 6", f"seed = {seed}")
        with tempfile.TemporaryDirectory() as folder:
            path = os.path.join(folder, "game.toml")
            with open(path, "w", encoding="utf-8") as out:
                out.write(body.replace('"../../', '"' + os.path.join(repo, "data") + "/"))
            command = [sys.executable, os.path.join(HERE, "serve.py"), path, "--seat", seat, "--start", "1942-08-24", "--port", str(port), "--sim-cli", sim_cli]
            process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
            try:
                line = process.stdout.readline()
                check(line.startswith("The table is at "), f"the bridge is up on seed {seed} for the {seat} seat")
                root = f"http://127.0.0.1:{port}"
                call = lambda tool, arguments=None: post(root + "/call", {"tool": tool, "arguments": arguments or {}})["ok"]
                call("end_day")
                return call("events", {"day": 1})["events"], call("list_contacts"), call("list_forces"), call
            finally:
                process.send_signal(signal.SIGTERM)
                process.wait(30)

    def words(events):
        return run_in_node(code, events)

    # seed 6, the Allied side: a hit
    events, contacts, forces, _ = play(6, "allied")
    check("submarines" not in forces and contacts.get("submarine_reports") and contacts.get("suspected_waters"),
          "the Allied side has no boat, holds a report of a submarine and the waters it is told of (and nothing of the enemy boat)")
    said = words(events)
    check(said["told"] == [{"thing": "submarine", "head": "We are torpedoed: ship:1, by a submarine", "n": 4}]
          or (len(said["told"]) == 1 and said["told"][0]["head"] == "We are torpedoed: ship:1, by a submarine"),
          f"a salvo that hit is the slip \"We are torpedoed: <ship>, by a submarine\", the ship's own events under it ({said['told']})")
    joined = " | ".join(said["lines"])
    check("Carrier, of Allied force, was fired on from a submarine at <5,0>" in joined and "A submarine is reported at <5,-1>" in joined and "Allied force hunted a submarine at <5,0>" in joined,
          f"the events are said as the staff would: fired on from a submarine, a submarine reported, the force's hunt ({joined[:700]})")
    # The kinds this game does not give, as the engine writes them (the Display of EventKind and SideEventKind in sim-core).
    own = lambda handle, name: [{"handle": handle, "name": name}]
    synthetic = [
        {"event_kind": "submarine_entered", "text": "submarine:1 came onto the map at (12,0)", "entities": own("submarine:1", "Placeholder submarine 1")},
        {"event_kind": "submarine_on_station", "text": "submarine:7 reached her station at (7,0)", "entities": own("submarine:7", "Placeholder submarine 7")},
        {"event_kind": "submarine_left_for_home", "text": "submarine:1 left for base:1, her home: she was damaged", "entities": own("submarine:1", "Placeholder submarine 1") + own("base:1", "Placeholder submarine base")},
        {"event_kind": "submarine_reached_home", "text": "submarine:1 reached base:1, her home, and left the map", "entities": own("submarine:1", "Placeholder submarine 1") + own("base:1", "Placeholder submarine base")},
        {"event_kind": "own_submarine_attacked_from_air", "text": "submarine:1 was attacked from the air at (5,0), damaged", "entities": own("submarine:1", "Placeholder submarine 1")},
        {"event_kind": "attacked_submarine_from_air", "text": "air_group:2 attacked a submarine at (5,0)", "entities": own("air_group:2", "VS-3")},
        {"event_kind": "submarine_report_dropped", "text": "Allied dropped its report of a submarine at (5,-1), grown too old", "entities": []},
        {"event_kind": "submarine_reported", "text": "Allied report of a submarine at (5,-1), replacing the report held", "entities": []},
        {"event_kind": "suspected_waters_told", "text": "Allied told on day 1 of waters where enemy submarines are suspected, 3 hexes round (5,0): Placeholder: words", "entities": []},
    ]
    said = words(synthetic)["lines"]
    check(said == ["Submarine 1 came onto the map at <12,0>.", "Submarine 7 is on station at <7,0>.", "Submarine 1 left for Submarine base, her home: she is damaged.",
                   "Submarine 1 reached Submarine base, her home.", "Submarine 1 was attacked from the air at <5,0> and is damaged.", "VS-3 attacked a submarine at <5,0>.",
                   "The report of a submarine at <5,-1> is dropped: it grew too old.", "A submarine is reported at <5,-1>, replacing the earlier report.",
                   "We are told of waters where enemy submarines are suspected, 3 hexes round <5,0>: Placeholder: words."],
          f"a boat on the map or on station, left for home, reached home, attacked from the air, our own air attack on one, a report dropped or replaced and waters told are said in the staff's words ({said})")
    check(not re.search(r"\b\w+:\d+\b", joined), "no line of them carries a handle")
    # seed 8, the Allied side: a miss
    events, _, _, _ = play(8, "allied")
    said = words(events)
    check(len(said["told"]) == 1 and said["told"][0]["head"] == "ship:1 was fired on by a submarine at (5,0)",
          f"a salvo that missed is a slip that one of ours was fired on by a submarine, and where ({said['told']})")
    # seed 13, the Japanese side: the boat lost
    events, contacts, forces, _ = play(13, "japanese")
    said = words(events)
    check("submarine_reports" not in contacts and "suspected_waters" not in contacts and forces["submarines"][0]["state"] == "sunk",
          "the Japanese side holds no report and no waters, and its boat is sunk")
    check([t["head"] for t in said["told"]] == ["Lost: submarine:1"], f"a boat lost is the slip \"Lost: <boat>\" ({said['told']})")
    joined = " | ".join(said["lines"])
    check("Submarine 1 fired 4 torpedoes at <5,0>, at what she took for a carrier" in joined and "A submarine reports 4 hits on a carrier at <5,0>" in joined
          and "was hunted at <5,0> and is damaged; she is held down until the afternoon of D1" in joined and "Submarine 1 was sunk" in joined,
          f"the boat's own events are said: fired, a claim, hunted, sunk ({joined[:700]})")
    # seed 6, the Japanese side: held down
    events, contacts, forces, call = play(6, "japanese")
    boat = forces["submarines"][0]
    check(boat["state"] == "on station" and boat["hands"]["by"] == "free" and boat["salvoes_left"] == 1 and boat["held_down_until"]["pulse"], f"a boat's card has what it reads: {boat['state']}, free, one salvo left, held down")


def the_patrol_through_the_page(sim_cli, port):
    """The patrol and a boat's order through the bridge, as the page makes the calls (decision 0218, stage 3): a preview of a patrol
    gives each boat its station; a hold and a station order are queued, listed by `queued` as the page reads SIGNED and taken back; a
    patrol approved is the staff's, and a boat is in its hands under it."""
    print("-- the patrol and a boat's order, as the page makes the calls", flush=True)
    repo = os.path.dirname(HERE)
    path = os.path.join(repo, SUBMARINE_GAMES, "129_the_patrol.toml")
    command = [sys.executable, os.path.join(HERE, "serve.py"), path, "--seat", "japanese", "--start", "1942-08-24", "--port", str(port), "--sim-cli", sim_cli]
    process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
    try:
        check(process.stdout.readline().startswith("The table is at "), "the bridge is up on the table's copy of scenario 129, the Japanese seat")
        root = f"http://127.0.0.1:{port}"
        call = lambda tool, arguments=None: post(root + "/call", {"tool": tool, "arguments": arguments or {}})["ok"]
        forces = call("list_forces")["submarines"]
        check(len(forces) == 7 and all(b["hands"]["by"] == "free" for b in forces), "seven boats, each in no one's hands")
        preview = call("preview", {"command": "set_intent", "intent_kind": "patrol", "hex": [5, 0], "distance": 2})
        stations = preview["with_change"]["stations"]
        check(len(stations) == 7 and stations[0]["hex"]["q"] == 5 and stations[0]["hex"]["r"] == 0 and all(s["name"] and s["submarine"].startswith("submarine:") for s in stations),
              "a preview of a patrol gives each boat its station, by name, as the page draws and says them")
        order = call("issue_orders", {"orders": [{"unit": "submarine:1", "order": {"kind": "station", "hex": [7, 0]}}]})
        check(order["results"][0]["status"] == "queued", "a direct order gives a boat a station")
        queued = call("queued")["entries"]
        station = [e for e in queued if e.get("order", {}).get("kind") == "station"][0]
        check(station["unit"] == "submarine:1" and station["order"]["hex"] == [7, 0] and station["acts"]["at"] == "dusk",
              "SIGNED reads it from queued: the boat, the hex, acting at dusk")
        back = call("take_back", {"queue": station["place"]["queue"], "number": station["place"]["number"], "as_of": call("queued")["as_of"]})
        check(not call("queued")["entries"], f"it is taken back, and nothing of it is left queued ({back.get('summary', '')[:60]})")
        call("hold", {"submarine": 1})
        holds = call("command_state")["holds"]
        check(len(holds) == 1 and holds[0]["target"] == {"submarine": 1}, "a hold on a boat is listed with its target, so that her card and the staff's menu read it")
        entry = call("queued")
        hold = [e for e in entry["entries"] if e.get("tool") == "hold"][0]
        call("take_back", {"queue": hold["place"]["queue"], "number": hold["place"]["number"], "as_of": entry["as_of"]})
        check(not call("queued")["entries"] and not call("command_state")["holds"], "a hold on a boat is taken back as any entry is")
        preview = call("preview", {"command": "set_intent", "intent_kind": "patrol", "hex": [5, 0], "distance": 2})
        approved = call("approve", {"preview": preview["preview"]})
        check(approved["command"]["intent_kind"] == "patrol", "a patrol is approved as an order")
        proposed = call("propose", {"intent": 1})
        check(len(proposed.get("stations", [])) == 7 and proposed["stations"][0]["name"], "a plan that waits is proposed with its stations, which the plan's paper lists under the order, each boat by name")
        call("set_leave", {"leave": "prepare", "given": True})
        call("set_leave", {"leave": "launch", "given": True})
        call("end_day")
        boats = call("list_forces")["submarines"]
        check(all(b["hands"]["by"] == "staff" and b["hands"]["intent"] == 1 for b in boats), "each boat is then the staff's, under the order named")
    finally:
        process.send_signal(signal.SIGTERM)
        process.wait(30)


def a_refusal_through_the_page(bridge):
    """A refusal shown as NOT SENT, forced (decision 0221, stage 4). The engine refuses a malformed order in a real issue_orders
    call through the bridge; the page's own way from that answer to its words, from `signUnit` (the expression it uses for a
    refused result), `refusalWords` with its table of refusals and `paperLines`, which draws the line, is cut from the page's text
    and run in node on that answer. What is chosen: the real engine's refusal, the page's real functions, a stand-in for the
    document that only collects text; the paper itself in a browser is tried by hand."""
    print("-- a refusal said as NOT SENT: a malformed order through the page's own functions", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    answer = person.call("issue_orders", {"orders": [{"unit": "task_force:1", "order": {"kind": "set_route", "waypoints": [[300, 300]]}},
                                                     {"unit": "task_force:77", "order": {"kind": "set_speed", "speed": "full"}}]})["ok"]
    results = answer["results"]
    check(answer["queued"] == 0 and [r["status"] for r in results] == ["refused", "refused"] and all(r.get("reason") for r in results), f"the engine refuses both malformed orders: {results}")
    text = page_text()
    import re
    expression = re.search(r"why: (refusalWords\(r \? .*? : 'refused'\))", text)
    check(expression is not None, "the page words a refused result with refusalWords from its reason")
    code = (text[text.index("  function cleanName"):text.index("  function sayIntent")] + text[text.index("  function orderWords"):text.index("  function nameOf(")]
            + text[text.index("  function commas(n)"):text.index("  function commas(n)") + 400].split("\n  }\n")[0] + "\n  }\n"
            + text[text.index("  var REFUSALS = ["):text.index("  // What the status line says of orders taken in")]
            + text[text.index("  function paperLines(lines)"):text.index("  function signUnit()")]
            + "\nfunction main(results) { const lines = results.map((r, i) => ({ ok: false, label: ['Course', 'Speed'][i], why: " + expression.group(1) + " })); paperLines(lines); return $('p-says').kids.map(k => k.textContent); }")
    said = run_in_node(code, results)
    check(said[0] == "Course: NOT SENT — Waypoint (300,300) is off the map." and said[1] == "Speed: NOT SENT — It is not one of yours.",
          f"each refused line says NOT SENT with the engine's reason in the table's words ({said})")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def no_points_in_play(bridge):
    """No points while the game is in play (the designer's standing rule; decision 0221, the ruling after stage 4). The real
    situation of the 39 days, which carries the objectives' worth and the levels of victory, is put through the page's own
    `situationLine` and `noPoints`, cut from its text and run in node, line by line as `situationDoc` reads it; nothing shown says
    "points". The page itself writes the word only in HOW IT ENDED (`readEnd`)."""
    print("-- no points in play: the situation of the 39 days through the page's own functions", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    text = person.call("get_briefing", {"text": True})["ok"]["text"]
    import re
    check(re.search(r"\bpoints\b", text) and "Levels:" in text and "These are estimates from reports" in text,
          "the engine's situation carries the objectives' worth, the levels and the note on estimates")
    page = page_text()
    code = (page[page.index("  function noPoints(line)"):page.index("  function situationDoc(doc, text)")]
            + "\nfunction main(text) { let part = '', shown = []; text.split('\\n').forEach(function (line) { if (/^[A-Z][A-Z ]+$/.test(line)) { part = line; shown.push(line); return; } const l = situationLine(line, part, false); if (l != null) shown.push(l); }); return shown; }")
    shown = run_in_node(code, text)
    check(not any(re.search(r"\bpoints\b", l, re.I) for l in shown) and not any("Levels:" in l for l in shown),
          f"no line of the situation shown in play says points or the levels ({[l for l in shown if 'point' in l.lower()][:3]})")
    check(any("Objective: " in l for l in shown) and any("What the enemy is after:" in l for l in shown),
          "each objective, and what the enemy is after, is still said in words")
    check(not any("These are estimates from reports" in l for l in shown), "the note on how estimates work is left out")
    check("line = situationLine(line, part, M.over); if (line == null) return;" in page,
          "the situation paper draws each line through situationLine, which keeps the worth for the game's end")
    end = page[page.index("  function readEnd"):]
    end = end[:end.index("\n  }\n") + 4]
    literals = [m.group(0) for m in re.finditer(r"'[^'\n]*\b[Pp]oints\b[^'\n]*'", page)]
    check(literals and all(l in end for l in literals), f"the page writes the word points only in HOW IT ENDED ({literals})")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def queue_through_the_bridge(bridge):
    """The two tools SIGNED rests on, as the page calls them: queued lists what was signed, with a unit's hold going with
    its order, and take_back withdraws one entry and what goes with it, refused with the token out of date."""
    print("-- queued and take_back, as the page calls them: the Eastern Solomons over five days, the person Allied", flush=True)
    check(begin(bridge, "eastern-solomons-five-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    nothing = person.call("queued")["ok"]
    check(nothing["entries"] == [] and nothing["as_of"], "with nothing signed, queued lists no entry")
    person.call("hold", {"task_force": 1})
    person.call("issue_orders", {"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "full"}}]})
    person.call("set_leave", {"leave": "launch", "given": True})
    listed = person.call("queued")["ok"]
    by = {(e["place"]["queue"], e["place"]["number"]): e for e in listed["entries"]}
    order = [e for e in listed["entries"] if e["what"] == "order"][0]
    hold = [e for e in listed["entries"] if e.get("tool") == "hold"][0]
    check(len(listed["entries"]) == 3 and hold["rests_on"] == [order["place"]] and hold["place"] in order["goes_with"],
          "queued lists the order, the hold that rests on it (which goes with it) and the leave")
    check("refused" in person.raw("take_back", {"queue": "dusk", "number": 1, "as_of": "not this"}), "a take back with an out of date token is refused")
    taken = person.call("take_back", {"queue": order["place"]["queue"], "number": order["place"]["number"], "as_of": listed["as_of"]})["ok"]
    check(len(taken["taken_back"]) == 2 and [e["tool"] for e in taken["queued"]] == ["set_leave"],
          "take_back withdraws the order and the hold that goes with it, and lists what is left")
    again = person.call("take_back", {"queue": "commands", "number": 1, "as_of": taken["as_of"]})["ok"]
    check(again["queued"] == [] and person.call("queued")["ok"]["entries"] == [], "and the leave is taken back too: nothing is queued")
    # A unit's order refused by the engine after the hold the paper sends first at dusk: the hold is taken back, nothing is left queued.
    person.call("hold", {"task_force": 1})
    refused = person.call("issue_orders", {"orders": [{"unit": "task_force:1", "order": {"kind": "join", "into": "task_force:99"}}]})["ok"]
    check(refused["results"][0]["status"] == "refused", "an order the engine refuses leaves the hold sent before it queued")
    queued = person.call("queued")["ok"]
    hold = [e for e in queued["entries"] if e.get("command", {}).get("kind") == "hold" and e["command"].get("task_force") == 1]
    check(len(hold) == 1, "the page finds that hold in queued, as a command of kind hold naming the force")
    person.call("take_back", {"queue": hold[0]["place"]["queue"], "number": hold[0]["place"]["number"], "as_of": queued["as_of"]})
    check(person.call("queued")["ok"]["entries"] == [], "and takes it back: nothing of the refused order is left queued")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


FOLLOWED_GAMES = "data/scenarios/a-force-followed/"


def follow_code(text):
    """The page's own functions for an order against an enemy force (decision 0220) and the few of the page's they use, with stubs for
    what draws, run in node: the card's two orders, the order's paper, the rings, the slips and the stops."""
    helpers = "\n".join(cut_function(text, h) for h in ("function round(n)", "function pulsesOld(n)", "function kv(body, rows)", "function hexWords(pl)"))
    block = "\n".join(cut_function(text, h) for h in ("function heldContact(n)", "function destroyOrderWords(i)", "function followPaper(i)", "function followRings(st)",
                                                      "function noticesOfTold(told, day)", "function noticesOfReport(rep, day)", "function refusalsAtDusk(evs)", "function noticeSlips(box)", "function followOffer(c)"))
    stops = "\n".join(cut_function(text, h) for h in ("function stopOn()", "function noticeStops()"))
    tick_line = [line for line in text.split("\n") if line.strip().startswith("var TICK_DEFAULT")][0]
    notice_line = [line for line in text.split("\n") if line.strip().startswith("var NOTICE_STOPS")][0]
    return """
Object.defineProperty(Node.prototype, 'nodeType', { get() { return this.tag === '#text' ? 3 : 1; } });
Node.prototype.addEventListener = function (k, f) { (this.listeners = this.listeners || {})[k] = f; };
var WATCH = null, calls = [], placed = [], read = [], lastNotices = [], ticks = {}, GIVEN_ON = {}, REFUSED_SEEN = {};
function whereIs(pl) { return '<' + pl.q + ',' + pl.r + '>'; }
function num(h) { return parseInt(String(h).split(':')[1], 10); }
function placeLink(pl) { placed.push([pl.q, pl.r]); const s = el('span', ''); s.appendChild(el('button', 'ref', '<' + pl.q + ',' + pl.r + '>')); return s; }
function act(tool, args, done, home) { calls.push([tool, args]); return Promise.resolve(true); }
function readPlan(n) { read.push(n); }
function slip(from, stamp, title, body, decide, into) { const a = el('article', 'slip'); a.appendChild(el('span', 'from', from)); (into || { appendChild() {} }).appendChild(a); return a; }
function rich(tag, cls, text) { return el(tag, cls, text); }
""" + helpers + "\n" + tick_line + "\n" + notice_line + "\n" + block + "\n" + stops + """
function all(node, f) { f(node); (node.kids || []).forEach(k => all(k, f)); }
function buttons(node) { const out = []; all(node, n => { if (n.tag === 'button') out.push(n); }); return out; }
function click(b) { b.listeners.click(); }
function shown(i) { const p = followPaper(i); return p ? p.textContent : null; }
function main(x) {
  M.contacts = x.contacts; M.state = x.state; const out = {};
  // the card's two orders
  const offer = followOffer(x.contacts[0]), bs = buttons(offer);
  out.offered = bs.map(b => b.textContent); out.pressed = bs.map(b => b.attrs['aria-pressed'] === undefined ? null : b.attrs['aria-pressed']);
  out.offerText = offer.textContent; out.disabledAfter = [];
  const destroy = bs.filter(b => b.textContent === 'DESTROY THIS FORCE')[0], shadow = bs.filter(b => b.textContent === 'SHADOW IT')[0];
  click(destroy); out.afterDestroy = JSON.parse(JSON.stringify(calls)); out.disabledAfter.push(destroy.disabled === true);
  click(bs[2]); click(shadow); out.afterShadow = JSON.parse(JSON.stringify(calls)); out.pressedAfter = bs.map(b => b.attrs['aria-pressed'] === undefined ? null : b.attrs['aria-pressed']);
  // the paper, the rings, the words
  out.papers = x.papers.map(i => shown(i)); out.placed = placed.slice();
  out.rings = x.states.map(s => followRings(s));
  out.words = x.papers.map(i => destroyOrderWords(i));
  out.told = x.told.map(t => noticesOfTold(t[0], t[1])); out.report = x.reports.map(r => noticesOfReport(r[0], r[1]));
  const box = el('div', ''); lastNotices = x.slips; noticeSlips(box); out.slips = box.kids.map(k => k.textContent);
  // an order given inside a day on a report, refused at the next dusk where the report was dropped
  GIVEN_ON['1'] = { place: x.contacts[0].place, distance: 2, search_only: false }; GIVEN_ON['4'] = { place: x.contacts[0].place, distance: 4, search_only: true };
  out.refusals = refusalsAtDusk(x.events); out.again = refusalsAtDusk(x.events);
  lastNotices = out.refusals; const rbox = el('div', ''); noticeSlips(rbox); out.refusalSlips = rbox.kids.map(k => k.textContent);
  lastNotices = x.slips;
  out.ticks = {};
  Object.keys(TICK_DEFAULT).forEach(k => { ticks[k] = TICK_DEFAULT[k]; }); out.defaultStops = noticeStops(); out.allStops = stopOn();
  ticks.contact_lost = false; out.untickedLost = noticeStops(); out.untickedAll = stopOn(); ticks.contact_lost = true;
  ticks.follows_other_sort = false; ticks.force_not_found = false; out.onlyLost = noticeStops();
  return out;
}
"""


REFUSAL_EVENTS = [
    {"day": 2, "event_kind": "command_refused", "number": 5, "pulse": None, "entities": [],
     "text": "Allied the commander's command refused (contact 1 is not held: the commander's picture holds no contact of that number): set intent:1 to destroy the force of contact 1, within 2 hexes"},
    {"day": 2, "event_kind": "command_refused", "number": 6, "pulse": None, "entities": [],
     "text": "Allied the commander's command refused (contact 4 is not held: the commander's picture holds no contact of that number): set intent:2 to shadow the force of contact 4, within 4 hexes"},
    {"day": 2, "event_kind": "command_refused", "number": 7, "pulse": None, "entities": [], "text": "Allied the commander's command refused (set_leave twice): set leave prepare"},
    {"day": 2, "event_kind": "command_taken", "number": 8, "pulse": None, "entities": [], "text": "Allied the commander's command taken: set intent:3 to destroy the force of contact 2, within 2 hexes"},
]


def follow_place(q, r):
    return {"lat": None, "lon": None, "name": None, "q": q, "r": r}


def follow_intent(contact, hex_, ships, taken_for, age, reported, takings=(), lost=False, last=None, search_only=False, carrier=False, distance=2, in_force=True):
    place = follow_place(*hex_)
    last = last or (hex_, reported["day"])
    return {"number": 1, "kind": "destroy_force", "in_force": in_force, "contact": contact, "distance": distance, "search_only": search_only,
            "following": {"contact": contact, "held": not lost, "place": place, "hex": list(hex_), "ships": ships, "taken_for": taken_for, "carrier": carrier, "age_pulses": age,
                          "reported": reported, "takings": list(takings), "lost": lost, "last_place": follow_place(*last[0]), "last_day": last[1], "distance": distance,
                          "search_only": search_only, "ring": {"place": place, "hex": list(hex_), "distance": distance, "dashed": lost}, "text": []}}


def a_force_ordered(report_sample=None):
    """The page's statements 1 to 14 (decision 0220, "The page's check"): an order against an enemy force, by the page's text and by its own
    functions run in node on answers of the tools' shapes. Statement 15 is the bridge, below."""
    import re
    text = page_text()
    held = {"contact": {"handle": "contact:1", "name": None}, "place": follow_place(8, 0), "ship_count": 2, "taken_for": "cruiser", "age_pulses": 0}
    taking = {"day": 3, "taken_from": 1, "contact": 2, "other_sort": False, "place": follow_place(14, 0), "ships": 2, "taken_for": "cruiser", "carrier": False}
    to_carrier = {"day": 5, "taken_from": 2, "contact": 3, "other_sort": True, "place": follow_place(9, 0), "ships": 3, "taken_for": "carrier", "carrier": True, "was_carrier": False}
    from_carrier = {"day": 6, "taken_from": 3, "contact": 4, "other_sort": True, "place": follow_place(10, 0), "ships": 2, "taken_for": "cruiser", "carrier": False, "was_carrier": True}
    followed = follow_intent(2, (14, 0), 2, "cruiser", 1, {"day": 3, "pulse": "morning"}, [taking])
    twice = follow_intent(4, (10, 0), 2, "cruiser", 1, {"day": 6, "pulse": "morning"}, [taking, to_carrier, from_carrier])
    carrier_now = follow_intent(2, (14, 0), 2, "carrier", 1, {"day": 3, "pulse": "morning"}, [taking], carrier=True)
    lost = follow_intent(3, (8, 0), 2, "cruiser", 7, {"day": 4, "pulse": "morning"}, [taking], lost=True)
    shadow = follow_intent(2, (14, 0), 2, "cruiser", 1, {"day": 3, "pulse": "morning"}, [taking], search_only=True)
    ended = follow_intent(3, (8, 0), 2, "cruiser", 7, {"day": 4, "pulse": "morning"}, [taking], in_force=False)
    ended["following"] = None
    papers = [followed, twice, carrier_now, lost, shadow, ended]
    states = [{"intents": [followed]}, {"intents": [follow_intent(2, (15, 1), 2, "cruiser", 1, {"day": 3, "pulse": "morning"}, [taking])]}, {"intents": [lost]}, {"intents": [ended]}, {"intents": []}]
    lost_words = "We have lost the force: it was last reported at (8,0) in the morning of day 4, and we are searching where it was."
    ended_words = "We have not found the force last reported at (8,0) in the morning of day 4: the order is ended."
    other_words = "We now take the force for a carrier force: we follow the force of 3 carriers last reported at (9,0) in the morning of day 4, where before we followed one that was not."
    told = [[[{"thing": "contact_lost", "events": [105], "words": [lost_words]}, {"thing": "force_not_found", "events": [110], "words": [ended_words]},
              {"thing": "follows_other_sort", "events": [90], "words": [other_words]}, {"thing": "contact", "events": [4]}], 6]]
    same_words = "We follow another report of the force: the force of 2 cruisers last reported at (8,0) in the morning of day 4."
    matters = [{"kind": "contact_lost", "history": [{"day": 6, "why": 102}], "words": lost_words}, {"kind": "force_not_found", "history": [{"day": 7, "why": 109}], "words": ended_words},
               {"kind": "follows", "history": [{"day": 5, "why": 85}], "words": other_words}]
    same_sort = [{"kind": "follows", "history": [{"day": 5, "why": 85}], "words": same_words}]
    reports = [[{"matters": matters}, 6], [{"matters": matters}, 7], [{"matters": matters}, 5], [{"matters": same_sort}, 5]]
    slips = [{"thing": "contact_lost", "day": 6, "words": lost_words}, {"thing": "force_not_found", "day": 7, "words": ended_words}, {"thing": "follows_other_sort", "day": 5, "words": other_words}]
    said = run_in_node(follow_code(text), {"events": REFUSAL_EVENTS, "contacts": [held], "state": states[0], "papers": papers, "states": states, "told": told, "reports": reports, "slips": slips})
    # 1. where it is
    check(said["offered"][0] == "DESTROY THIS FORCE" and said["offered"][-1] == "SHADOW IT" and said["offered"][1:-1] == ["WITHIN 80 MILES", "WITHIN 160 MILES"],
          f"the card of a held contact has DESTROY THIS FORCE with the choice of 2 or 4 hexes round, then SHADOW IT ({said['offered']})")
    check(said["pressed"][1:3] == ["true", "false"], "the distance chosen is 2 hexes until the player chooses 4")
    wrapper = [line for line in text.split("\n") if "cardContact = function (t) { var r = c.apply" in line]
    check(len(wrapper) == 1 and wrapper[0].index("ordersHere(watersAt(t.contact.place))") < wrapper[0].index("insertBefore(followOffer(t.contact), kvs.nextSibling)")
          and "!WATCH && !M.over && !playing" in wrapper[0], "the two orders are put above the orders on the contact's waters, under the facts of the contact")
    check(text.count("followOffer(") == 2 and [line for line in text.split("\n") if "followOffer(t.contact)" in line] == wrapper,
          "no other card offers them: the call is in the card of a held contact alone, and not in the card of a contact no longer held, a place, a force or a base")
    # 2. what is sent
    check(said["afterDestroy"] == [["set_intent", {"intent_kind": "destroy_force", "contact": 1, "distance": 2, "search_only": False}]],
          f"DESTROY THIS FORCE sends one order, destroy a force, with the contact's number, 2 hexes and search only off ({said['afterDestroy']})")
    check(said["afterShadow"][1:] == [["set_intent", {"intent_kind": "destroy_force", "contact": 1, "distance": 4, "search_only": True}]] and said["pressedAfter"][1:3] == ["false", "true"],
          f"SHADOW IT sends the same with search only on, at the distance chosen ({said['afterShadow'][1:]})")
    check(all("hex" not in c[1] for c in said["afterShadow"]), "neither sends a hex")
    offer_src = cut_function(text, "function followOffer(c)")
    check(offer_src.count("act('set_intent'") == 1 and "api(" not in offer_src and "approve" not in offer_src and "preview" not in offer_src, "no other call is made to give the order: the one call of set_intent")
    # 3. at any hour
    check("clock" not in offer_src and "dusk" not in offer_src and "acts" not in offer_src.lower(), "both are shown at every stop, with no test of the hour")
    check(text.count("contact: c.contact, distance: c.distance, search_only: c.search_only") == 2, "an order signed is said, with the force it names, in SIGNED and in the orders, with the hour the staff acts on it")
    check("say('signed', done" in text and "function actsWords(e)" in text and "return clock.at === 'dusk' ? ACTS_TONIGHT : a.at === 'pulse' ? ACTS_NOW : ACTS_DUSK;" in text,
          "an order signed inside a day is drawn at once with the hour the staff acts on it, as the other orders are")
    check(said["words"][0] == "Destroy the enemy force reported at <14,0>, within 80 miles" and said["words"][4].startswith("Shadow the enemy force"),
          f"an order is said by the force's report ({said['words'][0]})")
    # 4. the paper names the force by its report
    paper = said["papers"][0]
    check(all(w in paper for w in ("Reported", "2 ships", "Taken for", "cruiser", "Where", "Report 1 pulse old")), f"the paper says what it follows now in the words of the card: {paper!r}")
    check(not re.search(r"\w+:\d+|\bcontact\b|\bintent\b", paper) and [14, 0] in said["placed"] and "placeLink(f.place)" in cut_function(text, "function followPaper(i)"),
          "it gives no contact number and no handle, and the place is a link made by the function the cards use")
    # 5. each taking, in order
    twice_text = said["papers"][1]
    days = [int(d) for d in re.findall(r"Day (\d+):", twice_text)]
    check(days == [3, 5, 6] and said["papers"][0].count("Day ") == 1, f"each taking is a line with its day, the earliest first ({days}), and a refresh adds none")
    check("Day 5: we now take the force for a carrier force; before, for one that was not, 3 ships taken for carrier" in twice_text
          and "Day 6: we no longer take the force for a carrier force; we now take it for cruiser, 2 ships taken for cruiser" in twice_text,
          "a taking of the other sort says what it is now taken for and what it was")
    # 6. a kind that changes
    check("Taken for" in said["papers"][2] and "carrier" in said["papers"][2].split("Taken for")[1][:12] and said["papers"][2].count("Day ") == said["papers"][0].count("Day "),
          "a kind that changes changes the words taken for on the paper and adds no line")
    check(said["report"][3] == [] and all("another report" not in s["words"] for r in said["report"] for s in r),
          "and no slip: a taking of the same sort is none")
    check("'Taken for'" in cut_function(text, "function cardContact(t)"), "the card of the contact says taken for as the contact is now taken")
    # 7. lost, and search only
    lost_text = said["papers"][3]
    check("We have lost the force. It was last reported at <8,0> on day 4; we are searching there." in lost_text and [8, 0] in said["placed"], f"a lost order's paper says the force is lost and where it was last reported, with the day ({lost_text!r})")
    check(not re.search(r"strike|carrier", said["papers"][4], re.I) and "only search" in said["papers"][4], f"a search-only order's paper lists no strike and no carrier force ({said['papers'][4]!r})")
    check(said["papers"][5] is None, "an order that is ended has no paper of what it follows")
    # 8. the chart
    check(said["rings"][0] == [{"hex": [14, 0], "distance": 2, "dashed": False, "order": 1, "label": "DESTROY"}], f"the order's waters are round the place of the report it follows ({said['rings'][0]})")
    check(said["rings"][1][0]["hex"] == [15, 1] and said["rings"][2][0]["hex"] == [8, 0] and said["rings"][2][0]["dashed"] is True,
          "the ring moves when the answer's place moves, and a lost order's is dashed round the last place")
    check(said["rings"][3] == [] and said["rings"][4] == [], "an order ended or withdrawn draws none")
    ring_src = cut_function(text, "function followRings(st)")
    check("i.following.ring" in ring_src and "Math" not in ring_src and "+" not in ring_src.replace("function", ""), "the ring is drawn from the order as the tool gives it: the page works no centre of its own")
    check("followRings(M.state).forEach(function (r) { var h = hexAt[r.hex[0] + ',' + r.hex[1]];" in text and "WATCH" not in [l for l in text.split("\n") if "followRings(M.state)" in l][0], "drawn for the seat watched as for the seat playing")
    # 9. the slips
    check(said["told"][0][0]["words"] == lost_words and said["told"][0][1]["words"] == ended_words and said["told"][0][2]["words"] == other_words and len(said["told"][0]) == 3,
          "each of the three notices that may stop the clock is a slip, in the staff's words as the tool gives them, and the other things told are not")
    check(all(s.startswith("FROM STAFF") for s in said["slips"]) and said["slips"][0].endswith(lost_words) and all(w in " ".join(said["slips"]) for w in ("We have lost the force", "We have not found the force", "We now take the force for a carrier force")),
          f"the slips are the staff's, as we ({said['slips'][0][:60]})")
    check(all("last reported at" in w for w in (lost_words, ended_words, other_words)), "each names the force as the paper does, by its report")
    # 9b. an order given inside a day on a report dropped by the dusk: refused there, and a slip from the staff at the dusk
    refused = said["refusals"]
    check(len(refused) == 2 and said["again"] == [] and said["refusalSlips"][0].startswith("FROM STAFF \u00b7 DAY 1"),
          f"a command refused as a contact not held is a slip, each once, and no other refusal or taking is ({said['refusalSlips']})")
    check(refused[0]["words"] == "We could not take up the order: destroy the enemy force reported at <8,0>, within 80 miles. We no longer hold the report of the force."
          and refused[1]["words"] == "We could not take up the order: shadow the enemy force reported at <8,0>, within 160 miles. We no longer hold the report of the force.",
          f"it names the force by the report the order was given on, as the paper does, and says we no longer hold it ({refused[0]['words']})")
    check(not re.search(r"\w+:\d+|\bcontact\b|\bintent\b|\d+ (hex|point)", " ".join(r["words"] for r in refused)) and re.search(r"\bWe\b", refused[0]["words"]),
          "it gives no number as a handle, no intent and no hex count, and the staff speaks as we")
    check("if (r.clock.at === '0600') lastNotices = lastNotices.concat(await duskRefusals(r.clock.day))" in text and "duskRefusals(M.day)" in text and "GIVEN_ON[n] = { place: c.place" in text, "the page reads the dusk's refusal where the clock reaches the dusk, by a pulse or by a day, and keeps the report an order was given on")
    # 10. the stops
    for stop, words in (("follows_other_sort", "a carrier where there was none, or none where there was one"), ("contact_lost", "a force lost"), ("force_not_found", "an order against a force ended")):
        check(f'<label class="notice"><input type="checkbox" id="stop-{stop}"> {words}</label>' in text, f"the panel has the stop {stop}: {words}")
    check(said["defaultStops"] == ["follows_other_sort", "contact_lost", "force_not_found"] and said["untickedLost"] == ["follows_other_sort", "force_not_found"] and said["onlyLost"] == ["contact_lost"]
          and "force_not_found" in said["allStops"] and "contact_lost" not in said["untickedAll"], "each is ticked unless unticked; a call asks for the ones ticked and no other")
    check("api('run', { days: 1, stop_on: noticeStops() })" in text and "api('advance', { pulses: 1, stop_on: stopOn() })" in text, "the calls of time from the page send the stops ticked")
    check([r[0]["thing"] for r in said["report"][:3]] == ["contact_lost", "force_not_found", "follows_other_sort"] and [len(r) for r in said["report"]] == [1, 1, 1, 0],
          "the watcher's slips are read from the report, each on the day it was told, a taking of the same sort making none")
    # 11. nothing can serve it
    plan = cut_function(text, "function readPlan(root, focus, limitFocus)")
    check("t.nothing.push(x.r.worked || '')" in plan and "/cannot (meet|be met)/i.test(x.r.worked || '')" in plan and "destroy_force" not in plan,
          "where the staff says the order cannot be met, the paper says so in the staff's words as given, as a deny's paper does")
    # 12. words, 13. no points
    every = " ".join([p for p in said["papers"] if p] + said["slips"] + said["words"] + [said["offerText"]] + said["refusalSlips"])
    check("order" in every.lower() and not re.search(r"\bintents?\b", every, re.I), "the text drawn holds order and never intent")
    check("the enemy" in every and not re.search(r"\bhis\b", every), "it says the enemy and never his")
    check(re.search(r"\bWe\b", every) and all(re.search(r"\bwe\b", p, re.I) for p in (said["papers"][0], said["papers"][3], said["slips"][0])), "the staff speaks as we")
    check(not re.search(r"knots|\bfresh\b|steam|days (left|until|to go)|within the distance|where the force was|may have|same force because", every, re.I),
          "no sentence explains a rule, and no figure of a rule is given")
    check(not re.search(r"points|victory|margin|score", every + text[text.index("// ---- an order against an enemy force"):text.index("// ---- end of an order against an enemy force")], re.I), "no points and no result")
    # 14. a game watched
    check("!WATCH && !M.over && !playing && kvs" in text and "var following = followPaper(top);" in plan and "if (!WATCH)" not in plan.split("var following = followPaper(top);")[0].split("\n")[-1],
          "the paper, the ring and the slips are drawn for the seat watched; the buttons are not")
    check("noticesOfReport(await api('report'), day)" in text and "followPaper(" in plan, "the watcher's slips are read through the tool `report`, as the seat reads it")


def a_force_followed_through_a_bridge(sim_cli, port):
    """The page's statement 15: on the table's game of scenario 133's game A at the Allied seat (the frozen figures with the reports
    made exact, as the scenario's test has them), the page's own functions run in node on what the bridge answers."""
    import shutil
    print("-- an order against a force on a real game: the table's copy of scenario 133, game A", flush=True)
    repo = os.path.dirname(HERE)
    text = page_text()
    with open(os.path.join(repo, FOLLOWED_GAMES, "133_a_followed.toml"), encoding="utf-8") as f:
        original = f.read()
    with tempfile.TemporaryDirectory() as folder:
        reference = os.path.join(folder, "reference")
        os.makedirs(reference)
        frozen = os.path.join(repo, "data", "reference", "step1")
        for name in os.listdir(frozen):
            if not name.endswith(".md"):
                shutil.copy(os.path.join(frozen, name), os.path.join(reference, name))
        search = os.path.join(reference, "search.toml")
        with open(search, encoding="utf-8") as f:
            figures = f.read()
        for was, now in (('position_error_chance = "0.3"', 'position_error_chance = "0"'), ('count_over_chance = "0.2"', 'count_over_chance = "0"'), ('count_under_chance = "0.2"', 'count_under_chance = "0"')):
            check(figures.count(was) == 1, f"the figures have {was} once, to be made exact")
            figures = figures.replace(was, now)
        head, rows = figures.split("[kind_confusion.", 1)
        for block in ("[kind_confusion." + rows).split("[kind_confusion."):
            if "]" in block:
                kind = block.split("]")[0]
                head += f'[kind_confusion.{kind}]\n{kind} = "1"\n\n'
        with open(search, "w", encoding="utf-8") as f:
            f.write(head)
        # The game as the table plays it: the order is given by the page, so the scenario's own commands are left out.
        scenario = original[:original.index("# The Allied commander's commands")]
        scenario = scenario.replace('"../../reference/', '"' + reference + "/").replace('"../../map/', '"' + os.path.join(repo, "data", "map") + "/")
        path = os.path.join(folder, "game.toml")
        with open(path, "w", encoding="utf-8") as out:
            out.write(scenario)
        command = [sys.executable, os.path.join(HERE, "serve.py"), path, "--seat", "allied", "--start", "1942-08-24", "--port", str(port), "--sim-cli", sim_cli]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
        try:
            check(process.stdout.readline().startswith("The table is at "), "the bridge is up on the table's copy of scenario 133, game A, the Allied seat")
            root = f"http://127.0.0.1:{port}"
            call = lambda tool, arguments=None: post(root + "/call", {"tool": tool, "arguments": arguments or {}})["ok"]

            def played(contacts, state, papers=None):
                return run_in_node(follow_code(text), {"contacts": contacts, "state": state, "papers": papers or [], "states": [state], "told": [], "reports": [], "slips": []})

            contacts = call("list_contacts")["contacts"]
            at_start = [c for c in contacts if (c["place"]["q"], c["place"]["r"]) == (8, 0)]
            check(len(at_start) == 1, "on the first evening the Allied seat holds one contact, at (8,0)")
            first = played(at_start, {"intents": []})
            check(first["offered"][0] == "DESTROY THIS FORCE" and first["offered"][-1] == "SHADOW IT", f"on the first evening its card has both buttons ({first['offered']})")
            sent = first["afterDestroy"][0]
            check(sent[0] == "set_intent" and sent[1] == {"intent_kind": "destroy_force", "contact": 1, "distance": 2, "search_only": False}, "the order given at 2 hexes is the page's own call")
            answer = call(sent[0], sent[1])
            check(answer["status"] == "queued", f"the staff takes it in ({answer['summary']})")
            call("set_leave", {"leave": "prepare", "given": True})
            call("set_leave", {"leave": "launch", "given": True})
            stops = played(at_start, {"intents": []})["defaultStops"]
            check(stops == ["follows_other_sort", "contact_lost", "force_not_found"], "the three stops are ticked")
            runs = []
            for day in range(1, 8):
                runs.append(call("run", {"days": 1, "stop_on": stops}))
                if day not in (3, 6, 7):
                    continue
                state = call("command_state")
                order = [i for i in state["intents"] if i["kind"] == "destroy_force"][0]
                seen = played(at_start, state, [order])
                paper = seen["papers"][0]
                told = run_in_node(follow_code(text), {"contacts": at_start, "state": state, "papers": [order], "states": [state], "told": [[runs[-1].get("told", []), day]], "reports": [], "slips": []})
                if day == 3:
                    check("2 ships" in paper and "cruiser" in paper and paper.count("Day 3:") == 1 and paper.count("Day ") == 1 and [14, 0] in seen["placed"] and "Taken for" in paper,
                          f"after three days the paper says the force is followed at the place of (14,0), 2 ships, taken for cruisers, with one taking of day 3 ({paper!r})")
                    check(seen["rings"][0] == [{"hex": [14, 0], "distance": 2, "dashed": False, "order": 1, "label": "DESTROY"}], "and the chart's ring is round that place")
                if day == 6:
                    check("We have lost the force" in paper and [8, 0] in seen["placed"] and "day 4" in paper, f"after six days the paper says lost, with the place and the day ({paper!r})")
                    check(seen["rings"][0] == [{"hex": [8, 0], "distance": 2, "dashed": True, "order": 1, "label": "DESTROY"}], "the ring is dashed round the place of (8,0)")
                    check(len(told["told"][0]) == 1 and told["told"][0][0]["thing"] == "contact_lost" and told["told"][0][0]["words"].startswith("We have lost the force"), "and the slip of the loss was drawn, in the staff's words")
                if day == 7:
                    check(paper is None and seen["rings"][0] == [], "after seven days the order is ended, and no ring is drawn")
                    check(len(told["told"][0]) == 1 and told["told"][0][0]["thing"] == "force_not_found" and "the order is ended" in told["told"][0][0]["words"], "and its slip was drawn")
            check(all(r.get("stopped") != "told" and not r.get("told") for r in runs[:5]) and runs[5].get("stopped") == "told" and runs[6].get("told", [{}])[0].get("thing") == "force_not_found",
                  "run with the three stops ticked, the clock stopped for the loss and for the end and for nothing else of this order")
        finally:
            process.send_signal(signal.SIGTERM)
            process.wait(30)


def a_refusal_at_dusk_through_a_bridge(sim_cli, port):
    """The page's refusal slip on a real game: scenario 133's game B on the table's copy (the frozen figures with the reports made exact,
    a report dropped after 2 pulses and orders given inside a day), an order against contact 1 given at 0600 of day 1; the contact is
    dropped by the afternoon, and at the dusk the engine refuses the order. The page's own function runs in node on the day's events."""
    import shutil
    print("-- an order given inside a day on a report dropped by the dusk: the table's copy of scenario 133, game B", flush=True)
    repo = os.path.dirname(HERE)
    text = page_text()
    with open(os.path.join(repo, FOLLOWED_GAMES, "133_b_which_contact.toml"), encoding="utf-8") as f:
        original = f.read()
    with tempfile.TemporaryDirectory() as folder:
        reference = os.path.join(folder, "reference")
        os.makedirs(reference)
        frozen = os.path.join(repo, "data", "reference", "step1")
        for name in os.listdir(frozen):
            if not name.endswith(".md"):
                shutil.copy(os.path.join(frozen, name), os.path.join(reference, name))
        search = os.path.join(reference, "search.toml")
        with open(search, encoding="utf-8") as f:
            figures = f.read()
        for was, now in (('position_error_chance = "0.3"', 'position_error_chance = "0"'), ('count_over_chance = "0.2"', 'count_over_chance = "0"'), ('count_under_chance = "0.2"', 'count_under_chance = "0"'),
                         ("contact_drop_age_pulses = 5", "contact_drop_age_pulses = 2")):
            check(figures.count(was) == 1, f"the figures have {was} once, to be changed")
            figures = figures.replace(was, now)
        head, rows = figures.split("[kind_confusion.", 1)
        for block in ("[kind_confusion." + rows).split("[kind_confusion."):
            if "]" in block:
                kind = block.split("]")[0]
                head += f'[kind_confusion.{kind}]\n{kind} = "1"\n\n'
        with open(search, "w", encoding="utf-8") as out:
            out.write(head)
        scenario = original[:original.index("# The Allied commander's commands")]
        scenario = scenario.replace('"../../reference/', '"' + reference + "/").replace('"../../map/', '"' + os.path.join(repo, "data", "map") + "/")
        scenario = scenario.replace("days = 3", "days = 3\norders_inside_a_day = true", 1)
        path = os.path.join(folder, "game.toml")
        with open(path, "w", encoding="utf-8") as out:
            out.write(scenario)
        command = [sys.executable, os.path.join(HERE, "serve.py"), path, "--seat", "allied", "--start", "1942-08-24", "--port", str(port), "--sim-cli", sim_cli]
        process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
        try:
            check(process.stdout.readline().startswith("The table is at "), "the bridge is up on the table's copy of scenario 133, game B, the Allied seat, with orders inside a day")
            call = lambda tool, arguments=None: post(f"http://127.0.0.1:{port}/call", {"tool": tool, "arguments": arguments or {}})["ok"]
            contacts = call("list_contacts")["contacts"]
            call("advance", {"pulses": 1})
            given = call("set_intent", {"intent_kind": "destroy_force", "contact": 1, "distance": 2, "search_only": False})
            check(given["status"] == "queued", f"inside the day the order is taken at once ({given['summary']})")
            said = None
            for _ in range(2):
                answer = call("advance", {"pulses": 1})
            check(answer["clock"]["at"] == "dusk" and call("list_contacts")["contacts"] == [], "by the dusk the report is dropped from the picture")
            answer = call("advance", {"pulses": 1})
            check(answer["clock"]["at"] == "0600" and answer["clock"]["day"] == 2, "the dusk is handed in, and the next day opens")
            events = call("events", {"day": answer["clock"]["day"]})["events"]
            said = run_in_node(follow_code(text), {"contacts": contacts, "state": {"intents": []}, "papers": [], "states": [], "told": [], "reports": [], "slips": [], "events": events})
            check([r["words"] for r in said["refusals"]] == ["We could not take up the order: destroy the enemy force reported at <2,0>, within 80 miles. We no longer hold the report of the force."],
                  f"the page's slip at the dusk, from the day's events as the tool gives them ({said['refusals']})")
            check(said["refusalSlips"][0].startswith("FROM STAFF") and said["again"] == [], "it is a slip from the staff, drawn once")
        finally:
            process.send_signal(signal.SIGTERM)
            process.wait(30)


def http(method, url, body=None):
    """(status, answer) of one request; an answer that is not JSON is its text."""
    data = None if body is None else json.dumps(body).encode()
    request = urllib.request.Request(url, data=data, method=method, headers={"Content-Type": "application/json"})
    try:
        with urllib.request.urlopen(request, timeout=900) as reply:
            status, raw = reply.status, reply.read()
    except urllib.error.HTTPError as error:
        status, raw = error.code, error.read()
    try:
        return status, json.loads(raw)
    except ValueError:
        return status, raw.decode()


def get(url):
    return http("GET", url)[1]


def post(url, body=None):
    return http("POST", url, body if body is not None else {})[1]


def later(function, *arguments):
    """A request made on a thread of its own; `.join()` it, then read `.answer`."""
    thread = threading.Thread(target=lambda: setattr(thread, "answer", function(*arguments)))
    thread.start()
    return thread


def wait_until(test, seconds=30):
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        if test():
            return True
        time.sleep(0.2)
    return False


class Bridge:
    """`table/serve.py` with no scenario, read until it has printed its address."""

    def __init__(self, port, sim_cli, hold, keep):
        command = [sys.executable, os.path.join(HERE, "serve.py"), "--port", str(port), "--sim-cli", sim_cli, "--hold", str(hold)]
        if keep:
            command += ["--keep", keep]
        self.process = subprocess.Popen(command, stdout=subprocess.PIPE, text=True)
        line = self.process.stdout.readline()
        if not line.startswith("The table is at "):
            raise AssertionError(f"the bridge printed {line!r}")
        self.table = line.split()[4].rstrip("/")
        self.root = f"http://127.0.0.1:{port}"

    def stop(self):
        if self.process.poll() is None:
            self.process.send_signal(signal.SIGTERM)
            try:
                self.process.wait(90)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait()


class Person:
    """The person's page, as its script calls the bridge."""

    def __init__(self, table):
        self.table = table

    def raw(self, tool, arguments=None, **extra):
        return post(self.table + "/call", dict({"tool": tool, "arguments": arguments or {}}, **extra))

    def call(self, tool, arguments=None):
        """A call, waiting as the page does while the answer says it waits."""
        answer = self.raw(tool, arguments)
        while isinstance(answer, dict) and answer.get("waiting"):
            answer = self.raw("wait")
        return answer


class Agent:
    def __init__(self, door):
        self.door = door

    def call(self, tool, arguments=None):
        return post(self.door + "/call", {"tool": tool, "arguments": arguments or {}})

    def through(self, tool, arguments=None):
        answer = self.call(tool, arguments)
        while isinstance(answer, dict) and answer.get("waiting"):
            answer = self.call("wait")
        return answer


def begin(bridge, scenario, allied, japanese):
    return post(bridge.table + "/begin", {"scenario": scenario, "allied": allied, "japanese": japanese})


def the_screen(bridge):
    print("-- the new game screen", flush=True)
    status, page = http("GET", bridge.root + "/")
    check(status == 200 and "<html" not in page, "the page is not served without the table's key")
    check(http("GET", bridge.root + "/table/not-a-key/config")[0] == 403, "a wrong key is refused (403)")
    check("<html" in get(bridge.table + "/"), "the page is served under the table's key")
    check(get(bridge.table + "/config") == {"screen": True}, "with no game in hand the page is told to show the screen")
    listed = get(bridge.table + "/scenarios")["scenarios"]
    check([s["id"] for s in listed] == ["ladder-1-one-ship", "eastern-solomons", "coral-sea", "midway", "eastern-solomons-five-days", "guadalcanal-39-days"],
          "the screen lists the scenarios simplest first: rung 1, the Eastern Solomons, the Coral Sea, Midway, the Eastern Solomons over five days and the 39 days")
    check(all("file" not in p for s in listed for p in s["pairings"]), "the list given to the screen names no file")
    check([len(s["pairings"]) for s in listed] == [1, 4, 7, 7, 7, 7],
          "rung 1 offers 1 game, the Eastern Solomons 4, the Coral Sea 7, Midway 7, the five-day Eastern Solomons 7 and the 39 days 7")
    # Every battle that has boats has them (decisions 0225 and 0227): one game of the battle, and it plays the copies with boats in
    # table-boats/; its directives name each side's boats in the situation's last sentence.
    entries = {e["id"]: e for e in json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]}
    for gid, name, files, words in BOAT_GAMES:
        entry = entries[gid]
        check(entry["name"] == name and sorted({p["file"] for p in entry["pairings"]}) == sorted(("data/scenarios/table-rear-bases/" if gid == "guadalcanal-39-days" else "data/scenarios/table-boats/") + f for f in files),
              f"{name} is one game, and its pairings play the copies with submarines: " + ", ".join(files))
        for side in ("allied", "japanese"):
            situation = json.load(open(os.path.join(os.path.dirname(HERE), entry["directives"][side])))["directives"][0]["parts"][0]["text"]
            check(situation[-1].startswith(words[side]) and sum("submarine" in t for t in situation) == 1,
                  f"{name}: the {side} directive's situation ends with the one sentence naming the side's boats")
    seat = get(bridge.table + "/agent?side=allied&scenario=guadalcanal-39-days")["ok"]
    program = os.path.join(HERE, "seat_mcp.py")
    check(seat["claude"].startswith("claude mcp add flag-plot-allied -- python3 " + program + " --url " + bridge.root + "/seat/"),
          "the agent's seat gives the claude mcp add line with the whole path")
    settings = json.loads(seat["settings"])["mcpServers"]["flag-plot-allied"]
    check(settings["args"] == [program, "--url", seat["door"]], "and the same as an entry of tool-server settings")
    check("Seize and hold Tulagi" in seat["brief"] and "to its end" in seat["brief"] and "Read nothing but" in seat["brief"],
          "the brief gives the side, its directive, to play to the end and to read nothing else")
    check(len(seat["notes"]) == 3 and "fifty seconds" in seat["notes"][2], "the person is told the three things beside them")
    jap = get(bridge.table + "/agent?side=japanese&scenario=guadalcanal-39-days")["ok"]
    check("Hold Lunga Point and Tulagi, or retake them" in jap["brief"] and "Commander in Chief, Combined Fleet" in jap["brief"],
          "the other side's brief gives its own directive, from its own commander above")
    for entry in json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]:
        for side in ("allied", "japanese"):
            # A side only the engine ever holds needs no directive (decision 0237).
            if not any(p[side] in ("person", "agent") for p in entry["pairings"]):
                continue
            file = entry.get("directives", {}).get(side)
            first = json.load(open(os.path.join(os.path.dirname(HERE), file)))["directives"][0] if file else {}
            check(first.get("when") == "start" and any("mission" in p["heading"].lower() for p in first.get("parts", [])),
                  entry["id"] + ": the " + side + " side has a directive at the start, with a mission")
    check("no game is in hand" in Agent(seat["door"]).call("list_forces")["refused"], "a door with no game in hand refuses")
    check("not offered" in begin(bridge, "eastern-solomons", "person", "agent")["refused"],
          "a game the list does not offer is refused (the Eastern Solomons, a person against an agent)")
    return listed


def every_game(bridge, listed):
    """Each game the list offers begins and plays: its file loads and plays."""
    print("-- every game offered begins and plays", flush=True)
    doors = {side: get(bridge.table + f"/agent?side={side}&scenario=eastern-solomons")["ok"]["door"] for side in SIDES}
    for entry in listed:
        for pairing in entry["pairings"]:
            name = f"{entry['id']}: allied {pairing['allied']}, japanese {pairing['japanese']}"
            check(begin(bridge, entry["id"], pairing["allied"], pairing["japanese"]) == {"ok": True}, f"{name}: begun")
            config = get(bridge.table + "/config")
            person = next((s for s in SIDES if pairing[s] == "person"), None)
            agents = [s for s in SIDES if pairing[s] == "agent"]
            if person and not agents:
                brief = Person(bridge.table).call("get_briefing", {"text": False})["ok"]
                end = Person(bridge.table).call("end_day")["ok"]
                check(brief["heading"]["side"] == person and end["resolved"] is True and config["seat"] == person,
                      f"{name}: the person's briefing, and a day resolved")
                page = Person(bridge.table)
                if end.get("game_over"):
                    # A page loaded after the end reads the umpire's result from the final briefing.
                    final = page.call("get_briefing", {"text": False})["ok"]
                    check(final["objectives"].get("result") == end.get("result"),
                          f"{name}: after the end the briefing carries end_day's result, as a page loaded then reads it")
                elif "advance" in get(bridge.table + "/tools"):
                    # A page loaded inside a day, with no briefing, reads the game's last day from a yard measure.
                    page.call("advance", {"pulses": 1})
                    handle = page.call("list_forces")["ok"]["task_forces"][0]["task_force"]["handle"]
                    yard = page.call("measure", {"question": "yard", "task_force": handle})["ok"]
                    check("refused" in page.raw("get_briefing", {"text": False}) and yard["last_day"] == brief["heading"]["last_day"],
                          f"{name}: inside a day a yard measure gives the last day ({yard['last_day']}), as a page loaded then reads it")
            elif person:
                agent = Agent(doors[agents[0]])
                a = later(agent.through, "end_day")
                answer = Person(bridge.table).call("end_day")["ok"]
                a.join(60)
                check(answer["resolved"] is True and a.answer["ok"]["resolved"] is True and "versus" in config,
                      f"{name}: both finish, the bridge steps, both are answered")
            else:
                held = [later(Agent(doors[side]).through, "end_day") for side in agents]
                check(wait_until(lambda: get(bridge.table + "/standing")["ok"]["waiting_for"] == []), f"{name}: the agents are ready")
                step = post(bridge.table + "/step")["ok"]
                check(step["resolved"]["day"] == 1, f"{name}: the watcher steps day 1")
                # A game with orders inside a day is stepped a pulse at a time: step on until the day is resolved.
                for _ in range(6):
                    if not config["watch"]["inside_day"] or wait_until(lambda: not any(t.is_alive() for t in held), 5):
                        break
                    post(bridge.table + "/step")
                for t in held:
                    t.join(60)
                check(all(t.answer["ok"]["resolved"] is True for t in held) and config["watch"]["agents"] == agents,
                      f"{name}: the watcher steps, each agent's end_day answered")
    check(post(bridge.table + "/new") == {"ok": True} and get(bridge.table + "/config") == {"screen": True},
          "New game ends the game in hand and the screen is shown again")


def orders_on_the_card(bridge):
    """What the page reads to show an order in force on a place's card, and to
    tell the commander's own direct order from a staff order in the diary."""
    print("-- orders in force, and whose an accepted order was: the 39 days, the person Allied", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    page = get(bridge.table + "/")
    check("OR ONLY" in page and "Your direct order to " in page and "Staff order to " in page,
          "the page groups a wider order above its parts, and words a direct order as the commander's")
    person.call("set_leave", {"leave": "launch", "given": True})
    person.call("set_intent", {"intent_kind": "hold", "base": 3})
    person.call("set_intent", {"intent_kind": "operate_carriers", "task_force": 2, "hex": [6, -3]})
    held = [i for i in person.call("command_state")["ok"]["intents"] if i["kind"] == "hold"]
    check(len(held) == 1 and held[0]["base"]["handle"] == "base:3" and held[0]["in_force"] is True
          and held[0]["maker"] == "commander" and held[0]["parent"] is None,
          "a signed hold is in command_state at once, in force, by its kind, base, maker and parent, as the card reads it")
    person.call("end_day")
    queued = person.call("issue_orders", {"orders": [{"unit": "task_force:1", "order": {"kind": "set_speed", "speed": "cruise"}}]})["ok"]
    check(queued["results"][0]["status"] == "queued", "a direct order to Task Force 16 is queued")
    person.call("end_day")
    events = person.call("events", {"day": 2})["ok"]["events"]
    accepted = [(e, events[k + 1] if k + 1 < len(events) else None) for k, e in enumerate(events) if e["event_kind"] == "order_accepted"]
    staffs = lambda nx: nx is not None and nx["event_kind"] == "staff_reason" and "staff's reason for order:" in nx["text"] and "[serves intent " in nx["text"]
    own = [e for e, nx in accepted if "task_force:1" in e["text"] and "set_speed" in e["text"]]
    check(len(own) == 1 and not any(staffs(nx) for e, nx in accepted if e is own[0]),
          "the commander's direct order is accepted with no staff's reason after it")
    check(any(staffs(nx) for e, nx in accepted if "task_force:2" in e["text"]),
          "each staff order to Task Force 11 is accepted with the staff's reason straight after it, naming the order it serves")
    # The men put ashore at Lunga, and a hold signed as the page signs it, with no source named: the staff chooses one, the page does
    # not (decision 0228). The tool says where the order is supplied from, and who chose it.
    person.call("issue_orders", {"orders": [
        {"unit": "task_force:9", "order": {"kind": "set_route", "waypoints": [[2, -4]], "act_at": [[2, -4]]}},
        {"unit": "task_force:9", "order": {"kind": "unload"}}]})
    person.call("end_day")
    signed = person.call("set_intent", {"intent_kind": "hold", "base": 1})["ok"]
    person.call("end_day")
    state = person.call("command_state")["ok"]
    mine = [i for i in state["intents"] if i["number"] == signed["intent"]]
    check(len(mine) == 1 and "supplied_from" in mine[0],
          "command_state gives a hold where it is supplied from (decision 0228, rule 18)")
    said_from = mine[0]["supplied_from"] if mine else None
    check(said_from is not None and said_from["chosen_by"] == "staff" and said_from["base"] == 3,
          f"the staff chose Espiritu Santo, the one base with supply to spare, as the source of the hold of Lunga ({said_from})")
    land_unit_orders(bridge, person)
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def land_unit_orders(bridge, person):
    """The order form on a land unit's card (decision 0216), as far as a script can reach it: the page
    holds the form and its words, and the three orders go to the tool in the shapes the form sends them,
    after the hold the form sends first. The form itself is drawn in a browser and is not run here."""
    page = get(bridge.table + "/")
    check("function landControls(form, u, held)" in page and "NOT SENT" in page and "Acts at 1800, for the coming day" in page,
          "the page holds the land unit's controls for the one order paper, the line for a refusal and the line for an order kept to dusk")
    units = person.call("list_forces")["ok"]["land_units"]
    fighting = [u for u in units if u["men_ashore"] and u.get("march_to") and not u.get("working")]
    check(fighting, "a fighting unit with men ashore lists the places it can march to (march_to)")
    unit = fighting[0]
    handle, number, hex_to = unit["land_unit"]["handle"], int(unit["land_unit"]["handle"].split(":")[1]), unit["march_to"][0]
    check(person.call("hold", {"land_unit": number})["ok"]["status"] == "queued", "the hold the form sends first is taken for a land unit")
    sent = person.call("issue_orders", {"orders": [
        {"unit": handle, "order": {"kind": "set_posture", "posture": "attack", "when_spent": "hold"}},
        {"unit": handle, "order": {"kind": "march", "hex": [hex_to["q"], hex_to["r"]]}},
        {"unit": handle, "order": {"kind": "set_activity", "activity": "work", "job": "port"}}]})["ok"]
    check([r["status"] for r in sent["results"]] == ["queued"] * 3, "a posture with when spent, a march and a work order with a job are taken, as the form sends them")
    held = [h for h in person.call("command_state")["ok"]["holds"] if h["target"].get("land_unit") == number]
    check(len(held) == 1, "the unit is held from the staff, as its card says")
    refused = person.call("issue_orders", {"orders": [{"unit": handle, "order": {"kind": "march", "hex": [unit["place"]["q"] + 50, unit["place"]["r"]]}}]})["ok"]["results"][0]
    check(refused["status"] == "refused", "a march to a place no land link joins is refused, for the form to show as NOT SENT")
    check(person.call("release", {"land_unit": number})["ok"]["status"] == "queued", "the unit is released to the staff by the button the card has")


def the_paper_in_words(bridge):
    """The plan's paper, as its words are made from a real proposal (the 39 days, the order to control Guadalcanal):
    the tree is given by the tool, each task with its parent and each order with the task it serves, and the staff's
    words, run through the page's own cleaning in node, hold no engine phrase (a hex count, "nearest hex", a raw
    fatigue figure, a plural in brackets)."""
    import re
    import shutil
    print("-- the plan's paper: the tree from propose, and the staff's words made plain", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun")
    person = Person(bridge.table)
    shown = person.call("preview", {"command": "set_intent", "intent_kind": "control_island", "island": 1})["ok"]
    person.call("approve", {"preview": shown["preview"]})
    proposal = person.call("propose", {"intent": 1})["ok"]
    tree = proposal["tree"]
    numbers = {1} | {s["number"] for s in tree["sub_intents"]}
    check(tree["sub_intents"] and all(isinstance(s["parent"], int) and s["parent"] in numbers for s in tree["sub_intents"]),
          f"propose gives the tree: {len(tree['sub_intents'])} tasks, each with its parent in the tree")
    hand = proposal["would_hand_in"]["orders"]
    check(hand and all(x["intent"] in numbers and x["unit"] and x["order"]["kind"] for x in hand),
          f"each of the {len(hand)} orders it would hand in names the task it serves, the unit and the order")
    check(any(a["from_deg"] is not None and a["range_nm"] for a in tree["arcs"]) and all(a["intent"] in numbers for a in tree["arcs"]),
          "a search arc is a bearing range and a range, with the task it serves")
    texts = [proposal["reason"]["worked"]] + list(tree["no_orders"]) + [a["how"] for a in tree["arcs"]]
    for sub in tree["sub_intents"]:
        r = sub["reason"]
        texts += [r["worked"]] + [e["what"] for e in r["evidence"]] + r["binding"] + r["would_change"]
    for x in hand:
        texts += [x["reason"]["worked"]] + [e["what"] for e in x["reason"]["evidence"]] + x["reason"]["binding"]
    for g in tree["groups"]:
        texts += [o["worked"] for o in g["orders"]]
    node = shutil.which("node")
    with open(os.path.join(HERE, "index.html"), encoding="utf-8") as page:
        text = page.read()
    code = text[text.index("  function shareWords"):text.index("  function paperText")] + text[text.index("  function cutTop"):text.index("  function workedView")]
    with tempfile.TemporaryDirectory() as folder:
        path = os.path.join(folder, "words.js")
        with open(path, "w", encoding="utf-8") as out:
            out.write(code + "\nconst t = JSON.parse(require('fs').readFileSync(process.argv[2], 'utf8')); console.log(JSON.stringify(t.map(x => tidyWorked(paperPre(x)))));\n")
        data = os.path.join(folder, "texts.json")
        with open(data, "w", encoding="utf-8") as out:
            json.dump(texts, out)
        done = subprocess.run([node, path, data], capture_output=True, text=True)
        check(done.returncode == 0, "the page's cleaning of the staff's words runs in node" + ("" if done.returncode == 0 else ": " + done.stderr.strip()[:300]))
        said = json.loads(done.stdout)
    phrases = r"hex\(es\)|nearest hex|\bfatigue \d|\w\(s\)"
    raw = [t for t in texts if re.search(phrases, t)]
    check(raw, f"the staff's own words do hold the phrases the paper replaces ({len(raw)} of {len(texts)} texts)")
    left = [t for t in said if re.search(phrases + r"|\b(25|50|75) in 100\b", t)]
    check(not left, "made plain, none holds a hex count, \"nearest hex\", a raw fatigue figure, a plural in brackets or a share as N in 100 where a word says it" + (": " + left[0][:160] if left else ""))
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_staffs_reading(bridge):
    """The staff's reading in the order paper is the plan's tree (decision 0221, "As built: the staff's reading"). Real `preview`
    answers, as the paper asks them, are put through the page's own functions cut from its text and run in node: `proposedTree`, the
    plan paper's tree, with `readingTop` for the one line at the head, and `paperText`, which words every line of the tree. What is
    chosen: the real engine's answers and the page's real functions, with the few things they read from the rest of the page stood in
    (the forces by name, no map); the paper itself in a browser is tried by hand. Three orders: to deny waters near Port Moresby in the
    Coral Sea, the same with Task Force 17 already given an order, so that nothing can serve it, and to seize and hold Guadalcanal."""
    import re
    print("-- the staff's reading is the plan's tree: real previews through the page's own functions", flush=True)
    text = page_text()
    ask = text[text.index("  function askPreview() {"):text.index("  $('p-dry').addEventListener('click'")]
    check("It cannot yet" not in text and "commands of its own" not in text and "It would ' + p.shown" not in text and "countOf((w.orders" not in text,
          "the page no longer words the engine's paragraph: no \"It cannot yet\", no count of orders and commands of its own")
    check("readingView(out, p, e);" in ask and "proposedTree(top, {}, pr)" in text and "paperNode(root, true)" in text,
          "the reading is drawn by readingView from proposedTree and paperNode, the plan paper's own functions")
    check("This reading is worked with the limits you set." in ask and "args.limits" in ask and "limit: 'risk', risk: risk" in text and "Played at balanced risk" not in text and "worked at balanced risk" not in text,
          "the reading is worked with the limits and the risk chosen, and says so where any is sent; nothing says it is worked at balanced risk")
    check("'THE STAFF\\'S WORKING IN FULL'" in text[text.index("  function readingView"):text.index("  function askPreview() {")], "the staff's working in full is the reading's last fold")
    heads = ["commas", "cleanName", "orderWords", "plainCounts", "plainWords", "nameText", "intentWords", "reasonWords", "tidyWorked", "paperPre", "shareWords", "tiredBit",
             "paperText", "plainFact", "cautionPart", "cautionWords", "cautionTexts", "cutTop", "sayIntent", "newTask", "taskHex", "labelPlain", "arcsFrom", "finishTask",
             "threatsOf", "proposedTree", "draftTop", "readingTop"]
    code = "const config = { islands: [{ number: 1, name: 'New Guinea' }] }; let hexRaw = false, PAPER_TASKS = null; M.forces = [{ task_force: { handle: 'task_force:1', name: 'Task Force 17' } }]; M.bases = []; M.enemyBases = []; M.names['task_force:1'] = 'Task Force 17'; M.names['base:1'] = 'Port Moresby';\n"
    code += "function dateOf() { return ''; } function num(h) { return parseInt(String(h).split(':')[1], 10); }\n" + re.search(r"  var WORD = .*\n", text).group(0)
    code += "".join(cut_function(text, f"  function {h}(") + "\n" for h in heads)
    code += r"""
function main(input) {
  const out = [];
  input.forEach(function (a) {
    const w = a.preview.with_change, pr = { tree: w.tree, would_hand_in: w.would_hand_in, stations: null, reason: w.reason }, e = [a.preview.shown, a.end];
    const top = draftTop(e, a.preview), root = proposedTree(top, {}, pr); PAPER_TASKS[top.number] = 'this order';
    const lead = readingTop(root, top.kind), lines = [], all = [root];
    for (let i = 0; i < all.length; i++) { all[i].kids.forEach(k => all.push(k)); }
    all.forEach(t => { t.facts.forEach(f => lines.push(paperText(f, t.n))); t.reasons.forEach(r => (r.binding || []).concat(r.would_change || []).forEach(s => lines.push(paperText(s, t.n)))); t.nothing.forEach(s => lines.push(paperText(s, t.n))); });
    lines.push(paperText(w.reason.worked, null));
    out.push({ name: a.name, lead: lead.text, bad: !!lead.bad, lines: lines, root: top.number, kids: root.kids.map(k => k.label) });
    PAPER_TASKS = null;
  });
  return out;
}
"""
    previews = []
    for name, scenario, first, end in [
        ("coral", "coral-sea", None, {"intent_kind": "deny", "hex": [-18, -1], "distance": 4}),
        ("coral-taken", "coral-sea", {"intent_kind": "operate_carriers", "task_force": 1, "hex": [-14, -2]}, {"intent_kind": "deny", "hex": [-18, -1], "distance": 4}),
        ("days", "guadalcanal-39-days", None, {"intent_kind": "control_island", "island": 1}),
    ]:
        check(begin(bridge, scenario, "person", "engine") == {"ok": True}, f"begun: {name}")
        person = Person(bridge.table)
        if first:
            shown = person.call("preview", dict(command="set_intent", **first))["ok"]
            check(person.call("approve", {"preview": shown["preview"]})["ok"].get("kind") == "command", "an order for Task Force 17 is given first, so that it is not free")
        answer = person.call("preview", dict(command="set_intent", **end))["ok"]
        w = answer["with_change"]
        check(w.get("tree") and "would_hand_in" in w and "reason" in w, f"{name}: the engine's preview carries the tree, what it would hand in and its reason")
        previews.append({"name": name, "preview": answer, "end": end})
        check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    said = run_in_node(code, previews)
    banned = r"miles\(es\)|hex\(es\)|nm from the nearest|commands of its own|It cannot yet|\border \d+\b|\bintent\b|\bintent:\d+"
    for a in said:
        every = [a["lead"]] + a["lines"]
        bad = [t for t in every if re.search(banned, t)]
        check(not bad, f"{a['name']}: nothing in the reading says \"miles(es)\", \"hex(es)\", \"nm from the nearest\", a count of commands, \"It cannot yet\" or an order number the draft does not have" + (f": {bad[0][:160]}" if bad else ""))
    coral, taken, days = said
    check(coral["lead"].startswith("Task Force 17 would work these waters from ") and not coral["bad"], f"coral: the line at the head says what would serve the order ({coral['lead']})")
    check(any(t.startswith("No air base can reach these waters: ") and "in these waters" in t for t in coral["lines"]),
          "coral: Port Moresby, inside the waters, is said as in these waters, once, in a fact about what cannot be done")
    check(taken["lead"] == "Nothing can serve this order yet." and taken["bad"], f"coral with the force given an order: the head says nothing can serve it ({taken['lead']})")
    check(any("Task Force 17 is not free: it serves ⟦1⟧" in t for t in taken["lines"]),
          "that reason names the order the force serves, as a mark the page draws as a link to its paper")
    check(any("⟦1⟧ withdrawn" in t for t in taken["lines"]), "what would change it includes that order withdrawn")
    check(days["lead"] == "The staff would take it up in these tasks:" and len(days["kids"]) >= 3, f"the 39 days: the tasks are the tree under the order ({days['lead']}, {len(days['kids'])} under it)")
    check(any(t.startswith("No air base can reach ") for t in days["lines"]), "the 39 days: a raid the staff cannot form is said as a fact, in words")
    check("function orderLink(n)" in text and "orderFound(n)" in text and "orderLink(+m[5])" in text and "readPlan(found.root, found.task" in text,
          "the order a unit is not free for is found by its number, walked up to its root in command_state, and opens that order's paper at the task")
    check("serves ⟦" in text and "has a carrier task, intent:" in text, "the page reads the engine's phrase for a carrier task and for a task an order higher in priority holds")


# The games with submarines at the table (decisions 0225 and 0227): the game's id, its name, the files its pairings play, and the first
# words of the sentence that ends each side's directive.
BOAT_GAMES = [
    ("coral-sea", "The Coral Sea", ["03" + l + "_coral_sea_both_oilers_submarines.toml" for l in "bcd"],
     {"allied": "One of our submarines, S-42, is on patrol south-east of Cape St George", "japanese": "Three of our submarines are at sea and under your command"}),
    ("midway", "Midway", ["01" + l + "_midway_submarines.toml" for l in "bcd"],
     {"allied": "Fourteen of our submarines are on station", "japanese": "The submarine I-168 is off Midway"}),
    ("eastern-solomons-five-days", "The Eastern Solomons: five days", ["02" + l + "_eastern_solomons_oilers_submarines.toml" for l in "bcd"],
     {"allied": "One of our submarines, S-42, is due on station off Savo Island", "japanese": "Four of our submarines are under your command"}),
    ("guadalcanal-39-days", "Guadalcanal: the first thirty-nine days", ["07b_slice_submarines.toml", "07c_slice_submarines.toml", "07_full_slice_submarines.toml"],
     {"allied": "One of our submarines, S-38, is on patrol off Cape St George", "japanese": "No submarine of ours is on station yet"}),
]

# What each seat holds of its own boats on the first day, as list_forces gives them: the game, then each side's boats in the side's own
# order as (name, state). A boat that enters on a later day is listed, held apart, until her day.
ON, HELD = "on station", "held apart"
BOATS_ON_DAY_1 = [
    ("coral-sea", [("USS S-42", ON)], [("I-21", ON), ("RO-33", ON), ("RO-34", ON)]),
    ("midway", [(n, ON) for n in ("USS Tambor", "USS Trout", "USS Nautilus", "USS Gato", "USS Grayling", "USS Grouper", "USS Gudgeon", "USS Grenadier",
                                  "USS Flying Fish", "USS Narwhal", "USS Trigger", "USS Dolphin", "USS Cachalot", "USS Cuttlefish")], [("I-168", ON)]),
    ("eastern-solomons-five-days", [("USS S-42", HELD)], [("I-19", ON), ("I-17", ON), ("I-33", HELD), ("RO-34", ON)]),
    ("guadalcanal-39-days", [("USS S-38", ON), ("USS S-44", HELD), ("USS S-37", HELD), ("USS S-42", HELD), ("USS S-45", HELD)],
     [(n, HELD) for n in ("I-122", "I-123", "RO-34", "I-19", "I-17", "I-33", "I-26", "I-11")]),
]


LIMITS_IN_NODE = r"""
function whereIs() { return ''; } function dateOf(d) { return d === 10 ? '12 May' : ''; } function sayIntent(i) { return 'Capture Lunga'; }
const hexes = []; for (let q = -4; q <= 4; q++) for (let r = -4; r <= 4; r++) hexes.push([q, r, 0, 0, 0, 0]);
function main() {
  M.names['task_force:1'] = 'Task Force 17'; M.names['task_force:2'] = 'Task Force 11'; M.names['ship:7'] = 'USS Hornet'; M.names['air_group:3'] = 'VB-6'; M.names['base:2'] = 'Lunga'; M.names['land_unit:4'] = 'the 1st Marines'; M.names['submarine:1'] = 'USS Tambor';
  M.hexNames['0,0'] = 'Lunga'; M.state = { intents: [{ number: 5, kind: 'take_place', in_force: true }] };
  const out = {};
  const deny = newLim(); deny.ships = { mode: 'only', task_forces: [1], ships: [] }; deny.in_place = true; deny.air_groups.mode = 'none'; deny.targets = { kinds: ['carrier', 'cruiser'], waters_first: true };
  out.deny = limitBodies('deny', deny); out.denyLine = limitsLine('deny', deny); out.denyPlan = limitPlan('deny', deny).map(p => p.key);
  const lone_ = newLim(); lone_.ships = { mode: 'only', task_forces: [1], ships: [] }; out.coverShips = limitBodies('cover', lone_); out.coverOffers = LIMITS_OFFERED.cover; out.patrolOffers = LIMITS_OFFERED.patrol;
  out.back = limitsLine('deny', limFromBodies(out.deny.map(b => Object.assign({ intent: 3 }, b)))) === out.denyLine;
  out.backBodies = limitBodies('deny', limFromBodies(out.deny.map(b => Object.assign({ intent: 3 }, b))));
  out.none = limitBodies('hold', newLim()); out.noneLine = limitsLine('hold', newLim());
  const src = newLim(); src.source = 2; out.source = limitBodies('keep_supplied', src); out.sourceLine = limitsLine('keep_supplied', src);
  const lone = newLim(); lone.ships = { mode: 'only', task_forces: [], ships: [7] }; lone.ships.mode = 'only'; out.single = limitBodies('hold', lone); out.singleLine = limitsLine('hold', lone);
  const empty = newLim(); empty.ships.mode = 'only'; out.emptyOnly = limitBodies('hold', empty);
  const area = newLim(); area.area = { q: 0, r: 0, dist: 1 }; out.area = limitBodies('keep_supplied', area); out.areaLine = limitsLine('keep_supplied', area); out.hold_area = limitBodies('hold', area);
  const land = newLim(); land.not_before = 11; land.wait_for = { task_force: 2 }; land.after = 5; land.knowledge = { knowledge: 'recent_count', within_days: 3 }; land.risk = 'press';
  out.land = limitBodies('take_place', land); out.landLine = limitsLine('take_place', land); out.landOnHold = limitBodies('hold', land);
  out.landBack = limitBodies('take_place', limFromBodies(out.land.map(b => Object.assign({ intent: 3 }, b))));
  out.riskLine = limitsLine('hold', limFromBodies([{ limit: 'risk', risk: 'press', intent: 1 }]));
  const st = limFromState({ forces: { ships: [], task_forces: [1], air_groups: [], land_units: [], source: 'base:2', none_of: ['air_groups'], in_place: true }, area: [[0, 0], [1, 0], [0, 1], [-1, 1], [-1, 0], [0, -1], [1, -1]], targets: { kinds: ['carrier'], waters_first: false }, protected: [{ handle: 'ship:7' }], risk: 'preserve', knowledge: { setting: 'count', within_days: null }, wait_for: { land_unit: 4 } });
  out.state = limitsLine('take_place', st); out.stateArea = st.area;
  return out;
}
"""


def the_limits_on_the_paper(bridge):
    """The LIMITS part of the order paper (decision 0222, "As built: step 4, the table"): the page holds it and sends the order with
    its limits as one preview and one approval; the model that says what each kind offers, what a draft sends and how it is said is
    cut from the page's text and run in node; and the engine takes what the model sends, as one order with its limits, and refuses
    one of them by its place in the batch."""
    import re
    print("-- the limits on the order paper: the page's model in node, and the engine taking what it sends", flush=True)
    text = page_text()
    sign = cut_function(text, "  function signStaff() {")
    check("var LIMITS_OFFERED" in text and "function limitBodies(" in text and "function limitsLine(" in text and "function limFromBodies(" in text and "function limFromState(" in text and 'id="p-limits"' in text and "args.limits" in text,
          "the page holds the limits model, the LIMITS part of the paper and the order's limits sent with its preview")
    check("set_limit" not in sign and "supplyForces" not in sign and "api('approve', { preview: no })" in sign and "limits_queued" in sign,
          "signing is one approval of the preview: it sends no limit and no risk itself, and checks the count of limits that went")
    check("operate_carriers: ['targets', 'risk']," in text and "patrol: ['submarines']" in text,
          "a carrier force's own order offers no air groups (the groups aboard go with their force) and a patrol only its boats")
    check("if (t && !f && (r.limits || []).length) t.reasons.push(" in text,
          "a reason the reading has no plain words for keeps its list of limits, so a rank it speaks of is still a link")
    check("supplyForces" not in text and "WE CHOSE IT FOR YOU" not in text and "lim.chosen" not in text and "limit.air_groups" not in text,
          "the page chooses no source of its own (decision 0228, rule 21): an order from a base's card is sent with no source unless the player sets one")
    check("top.supplied_from && top.supplied_from.chosen_by === 'staff'" in text and "', our choice'" in text,
          "the plan's paper says of the base the staff chose that it is our choice, with the CHANGE the line has (decision 0228, rule 19)")
    check("area: pickArea" in text and "picking === 'route' || picking === 'station' || picking === 'area'" in text and "MOVE ON THE CHART" in text,
          "the chart's touch moves the area's middle, and tearing the paper up ends it")
    check("BACK TO THE STAFF\u2019S CHOICE" in text and "'Strike ' + word + ' ' + m[2]" in text and "These waters first" in text, "each set line has its way back, and the strike order has its up and down")
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the limits model ----")]
    code = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ["cleanName", "num", "hexDist", "hexWords"]) + model + LIMITS_IN_NODE
    said = run_in_node(code, None)
    deny = [{"limit": "forces", "task_forces": [1], "ships": [], "none_of": ["air_groups"], "in_place": True},
            {"limit": "targets", "kinds": ["carrier", "cruiser"], "waters_first": True}]
    check(said["deny"] == deny, f"a Coral Sea deny with Task Force 17 only, from where it lies, no air groups, carriers then cruisers first and these waters first sends exactly those two limits ({said['deny']})")
    check(said["denyPlan"] == ["ships", "targets"], "each limit sent knows the line it belongs to")
    check(said["coverShips"] == [] and "ships" not in said["coverOffers"] and "air_groups" in said["coverOffers"], "a cover offers no ships control, and a ships limit set on it is not sent")
    check("risk" not in said["patrolOffers"] and said["patrolOffers"] == ["submarines"], "a patrol offers the boats and no risk")
    line = said["denyLine"]
    check("Task Force 17" in line and "from where they lie" in line and "Air groups: none" in line and "carriers, then cruisers" in line and not re.search(r"\b(task_force|ship|base|air_group|land_unit|submarine)[: ]?\d|\b\d{2,}\b", line.replace("17", "")) and not re.search(r"\b[a-z_]+:\d+", line),
          f"the deny's limits are said by name and in the table's words, with no handle or number of the engine's ({line})")
    check(said["back"] and said["backBodies"] == deny, "what a draft sends reads back, from the limits queued, as the same draft")
    check(said["none"] == [] and said["noneLine"] == "The staff’s choice in all.", "a draft with every line at the staff's choice sends no limit and says so")
    check(said["source"] == [{"limit": "forces", "source": 2}] and said["sourceLine"] == "Supplied from: Lunga", "a source alone is a forces limit naming it, said by the base's name")
    check(said["single"] == [{"limit": "forces", "task_forces": [], "ships": [7]}] and "USS Hornet" in said["singleLine"], "a single ship is named by its name and sent in ships")
    check(said["emptyOnly"] == [], "\"use only these\" with nothing ticked sends nothing, not a limit that uses nothing")
    check(len(said["area"]) == 1 and said["area"][0]["limit"] == "area" and sorted(map(tuple, said["area"][0]["hexes"])) == sorted([(0, 0), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)])
          and said["areaLine"] == "Keep within: within 40 miles of Lunga" and said["hold_area"] == [], "an area is every hex of the map within its distance of its middle, and a hold is not offered one")
    check([b["limit"] for b in said["land"]] == ["not_before", "wait_for", "after", "knowledge"] and said["land"][0]["from_day"] == 11 and said["land"][1] == {"limit": "wait_for", "task_force": 2}
          and said["land"][2] == {"limit": "after", "other_intent": 5} and said["land"][3] == {"limit": "knowledge", "knowledge": "recent_count", "within_days": 3} and said["landOnHold"] == [],
          "a landing's four are sent as their own limits and only on a take or an island")
    check(said["landLine"] == "Not before: 12 May · Wait for: Task Force 11 · After: Capture Lunga · Knowledge: a count no older than 3 days · Risk: press", f"the landing's four are said as a date, names and a count's age, with the risk ({said['landLine']})")
    check(said["landBack"] == said["land"] and said["riskLine"] == "Risk: press", "the landing's four read back from the queue; a risk read back is said")
    check("Task Force 17" in said["state"] and "Air groups: none" in said["state"] and "Supplied from: Lunga" in said["state"] and "Keep within: within 40 miles of Lunga" in said["state"] and "Wait for: the 1st Marines" in said["state"]
          and "Knowledge: a count" in said["state"] and "Risk: preserve" in said["state"] and said["stateArea"] == {"q": 0, "r": 0, "dist": 1},
          f"the limits an order in force shows are said as a draft's are, an area as its middle and its reach ({said['state']})")
    # The engine takes what the model sends, as one order with its limits.
    check(begin(bridge, "coral-sea", "person", "engine") == {"ok": True}, "begun: the Coral Sea, the person Allied")
    person = Person(bridge.table)
    order = {"command": "set_intent", "intent_kind": "deny", "hex": [-18, -1], "distance": 4}
    shown = person.call("preview", dict(order, limits=deny))["ok"]
    check(shown.get("status") != "refused" and len(shown.get("shown_limits", [])) == 2, f"the preview of that deny with those limits is not refused and shows its two limits ({shown.get('status')})")
    hold = person.call("preview", {"command": "set_intent", "intent_kind": "hold", "base": 1, "limits": [{"limit": "targets", "kinds": ["carrier"]}]})["ok"]
    check(hold.get("status") != "refused" and len(hold.get("shown_limits", [])) == 1, "a targets limit without waters_first is taken, as a hold's is sent")
    signed = person.call("approve", {"preview": shown["preview"]})["ok"]
    check(signed.get("limits_queued") == 2, f"approving it queues the order and its two limits ({signed.get('limits_queued')})")
    queued = person.call("queued")["ok"]
    intents = [e for e in queued["entries"] if e["command"]["kind"] == "set_intent"]
    check(len(intents) == 1 and len(intents[0].get("goes_with", [])) == 2, "the queue holds one set_intent with the two limits going with it")
    entries = {(e["place"]["queue"], e["place"]["number"]): e for e in queued["entries"]}
    goes = [entries[(w["queue"], w["number"])]["command"] for w in intents[0]["goes_with"]]
    check(all(c["kind"] == "set_limit" for c in goes), "what goes with the order is its limits")
    taken = person.call("take_back", {"queue": intents[0]["place"]["queue"], "number": intents[0]["place"]["number"], "as_of": queued["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], f"taking the order back takes its limits with it and leaves the queue empty")
    refused = person.call("preview", dict(order, limits=[{"limit": "forces", "task_forces": [1], "none_of": ["ships"]}]))["ok"]
    check(refused.get("status") == "refused" and (refused.get("of_batch") or {}).get("position") == 2, f"a limit the engine refuses is answered with its place in the batch ({refused.get('of_batch')})")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


LIMITS_FORCE_IN_NODE = r"""
function whereIs() { return ''; } function dateOf() { return ''; } function sayIntent() { return 'Hold Port Moresby'; }
const hexes = [];
function main() {
  M.names['task_force:1'] = 'Task Force 17'; M.names['base:1'] = 'Port Moresby'; M.state = { intents: [] };
  const out = {};
  out.cleared = limitSignedPart('hold', { limit: 'forces', cleared: true, intent: 1 });
  out.forces = limitSignedPart('deny', { limit: 'forces', task_forces: [1], ships: [], none_of: ['air_groups'], in_place: true, intent: 1 });
  out.rank = [limitSignedPart('hold', { limit: 'priority', rank: 1 }), limitSignedPart('hold', { limit: 'priority', rank: 3 }), limitSignedPart('hold', { limit: 'priority', rank: null })];
  out.targets = limitSignedPart('deny', { limit: 'targets', kinds: [] });
  out.risk = limitSignedPart('hold', { limit: 'risk', risk: 'press' });
  out.area = limitSignedPart('keep_supplied', { limit: 'area', cleared: true });
  const none = newLim();
  out.bodies = ['forces', 'area', 'protected', 'targets', 'risk'].map(g => limGroupBody('keep_supplied', none, g));
  const set = newLim(); set.ships = { mode: 'only', task_forces: [1], ships: [] }; set.in_place = true; set.source = 1;
  out.partsBefore = [limPart('deny', set, 'ships'), limPart('hold', set, 'source'), limPart('deny', set, 'air_groups')];
  limReset(set, 'ships'); out.afterReset = limGroupBody('hold', set, 'forces'); limReset(set, 'source'); out.afterBoth = limGroupBody('hold', set, 'forces');
  out.groups = [limGroup('source'), limGroup('in_place'), limGroup('area'), limGroup('risk')];
  out.offered = { deny: PLAN_LINES.filter(k => k === 'risk' ? offers('deny', 'risk') : lineOffered('deny', k)), take: PLAN_LINES.filter(k => lineOffered('take_place', k)), patrol: PLAN_LINES.filter(k => lineOffered('patrol', k)) };
  return out;
}
"""


def limits_in_force(bridge):
    """The limits of an order in force, on its plan's paper (decision 0222, "As built: step 4, the table"): the page holds the
    LIMITS part of the plan's paper, its own paper for a limit changed, the way to open the plan at a limit, a base's reserve and
    SIGNED's words; the pure pieces (what a signed limit says, the body a line sends) are run in node; and the engine takes what
    the page sends, a limit changed or cleared on an order in force, one by itself, and a base's reserve."""
    import re
    print("-- the limits of an order in force: the plan's paper, the paper for a change, and the engine taking it", flush=True)
    text = page_text()
    plan = text[text.index("// ---- the plan's paper as one tree ----"):text.index("// ---- the dry run (decisions")]
    check("function planLimits(top)" in plan and "function limitJump(kind)" in plan and "function readPlan(root, focus, limitFocus)" in plan and "limitFocus.kind" in plan and "'LIMITS'" in plan
          and "doc.appendChild(planLimits(top))" in plan and "'Order ' + top.number + ' · given day ' + top.set_day" in plan and "limited by you" not in plan and "of the staff\\'s choosing" not in plan,
          "the plan's paper has a LIMITS part after its head line, which no longer says the risk or whether the forces are limited, and opens at a limit (readPlan's third argument)")
    check("'SET IT'" not in plan and "TRY IT FIRST" in plan and "'Not before day'" in plan and "'Pursuit'" in plan,
          "the plan's RISK AND TIMING keeps TRY IT FIRST, the not-before what-if and the pursuit, and has no SET IT: the risk is changed in LIMITS")
    check("openPaper('limit'" in text and "function openLimitPaper(root, key, back)" in text and "function askLimitPreview()" in text and "paper.kind === 'limit'" in text and "'Change the limits of '" in text and "limReset(l, key)" in text,
          "a limit is changed on the one order paper, as its own kind: the order named by its words and linked, the line, the staff's reading, a signature")
    check("data-key" in plan or "row.dataset.key = key" in plan, "each line of LIMITS is a row with its key, so that a link can scroll to it and mark it")
    check("'Set by us: '" in plan and "'From your order: '" in text and "limitJump(x.group" in text and "limitNotes(t, i, byNum)" in plan,
          "a task says what the staff set on it, and what it reads from the order, linked to that order's LIMITS line")
    check("function reserveLine(body, b)" in text and "Kept in reserve: " in text and "limit: 'reserve', base: n, supply_t: String(sv), fuel_t: String(fv)" in text and "reserveLine(body, b);" in text
          and "'Change what is kept in reserve at '" in text and "'Sign the reserve at '" in text, "an own base's card says what it keeps in reserve, with CHANGE and two whole-number fields in tons and SIGN, labelled")
    signed = cut_function(text, "  function signedLine(e) {")
    check("orderCap(c.intent)" in signed and "limitSignedPart(intentKindOf(c.intent), c)" in signed and "'Order ' + c.intent" not in signed and "kept in reserve" in signed and "pursuit " in signed and "': withdrawn'" in signed,
          "SIGNED says a limit, a pursuit and a withdrawal by the order's words, never by its number, and a reserve by the base's name")
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the limits model ----")]
    code = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ["cleanName", "num", "hexDist", "hexWords"]) + model + LIMITS_FORCE_IN_NODE
    said = run_in_node(code, None)
    check(said["cleared"] == "Forces: back to the staff’s choice" and said["area"] == "Keep within: back to the staff’s choice" and said["targets"] == "Strike first: back to the staff’s choice",
          f"a limit cleared is said as back to the staff's choice ({said['cleared']}; {said['area']}; {said['targets']})")
    check(said["forces"] == "Ships and forces: Task Force 17, from where they lie · Air groups: none" and said["risk"] == "Risk: press" and said["rank"] == ["ranked first", "ranked third", "not ranked"],
          f"a limit set is said in the table's words, a rank as first, second and so on ({said['forces']}; {said['rank']})")
    check(said["bodies"] == [{"limit": "forces", "cleared": True}, {"limit": "area", "cleared": True}, {"limit": "protected", "ships": []}, {"limit": "targets"}, {"limit": "risk", "risk": "balanced"}],
          f"a limit wholly at the staff's choice sends the body that clears it ({said['bodies']})")
    check(said["partsBefore"] == ["Ships and forces: Task Force 17, from where they lie", "Supplied from: Port Moresby", None] and said["afterReset"] == {"limit": "forces", "source": 1} and said["afterBoth"] == {"limit": "forces", "cleared": True},
          f"one forces line put back leaves the others, and all of them put back clears the limit ({said['afterReset']}; {said['afterBoth']})")
    check(said["groups"] == ["forces", "forces", "area", "risk"], "the forces lines are one limit, and every other line is its own")
    check(said["offered"]["deny"] == ["ships", "air_groups", "targets", "risk"] and "area" in said["offered"]["take"] and "knowledge" in said["offered"]["take"] and said["offered"]["patrol"] == ["submarines"],
          f"each kind of order shows the lines it offers ({said['offered']})")
    # The engine takes what the page sends.
    check(begin(bridge, "coral-sea", "person", "engine") == {"ok": True}, "begun: the Coral Sea, the person Allied")
    person = Person(bridge.table)
    deny = [{"limit": "forces", "task_forces": [1], "ships": [], "none_of": ["air_groups"], "in_place": True}]
    order = {"command": "set_intent", "intent_kind": "deny", "hex": [-18, -1], "distance": 4, "limits": deny}
    shown = person.call("preview", order)["ok"]
    check(person.call("approve", {"preview": shown["preview"]})["ok"].get("limits_queued") == 1, "a deny with Task Force 17 in place is signed")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends and the order is in force")
    state = person.call("command_state")["ok"]
    mine = [i for i in state["intents"] if i["number"] == 1][0]
    check(mine["limits"]["forces"]["task_forces"] == [1] and mine["limits"]["forces"]["in_place"] is True and mine["limits_read_up"]["forces_from"] == 1,
          "the order in force shows its forces limit, and that it is read from itself")
    clear = person.call("preview", {"command": "set_limit", "limit": "forces", "intent": 1, "cleared": True})["ok"]
    check(clear.get("status") != "refused" and "tree" in (clear.get("with_change") or {}), f"clearing the forces limit of an order in force is read by the staff and not refused ({clear.get('status')})")
    given = person.call("approve", {"preview": clear["preview"]})["ok"]
    check(given.get("status") == "queued", f"and is signed ({given.get('status')})")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends")
    after = [i for i in person.call("command_state")["ok"]["intents"] if i["number"] == 1][0]
    check(after["limits"]["forces"] is None, f"the order's forces limit is back to the staff's choice ({after['limits']['forces']})")
    targets = person.call("preview", {"command": "set_limit", "limit": "targets", "intent": 1, "kinds": ["carrier"], "waters_first": True})["ok"]
    check(targets.get("status") != "refused", f"strike first, with these waters first, is read on an order in force ({targets.get('status')})")
    queued = person.call("set_limit", {"limit": "reserve", "base": 1, "supply_t": "100", "fuel_t": "0"})["ok"]
    check(queued.get("status") == "queued", f"a base's reserve is signed ({queued.get('status')})")
    held = person.call("queued")["ok"]
    mine = [e for e in held["entries"] if e["command"]["kind"] == "set_limit" and e["command"]["limit"] == "reserve"]
    check(len(mine) == 1, "and is under SIGNED")
    taken = person.call("take_back", {"queue": mine[0]["place"]["queue"], "number": mine[0]["place"]["number"], "as_of": held["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], "and taken back")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


RANKS_IN_NODE = r"""
function main() {
  const out = {};
  // Case 1: three ranked, the third moved to the top.
  out.third_up = rankSends([1, 2, 3], [3, 1, 2], { 1: 1, 2: 2, 3: 3 });
  // Case 2: two ranked and one not; the first two swap, the unranked stays below.
  out.swap = rankSends([1, 2, 3], [2, 1, 3], { 1: 1, 2: 2, 3: null });
  // Case 3: an unranked one moved to the top of those ranked.
  out.unranked_up = rankSends([1, 2, 3], [3, 1, 2], { 1: 1, 2: null, 3: null });
  // Case 4: nothing moved.
  out.none = rankSends([1, 2], [1, 2], { 1: 1, 2: 2 });
  // Case 5: moved and moved back.
  out.back = rankSends([4, 5, 6], [4, 5, 6], { 4: null, 5: null, 6: null });
  return out;
}
"""


def the_orders_paper(bridge):
    """THE ORDERS (decision 0222, step 4, the designer's answer 3), by the page's text, by the pure ordering run in node, and through
    the bridge: the paper opens from YOUR ORDERS and lists the orders in rank; each rank is its own `priority` limit under one
    SIGN AND SEND (a preview takes no batch of them, so no reading of the whole is shown); SIGNED groups the ranks as one entry with
    one TAKE BACK; and the engine takes the ranks, takes them back and puts them in force."""
    print("-- THE ORDERS: the paper, the ranks sent one by one, and the engine taking them", flush=True)
    text = page_text()
    orders = cut_function(text, "  function readOrders(focus) {")
    draw = cut_function(text, "  function drawOrders(focus) {")
    signed = cut_function(text, "  function showSigned() {")
    check("readOrders = tracked('orders', readOrders)" in text and "sheetAs('THE ORDERS')" in orders and "api('staff_orders')" in orders and "'THE ORDERS'" in cut_function(text, "  function showOrders() {")
          and "readOrders();" in cut_function(text, "  function showOrders() {"),
          "YOUR ORDERS has a THE ORDERS button, which opens the paper in the sheet; the staff's orders are read once when it opens")
    check("'Waits for your leave'" in text and "'Launched'" in text and "'Cannot be met'" in text and "'Preparing'" in text and "'In force'" in text and "'Signed · ' + actsWords(e)" in draw
          and "limitsLine(i.kind, limFromState(i.limits))" in draw and "limFromBodies(lims)" in draw and "readPlan(n)" in draw and "openOrder(c.intent)" in draw,
          "each line has its state, its limits in a line and a link to its plan, or for one signed its entry under SIGNED")
    check("'Rank ' + lowerFirst(words) + ' ' + d[2]" in draw and "'higher'" in draw and "'lower'" in draw and "e.altKey" in draw and "'ArrowUp'" in draw and "'ArrowDown'" in draw
          and "focus.at" in draw and "'WITHDRAW'" in draw and "act('withdraw_intent', { intent: n }" in draw and "'Ranks changed'" in draw and "'SIGN AND SEND'" in draw and "'PUT BACK'" in draw,
          "an order in force is ranked with two labelled buttons or with Alt and the arrow keys, the focus staying where it was, and withdrawn; a change shows SIGN AND SEND and PUT BACK")
    check("'Not ranked'" in draw and "pend.length" in draw and "ro.before.slice().sort" in draw and "takeBackEntries(pend.slice().reverse())" in draw and "'Signed · ' + actsWords(pend[pend.length - 1])" in draw,
          "ranks signed and not in force show the list in the signed order with Signed at its head and one TAKE BACK, without arrows")
    send = cut_function(text, "  function sendRanks(go, back) {")
    check("api('set_limit', b)" in send and "rankSends(ro.before, ro.now, ro.rankOf)" in send and "'NOT SENT'" in draw and "preview" not in send and "propose" not in send and "staff" not in send.lower(),
          "SIGN AND SEND sends each rank as its own set_limit, says NOT SENT for one refused, and asks the staff for no reading of the whole")
    check("isRankEntry(e)" in signed and "rankLine(ranksOf[" in signed and "takeBackEntries(group)" in signed and "'Ranks: '" in text and "ORDINAL[rank - 1]" in text,
          "SIGNED groups the priority limits queued together as one line, \"Ranks: first: ...; second: ...\", with one TAKE BACK that takes each back")
    check("limit: 'priority'" in text and "draft.priority" not in text, "THE ORDERS sends limit: 'priority'")
    # the pure ordering, in node
    code = cut_function(text, "  function rankSends(before, after, rankOf) {") + "\n" + RANKS_IN_NODE
    got = run_in_node(code, None)
    check(got["third_up"] == [{"limit": "priority", "intent": 3, "rank": 1}, {"limit": "priority", "intent": 1, "rank": 2}, {"limit": "priority", "intent": 2, "rank": 3}],
          f"the third of three ranked orders put first sends all three ranks ({got['third_up']})")
    check(got["swap"] == [{"limit": "priority", "intent": 2, "rank": 1}, {"limit": "priority", "intent": 1, "rank": 2}] and got["none"] == [] and got["back"] == [],
          f"two swapped sends two ranks and the unranked one below is left alone; nothing moved sends nothing ({got['swap']})")
    check(got["unranked_up"] == [{"limit": "priority", "intent": 3, "rank": 1}, {"limit": "priority", "intent": 1, "rank": 2}, {"limit": "priority", "intent": 2, "rank": 3}],
          f"an unranked order moved to the top is ranked first, the ranked one it passed moves to second, and the unranked one it passed is ranked third ({got['unranked_up']})")
    # through the bridge, on the 39 days
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    person = Person(bridge.table)
    for order in ({"command": "set_intent", "intent_kind": "control_island", "island": 1},
                  {"command": "set_intent", "intent_kind": "deny", "hex": [2, -4], "distance": 3}):
        shown = person.call("preview", order)["ok"]
        check(shown.get("status") != "refused", f"{order['intent_kind']}: the order is read ({shown.get('status')})")
        check(person.call("approve", {"preview": shown["preview"]})["ok"].get("status") == "queued", f"{order['intent_kind']}: and signed")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends and both orders are in force")
    mine = [i for i in person.call("command_state")["ok"]["intents"] if i["in_force"] and i.get("parent") is None and i.get("maker") == "commander"]
    first, second = sorted(i["number"] for i in mine)[:2]
    check(len(mine) >= 2 and all(i["limits"]["priority"] is None for i in mine), f"two orders of the commander's are in force, neither ranked ({[i['number'] for i in mine]})")
    for body in ({"limit": "priority", "intent": second, "rank": 1}, {"limit": "priority", "intent": first, "rank": 2}):
        sent = person.call("set_limit", body)["ok"]
        check(sent.get("status") == "queued", f"the rank {body['rank']} of order {body['intent']} is signed ({sent.get('status')})")
    held = person.call("queued")["ok"]
    ranks = [e for e in held["entries"] if e["command"].get("limit") == "priority"]
    check(len(ranks) == 2, "the queue holds both ranks, each its own entry")
    for entry in reversed(ranks):
        held = person.call("queued")["ok"]
        taken = person.call("take_back", {"queue": entry["place"]["queue"], "number": entry["place"]["number"], "as_of": held["as_of"]})["ok"]
        check(taken.get("status") != "refused", "a rank is taken back, the later first, then the earlier")
    check(person.call("queued")["ok"]["entries"] == [], "the queue is empty after both are taken back")
    for body in ({"limit": "priority", "intent": second, "rank": 1}, {"limit": "priority", "intent": first, "rank": 2}):
        check(person.call("set_limit", body)["ok"].get("status") == "queued", f"the rank {body['rank']} is signed again")
    check(person.call("end_day")["ok"].get("resolved"), "the day ends and the ranks are in force")
    state = person.call("command_state")["ok"]["intents"]
    rank = {i["number"]: i["limits"]["priority"] for i in state if i["number"] in (first, second)}
    check(rank == {second: 1, first: 2}, f"the command state shows the ranks on the right orders ({rank})")
    # what the staff said of it: any reason that lists a priority among its limits, over the days that follow (none is required)
    found = total = 0
    for _ in range(5):
        check(person.call("end_day")["ok"].get("resolved"), "a day ends")
        so = person.call("staff_orders")["ok"]
        reasons = [o["reason"] for o in (so.get("orders") or []) + (so.get("commands") or []) if o.get("reason")] + list(so.get("no_orders") or [])
        total += len(reasons)
        found += sum(1 for r in reasons if any((l or {}).get("kind") == "priority" for l in (r.get("limits") or [])))
    print(f"   found: {found} of {total} reasons over five days list a priority limit", flush=True)
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def reason_limits_as_links(bridge):
    """A reason's limits are links (decision 0222, rules 11, 12 and 17): by the page's text, by the words of each kind run in node, and
    through the bridge, where the engine's real reasons are asked for and put through the page's own `proposedTree`. What is chosen: the
    engine's `limits` list on a reason is what the page reads (nothing in the page reads limit words out of a reason's text); the link
    in a browser is tried by hand."""
    import re
    print("-- a reason's limits are links: the words, where a link goes, and the engine's reasons through proposedTree", flush=True)
    text = page_text()
    words = text[text.index("  var LIMIT_LINK ="):text.index("  function limitLinkWords(")] + cut_function(text, "  function limitLinkWords(") + "\n" + cut_function(text, "  function limitInState(")
    blocks = [cut_function(text, h) for h in ("  function limitLinks(", "  function openLimit(", "  function limitSetOn(")]
    check("function limitLinkWords(" in text and "function openLimit(" in text and "'YOUR LIMITS'" in text and "block('YOUR LIMITS', limitLinks(t.reasons, PAPER_ROOT))" in cut_function(text, "  function whyView("),
          "the page has limitLinkWords, openLimit and a YOUR LIMITS block in WHY")
    check("'YOUR LIMITS'" in cut_function(text, "  function reasonView(") and "limitLinks([r], null)" in cut_function(text, "  function reasonView("),
          "a reason laid out for an interrupt or the war diary has the same YOUR LIMITS block")
    check(not any(re.search(r"\.exec\(|\.test\(|\.match\(|\.replace\(", b) for b in blocks) and all("worked" not in b and "binding" not in b and "would_change" not in b for b in blocks),
          "the code that makes the links reads the engine's `limits` list: no pattern is run on a reason's words")
    check("readPlan(found.root, found.task !== found.root ? found.task : null, { kind: l.kind })" in blocks[1] and "readOrders({ n: l.intent" in blocks[1] and "see(" in blocks[1]
          and "openOrder(l.intent)" in blocks[1] and "Tear up this order to read about your other order?" in blocks[1] and "isDraftNo(l.intent)" in blocks[1],
          "a link goes to the draft's LIMITS part, THE ORDERS, the base's card, the plan of an order in force at that limit, or the order's entry under SIGNED, and asks before tearing up a draft")
    code = "function nm(h) { return M.names[h] || h; }\n" + words + r"""
function main() {
  const kinds = ['forces', 'source', 'area', 'targets', 'risk', 'priority', 'protected', 'not_before', 'wait_for', 'after', 'knowledge'], out = {};
  kinds.forEach(k => { out[k] = [limitLinkWords(k, true), limitLinkWords(k, false)]; });
  out.reserve = [limitLinkWords('reserve', true, 1), limitLinkWords('reserve', true, 'base:1')];
  out.state = [limitInState('forces', { forces: { none_of: ['ships'] } }), limitInState('forces', { forces: {} }), limitInState('forces', null), limitInState('source', { forces: { source: 3 } }),
               limitInState('area', { area: [] }), limitInState('priority', { priority: 0 }), limitInState('protected', { protected: [] }), limitInState('targets', { targets: { kinds: ['carrier'] } })];
  return out;
}
"""
    got = run_in_node(code, None)
    want = {"forces": ["the forces you named", "the forces it may use"], "source": ["the base it is supplied from", "a base to supply it from"],
            "area": ["the waters you kept it within", "waters to keep it within"], "targets": ["what it strikes first", "what it strikes first"],
            "risk": ["the risk you set", "its risk"], "priority": ["its rank", "its rank"], "protected": ["the ships you protected", "ships to protect"],
            "not_before": ["the day it waits for", "a day to wait for"], "wait_for": ["what it waits for", "a force or unit to wait for"],
            "after": ["the order it waits on", "an order to wait on"], "knowledge": ["what must be known first", "what must be known first"],
            "reserve": ["the reserve kept at Lunga", "the reserve kept at Lunga"]}
    check(all(got[k] == v for k, v in want.items()), "each kind of limit is said in the table's words, set and not set, and a reserve by the base's name")
    check(not any(re.search(r"limit|intent|\d", w, re.I) for v in want.values() for w in v), "and none of those words says \"limit\", \"intent\" or a number")
    check(got["state"] == [True, False, False, True, False, True, False, True], f"whether a limit is set is read from the command state's own shape ({got['state']})")
    # through the bridge: the engine's reasons, with what each carries
    heads = ["commas", "cleanName", "orderWords", "plainCounts", "plainWords", "nameText", "intentWords", "reasonWords", "tidyWorked", "paperPre", "shareWords", "tiredBit",
             "paperText", "plainFact", "cautionPart", "cautionWords", "cautionTexts", "cutTop", "sayIntent", "newTask", "taskHex", "labelPlain", "arcsFrom", "finishTask",
             "threatsOf", "proposedTree", "draftTop", "readingTop"]
    code = "const config = { islands: [{ number: 1, name: 'New Guinea' }] }; let hexRaw = false, PAPER_TASKS = null; M.forces = [{ task_force: { handle: 'task_force:1', name: 'Task Force 17' } }]; M.bases = []; M.enemyBases = []; M.names['task_force:1'] = 'Task Force 17'; M.names['base:1'] = 'Port Moresby';\n"
    code += "function dateOf() { return ''; } function num(h) { return parseInt(String(h).split(':')[1], 10); }\n" + re.search(r"  var WORD = .*\n", text).group(0)
    code += "".join(cut_function(text, f"  function {h}(") + "\n" for h in heads)
    code += r"""
function main(input) {
  const w = input.with_change, pr = { tree: w.tree, would_hand_in: w.would_hand_in, stations: null, reason: w.reason }, e = [input.shown, input.end];
  const top = draftTop(e, input), root = proposedTree(top, {}, pr), all = [root], kept = [];
  for (let i = 0; i < all.length; i++) { all[i].kids.forEach(k => all.push(k)); }
  all.forEach(t => t.reasons.forEach(r => (r.limits || []).forEach(l => kept.push(l))));
  return { kept: kept, tasks: all.length, number: top.number };
}
"""

    def reasons_of(answer):
        w = answer["with_change"]
        found = [("the reason", w.get("reason"))]
        found += [("a reason of no order", r) for r in (w.get("would_hand_in") or {}).get("no_orders") or []]
        found += [("an order's reason", o.get("reason")) for o in (w.get("would_hand_in") or {}).get("orders") or []]
        found += [("a task's reason", s.get("reason")) for s in (w.get("tree") or {}).get("sub_intents") or []]
        return [(k, r) for k, r in found if isinstance(r, dict)]

    check(begin(bridge, "coral-sea", "person", "engine") == {"ok": True}, "begun: the Coral Sea, the person Allied")
    person = Person(bridge.table)
    end = {"intent_kind": "deny", "hex": [-18, -1], "distance": 4}
    attempts = [[{"limit": "forces", "none_of": ["ships", "air_groups"]}], [{"limit": "forces", "none_of": ["air_groups"]}]]
    answer, mine = None, []
    for limits in attempts:
        answer = person.call("preview", dict(command="set_intent", limits=limits, **end))["ok"]
        check(answer.get("status") != "refused" and answer.get("with_change"), f"the deny is read with {limits[0]['none_of']} none")
        mine = [l for _, r in reasons_of(answer) for l in (r.get("limits") or []) if l.get("kind") == "forces" and l.get("intent") == answer["intent"]]
        if mine:
            break
    every = reasons_of(answer)
    carrying = [(k, [l.get("kind") for l in r.get("limits") or []]) for k, r in every if r.get("limits")]
    kinds = sorted({l.get("kind") for _, r in every for l in r.get("limits") or []})
    print(f"   found: {len(carrying)} of {len(every)} reasons carry limits; kinds {kinds}; the draft is order {answer['intent']}", flush=True)
    check(mine, "the engine's reasons name the forces limit on the draft's own number: {kind: forces, intent: <the draft>}")
    check(all(set(l) <= {"kind", "intent", "base"} for _, r in every for l in r.get("limits") or []), "every limit a reason carries is {kind, intent} or {kind: reserve, base}")
    seen = run_in_node(code, dict(answer, end=end))
    check(seen["number"] == answer["intent"] and {"kind": "forces", "intent": answer["intent"]} in seen["kept"],
          f"proposedTree keeps the forces limit in a task's reasons ({len(seen['kept'])} limits over {seen['tasks']} tasks)")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    # the hold of Lunga: its reason, where it names no source, says which limit would give it one
    check(begin(bridge, "eastern-solomons-five-days", "person", "engine") == {"ok": True}, "begun: the Eastern Solomons in five days, the person Allied")
    person = Person(bridge.table)
    lunga = [b["base"]["handle"] for b in person.call("list_bases")["ok"]["bases"] if b["base"]["name"] == "Lunga"]
    check(lunga, "Lunga is among the side's bases")
    held = person.call("preview", {"command": "set_intent", "intent_kind": "hold", "base": int(lunga[0].split(":")[1])})["ok"]
    check(held.get("status") != "refused" and held.get("with_change"), "the hold of Lunga is read")
    every = reasons_of(held)
    sources = [(k, l) for k, r in every for l in r.get("limits") or [] if l.get("kind") == "source"]
    print(f"   found: the hold of Lunga: {sum(1 for _, r in every if r.get('limits'))} of {len(every)} reasons carry limits; "
          f"kinds {sorted({l.get('kind') for _, r in every for l in r.get('limits') or []})}; source on {[k for k, _ in sources]}", flush=True)
    texts = [r.get("worked", "") for _, r in every]
    print("   found: " + " | ".join(t[:160] for t in texts if "source" in t or "supply" in t), flush=True)
    check(sources and all(l.get("intent") == held["intent"] for _, l in sources) or any("chose the source of supply" in t or "lives on what it has" in t for t in texts),
          "the hold with no source named: the table's game has the staff choose its sources (decision 0228), so its reasons say which base it chose, or that the place lives on what it has")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def midway_on_its_chart(bridge):
    """What the Midway game gives the page that the old map's games did not, and
    that the page words as it should (the mends of "Table: Midway on its own chart").
    The words themselves are the page's, seen in a browser; here the game is checked to
    give each case, and the page to hold the wording that answers it."""
    print("-- Midway: the cases its cards and papers meet", flush=True)
    page = get(bridge.table + "/")
    check(begin(bridge, "midway", "person", "engine") == {"ok": True}, "Midway begun, the person Allied")
    frame = get(bridge.table + "/frame")
    check(frame and frame["west"] == 163 and frame["east"] == 194, "the bridge serves the Midway map's frame, across the date line (163 to 194)")
    names = {n["name"]: n for n in get(bridge.table + "/names")}
    check(-166.211 + 360 > frame["east"] - 0.5, "French Frigate Shoals lies within half a degree of the chart's eastern edge")
    check("edge = 80" in page and "c = [Math.max(edge + half, Math.min(W - edge - half, c[0])), c[1]]" in page and "French Frigate Shoals" in names,
          "and the page sets a name near the edge inside the chart, not cut by its border")
    person = Person(bridge.table)
    bases = person.call("list_bases")["ok"]["bases"]
    air = [g for g in person.call("get_briefing", {"text": False})["ok"]["air_groups"] if (g.get("location") or {}).get("handle") == "base:1"]
    check(bases[0]["base"]["name"] == "Midway" and bases[0]["airfield_size"] == 0 and len(air) == 7,
          "Midway has no airfield size on the map and seven air groups flying from it")
    check("M.air.some(function (x) { return x.location && x.location.handle === h; }) ? null : 'none'" in page,
          "and the page's base card says no 'Airfield: none' for a base with aircraft flying from it")
    check(begin(bridge, "midway", "engine", "person") == {"ok": True}, "Midway begun, the person Japanese")
    person = Person(bridge.table)
    shown = json.dumps(person.call("preview", {"command": "set_intent", "intent_kind": "take_place", "base": 1}))
    check("land_unit:2 (2nd Combined Special Naval Landing Force)" in shown and "8.8 kn" in shown,
          "the staff's words for the take name a unit whose name begins with a figure, and give a speed with a decimal")
    check("([A-Z0-9][\\w .'’-]*?) \\(\\1\\)" in page and "(\\d+\\.\\d+)(?!\\w|\\.\\d)" in page,
          "and the page says such a name once and shows every figure whole")
    carriers = person.call("get_entity", {"handle": "task_force:1"})["ok"]["entity"]["ships"]
    check(any(" (" in str(s.get("class")) for s in carriers), "a Japanese carrier's class carries a note in brackets")
    check(".replace(/ \\([^()]*\\)/g, '')" in page, "and the page names the class without it")
    # The boats of each game (decisions 0225 and 0227): each seat has its own boats and none of the other's, and a boat takes a
    # station order.
    for gid, allied, japanese in BOATS_ON_DAY_1:
        for seat, want in (("allied", allied), ("japanese", japanese)):
            check(begin(bridge, gid, "person" if seat == "allied" else "engine", "engine" if seat == "allied" else "person") == {"ok": True},
                  f"{gid} begun, the person {seat.capitalize()}, for the boats")
            boats = Person(bridge.table).call("list_forces")["ok"].get("submarines", [])
            check([(b["submarine"].get("name"), b["state"]) for b in boats] == want,
                  f"{gid}: the {seat.capitalize()} seat has its own {len(want)} boats, those that have entered on station, and nothing of the other side's ({[b['submarine'] for b in boats]})")
    check(begin(bridge, "midway", "person", "engine") == {"ok": True}, "Midway begun again, the person Allied, for a station order")
    order = Person(bridge.table).call("issue_orders", {"orders": [{"unit": "submarine:3", "order": {"kind": "station", "hex": [5, -3]}}]})["ok"]
    check(order["results"][0]["status"] == "queued", "a direct order gives a boat of Midway's a station")
    check("base_attacked_against: 'bombing of a base'" in page and "(n === 1 ? 'report' : 'reports')" in page,
          "the plot folds the rest of a kind of report under a plain name, not the event's own")


def person_against_agent(bridge, hold):
    print("-- a person against an agent: the 39 days, the person Allied", flush=True)
    check(begin(bridge, "guadalcanal-39-days", "person", "agent") == {"ok": True}, "begun")
    person = Person(bridge.table)
    door = get(bridge.table + "/agent?side=japanese")["ok"]["door"]
    agent = Agent(door)
    config = get(bridge.table + "/config")
    check(config["versus"] == {"other": "japanese", "inside_day": False} and "watch" not in config,
          "the page is told it is a game against an agent, and of no watcher")
    check(config["directives"] and config["directives"][0]["from"].startswith("Commander in Chief, U.S."),
          "the page is given its own side's directives")

    # Not yet at its seat, until the door's first call.
    standing = get(bridge.table + "/standing")["ok"]
    other = [s for s in standing["seats"] if s["seat"] == "japanese"][0]
    check(other == {"seat": "japanese", "at_seat": False, "waited_for": True},
          "the other side is \"not yet at its seat\", and nothing more is said of it")
    words = get(door + "/instructions")["ok"]["instructions"]
    check("no one else at the clock" in words and "any call, not only a call of time, may be held" in words
          and "watcher" not in words, "the door's opening words: both commanders, any call may be held, no watcher")
    tools = [t["name"] for t in get(door + "/tools")["ok"]["tools"]]
    check("wait" in tools and "run" in tools, f"the door lists its seat's tools and wait ({len(tools)})")
    other = [s for s in get(bridge.table + "/standing")["ok"]["seats"] if s["seat"] == "japanese"][0]
    check(other["at_seat"] is True, "after the door's first call the agent is at its seat")

    # The sides kept apart: every watcher request refused, nothing of the agent's side.
    mine = {f["task_force"]["name"] for f in person.call("list_forces")["ok"]["task_forces"]}
    theirs = {f["task_force"]["name"] for f in agent.call("list_forces")["ok"]["task_forces"]}
    check(mine and theirs and not mine & theirs, f"the person's forces and the agent's are apart ({len(mine)} and {len(theirs)})")
    check(http("GET", bridge.table + "/seat_calls?side=japanese")[0] == 403, "the watcher's seat_calls is refused")
    check(http("GET", bridge.table + "/seat_calls?side=allied")[0] == 403, "the watcher's seat_calls of his own side is refused too")
    check(http("POST", bridge.table + "/step", {})[0] == 403, "the watcher's step is refused")
    for side in SIDES:
        answer = person.raw("get_briefing", {"text": False}, side=side)
        check("refused" in answer and "no watcher" in answer["refused"], f"a watcher's read from the {side} side is refused")
    check(get(bridge.table + "/tools?side=japanese") == get(bridge.table + "/tools"), "/tools gives the seat's own list whatever is asked")
    check(person.call("get_briefing", {"text": False})["ok"]["heading"]["side"] == "allied", "the person's briefing is his own side's")
    text = json.dumps(get(bridge.table + "/config")) + json.dumps(get(bridge.table + "/standing"))
    check(not any(name in text for name in theirs), "neither the config nor the standing names anything of the agent's side")

    # A call of time held: from the moment it is sent, every call the page makes on load and on refresh
    # is answered (as his own side), before the door's wait is up and after.
    t = later(person.raw, "run", {"days": 1})
    check(wait_until(lambda: [s for s in get(bridge.table + "/standing")["ok"]["seats"] if s["seat"] == "allied"][0].get("held")),
          "the standing tells the page its call is held")
    for when in ("at once", "after the door's wait"):
        if when != "at once":
            t.join(hold + 30)
        for path in ("/config", "/map", "/standing", "/tools"):
            check(http("GET", bridge.table + path)[0] == 200, f"on load while held ({when}): {path} answered")
        for tool, arguments in PAGE_LOAD_CALLS:
            answer = person.raw(tool, arguments)
            if tool in NOT_WHILE_HELD:
                # Not a watcher's read: refused while held, and the page does without (SIGNED is empty then; nothing can be signed).
                check("refused" in answer, f"on load and refresh while held ({when}): {tool} is refused, and the page does without it")
            else:
                check("ok" in answer, f"on load and refresh while held ({when}): {tool} answered")
    check(t.answer.get("waiting") is True, f"the person's run, the agent not finished, answers that it waits after {min(hold, 20)} s")
    # A page loaded again picks the wait up, and is answered when the clock moves.
    reloaded = later(person.call, "wait")
    time.sleep(0.5)
    check(reloaded.is_alive() or reloaded.answer.get("ok") is None, "a reloaded page's wait is held")
    standing = get(bridge.table + "/standing")["ok"]
    check(standing["waiting_for"] == ["japanese"], "the standing says the other commander is waited for")
    while_held = {f["task_force"]["name"] for f in person.raw("list_forces")["ok"]["task_forces"]}
    check(while_held == mine, "while held, the person reads his own forces (the watcher's read of his side)")
    check("refused" in person.raw("preview", {"command": "set_leave", "leave": "launch", "given": True}),
          "while held, a preview is refused")
    check("refused" in person.raw("issue_orders", {"orders": []}), "while held, an order is refused")
    check(get(bridge.table + "/standing")["ok"]["clock"]["day"] == 1, "the clock has not moved")
    agent_end = agent.through("end_day", {"reason": "nothing yet"})
    check(agent_end["ok"]["resolved"] is True and agent_end["ok"]["day"] == 1, "the agent finishes; the bridge steps; its end_day answered")
    reloaded.join(60)
    answer = reloaded.answer["ok"]
    check(answer["kind"] == "run" and answer["days_resolved"]["count"] == 1,
          "the reloaded page's wait is answered when the clock moves: his run's answer, one day")
    check("refused" in person.raw("wait"), "once given, nothing waits")

    # A few days: the agent ready for three, the person a day at a time.
    a = later(agent.through, "run", {"days": 3})
    for day in (2, 3, 4):
        r = person.call("run", {"days": 1})["ok"]
        check(r["days_resolved"]["last"] == day, f"day {day} resolved at the person's word, the agent still ready")
    a.join(60)
    check(a.answer["ok"]["days_resolved"]["count"] == 3, "the agent's run of three days answered after the third")
    check(get(bridge.table + "/standing")["ok"]["clock"]["day"] == 5, "the clock stands at day 5")
    check("refused" in post(bridge.table + "/restart"), "begin again is refused while the agent has lately called")

    # The agent's call held when a new game begins.
    held = agent.call("end_day")
    check(held.get("waiting") is True, "the agent's end_day, the person not finished, answers that it waits")
    inflight = later(agent.call, "wait")
    time.sleep(0.5)
    check(begin(bridge, "guadalcanal-39-days", "person", "agent") == {"ok": True}, "a new game is begun while the agent's call is held")
    inflight.join(30)
    check("ended" in inflight.answer.get("refused", "") and "new one" in inflight.answer["refused"],
          "the held call is answered that the game has ended and a new one begun")
    other = [s for s in get(bridge.table + "/standing")["ok"]["seats"] if s["seat"] == "japanese"][0]
    check(other["at_seat"] is False, "in the new game the agent is not yet at its seat")
    check(get(door + "/instructions")["ok"]["instructions"], "the same door serves the new game")
    check(get(bridge.table + "/standing")["ok"]["clock"]["day"] == 1, "the new game stands at day 1")
    check(agent.call("list_forces")["ok"]["task_forces"], "the agent reads its forces in the new game")

    # A held call answered "waiting", then a new game: its wait is told.
    check(agent.call("end_day").get("waiting") is True, "the agent's end_day waits again")
    check(post(bridge.table + "/new") == {"ok": True}, "New game ends it (the agent has lately called: New game is not bound)")
    told = agent.call("wait")
    check("ended" in told.get("refused", ""), "the agent's next wait is told the game has ended")
    check("no game is in hand" in agent.call("list_forces")["refused"], "and with no game in hand the door refuses")


def seat_mcp(bridge):
    print("-- seat_mcp.py at a door of a game against an agent", flush=True)
    check(begin(bridge, "eastern-solomons", "agent", "engine") == {"ok": True}, "the Eastern Solomons, an agent Allied, watched")
    door = get(bridge.table + "/agent?side=allied")["ok"]["door"]
    standing = get(bridge.table + "/standing")["ok"]
    check([s.get("at_seat") for s in standing["seats"] if s["seat"] == "allied"] == [False],
          "the watcher is told the agent is not yet at its seat")
    mcp = subprocess.Popen([sys.executable, os.path.join(HERE, "seat_mcp.py"), "--url", door],
                           stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        def rpc(n, method, params):
            mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": n, "method": method, "params": params}) + "\n")
            mcp.stdin.flush()
            return json.loads(mcp.stdout.readline())
        init = rpc(1, "initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "check", "version": "0"}})
        check("any call, not only a call of time, may be held" in init["result"]["instructions"],
              "seat_mcp.py gives the door's words: any call may be held")
        names = [t["name"] for t in rpc(2, "tools/list", {})["result"]["tools"]]
        check("wait" in names, f"seat_mcp.py lists the door's tools ({len(names)}, wait among them)")
        brief = rpc(3, "tools/call", {"name": "get_briefing", "arguments": {}})["result"]
        check(brief["isError"] is False and "FLAG PLOT BRIEFING" in brief["content"][0]["text"], "seat_mcp.py gives the briefing as text")
    finally:
        mcp.stdin.close()
        mcp.wait(10)
    standing = get(bridge.table + "/standing")["ok"]
    check([s.get("at_seat") for s in standing["seats"] if s["seat"] == "allied"] == [True], "then it is at its seat")
    check(post(bridge.table + "/new") == {"ok": True}, "New game ends it")


# ---- decision 0232, step 3: the table of a plan the commander writes ----
#
# THE ARCHITECT'S STATEMENTS, written on Nov 27, 2026, before any code of step 3 (decision 0233, A, item 3), and amended the same day on
# the plan's review and the designer's answers ("Amended on review (Nov 27, 2026)" in the file named next). The plan they are of is in
# docs/decisions/as-built/0232-a-plan-the-commander-writes.md, "Planned: step 3"; "item N" below is an item of that section, and "rule N" a
# rule of decision 0232. Each function below is empty and fails "not built". A developer fills its body to the statements in its doc
# comment and changes no statement, no word in quotes and no expected value: one that cannot be filled as written goes back to the
# architect. `the_island_of_two_places` is called from main() from the island's pull request on; the five after it are called from main(),
# in the order they stand here and after it, from the first commit of the screen's pull request (item 2), which is begun once the
# preview of a withdrawal (item 10, agent interface 58) is merged: W8 needs it.
#
# The game they play is the 39 days as the list offers it, begin(bridge, "guadalcanal-39-days", "person", "engine"): the Allied seat, the
# first evening, where Lunga (base 1) and Tulagi (base 4) are the enemy's. H1 is the hex of base 1 and H4 the hex of base 4, each read from
# `list_bases` and never written into the check. N is the number the reading of the bare order gives it (`intent`).
#
# THE WORKED DRAFT, used by statements below. The order: seize and hold island 3, nothing set on it. Under it two landings written:
#   the landing at Lunga:  After: the landing at Tulagi. Not before: day 3. By the end of: day 6. Risk: preserve.
#   the landing at Tulagi: Ships and forces: use only the task force numbered 10 (the transports that lie loaded for Tulagi).
# Its lines stand Lunga, then Tulagi (the island's order); it is sent Tulagi first, since Lunga waits for it (item 15).


def worked_under(n, h1, h4):
    """What the worked draft sends as `under` for an order numbered `n` (item 15). The architect's: not to be changed."""
    return [
        {"intent_kind": "take_place", "hex": list(h4), "intent": n + 1, "limits": [{"limit": "forces", "task_forces": [10], "ships": []}]},
        {"intent_kind": "take_place", "hex": list(h1), "intent": n + 2, "by_day": 6,
         "limits": [{"limit": "not_before", "from_day": 3}, {"limit": "after", "other_intent": n + 1}, {"limit": "risk", "risk": "preserve"}]},
    ]


def _spread(lowest, middle, highest):
    return {"lowest": lowest, "middle": middle, "highest": highest}


def _costs(men, unfed, sunk, out, aircraft):
    return {"men_lost_to_the_enemy": _spread(*men), "men_lost_unfed": _spread(*unfed), "ships_sunk": _spread(*sunk),
            "ships_out_of_action": _spread(*out), "aircraft_lost": _spread(*aircraft)}


_RUNS = [1, 2, 3, 4, 5, 6, 7, 8]
_YOURS = {"worlds": _RUNS, "met_and_kept": [1, 2, 3, 5, 8], "met_days": {"earliest": 6, "middle": 8, "latest": 11}, "met_and_lost_again": [4], "not_met": [6, 7]}
_STAFFS = {"worlds": _RUNS, "met_and_kept": [1, 2, 3, 4, 5, 6, 8], "met_days": {"earliest": 9, "middle": 9, "latest": 9}, "met_and_lost_again": [], "not_met": [7]}

# A trial's answer, made up by the architect for the check (no game gave it): an order tried with its landings written, 8 runs of 14 days.
# Your plan wins and holds the island in runs 1, 2, 3, 5 and 8; the staff's in those and in runs 4 and 6; neither in run 7.
TWO_PLANS_ANSWER = {
    "kind": "trial", "estimate": 3, "first_day": 1, "last_day": 14, "has_end": True, "could_not_be_played": [],
    "asked": {"worlds": 8, "days": 14, "preview": 1, "intent": None, "tried_intent": 1, "beside": "left_to_the_staff", "inside_day": None},
    "how_it_went": _YOURS,
    "cost": _costs((485, 1660, 1960), (0, 0, 0), (0, 0, 1), (0, 0, 7), (10, 16, 19)),
    "enemy_lost": _costs((20, 2453, 2568), (0, 0, 0), (0, 0, 3), (0, 0, 0), (0, 0, 36)),
    "beside": {
        "of": "left_to_the_staff", "order": 1, "last_day": 14, "could_not_be_played": [],
        "with_your_plan": _YOURS, "how_it_went": _STAFFS,
        "cost": _costs((1640, 1720, 2120), (0, 0, 0), (0, 0, 1), (0, 0, 0), (13, 16, 19)),
        "enemy_lost": _costs((796, 2646, 2844), (0, 0, 0), (0, 0, 2), (0, 0, 2), (0, 0, 36)),
        "world_for_world": {"both": [1, 2, 3, 5, 8], "with_your_plan_alone": [], "with_the_staffs_alone": [4, 6], "neither": [7], "apart": []},
    },
    "other_plans": [], "assumed": [], "partings": [], "leaves_given_to_the_side": [], "text": [],
}
# What the page shows of it (item 21): the heading, the line under it, the two columns, each row as (its name, your plan, the staff's
# plan), and the line run for run. It has these five keys and no other: `left_out` and `no_cost` are absent, not empty (D4, D5). A cost's
# row is named "Your " or "The enemy’s " and the page's own name for the cost with its first letter small, the enemy's ending
# ", in the dry run"; the row for men unfed, where it has one, is "Your men lost unfed" and "The enemy’s men lost unfed, in the dry run".
# One exception (item 21, amended Nov 27, 2026): the enemy's first row is "The enemy’s men lost, in the dry run", not "men lost to the
# enemy": the enemy's men are not lost to the enemy.
# The architect's working: 5 and 7 of 8 runs kept, on days 6 to 11 and on day 9; 1 and 0 lost again; 2 and
# 1 not won; a cost's row only where either column's highest is above nothing, so no row for men unfed on either side; a spread whose
# lowest and highest are one figure is said as that figure.
TWO_PLANS_SHOWN = {
    "heading": "Your plan beside the staff’s",
    "staffs_is": "The staff’s plan is this order with every landing left to the staff.",
    "columns": ["Your plan", "The staff’s plan"],
    "rows": [
        ["Island won and held", "5 of 8 runs, days 6 to 11", "7 of 8 runs, day 9"],
        ["Won and lost again", "1 of 8 runs", "no run"],
        ["Not won by day 14", "2 of 8 runs", "1 of 8 runs"],
        ["Your men lost to the enemy", "485 to 1,960, the middle 1,660", "1,640 to 2,120, the middle 1,720"],
        ["Your ships sunk", "0 to 1, the middle 0", "0 to 1, the middle 0"],
        ["Your ships out of action at the end", "0 to 7, the middle 0", "0"],
        ["Your aircraft lost", "10 to 19, the middle 16", "13 to 19, the middle 16"],
        ["The enemy’s men lost, in the dry run", "20 to 2,568, the middle 2,453", "796 to 2,844, the middle 2,646"],
        ["The enemy’s ships sunk, in the dry run", "0 to 3, the middle 0", "0 to 2, the middle 0"],
        ["The enemy’s ships out of action at the end, in the dry run", "0", "0 to 2, the middle 0"],
        ["The enemy’s aircraft lost, in the dry run", "0 to 36, the middle 0", "0 to 36, the middle 0"],
    ],
    "run_for_run": "Run for run: won and held with both plans in 5; with your plan alone in 0; with the staff’s plan alone in 2; with neither in 1.",
}

# Reasons of the staff's, made up by the architect in the shapes `staff_orders` gave on the 39 days on Nov 27, 2026, and for each the
# numbers of the commander's nodes it turns on (item 27), his nodes being intents 2 and 3. D turns on none: the words "the commander's"
# alone make no reason his, and intent:30 is not intent:3. A real reason with no limit has no `limits` key at all: `hisNodesIn` reads a
# missing key as it reads None (R1).
HIS_NODES = [2, 3]
REASONS_AND_HIS_NODES = [
    ({"serves": {"intent": 1}, "limits": None, "worked": "intent:3, the commander's, plans the place at (2,-4): the staff forms no take beside it."}, [3]),
    ({"serves": {"intent": 3}, "limits": [{"intent": 3, "kind": "risk"}], "worked": "the side's estimate is 5275 enemy men at (2,-4); at preserve the staff wants 3 men for each."}, [3]),
    ({"serves": {"intent": 5}, "limits": None, "worked": "task_force:9 has fighting men aboard and intent:3 is in force for a place not yet the side's: it is the takes'."}, [3]),
    ({"serves": {"intent": 4}, "limits": [{"intent": 1, "kind": "forces"}], "worked": "intent:4 holds base:1 (Lunga) under intent:1 (the commander's); intent:30 has the force."}, []),
    ({"serves": {"intent": 1}, "limits": [{"intent": 2, "kind": "forces"}], "worked": "intent:3 waits for intent:2."}, [2, 3]),
]


# What the page says of a reason's words where they name a landing of his (item 28 as amended on Nov 27, 2026; R2). The engine's
# sentences are as `sim-staff` writes them (island.rs, air.rs, cover.rs), read by the architect in the code; intent 3 is his landing and
# intent 1 the order above it. ⟦3⟧ is the mark the page already makes a link from. Whose intent 3 is, is decided by its number before
# any of this is read (item 27): the words after a handle known to be his are dropped, for the link says whose it is, and without it
# the page's own "the commander's" to "your" would say "your landing at Lunga, your, is in force". The architect's: not to be changed.
HIS_LANDING_READ = [
    ("intent:3, the commander's, plans the place at (2,-4): the staff forms no take beside it.",
     "⟦3⟧ stands: the staff forms no take beside it."),
    ("intent:3, the commander's, plans the place at (2,-4), and its plan is cancelled: the staff forms no take beside it, and intent:1 cannot be controlled while the place is his to take.",
     "⟦3⟧ stands, and its plan is cancelled: the staff forms no take beside it, and intent:1 cannot be controlled while the place is yours to take."),
    ("intent:1 is to be done by the end of day 6; intent:3, the commander's take of the place at (2,-4), is to be done by the end of day 9, 3 days later. The staff carries out his date.",
     "intent:1 is to be done by the end of day 6; ⟦3⟧ is to be done by the end of day 9, 3 days later. The staff carries out your date."),
    ("intent:3, the commander's take under intent:1, waits for the plan of intent:1: it is preparing, and a take under an island is served once the island's plan is released, so the staff gives no order under intent:3 and its plan stays a draft.",
     "⟦3⟧ waits for the plan of intent:1: it is preparing, and a take under an island is served once the island's plan is released, so the staff gives no order under ⟦3⟧ and its plan stays a draft."),
    ("intent:3 is the commander's take under an island", "⟦3⟧ is your landing under an island"),
    ("intent:3, the commander's, is in force and its plan is preparing", "⟦3⟧ is in force and its plan is preparing"),
    ("intent:3, the commander's, is to be done by the end of day 9", "⟦3⟧ is to be done by the end of day 9"),
    ("task_force:9 has fighting men aboard and intent:3 is in force for a place not yet the side's: it is the takes'.",
     "task_force:9 has fighting men aboard and ⟦3⟧ is in force for a place not yet the side's: it is the takes'."),
    ("intent:3 stands", "⟦3⟧ stands"),
    # No landing of his is named: nothing is read, and "is his to take" is left as it is. intent:30 is not intent:3.
    ("intent:4 holds base:1 (Lunga) under intent:1 (the commander's); intent:30 has the force.",
     "intent:4 holds base:1 (Lunga) under intent:1 (the commander's); intent:30 has the force."),
    ("intent:30, the commander's, plans the place at (5,5): the place is his to take.", "intent:30, the commander's, plans the place at (5,5): the place is his to take."),
]


def the_island_of_two_places(bridge):
    """The island "Guadalcanal and Tulagi" in the table's 39 days (decision 0232, the designer's answer 2; items 5 to 8). Called from
    main() from the island's pull request on. The generator's `--check` and `tests/table_rear_bases_files.rs` hold the files; this holds
    the list and the games.

    I1. The list's entry `guadalcanal-39-days` has three islands in this order: {number 1, "Guadalcanal", base 1}, {number 2, "Tulagi",
        base 4} and {number 3, "Guadalcanal and Tulagi", base 1, places [1, 4]}. No other entry has an island numbered 3 or the key
        `places`.
    I2. In the page's text, an island is offered on a base's card where the base is the island's `base` or is in its `places`.
    I3. In each game of the 39 days a person can sit in (the person Allied against the engine; the person Japanese against the engine; the
        person Allied against an agent), a preview of `{"command": "set_intent", "intent_kind": "control_island", "island": 3}` is not
        refused.
    I4. (Item 6, on the designer's yes of Nov 27, 2026.) In the person Allied against an agent, previews of islands 1 and 2 are not
        refused either.
    I5. The person Allied against the engine: the tree of that preview (`with_change.tree.sub_intents`) has exactly two entries of kind
        `take_place` whose parent is the order, both with `maker` "staff", at H1 and then at H4.
    I6. (Item 7, on the designer's yes of Nov 27, 2026.) In that game the seat is offered the tool `trial`: a call of it with that
        preview and `describe` true is answered and not refused.
    Each game is put away after it."""
    print("-- the island of two places: the list, the card and the games of the 39 days", flush=True)
    entries = {e["id"]: e for e in json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]}
    islands = entries["guadalcanal-39-days"]["islands"]
    check(islands == [{"number": 1, "name": "Guadalcanal", "base": 1}, {"number": 2, "name": "Tulagi", "base": 4},
                      {"number": 3, "name": "Guadalcanal and Tulagi", "base": 1, "places": [1, 4]}],
          f"I1: the 39 days lists three islands in order, the third of two places ({islands})")
    check(all(i["number"] != 3 and "places" not in i for gid, e in entries.items() if gid != "guadalcanal-39-days" for i in e.get("islands", [])),
          "I1: no other entry has an island numbered 3 or the key places")
    page = page_text()
    check("config.islands.forEach(function (i) { if (i.base === n || (i.places && i.places.indexOf(n) >= 0)) {" in page,
          "I2: the page offers an island on the card of its base and of each base in its places")
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    for allied, japanese in (("person", "engine"), ("engine", "person"), ("person", "agent")):
        check(begin(bridge, "guadalcanal-39-days", allied, japanese) == {"ok": True}, f"begun: the 39 days, {allied} Allied against {japanese} Japanese")
        person = Person(bridge.table)
        answer = person.call("preview", command)
        check("ok" in answer and answer["ok"].get("status") != "refused" and answer["ok"].get("valid") is True,
              f"I3: a preview of island 3 is not refused and is valid ({allied} against {japanese}): {str(answer)[:120]}")
        if (allied, japanese) == ("person", "agent"):
            for number in (1, 2):
                answer = person.call("preview", dict(command, island=number))
                check("ok" in answer and answer["ok"].get("status") != "refused" and answer["ok"].get("valid") is True,
                      f"I4: in the person Allied against an agent a preview of island {number} is not refused and is valid: {str(answer)[:120]}")
        check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    # H1 and H4, read from a seat that holds the bases (the Japanese, who hold Lunga and Tulagi on the first evening)
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    shown = person.call("preview", command)["ok"]
    takes = [i for i in shown["with_change"]["tree"]["sub_intents"] if i["kind"] == "take_place" and i["parent"] == shown["intent"]]
    check([(t["maker"], t["hex"]) for t in takes] == [("staff", h1), ("staff", h4)],
          f"I5: the tree of that preview has two takes under the order, the staff's, at the hex of Lunga then of Tulagi ({[(t['maker'], t['hex']) for t in takes]} for {h1}, {h4})")
    tried = person.call("trial", {"preview": shown["preview"], "describe": True})
    check("ok" in tried and tried["ok"].get("status") != "refused", f"I6: a dry run of that preview, described, is answered and not refused: {str(tried)[:120]}")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def your_plan_on_the_order_paper(bridge):
    """Rule 27: YOUR PLAN on the order's paper (items 11 to 16).

    In the page's text:
    P1. The paper has a fold with the id `p-plan` after `p-limits` and before `p-acts`, whose head begins "YOUR PLAN". It is shown for a
        new order to the staff whose end is `control_island` and for no other kind of paper, and not in a watched game.
    P2. The page holds the functions `planLines(`, `writtenUnder(` and `planSummary(` and the words "WRITE IT", "LEAVE IT TO THE STAFF",
        "left to the staff" and "By the end of". It does not hold "Sails from": the forces' source is "Supplied from", as LIMITS has it.

    The page's own functions, cut from its text and run in node:
    P3. `planLines` of the tree of a bare reading (the shape I5 reads: takes of the staff's at two hexes, holds, a deny, and a
        `ready_landing` under each take) and the order's number gives two lines, at those hexes, in the tree's order; an entry that is
        not a `take_place`, or whose parent is not the order, gives no line. A tree with no such take gives an empty list; no tree
        (null, or a reading that was refused and has none) gives null.
    P4. `planSummary` says "The staff plans every landing." with nothing written; "Yours: the landing at Tulagi, the landing at Lunga."
        for the worked draft; "The staff plans no landing under this order: there is none to write." for an empty list of lines; and
        nothing (an empty string) for null. The page's text holds "There is no reading of this order to take its landings from." and
        does not hold "Every place is ours".
    P5. `writtenUnder` of the worked draft, the order numbered 1, the hexes (2,-4) for Lunga and (3,-5) for Tulagi, is exactly
        `worked_under(1, (2, -4), (3, -5))`. With the "After" of Lunga taken off, the entries stand in the lines' order: Lunga numbered 2,
        Tulagi numbered 3. A landing with nothing set is sent with `limits` empty and no `by_day`. Risk (item 14): a landing whose risk
        was never set sends no risk; with Tulagi's risk set to balanced its limits end with {"limit": "risk", "risk": "balanced"}; put
        back to the staff's choice, it sends none again.
    P6. What a written landing offers is, in this order, and nothing else: Ships and forces, Land units, Supplied from, Not before,
        Wait for, After, Knowledge, Risk, By the end of. It offers no single ship, no air group, no area, no targets, no protected ships
        and no "none of" for its ships or its land units, and a draft that has any of those set on a landing sends none of them (no
        `none_of` among them). Risk offers four choices: the staff's choice, preserve, balanced, press.
    P7. With Lunga after Tulagi, the "After" of Tulagi does not offer the landing at Lunga; it offers it again once Lunga's is put back.
    P8. No words the model gives hold a handle or a number of the engine's (no "intent", no "task_force:", no number of an intent).

    Through the bridge:
    P9. `planLines` of the real bare reading of island 3 gives two lines, at H1 and then at H4.
    P10. A preview of the order with `under` as `worked_under(N, H1, H4)` is not refused; its `intent` is N; its `under` numbers are N + 1
         and N + 2; its tree has a `take_place` of the commander's, `standing`, at H4 and at H1, and no `take_place` of the staff's.
         The lines the page shows are still Lunga, then Tulagi.
    P11. The same with Tulagi's landing given "Risk: press" (the order at balanced) is refused with `of_batch.position` 4 (the order, the
         landing at Tulagi, its forces, its risk), the engine's reason being "the limit is wider than what its parent intent:N
         allows" (`CommandRefusal::WiderThanParent`, crates/sim-core/src/command.rs). The page's words for it are exactly "The landing
         at Tulagi: Risk: It is wider than the order above it allows.": `refusalWords` of the engine's reason gives "It is wider than
         the order above it allows.", with no handle and no number in it (amended Nov 27, 2026: the engine's sentence names its parent
         by handle, which the page's refusal words would have said as a number).
    Nothing is signed, and the game is put away."""
    import re
    print("-- YOUR PLAN on the order paper: the part, its model in node, and the engine taking what it sends", flush=True)
    text = page_text()
    # P1
    at = lambda i: text.index(f'id="{i}"')
    check(at("p-limits") < at("p-plan") < at("p-acts-t") and re.search(r'<details[^>]*id="p-plan"[^>]*><summary[^>]*>YOUR PLAN</summary>', text) is not None,
          "P1: the order paper has the fold p-plan after p-limits and before p-acts, whose head begins YOUR PLAN")
    end = cut_function(text, "  function planEnd() {")
    check("'control_island'" in end and "paper.kind !== 'staff'" in end and "WATCH" in end and "part.hidden = !end" in cut_function(text, "  function drawPlan(focusId) {"),
          "P1: it is shown for a new order to the staff whose end is control_island, and for no other paper and not in a watched game")
    check("'YOUR PLAN · '" in cut_function(text, "  function planHead() {") and "$('p-plan').hidden = true" in cut_function(text, "  function dropPaper() {") and "planReset();" in cut_function(text, "  function dropPaper() {"),
          "P1: the head is YOUR PLAN and then what the model says of it; a paper put away puts it away")
    # P2
    check(all(w in text for w in ("function planLines(", "function writtenUnder(", "function planSummary(", "WRITE IT", "LEAVE IT TO THE STAFF", "left to the staff", "By the end of")),
          "P2: the page holds planLines, writtenUnder and planSummary, and the words WRITE IT, LEAVE IT TO THE STAFF, left to the staff and By the end of")
    check("Sails from" not in text, "P2: the page does not hold \"Sails from\": the forces' source is \"Supplied from\", as LIMITS has it")
    # P4, in the page's text
    check("There is no reading of this order to take its landings from." in text and "Every place is ours" not in text,
          "P4: the page says there is no reading to take the landings from where there is none, and never that every place is ours")
    ask = cut_function(text, "  function askPreview() {")
    check("args.under = writtenUnder(previewIntent, planState.written)" in ask and "if (planEnd() && !landings.length) planState.lines = planLines(" in ask and "p.intent !== asked" in ask,
          "P10: the staff is asked with the landings numbered from the order's number, the lines are taken from a reading of the bare order alone, and a changed number is asked again")
    # The model in node
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("cleanName", "num", "hexDist", "hexWords"))
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the written plan model ----")]
    refusal = (text[text.index("  function cleanName"):text.index("  function sayIntent")] + text[text.index("  function orderWords"):text.index("  function nameOf(")]
               + cut_function(text, "  function commas(n)") + "\n" + text[text.index("  var REFUSALS = ["):text.index("  // What the status line says of orders taken in")])
    code = helpers + model + refusal + r"""
const hexes = []; function dateOf(d) { return 'day ' + d; }
let HX = [[2, -4], [3, -5]];
function worked() {
  const lunga = newLanding(HX[0]), tulagi = newLanding(HX[1]);
  tulagi.lim.ships = { mode: 'only', task_forces: [10], ships: [] };
  lunga.lim.not_before = 3; lunga.lim.after = { landing: HX[1] }; lunga.lim.risk = 'preserve'; lunga.by_day = 6;
  return { lunga: lunga, tulagi: tulagi, written: [lunga, tulagi] };
}
function main(x) {
  HX = [x.h1, x.h4]; M.hexNames[x.h1[0] + ',' + x.h1[1]] = 'Lunga'; M.hexNames[x.h4[0] + ',' + x.h4[1]] = 'Tulagi';
  const out = {}, n = x.n;
  out.lines = planLines(x.tree, 1); out.none = planLines(x.bare, 1); out.noTree = [planLines(null, 1), planLines(undefined, 1), planLines({}, 1)];
  out.realLines = x.real ? planLines(x.real, n) : null; out.underLines = x.under_tree ? planLines(x.under_tree, n) : null;
  let w = worked(), lines = [{ hex: HX[0] }, { hex: HX[1] }];
  out.summary = { empty: planSummary(lines, []), worked: planSummary(lines, w.written), none: planSummary([], []), null_: planSummary(null, []), nullWritten: planSummary(null, w.written) };
  out.under = writtenUnder(n, w.written);
  w = worked(); w.lunga.lim.after = null; out.noAfter = writtenUnder(1, w.written).map(u => [u.hex, u.intent]);
  w = worked(); w.tulagi.lim.risk = 'balanced'; out.balanced = writtenUnder(1, w.written)[0].limits; w.tulagi.lim.risk = null; out.balancedBack = writtenUnder(1, w.written)[0].limits;
  out.bare = writtenUnder(1, [newLanding(HX[0])]);
  // what a landing does not offer is not sent, if it is set
  const odd = newLanding(HX[0]); odd.by_day = 5;
  odd.lim.ships = { mode: 'none', task_forces: [], ships: [7] }; odd.lim.land_units = { mode: 'none', list: [4] }; odd.lim.air_groups = { mode: 'only', list: [3] }; odd.lim.submarines = { mode: 'only', list: [1] };
  odd.lim.area = { q: 0, r: 0, dist: 2 }; odd.lim.targets = { kinds: ['carrier'], waters_first: true }; odd.lim.protected = [7]; odd.lim.in_place = true;
  out.odd = writtenUnder(1, [odd]);
  const odd2 = newLanding(HX[0]); odd2.lim.ships = { mode: 'only', task_forces: [10], ships: [7] }; odd2.lim.land_units = { mode: 'only', list: [4] }; odd2.lim.source = 1; out.odd2 = writtenUnder(1, [odd2]);
  out.offered = LANDING_LINES.map(landingLineName); out.risks = LANDING_RISKS; out.offers = LIMITS_OFFERED.written_landing;
  // After
  w = worked(); out.afterTulagi = afterOptions(w.written, w.tulagi).map(o => o[1]); out.afterLunga = afterOptions(w.written, w.lunga).map(o => o[1]);
  w.lunga.lim.after = null; out.afterTulagiBack = afterOptions(w.written, w.tulagi).map(o => o[1]);
  const three = [newLanding(HX[0]), newLanding(HX[1]), newLanding([4, 4])]; three[0].lim.after = { landing: HX[1] }; three[1].lim.after = { landing: [4, 4] };
  out.afterThree = afterOptions(three, three[2]).map(o => o[1]);
  // The words the model gives
  w = worked(); const landings = landingsSent(n, w.written);
  out.words = [out.summary.worked, out.summary.none, out.summary.empty, landingCap(HX[0]), out.offered.join('|')].concat(out.afterTulagi, out.afterLunga, out.risks);
  if (x.reason) {
    const tul = newLanding(HX[1]); tul.lim.ships = { mode: 'only', task_forces: [10], ships: [] }; tul.lim.risk = 'press';
    const sent = landingsSent(n, [tul]);
    out.refusedWhy = refusalWords(x.reason); out.refusedAt = batchAt([], sent, x.position); out.refusedWords = batchWords(out.refusedAt, out.refusedWhy);
    out.refusedOrder = batchWords(batchAt([], sent, 1), 'x'); out.refusedLanding = batchWords(batchAt([], sent, 2), 'x'); out.refusedLimit = batchWords(batchAt([{ key: 'risk' }], [], 2), 'x');
    out.words.push(out.refusedWords);
  }
  return out;
}
"""
    sub = lambda number, kind, parent, hex_=None, maker="staff": {"number": number, "kind": kind, "parent": parent, "hex": hex_, "maker": maker}
    tree = {"sub_intents": [sub(2, "take_place", 1, [2, -4]), sub(3, "hold", 1), sub(4, "hold", 1), sub(5, "take_place", 1, [3, -5]), sub(6, "deny", 1, [2, -4]),
                            sub(7, "ready_landing", 2, [2, -4]), sub(8, "ready_landing", 5, [3, -5]), sub(9, "take_place", 3, [9, 9])]}
    bare = {"sub_intents": [sub(3, "hold", 1), sub(9, "take_place", 3, [9, 9])]}
    said = run_in_node(code, {"tree": tree, "bare": bare, "n": 1, "h1": [2, -4], "h4": [3, -5], "reason": None, "position": 4})
    # P3
    check(said["lines"] == [{"hex": [2, -4]}, {"hex": [3, -5]}], f"P3: the bare reading's tree gives two lines, at those hexes, in the tree's order, and a hold, a deny, a ready_landing or a take under another gives none ({said['lines']})")
    check(said["none"] == [] and said["noTree"] == [None, None, None], f"P3: a tree with no such take gives an empty list, and no tree gives null ({said['none']}, {said['noTree']})")
    # P4
    s = said["summary"]
    check(s["empty"] == "The staff plans every landing." and s["worked"] == "Yours: the landing at Tulagi, the landing at Lunga." and s["none"] == "The staff plans no landing under this order: there is none to write."
          and s["null_"] == "" and s["nullWritten"] == "", f"P4: the summary says every landing is the staff's, the worked draft's two in the order sent, that there is none to write, and nothing for no reading ({s})")
    # P5
    check(said["under"] == worked_under(1, (2, -4), (3, -5)), f"P5: writtenUnder of the worked draft is exactly the architect's `under` ({said['under']})")
    check(said["noAfter"] == [[[2, -4], 2], [[3, -5], 3]], f"P5: with Lunga's After taken off the entries stand in the lines' order, Lunga numbered 2 and Tulagi 3 ({said['noAfter']})")
    check(said["bare"] == [{"intent_kind": "take_place", "hex": [2, -4], "intent": 2, "limits": []}], f"P5: a landing with nothing set is sent with limits empty and no by_day ({said['bare']})")
    check(not any(l["limit"] == "risk" for l in said["balancedBack"]) and said["balanced"][-1] == {"limit": "risk", "risk": "balanced"} and said["balanced"][:-1] == said["balancedBack"],
          f"P5: a risk never set sends none; set to balanced it ends the landing's limits; put back to the staff's choice it sends none again ({said['balanced']})")
    # P6
    check(said["offered"] == ["Ships and forces", "Land units", "Supplied from", "Not before", "Wait for", "After", "Knowledge", "Risk", "By the end of"],
          f"P6: a written landing offers these lines in this order ({said['offered']})")
    unoffered = {"air_groups", "submarines", "area", "targets", "protected", "in_place"}
    odd = said["odd"][0]
    check(odd["limits"] == [] and odd["by_day"] == 5 and "none_of" not in json.dumps(said["odd"]) and not any(k in json.dumps(said["odd"]) for k in unoffered),
          f"P6: single ships, air groups, boats, an area, targets, protected ships, from where they lie and \"none of\" set on a landing are not sent ({odd})")
    check(said["odd2"][0]["limits"] == [{"limit": "forces", "task_forces": [10], "ships": [], "source": 1, "land_units": [4]}],
          f"P6: a forces limit of a landing carries task forces, land units and a base, and no single ship ({said['odd2']})")
    rows = cut_function(text, "  function landingRows(l) {") + cut_function(text, "  function planKind(l, key, items) {")
    check(not any(k in rows for k in ("'air_groups'", "'submarines'", "'area'", "'targets'", "'protected'", "'in_place'", "'None'", "none_of", "limShips", "limAir", "M.air")) and "['only', 'Use only these']" in rows and "LANDING_RISKS.map" in rows,
          "P6: the lines drawn offer no single ship, no air group, no area, no targets, no protected ships and no \"none of\": a list is the staff's choice or use only these")
    check(said["risks"] == ["preserve", "balanced", "press"] and "[['', STAFFS]].concat(options)" in cut_function(text, "  function planChoose(l, key, options, value, change, reset, more) {"),
          "P6: Risk offers four choices: the staff's choice, preserve, balanced and press")
    check(said["offers"] == ["ships", "land_units", "source", "landing", "risk"], f"P6: the limits model's kind for a written landing offers these ({said['offers']})")
    # P7
    check(said["afterTulagi"] == [] and said["afterLunga"] == ["The landing at Tulagi"] and said["afterTulagiBack"] == ["The landing at Lunga"] and said["afterThree"] == [],
          f"P7: a landing that waits for another, directly or through a third, is not offered to it, and is again once put back ({said['afterTulagi']}, {said['afterTulagiBack']}, {said['afterThree']})")
    # P8
    bad = [w for w in said["words"] if re.search(r"intent|task_force|\w:\d|\d", w)]
    check(not bad, f"P8: no words the model gives hold a handle or a number of the engine's ({bad})")
    # The engine
    begin_ok = begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}
    check(begin_ok, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    shown = person.call("preview", command)["ok"]
    n = shown["intent"]
    # P9
    real = run_in_node(code, {"tree": tree, "bare": bare, "n": n, "h1": h1, "h4": h4, "real": shown["with_change"]["tree"], "reason": None, "position": 4})
    check(real["realLines"] == [{"hex": h1}, {"hex": h4}], f"P9: planLines of the real bare reading of island 3 gives two lines, at the hex of Lunga and then of Tulagi ({real['realLines']})")
    # P10
    under = worked_under(n, h1, h4)
    drafted = person.call("preview", dict(command, under=under))["ok"]
    sub_ = drafted["with_change"]["tree"]["sub_intents"]
    takes = [(i["maker"], i.get("standing"), i["hex"]) for i in sub_ if i["kind"] == "take_place"]
    check(drafted.get("status") != "refused" and drafted["intent"] == n and [u["intent"] for u in drafted["under"]] == [n + 1, n + 2],
          f"P10: a preview of the order with the worked draft under it is not refused; its intent is {n}, its under numbers {n + 1} and {n + 2}")
    check(sorted(takes) == sorted([("commander", True, h4), ("commander", True, h1)]) and not any(t[0] == "staff" for t in takes), f"P10: its tree has his takes standing at the hex of Tulagi and of Lunga, and none of the staff's ({takes})")
    lines = run_in_node(code, {"tree": tree, "bare": bare, "n": n, "h1": h1, "h4": h4, "real": shown["with_change"]["tree"], "under_tree": drafted["with_change"]["tree"], "reason": None, "position": 4})
    check(lines["underLines"] == [{"hex": h4}, {"hex": h1}] and lines["realLines"] == [{"hex": h1}, {"hex": h4}] and lines["under"] == worked_under(n, h1, h4),
          f"P10: the reading with his landings under it gives its takes in the order sent, Tulagi and then Lunga ({lines['underLines']}); the lines the page shows are still those of the bare reading, Lunga and then Tulagi, and the page sends what the architect's `under` is")
    # P11
    press = [{"intent_kind": "take_place", "hex": h4, "intent": n + 1, "limits": [{"limit": "forces", "task_forces": [10], "ships": []}, {"limit": "risk", "risk": "press"}]}]
    refused = person.call("preview", dict(command, under=press))["ok"]
    check(refused.get("status") == "refused" and refused["of_batch"]["position"] == 4 and refused["reason"] == f"the limit is wider than what its parent intent:{n} allows",
          f"P11: a landing at press under an order at balanced is refused at position 4, the engine's reason being the limit is wider than what its parent allows ({refused.get('of_batch')}, {refused.get('reason')})")
    words = run_in_node(code, {"tree": tree, "bare": bare, "n": n, "h1": h1, "h4": h4, "reason": refused["reason"], "position": refused["of_batch"]["position"]})
    check(words["refusedWhy"] == "It is wider than the order above it allows." and not re.search(r"\d|intent|:", words["refusedWhy"]),
          f"P11: refusalWords of the engine's reason gives It is wider than the order above it allows., with no handle or number ({words['refusedWhy']})")
    check(words["refusedWords"] == "The landing at Tulagi: Risk: It is wider than the order above it allows.", f"P11: the page's words for it are exactly that line, said at the landing's line ({words['refusedWords']})")
    check(words["refusedOrder"] == "x" and words["refusedLanding"] == "The landing at Tulagi: x" and words["refusedLimit"] == "Risk: x",
          f"P11: position 1 is the order, 2 the landing and a limit of the order its own line ({words['refusedOrder']}, {words['refusedLanding']}, {words['refusedLimit']})")
    check(post(bridge.table + "/new") == {"ok": True}, "nothing was signed, and the game is put away")


def the_draft_is_read_and_signed_as_one(bridge):
    """Rule 28: one reading, one approval, one entry under SIGNED, one TAKE BACK (items 17 to 19).

    In the page's text:
    S1. The reading is asked with `under` where a landing is written, by the one function that asks a reading today; signing is still
        one `approve` of the preview and sends no `set_intent`, `set_limit` or `form_intent` of its own; after it the page checks
        `under_queued`.

    Through the bridge, the worked draft:
    S2. `approve` of its preview answers `status` "queued", `limits_queued` 0, and `under_queued` of two: {intent N + 1, limits_queued 1}
        and {intent N + 2, limits_queued 3}.
    S3. `queued` then holds seven entries (the order; each landing; one limit of Tulagi's and three of Lunga's): one `set_intent` with
        no `parent`, whose `goes_with` names the other six. The page's own
        function for SIGNED, run in node on that answer, makes one line of them: it says the order as the page says it, then
        "Yours: the landing at Tulagi, the landing at Lunga.", and holds no handle or number of the engine's. No entry with a `parent`,
        and no limit of one, is a line of its own. The limits the line says are the order's own and no other: those `set_limit`
        entries whose `intent` is N. This order has none, so nothing stands between the order's words and "Yours:" but the page's
        mark between two parts of a line: no limit of a landing (Tulagi's forces; Lunga's Not before, After and Risk) is said as
        the order's (made exact Nov 27, 2026: `goes_with` of the order names the landings' limits too).
    S4. One `take_back` of that entry's place leaves `queued` empty.
    S5. The worked draft with the order's own risk at press: `limits_queued` is 1, and SIGNED's line says the risk before "Yours:",
        once, as press, and says no other limit before it (Lunga's "Risk: preserve" is not the order's).
    S6. (New, Nov 27, 2026.) THE ORDERS' Signed list holds no entry with a `parent`: a landing is not listed as an order. The page's
        `signedOrders`, in node on S3's queue, gives one entry, the order's; and that order's row in THE ORDERS says, as its limits,
        the order's own alone, as S3 and S5 have SIGNED say them.
    The game is put away."""
    import re
    print("-- the draft read and signed as one: one preview, one approval, one line under SIGNED, one TAKE BACK", flush=True)
    text = page_text()
    # S1
    ask = cut_function(text, "  function askPreview() {")
    sign = cut_function(text, "  function signStaff() {")
    check(ask.count("api('preview'") == 1 and "args.under = writtenUnder(" in ask and text.count("function askPreview(") == 1,
          "S1: the reading of a draft with a landing written is asked with under, by the one function that asks a reading")
    check(sign.count("api('approve', { preview: no })") == 1 and not any(f"api('{t}'" in sign for t in ("set_intent", "set_limit", "form_intent", "withdraw_intent")) and "under_queued" in sign and "limits_queued" in sign,
          "S1: signing is one approve of the preview, sends no set_intent, set_limit or form_intent of its own, and checks limits_queued and under_queued after it")
    # The engine: the worked draft, signed
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    n = person.call("preview", command)["ok"]["intent"]
    # The page's own function for SIGNED, and THE ORDERS' list, in node
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("cleanName", "num", "hexDist", "hexWords", "nm", "unitNo", "nameText", "sayIntent", "signedWith", "signedOwnLimits",
                                                                                "signedPlanWords", "signedLine", "signedOrders"))
    word = re.search(r"^  var WORD = \{.*?\};$", text, re.M).group(0)
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the written plan model ----")]
    code = helpers + word + "\n" + model + r"""
const hexes = [], config = { islands: [{ number: 3, name: 'Guadalcanal and Tulagi', base: 1, places: [1, 4] }] }; let hexRaw = false;
function dateOf(d) { return 'day ' + d; } function rich(tag, cls, w) { return el(tag, cls, w); } function placeLink() { return el('span', '', '?'); } function refTo() { return el('span', '', '?'); }
function baseByNumber() { return null; } function plainWords(t) { return t; }
function main(x) {
  M.hexNames[x.h1[0] + ',' + x.h1[1]] = 'Lunga'; M.hexNames[x.h4[0] + ',' + x.h4[1]] = 'Tulagi';
  const out = {};
  x.queues.forEach(function (q, k) {
    M.queue = { entries: q };
    const order = q.filter(function (e) { return e.command.kind === 'set_intent' && e.command.parent == null; })[0];
    out['line' + k] = signedLine(order).textContent;
    out['signed' + k] = signedOrders().map(function (e) { return e.place.number; });
    out['own' + k] = limitsLine('control_island', limFromBodies(signedOwnLimits(order, q)));
    out['said' + k] = signedOwnLimits(order, q).length;
    out['saidOrder' + k] = 'Order: ' + sayIntent({ number: 'signed 1', kind: 'control_island', island: 3 }, {});
  });
  return out;
}
"""
    # S2
    drafted = person.call("preview", dict(command, under=worked_under(n, h1, h4)))["ok"]
    approved = person.call("approve", {"preview": drafted["preview"]})["ok"]
    check(approved["status"] == "queued" and approved["limits_queued"] == 0 and approved["under_queued"] == [{"intent": n + 1, "limits_queued": 1}, {"intent": n + 2, "limits_queued": 3}],
          f"S2: approve of the worked draft's preview answers queued, limits_queued 0 and under_queued of two ({approved.get('status')}, {approved.get('limits_queued')}, {approved.get('under_queued')})")
    # S3
    held = person.call("queued")["ok"]
    entries = held["entries"]
    top = [e for e in entries if e["command"]["kind"] == "set_intent" and "parent" not in e["command"]]
    others = [e for e in entries if e not in top]
    check(len(entries) == 7 and len(top) == 1 and len(top[0]["goes_with"]) == 6 and all({"queue": e["place"]["queue"], "number": e["place"]["number"]} in top[0]["goes_with"] for e in others),
          f"S3: queued holds seven entries, one set_intent with no parent whose goes_with names the other six ({len(entries)} entries, {len(top)} orders)")
    check(sum(1 for e in entries if e["command"]["kind"] == "set_limit") == 4 and sum(1 for e in entries if e["command"]["kind"] == "set_intent" and "parent" in e["command"]) == 2,
          "S3: they are the order, each landing, one limit of Tulagi's and three of Lunga's")
    first = run_in_node(code, {"queues": [entries], "h1": h1, "h4": h4})
    line, expect = first["line0"], first["saidOrder0"]
    check(line == expect + " · Yours: the landing at Tulagi, the landing at Lunga." and not re.search(r"intent|\w:\d|\d", line[len("Order: "):]),
          f"S3: the page's own function for SIGNED makes one line of them: the order as the page says it, then Yours: the landing at Tulagi, the landing at Lunga., with no handle or number ({line})")
    check(not [e for e in entries if e["command"]["kind"] == "set_limit" and e["command"]["intent"] == n] and first["said0"] == 0
          and not any(w in line for w in ("Not before", "After", "Risk", "Ships and forces", "forces", "preserve")) and line.index(" · Yours:") == len(expect),
          f"S3: the limits the line says are the order's own, which this order has none of: nothing stands between the order's words and Yours: but the page's mark between two parts of a line ({line})")
    check(all(sum(1 for e in entries if e["command"]["kind"] == "set_limit" and e["command"]["intent"] == i) >= 1 for i in (n + 1, n + 2)),
          "S3: the landings' limits are on their own numbers, and none of them is said as the order's")
    # S6, on this queue
    check(first["signed0"] == [top[0]["place"]["number"]] and first["own0"] == "The staff’s choice in all.",
          f"S6: THE ORDERS' Signed list, in node on this queue, holds the order alone, and its row says no limit of the order's, none of a landing's ({first['signed0']}, {first['own0']})")
    # S4
    taken = person.call("take_back", {"queue": top[0]["place"]["queue"], "number": top[0]["place"]["number"], "as_of": held["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], f"S4: one take_back of that entry's place leaves queued empty ({taken.get('status')})")
    # S5
    risk = {"limit": "risk", "risk": "press"}
    pressed = person.call("preview", dict(command, limits=[risk], under=worked_under(n, h1, h4)))["ok"]
    approved = person.call("approve", {"preview": pressed["preview"]})["ok"]
    check(approved["status"] == "queued" and approved["limits_queued"] == 1, f"S5: the worked draft with the order's own risk at press: limits_queued is 1 ({approved.get('status')}, {approved.get('limits_queued')})")
    entries = person.call("queued")["ok"]["entries"]
    second = run_in_node(code, {"queues": [entries], "h1": h1, "h4": h4})
    line, expect = second["line0"], second["saidOrder0"]
    check(line == expect + " · Risk: press · Yours: the landing at Tulagi, the landing at Lunga." and line.count("Risk") == 1 and "preserve" not in line,
          f"S5: SIGNED's line says the risk before Yours:, once, as press, and says no other limit before it ({line})")
    check(second["signed0"] == [entries[0]["place"]["number"]] and second["own0"] == "Risk: press" and second["said0"] == 1,
          f"S6: the order's row in THE ORDERS says the order's own risk alone ({second['own0']}); the page's signedOrders gives one entry ({second['signed0']})")
    # S6: a landing is no order of its own, and both of THE ORDERS' and SIGNED's words are one function's
    draw = cut_function(text, "  function drawOrders(focus) {")
    check("signedOwnLimits(e, (M.queue && M.queue.entries) || [])" in draw and "limFromBodies(lims)" in draw and "signedOwnLimits(e, (M.queue && M.queue.entries) || [])" in cut_function(text, "  function signedLine(e) {")
          and "e.command.parent == null" in cut_function(text, "  function signedOrders() {"),
          "S6: THE ORDERS' Signed list holds no entry with a parent, and its row and SIGNED's line say the order's limits by one function, the limits on the order's own number")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_dry_run_shows_two_plans(bridge):
    """Rule 29: the dry run's two columns (items 20 to 22).

    The page's own function `twoPlans(j, landing)`, cut from its text and run in node. `landing` is the name of the place of the landing
    tried, which the page gives it from `command_state` (the hex of the intent `asked.tried_intent`), or null where the order itself was
    tried; the function reads no word of the answer for it (item 21):
    D1. `twoPlans(TWO_PLANS_ANSWER, null)` is exactly `TWO_PLANS_SHOWN`: the keys `heading`, `staffs_is`, `columns`, `rows` and
        `run_for_run`, with those values, and no other key (no `left_out`, no `no_cost`).
    D2. Of the same answer with `beside.of` "given_back_to_the_staff" and `asked.tried_intent` 3, `twoPlans(that, "Lunga")` has
        `staffs_is` "The staff’s plan is this order with the landing at Lunga given back to the staff from 1800." and the rest is as D1.
    D2a. Of the same answer with `beside.of` "left_to_the_staff" and `asked.tried_intent` 3, `twoPlans(that, "Lunga")` has `staffs_is`
        "The staff’s plan is this order with the landing at Lunga left to the staff." and the rest is as D1.
    D3. Of the same answer with `beside.of` "without_it", "withdrawn" or "as_it_stands", or with no `beside`, it gives nothing, and the
        sheet is as it is today.
    D4. With `world_for_world.apart` [8] (and 8 taken out of `both`) it gains the key `left_out`, "1 run is left out: it was not played
        under both plans.", shown after the line run for run; with two in `apart`, "2 runs are left out: they were not played under both
        plans." (`apart` holds a run that could not be played under one plan, or under either.)
    D5. With every cost of both sides at nothing in both columns the rows are the island's alone, and it gains the key `no_cost`, "No
        man, ship or aircraft is lost in any run."
    D6. With every string of `text` in the answer changed to other words, what `twoPlans` gives does not change.

    In the page's text:
    D7. The body of `twoPlans` does not name `text`. The sheet for these two kinds does not say "Beside your side without this order",
        and holds none of the words "better", "worse", "best", "recommend", "advise", "should", "per cent", "%", "points", "chance" in
        what it writes; neither column is given a class or a colour the other is not.
    D8. The order paper's DRY RUN sends `trial` with the preview and the size and no `compare`.

    Through the bridge, on the 39 days, which have a trial from the island's pull request on (item 7):
    D9. `trial` of the worked draft's preview at the size small (a size the table offers on the 39 days: D11) is answered with `beside.of` "left_to_the_staff", `beside.order` N,
        `asked.compare` true, and `beside.with_your_plan` equal to `how_it_went`. Whatever counts it gives, what `twoPlans` makes of it
        says them: each cell of the first row begins with the page's words for the count of `met_and_kept` of its column, the four
        counts run for run are those of `world_for_world`, and together with `apart` they are the answer's `asked.worlds`.
    D10. The preview tried is not spent: it is approved after the trial. The game is put away.

    The sizes offered (item 22a; the designer's answer of Nov 27, 2026, after timing the 39 days on his machine):
    D11. In `table/scenarios.json` the entry `guadalcanal-39-days` has the key `dry_run`, exactly
        {"sizes": ["small", "standard"], "seconds": {"standard": 45}}, and no other entry has the key.
    D12. The page's pure function `sizesOffered(dry)`, in node. Of that value it gives exactly
        {"sizes": ["small", "standard"], "says": ["Standard takes about 45 seconds."]}. Of undefined, of null, of {} and of
        {"sizes": []} it gives {"sizes": ["small", "standard", "full"], "says": []}. Of
        {"sizes": ["full", "huge", "small"], "seconds": {"small": 2, "full": 1, "standard": 45}} it gives
        {"sizes": ["small", "full"], "says": ["Small takes about 2 seconds.", "Full takes about 1 second."]}: the order is always
        small, standard, full; a size it does not know is left out; a time is said only of a size offered.
    D13. In the page's text: the three sizes are listed in the body of `sizesOffered` and nowhere else; `dryBox` draws its size
        buttons from `sizesOffered(config.dry_run).sizes` and a line for each of its `says` under them; `drySize` begins as
        "small", and a DRY RUN sends the first size offered where the size in hand is not offered. Neither function names a
        game ("guadalcanal", "39"). Through the bridge: `/config` of a game of the 39 days has `dry_run` equal to the entry's, and
        `/config` of a game of the Eastern Solomons over five days has no such key.
    No count from a real trial is an expected value here (item 30): the developer reports them to the architect in "Built: step 3"."""
    import copy
    import re
    print("-- the dry run's two columns: twoPlans in node on the architect's answer, and on a real trial of the worked draft", flush=True)
    text = page_text()
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("commas", "lowerFirst", "spreadOf", "runsIn", "twoPlans"))
    code = helpers + text[text.index("  var COSTS ="):text.index("  function spreadOf")] + r"""
function main(x) {
  return { shown: x.cases.map(function (c) { return twoPlans(c.answer, c.landing); }),
           runs: x.cases.map(function (c) { var b = c.answer.beside; return b ? [runsIn(b.with_your_plan.met_and_kept, b.with_your_plan.worlds.length), runsIn(b.how_it_went.met_and_kept, b.how_it_went.worlds.length)] : null; }) };
}
"""
    def ask(*cases):
        return run_in_node(code, {"cases": [{"answer": a, "landing": l} for a, l in cases]})
    answer = TWO_PLANS_ANSWER
    def varied(beside_of=None, tried=None, mend=None):
        a = copy.deepcopy(answer)
        if beside_of is not None:
            a["beside"]["of"] = beside_of
        if tried is not None:
            a["asked"]["tried_intent"] = tried
        if mend:
            mend(a)
        return a
    def apart_one(a):
        w = a["beside"]["world_for_world"]; w["both"] = [1, 2, 3, 5]; w["apart"] = [8]
    def apart_two(a):
        w = a["beside"]["world_for_world"]; w["both"] = [1, 2, 3]; w["apart"] = [5, 8]
    def no_cost(a):
        for holder in (a, a["beside"]):
            holder["cost"] = _costs((0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0))
            holder["enemy_lost"] = _costs((0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0), (0, 0, 0))
    given = varied("given_back_to_the_staff", 3)
    left = varied("left_to_the_staff", 3)
    said = ask((answer, None), (given, "Lunga"), (left, "Lunga"), (varied("without_it"), None), (varied("withdrawn"), None), (varied("as_it_stands"), None),
               (dict((k, v) for k, v in answer.items() if k != "beside"), None), (varied(mend=apart_one), None), (varied(mend=apart_two), None), (varied(mend=no_cost), None),
               (varied(mend=lambda a: a.update(text=["Tried: preview 1, the whole order.", "How it went: met and held in 5 of 8 worlds."])), None),
               (varied(mend=lambda a: a.update(text=["Quite other words, none of them the first's.", "Nor is this one."])), None),
               (given, None))
    shown = said["shown"]
    # D1
    check(shown[0] == TWO_PLANS_SHOWN and set(shown[0]) == {"heading", "staffs_is", "columns", "rows", "run_for_run"},
          f"D1: twoPlans of the architect's answer is exactly the architect's table, with no left_out and no no_cost ({shown[0]})")
    # D2, D2a
    check(shown[1] == dict(TWO_PLANS_SHOWN, staffs_is="The staff’s plan is this order with the landing at Lunga given back to the staff from 1800."),
          f"D2: a landing given back is said as given back to the staff from 1800, the rest as D1 ({shown[1]['staffs_is']})")
    check(shown[12] == dict(TWO_PLANS_SHOWN, staffs_is="The staff’s plan is this order with the landing given back to the staff from 1800."),
          f"D2: a landing given back and no place for it to name says the landing given back to the staff from 1800 ({shown[12]['staffs_is']})")
    check(shown[2] == dict(TWO_PLANS_SHOWN, staffs_is="The staff’s plan is this order with the landing at Lunga left to the staff."),
          f"D2a: a landing left to the staff is said as left to the staff, the rest as D1 ({shown[2]['staffs_is']})")
    # D3
    check(shown[3] is None and shown[4] is None and shown[5] is None and shown[6] is None, "D3: beside of without_it, withdrawn, as_it_stands, or none gives nothing: the sheet is as it is today")
    # D4
    four = "Run for run: won and held with both plans in 4; with your plan alone in 0; with the staff’s plan alone in 2; with neither in 1."
    check(shown[7] == dict(TWO_PLANS_SHOWN, run_for_run=four, left_out="1 run is left out: it was not played under both plans.") and list(shown[7]).index("left_out") > list(shown[7]).index("run_for_run"),
          f"D4: with one run apart it gains left_out, after the line run for run ({shown[7].get('left_out')})")
    three = "Run for run: won and held with both plans in 3; with your plan alone in 0; with the staff’s plan alone in 2; with neither in 1."
    check(shown[8] == dict(TWO_PLANS_SHOWN, run_for_run=three, left_out="2 runs are left out: they were not played under both plans."),
          f"D4: with two apart it says 2 runs are left out: they were not played under both plans. ({shown[8].get('left_out')})")
    # D5
    check(shown[9] == dict(TWO_PLANS_SHOWN, rows=TWO_PLANS_SHOWN["rows"][:3], no_cost="No man, ship or aircraft is lost in any run."),
          f"D5: with every cost at nothing the rows are the island's alone and it gains no_cost ({shown[9]['rows'] if shown[9] else None})")
    # D6
    check(shown[10] == TWO_PLANS_SHOWN and shown[11] == TWO_PLANS_SHOWN, "D6: with every string of text changed to other words what twoPlans gives does not change")
    # D7
    body = cut_function(text, "  function twoPlans(j, landing) {")
    sheet = body + cut_function(text, "  function twoPlansSheet(box, two) {")
    drawn = cut_function(text, "  function twoPlansSheet(box, two) {")
    check(not re.search(r"\btext\b", body), "D7: the body of twoPlans does not name text")
    check(not re.search(r"better|worse|best|recommend|advise|should|per cent|%|points|chance", sheet, re.I) and "Beside your side without this order" not in sheet
          and not any(w in drawn for w in ("className", "classList", ".style", "setAttribute")) and drawn.count("el('td', ''") == 2 and drawn.count("el('th', ''") == 3,
          "D7: the sheet for the two plans says none of better, worse, best, recommend, advise, should, per cent, a per cent sign, points or chance; neither column is given a class or a colour the other is not")
    # D8
    dry = text[text.index("$('p-dry').addEventListener('click'"):]
    dry = dry[:dry.index("\n  });") + 6]
    check("playDry({ preview: previewNo, size: drySize }" in dry and "compare" not in dry, "D8: the order paper's DRY RUN sends trial with the preview and the size and no compare")
    # The engine, on the 39 days
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    n = person.call("preview", command)["ok"]["intent"]
    drafted = person.call("preview", dict(command, under=worked_under(n, h1, h4)))["ok"]
    tried = person.call("trial", {"preview": drafted["preview"], "size": "small"})["ok"]
    b = tried.get("beside") or {}
    # D9
    check(b.get("of") == "left_to_the_staff" and b.get("order") == n and tried["asked"].get("compare") is True and b.get("with_your_plan") == tried["how_it_went"],
          f"D9: trial of the worked draft at the size small is answered with beside of left_to_the_staff, beside.order {n}, asked.compare true and with_your_plan equal to how_it_went ({b.get('of')}, {b.get('order')}, {tried['asked'].get('compare')})")
    real = ask((tried, None))
    two, runs = real["shown"][0], real["runs"][0]
    w = b["world_for_world"]
    counts = [int(c) for c in re.findall(r"in (\d+)", two["run_for_run"])]
    check(two is not None and two["rows"][0][1].startswith(runs[0]) and two["rows"][0][2].startswith(runs[1]) and two["rows"][0][0] == "Island won and held",
          f"D9: each cell of the first row begins with the page's words for the count of met_and_kept of its column ({two['rows'][0]}, {runs})")
    check(counts == [len(w["both"]), len(w["with_your_plan_alone"]), len(w["with_the_staffs_alone"]), len(w["neither"])] and sum(counts) + len(w["apart"]) == tried["asked"]["worlds"],
          f"D9: the four counts run for run are those of world_for_world, and with apart they are the answer's {tried['asked']['worlds']} runs ({counts}, {len(w['apart'])} apart)")
    # D10
    approved = person.call("approve", {"preview": drafted["preview"]})["ok"]
    check(approved.get("status") == "queued", f"D10: the preview tried is not spent: it is approved after the trial ({approved.get('status')})")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    # D11
    with open(os.path.join(HERE, "scenarios.json"), encoding="utf-8") as entries_file:
        entries = json.load(entries_file)["scenarios"]
    holders = [e["id"] for e in entries if "dry_run" in e]
    check(holders == ["guadalcanal-39-days"] and [e for e in entries if e["id"] == "guadalcanal-39-days"][0]["dry_run"] == {"sizes": ["small", "standard"], "seconds": {"standard": 45}},
          f"D11: the entry guadalcanal-39-days has dry_run exactly small and standard with standard at 45 seconds, and no other entry has the key ({holders})")
    # D12
    offered = run_in_node(cut_function(text, "  function sizesOffered(dry) {") + "\nfunction main(x) { return x.map(function (d) { return sizesOffered(d === 'undefined' ? undefined : d); }); }\n",
                          ["undefined", None, {}, {"sizes": []}, {"sizes": ["small", "standard"], "seconds": {"standard": 45}},
                           {"sizes": ["full", "huge", "small"], "seconds": {"small": 2, "full": 1, "standard": 45}}])
    every = {"sizes": ["small", "standard", "full"], "says": []}
    check(offered[4] == {"sizes": ["small", "standard"], "says": ["Standard takes about 45 seconds."]},
          f"D12: the 39 days' value gives small and standard and the line for standard ({offered[4]})")
    check(offered[0] == every and offered[1] == every and offered[2] == every and offered[3] == every,
          f"D12: undefined, null, {{}} and {{sizes: []}} give the three sizes and no line ({offered[:4]})")
    check(offered[5] == {"sizes": ["small", "full"], "says": ["Small takes about 2 seconds.", "Full takes about 1 second."]},
          f"D12: the order is small, standard, full; an unknown size is left out; a time is said only of a size offered ({offered[5]})")
    # D13
    sizes_body = cut_function(text, "  function sizesOffered(dry) {")
    box = cut_function(text, "  function dryBox(doc, root) {")
    outside = text.replace(sizes_body, "")
    check("'small', 'standard', 'full'" in sizes_body and not re.search(r"'standard'|'full'|\"standard\"|\"full\"", outside.replace("value=\"full\"", "").replace("=== 'full'", "").replace("speed: 'full'", "").replace("'full' ?", "")),
          "D13: the three sizes are listed in the body of sizesOffered and nowhere else")
    check("sizesOffered(config.dry_run).sizes" in box.replace("offered.sizes", "sizesOffered(config.dry_run).sizes") and "var offered = sizesOffered(config.dry_run);" in box and "offered.sizes.forEach" in box
          and "offered.says.forEach" in box and "'small', 'standard', 'full'" not in box,
          "D13: dryBox draws its size buttons from sizesOffered(config.dry_run).sizes and a line for each of its says under them")
    play = cut_function(text, "  function playDry(args, says, lock, done) {")
    check("var drySize = 'small'" in text and "offeredNow.indexOf(drySize) < 0" in play and "drySize = offeredNow[0]" in play and "args.size = drySize" in play,
          "D13: drySize begins as small, and a DRY RUN sends the first size offered where the size in hand is not offered")
    check(not re.search(r"guadalcanal|39", sizes_body + box, re.I), "D13: neither sizesOffered nor dryBox names a game")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the 39 days, the person Allied")
    check(get(bridge.table + "/config").get("dry_run") == {"sizes": ["small", "standard"], "seconds": {"standard": 45}},
          "D13: /config of a game of the 39 days has dry_run equal to the entry's")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "eastern-solomons-five-days", "person", "engine") == {"ok": True}, "begun: the Eastern Solomons over five days")
    check("dry_run" not in get(bridge.table + "/config"), "D13: /config of a game of the Eastern Solomons has no dry_run")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_plan_paper_marks_whose_line(bridge):
    """Rule 30: the plan's paper marks every line YOURS or THE STAFF’S (items 23 to 26).

    The page's own functions, in node:
    W1. `whoseLine` of {maker "commander", parent 1} is "YOURS"; of {maker "staff", parent 1} and of {maker "staff", parent 2} it is
        "THE STAFF’S"; of {maker "commander", parent null} it is ""; of {maker "staff", parent 1, text "the commander's take"} it is
        "THE STAFF’S": the mark is the field and never the words.
    W2. The page's function for SIGNED says a queued `withdraw_intent` of a landing of his at Lunga as "The landing at Lunga: given back
        to the staff", and a withdrawal of a root as it says it today.

    In the page's text:
    W3. The plan's paper marks a task from `maker`, of `command_state`'s intent for a plan in force and of `propose`'s entry for a plan
        that waits; the page matches or cuts no text for it: the body of `whoseLine` names `maker` and `parent` and no other field,
        and holds neither "commander's" nor "Yours:" (amended Nov 27, 2026: the page as it stood already held both elsewhere, in
        its words for a stop and for a hold; the readings of R2 are a reason's words and mark no line).
    W4. GIVE IT BACK TO THE STAFF opens the order paper and asks the staff's reading: `preview` with `command` "withdraw_intent" and
        the landing's number (item 25; agent interface 58). Nothing is queued until SIGN AND SEND, which is one `approve` of that
        preview and nothing else: the page calls the tool `withdraw_intent` nowhere. Its DRY RUN sends `trial` with `intent`, the
        landing's number, and never that preview's. THE ORDER says "Give the landing at Lunga back to the staff." (the place by its
        name).

    Through the bridge: the worked draft signed, leave to launch given, one day resolved:
    W5. In `command_state` the intents under the order are intents N + 1 and N + 2 with `maker` "commander" and the rest with `maker`
        "staff"; the page marks N + 1 and N + 2 YOURS, every other task under the order THE STAFF’S, the `ready_landing` beneath a
        landing of his among them, and the order's own line not at all. The tool `order` for N gives each task the same `maker`.
    W6. His landing at Lunga shows its limits: After and Not before, each with CHANGE and BACK TO THE STAFF’S CHOICE; Risk with CHANGE
        alone (no tool unsets a risk: item 24); and its day ("By the end of") with neither. What CHANGE sends for "Not before: day 5"
        (`preview` of a `set_limit` `not_before` on N + 2) is not refused and is approved; what BACK TO THE STAFF’S CHOICE sends for
        its "Not before" is not refused; what CHANGE sends for "Risk: balanced" (`preview` of a `set_limit` `risk` on N + 2) is not
        refused.
    W7. Before W8, nothing being queued: `trial` with `intent` N + 2 at the size small (a size the table offers there: D11) is answered with `beside.of`
        "given_back_to_the_staff" and `beside.order` N, and `twoPlans` of it and "Lunga", the name the page gives from the hex of
        intent N + 2 in `command_state`, says "the landing at Lunga given back to the staff from 1800".
    W8. `preview` of {"command": "withdraw_intent", "intent": N + 2} is valid and its `intent` is N: its tree has a `take_place` at H1
        with `maker` "staff" and none of the commander's there, and the landing at Tulagi, N + 1, still the commander's and `standing`;
        the page's reading of it marks the take at H1 THE STAFF’S and the landing at Tulagi YOURS. `approve` of it answers `status`
        "queued"; `queued` then holds one entry, a `withdraw_intent` of N + 2, which is one line under SIGNED in the words of W2, and
        is taken back by one `take_back`.
    W9. Signed again and a day resolved: no intent N + 2 is in force; a `take_place` at H1 stands under the order with `maker` "staff";
        the page marks it THE STAFF’S, and its line has no GIVE IT BACK TO THE STAFF, no WRITE IT and no CHANGE of a landing's lines.
        (Amended Nov 27, 2026, for pull request C: this held the whole plan's paper to no WRITE IT, which was item 26 for B. A take
        the staff has yet to form, in a plan that waits, is offered it by C; a line of his and a take of the staff's in force are
        still offered none, and the check reads `hisBlock`, `hisLine` and `hisLineRows`, no longer `paperNode`.)
    W10. (New, Nov 27, 2026.) Between W8's `approve` and its `take_back`, the give-back of N + 2 being queued: the line of his
        landing at Lunga, drawn by the page's own functions in node from `command_state` and `queued` as they then stand, is still
        marked YOURS and offers no GIVE IT BACK TO THE STAFF, no CHANGE and no BACK TO THE STAFF’S CHOICE; the line of his landing at
        Tulagi offers them as W6 has it. After the `take_back` Lunga's line offers them again. The page learns that a give-back is
        queued from `queued` alone: an entry whose command is a `withdraw_intent` of that landing's number.
    The game is put away."""
    import re
    print("-- the plan's paper marks whose line: whoseLine in node, the lines of his, the give-back, and the engine's plan before and after", flush=True)
    text = page_text()
    # In the page's text: W3, W4, W9
    body = cut_function(text, "  function whoseLine(t) {")
    fields = set(re.findall(r"\bt\.(\w+)", body))
    check(fields == {"maker", "parent"} and "commander's" not in body and "Yours:" not in body,
          f"W3: the body of whoseLine names maker and parent and no other field, and holds neither \"commander's\" nor \"Yours:\" ({sorted(fields)})")
    check("t.maker = i.maker; t.parent = i.parent;" in cut_function(text, "  function readPlan(") and "by[s.number].maker = s.maker; by[s.number].parent = s.parent;" in cut_function(text, "  function proposedTree(")
          and "var mark = whoseLine(t); if (mark) who.appendChild(el('span', 'whose', mark))" in cut_function(text, "  function paperNode("),
          "W3: the plan's paper marks a task from maker, of command_state's intent for a plan in force and of propose's entry for a plan that waits, and the line says it")
    ask, dry, sign, openg = (cut_function(text, f"  function {h}(") for h in ("askGiveBack", "giveBackDry", "signGiveBack", "openGiveBack"))
    check("api('preview', { command: 'withdraw_intent', intent: g_.number })" in ask and "api('approve', { preview: no })" in sign and sign.count("api('") == 1,
          "W4: GIVE IT BACK TO THE STAFF asks the staff's reading, preview of withdraw_intent with the landing's number, and SIGN AND SEND is one approve of it")
    check(not any(re.search(r"api\('(withdraw_intent|set_intent|set_limit|form_intent)'|act\('withdraw_intent'", f) for f in (ask, dry, sign, openg, cut_function(text, "  function hisBlock("))),
          "W4: the page's give-back calls the tool withdraw_intent nowhere, and sends no set_intent, set_limit or form_intent")
    check("playDry({ intent: g_.number, size: drySize }" in dry and "preview" not in dry and "compare" not in dry,
          "W4: its DRY RUN sends trial with the landing's number, and never the reading's preview")
    check("giveBackWords(hex)" in openg and "paper.kind === 'giveback'" in cut_function(text, "  function askPreview(") and "signGiveBack()" in text[text.index("$('paper').addEventListener('submit'"):][:300],
          "W4: the order paper opens as a give-back whose ORDER is giveBackWords, is asked again where the draft is, and is signed by signGiveBack")
    nodes = "".join(cut_function(text, f"  function {h}(") for h in ("hisBlock", "hisLine", "hisLineRows"))
    check("WRITE IT" not in nodes and "LEAVE IT TO THE STAFF" not in nodes and "LEAVE IT TO THE STAFF" not in cut_function(text, "  function paperNode("),
          "W9: a line of his on the plan's paper holds no WRITE IT, and no line of it LEAVE IT TO THE STAFF, which is the order paper's")
    # The page's own functions in node
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("cleanName", "num", "hexDist", "hexWords", "nameText", "sayIntent", "signedWith", "signedOwnLimits", "signedPlanWords", "signedLine",
                                                                              "whoseLine", "marksOf", "giveBackQueued", "hisLineRows", "hisLine", "giveBackWords", "hisLanding", "landingTried", "twoPlans",
                                                                              "commas", "lowerFirst", "spreadOf", "runsIn"))
    word = re.search(r"^  var WORD = \{.*?\};$", text, re.M).group(0)
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the written plan model ----")]
    code = helpers + word + "\n" + model + text[text.index("  var COSTS ="):text.index("  function spreadOf")] + r"""
const hexes = [], config = { islands: [{ number: 3, name: 'Guadalcanal and Tulagi', base: 1, places: [1, 4] }] }; let hexRaw = false;
function dateOf(d) { return 'day ' + d; } function rich(tag, cls, w) { return el(tag, cls, w); } function placeLink() { return el('span', '', '?'); } function refTo() { return el('span', '', '?'); }
function baseByNumber() { return null; } function plainWords(t) { return t; } function orderCap(n) { return 'Order ' + n + ' words'; } function intentKindOf() { return null; }
function intentWords(n) { return 'take the place at (2,-4)'; }
function main(x) {
  M.hexNames[x.h1[0] + ',' + x.h1[1]] = 'Lunga'; M.hexNames[x.h4[0] + ',' + x.h4[1]] = 'Tulagi';
  const out = {};
  out.w1 = [whoseLine({ maker: 'commander', parent: 1 }), whoseLine({ maker: 'staff', parent: 1 }), whoseLine({ maker: 'staff', parent: 2 }), whoseLine({ maker: 'commander', parent: null }),
            whoseLine({ maker: 'staff', parent: 1, text: "the commander's take" }), whoseLine({ maker: 'commander', parent: 1, text: 'a hold of the staff' })];
  M.state = { intents: [{ number: 5, maker: 'commander', parent: 4, hex: x.h1, kind: 'take_place' }] };
  const root = (c, k) => ({ command: c, place: { queue: 'commands', number: k }, goes_with: [], acts: { at: 'dusk' } });
  out.w2 = [signedLine(root({ kind: 'withdraw_intent', intent: 5 }, 1)).textContent, signedLine(root({ kind: 'withdraw_intent', intent: 4 }, 2)).textContent, signedLine(root({ kind: 'withdraw_intent', intent: 9 }, 3)).textContent];
  out.words = giveBackWords(x.h1);
  if (!x.state) return out;
  M.state = x.state; M.queue = { entries: x.queue }; const st = x.state.intents, by = {}; st.forEach(i => { by[i.number] = i; });
  out.marks = marksOf(st); out.tree = x.tree ? marksOf(x.tree) : null;
  const rows = i => { const h = hisLine(by[i], x.queue, true); return h && { mark: h.mark, giveBack: h.giveBack, held: h.held, rows: h.rows.map(r => [r.key, r.set, r.change, r.back, r.said]) }; };
  out.lunga = rows(x.lunga); out.tulagi = rows(x.tulagi); out.staff = x.staff.map(n => hisLine(by[n], x.queue, true));
  out.cantSign = hisLine(by[x.lunga], x.queue, false);
  out.signed = x.queue.map(e => signedLine(e).textContent);
  // What CHANGE and BACK TO THE STAFF’S CHOICE send for the landing's lines
  const top = by[x.lunga], bodyOf = (key, change, back) => { const l = limFromState(top.limits); if (back) limReset(l, key); if (change) change(l); return limGroupBody(top.kind, l, limGroup(key)); };
  out.sends = { not_before: bodyOf('not_before', l => { l.not_before = 5; }, false), not_before_back: bodyOf('not_before', null, true), after_back: bodyOf('after', null, true), risk: bodyOf('risk', l => { l.risk = 'balanced'; }, false) };
  if (x.trial) { M.queue = { entries: [] }; const land = landingTried(x.trial); out.landing = land; out.two = twoPlans(x.trial, land); }
  return out;
}
"""
    def hex_of(person):
        placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in person.call("list_bases")["ok"]["bases"]}
        return placed["base:1"], placed["base:4"]
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    h1, h4 = hex_of(Person(bridge.table))
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    # W1, W2
    first = run_in_node(code, {"h1": h1, "h4": h4})
    check(first["w1"] == ["YOURS", "THE STAFF’S", "THE STAFF’S", "", "THE STAFF’S", "YOURS"], f"W1: whoseLine is the field and never the words ({first['w1']})")
    check(first["w2"][0] == "The landing at Lunga: given back to the staff" and first["w2"][1] == "Order 4 words: withdrawn" and first["w2"][2] == "Order 9 words: withdrawn",
          f"W2: SIGNED says a withdrawal of a landing of his as given back to the staff, and a root's as it says it today ({first['w2']})")
    check(first["words"] == "Give the landing at Lunga back to the staff.", f"W4: THE ORDER says Give the landing at Lunga back to the staff. ({first['words']})")
    # The engine: the worked draft signed, leave to launch given, one day resolved
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    n = person.call("preview", command)["ok"]["intent"]
    drafted = person.call("preview", dict(command, under=worked_under(n, h1, h4)))["ok"]
    check(person.call("approve", {"preview": drafted["preview"]})["ok"]["status"] == "queued", "the worked draft is signed")
    person.call("set_leave", {"leave": "launch", "given": True})
    person.call("end_day")
    state = person.call("command_state")["ok"]
    tulagi, lunga = n + 1, n + 2
    # W5
    marks = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": None})["marks"]
    mine = {i["number"] for i in state["intents"] if i["parent"] == n and i["maker"] == "commander"}
    under = [i for i in state["intents"] if i["number"] != n]
    check(mine == {tulagi, lunga} and all(i["maker"] == ("commander" if i["number"] in (tulagi, lunga) else "staff") for i in under),
          f"W5: in command_state the intents under the order are {tulagi} and {lunga} with maker commander and the rest staff")
    check(all(marks[str(i["number"])] == ("YOURS" if i["number"] in (tulagi, lunga) else "THE STAFF’S") for i in under) and marks[str(n)] == "",
          f"W5: the page marks {tulagi} and {lunga} YOURS, every other task under the order THE STAFF’S and the order's own line not at all ({sorted(marks.items(), key=lambda kv: int(kv[0]))})")
    ready = [i for i in under if i["kind"] == "ready_landing" and i["parent"] == tulagi]
    check(ready and marks[str(ready[0]["number"])] == "THE STAFF’S", "W5: the ready_landing beneath a landing of his is THE STAFF’S")
    ordered = person.call("order", {"intent": n})["ok"]["tasks"]
    check(ordered and all(t["maker"] == next(i["maker"] for i in state["intents"] if i["number"] == t["intent"]) for t in ordered) and {t["intent"] for t in ordered} >= {tulagi, lunga},
          f"W5: the tool order for {n} gives each task the same maker ({len(ordered)} tasks)")
    # W6
    full = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": None})
    got = {r[0]: r[1:4] for r in full["lunga"]["rows"]}
    check(got.get("not_before") == [True, True, True] and got.get("after") == [True, True, True] and got.get("risk") == [True, True, False] and got.get("by_day") == [True, False, False]
          and full["lunga"]["mark"] == "YOURS" and full["lunga"]["giveBack"] is True,
          f"W6: his landing at Lunga shows After and Not before with CHANGE and BACK, Risk with CHANGE alone and its day with neither ({got})")
    check([r[0] for r in full["lunga"]["rows"]] == ["ships", "land_units", "source", "not_before", "wait_for", "after", "knowledge", "risk", "by_day"]
          and all(r[2] for r in full["lunga"]["rows"] if r[0] != "by_day") and all(not r[3] for r in full["lunga"]["rows"] if r[1] is False),
          "W6: every line of item 14 is shown with CHANGE (the day apart), and BACK TO THE STAFF’S CHOICE only where the line is set")
    tg = {r[0]: r[1:4] for r in full["tulagi"]["rows"]}
    check(tg["ships"] == [True, True, True] and tg["risk"] == [False, True, False] and tg["not_before"] == [False, True, False] and "by_day" not in tg, f"W6: the landing at Tulagi offers its set forces with both and the lines unset with CHANGE ({tg})")
    sends = full["sends"]
    check(sends["not_before"] == {"limit": "not_before", "from_day": 5} and sends["risk"] == {"limit": "risk", "risk": "balanced"} and sends["not_before_back"] == {"limit": "not_before"},
          f"W6: what CHANGE sends for Not before: day 5, for Risk: balanced, and BACK for Not before ({sends['not_before']}, {sends['risk']}, {sends['not_before_back']})")
    for words, body in (("CHANGE of Not before: day 5", sends["not_before"]), ("BACK TO THE STAFF’S CHOICE of Not before", sends["not_before_back"]), ("CHANGE of Risk: balanced", sends["risk"])):
        seen = person.call("preview", dict(body, command="set_limit", intent=lunga))["ok"]
        check(seen.get("status") != "refused" and seen.get("valid") is True, f"W6: what {words} sends is not refused ({seen.get('status')}, {str(seen.get('reason'))[:80]})")
    seen = person.call("preview", dict(sends["not_before"], command="set_limit", intent=lunga))["ok"]
    check(person.call("approve", {"preview": seen["preview"]})["ok"]["status"] == "queued", "W6: what CHANGE sends for Not before: day 5 is approved")
    held = person.call("queued")["ok"]
    check(len(held["entries"]) == 1 and held["entries"][0]["command"]["kind"] == "set_limit" and held["entries"][0]["command"]["intent"] == lunga, "W6: it stands as one set_limit of the landing under SIGNED")
    person.call("take_back", {"queue": held["entries"][0]["place"]["queue"], "number": held["entries"][0]["place"]["number"], "as_of": held["as_of"]})
    check(person.call("queued")["ok"]["entries"] == [], "W6: and one take_back leaves nothing queued")
    # W7
    trial = person.call("trial", {"intent": lunga, "size": "small"})["ok"]
    b = trial.get("beside") or {}
    check(trial.get("status") != "refused" and b.get("of") == "given_back_to_the_staff" and b.get("order") == n,
          f"W7: trial with intent {lunga} at the size small is answered with beside.of given_back_to_the_staff and beside.order {n} ({b.get('of')}, {b.get('order')})")
    said = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": None, "trial": trial})
    check(said["landing"] == "Lunga" and "the landing at Lunga given back to the staff from 1800" in said["two"]["staffs_is"],
          f"W7: twoPlans of it and {said['landing']}, the name the page gives from the hex of intent {lunga}, says the landing at Lunga given back to the staff from 1800 ({said['two']['staffs_is']})")
    # W8
    withdrawal = person.call("preview", {"command": "withdraw_intent", "intent": lunga})["ok"]
    sub = withdrawal["with_change"]["tree"]["sub_intents"]
    at_h1 = [i for i in sub if i["kind"] == "take_place" and i["hex"] == h1]
    tul = [i for i in sub if i["number"] == tulagi]
    check(withdrawal.get("valid") is True and withdrawal["intent"] == n and at_h1 and all(i["maker"] == "staff" for i in at_h1) and tul and tul[0]["maker"] == "commander" and tul[0].get("standing") is True,
          f"W8: the preview of withdraw_intent {lunga} is valid, its intent is {n}, its tree has the staff's take at the hex of Lunga and none of the commander's there, and Tulagi's landing still his and standing")
    seen = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": sub})
    check(seen["tree"][str(at_h1[0]["number"])] == "THE STAFF’S" and seen["tree"][str(tulagi)] == "YOURS",
          f"W8: the page's reading marks the take at the hex of Lunga THE STAFF’S and the landing at Tulagi YOURS ({seen['tree'][str(at_h1[0]['number'])]}, {seen['tree'][str(tulagi)]})")
    approved = person.call("approve", {"preview": withdrawal["preview"]})["ok"]
    held = person.call("queued")["ok"]
    check(approved["status"] == "queued" and len(held["entries"]) == 1 and held["entries"][0]["command"] == {"kind": "withdraw_intent", "intent": lunga} and held["entries"][0]["goes_with"] == [],
          f"W8: approve of it answers queued, and queued holds one entry, a withdraw_intent of {lunga} ({approved.get('status')}, {len(held['entries'])})")
    # W10: while it is queued
    queued = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": held["entries"], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": None})
    check(queued["signed"] == ["The landing at Lunga: given back to the staff"], f"W8: it is one line under SIGNED in the words of W2 ({queued['signed']})")
    lq = queued["lunga"]
    check(lq["mark"] == "YOURS" and lq["held"] is True and lq["giveBack"] is False and not any(r[2] or r[3] for r in lq["rows"]),
          f"W10: with the give-back queued the line of his landing at Lunga is still YOURS and offers no GIVE IT BACK, no CHANGE and no BACK TO THE STAFF’S CHOICE ({lq['mark']}, {lq['giveBack']})")
    tq = queued["tulagi"]
    check(tq["giveBack"] is True and {r[0]: r[1:4] for r in tq["rows"]} == tg, "W10: the line of his landing at Tulagi offers them as W6 has it")
    check(text.count("giveBackQueued(") >= 2 and "e.command.kind === 'withdraw_intent' && e.command.intent === n" in cut_function(text, "  function giveBackQueued("),
          "W10: the page learns that a give-back is queued from the queue alone: an entry whose command is a withdraw_intent of that landing's number")
    taken = person.call("take_back", {"queue": held["entries"][0]["place"]["queue"], "number": held["entries"][0]["place"]["number"], "as_of": held["as_of"]})["ok"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [], "W8: one take_back of it leaves nothing queued")
    after = run_in_node(code, {"h1": h1, "h4": h4, "state": state, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [], "tree": None})["lunga"]
    check(after["giveBack"] is True and {r[0]: r[1:4] for r in after["rows"]} == got, "W10: after the take_back Lunga's line offers them again")
    # W9: signed again, and a day resolved
    again = person.call("preview", {"command": "withdraw_intent", "intent": lunga})["ok"]
    check(person.call("approve", {"preview": again["preview"]})["ok"]["status"] == "queued", "W9: the give-back is signed again")
    person.call("end_day")
    later_ = person.call("command_state")["ok"]
    gone = [i for i in later_["intents"] if i["number"] == lunga and i["in_force"]]
    took = [i for i in later_["intents"] if i["kind"] == "take_place" and i["parent"] == n and i["hex"] == h1 and i["in_force"]]
    check(not gone and took and all(i["maker"] == "staff" for i in took), f"W9: no intent {lunga} is in force; a take_place at the hex of Lunga stands under the order with maker staff ({[i['number'] for i in took]})")
    last = run_in_node(code, {"h1": h1, "h4": h4, "state": later_, "queue": [], "lunga": lunga, "tulagi": tulagi, "staff": [t["number"] for t in took], "tree": None})
    check(all(last["marks"][str(t["number"])] == "THE STAFF’S" for t in took) and all(x is None for x in last["staff"]),
          "W9: the page marks it THE STAFF’S, and its line has no GIVE IT BACK TO THE STAFF and no CHANGE of a landing's lines")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def a_reason_leads_to_his_line(bridge):
    """Rule 31: a reason that turns on a node of his leads to that node's line (items 27 and 28).

    The page's own functions, in node:
    R1. `hisNodesIn(reason, HIS_NODES)` gives, for each pair of `REASONS_AND_HIS_NODES`, the numbers beside it, ascending; and the
        same for each with its `limits` key taken out where that is None.
    R2. (Amended Nov 27, 2026.) The page's pure function `writtenMarks(text, his)`, `his(n)` saying whether intent n is a node of
        his (decided by number, item 27, before any word is read): for each pair of `HIS_LANDING_READ`, with his nodes [3], it gives
        exactly the second of the pair. With no node of his it gives every text back unchanged. It reads these and no other, each
        only where K is a node of his: "intent:K, the commander's, plans the place at (q,r)" is "⟦K⟧ stands"; "intent:K, the
        commander's take of the place at (q,r)," and "intent:K, the commander's take under intent:M," are "⟦K⟧"; "intent:K is the
        commander's take under an island" is "⟦K⟧ is your landing under an island"; any other "intent:K, the commander's," is "⟦K⟧";
        any other "intent:K" is "⟦K⟧"; and, in a text so marked, "is his to take" is "is yours to take" and "carries out his date"
        "carries out your date". Every string of a reason the page shows (its words, what it rests on, what binds, what would change
        it) goes through it before the page's other words for a reason. A reason shown under his own node's line says that node as
        the page says it today ("this task"): there the page gives `his` as false for that one number.
    R2a. The first of those reasons, shown under the order's line with his landing 3 at Lunga: its words are, but for the case of the
        first letter, "your landing at Lunga stands: the staff forms no take beside it.", where "your landing at Lunga" is a link;
        no "intent:", no "the commander" and no ", your," is left in what is shown; its WHY ends with "YOUR PLAN" and one link, "The
        landing at Lunga". The fifth says "your landing at Lunga waits for your landing at Tulagi" (his landing 2 at Tulagi), each
        a link.
    R3. The second, shown under his own landing's line: no YOUR PLAN link to the line it stands under; its limit is under YOUR LIMITS as
        today, and that link leads to the Risk line of his landing.
    R4. The fourth has no YOUR PLAN.

    In the page's text:
    R5. A YOUR PLAN link in a plan's paper opens that plan's paper at his node's line (`readPlan` with the node as its focus); in the
        reading of a draft it opens the fold `p-plan` at that landing.

    Through the bridge: the worked draft signed, leave to launch given, one day resolved:
    R6. Among the reasons `staff_orders` gives, at least one serves the order N and carries the handle of N + 2 in its words, and at
        least one serves N + 2 itself. For every reason given, `hisNodesIn(reason, [N + 1, N + 2])` is what the three tests of item 27
        give when the check works them itself from `serves.intent`, `limits` and the handles in `worked`.
    R7. The page's view of the first kind, in node on the real reason, ends its WHY with YOUR PLAN and the link "The landing at Lunga",
        and shows no handle, no "the commander" and no ", your,": the engine's evidence there is "intent:K, the commander's, is in
        force and its plan is ..." (crates/sim-staff/src/island.rs).
    The game is put away."""
    import copy
    import re
    print("-- a reason that leads to his line: hisNodesIn and writtenMarks in node, the page's view of a reason, and the engine's reasons", flush=True)
    text = page_text()
    # R5, in the page's text
    check("openOrder(n)" in cut_function(text, "  function goToHis(") and "planFocus(hex)" in cut_function(text, "  function goToHis(") and "$('p-plan').open = true" in cut_function(text, "  function planFocus(")
          and "readPlan(found.root, found.task !== found.root ? found.task : null)" in cut_function(text, "  function openOrder(") and "limitJump(limitFocus.kind, focus)" in cut_function(text, "  function readPlan("),
          "R5: a YOUR PLAN link in a plan's paper opens that plan's paper at his node's line (readPlan with the node as its focus); in the reading of a draft it opens the fold p-plan at that landing")
    check("block('YOUR PLAN', planLinks(t.reasons, t.n))" in cut_function(text, "  function whyView(") and "writtenMarks(t," in cut_function(text, "  function markHis(")
          and all("markHis(" in cut_function(text, f"  function {f}(") for f in ("whyView", "paperLine")) and "reasonMarked(" in cut_function(text, "  function workedView(") and "reasonMarked(" in cut_function(text, "  function reasonView("),
          "R2: every string of a reason goes through writtenMarks before the page's other words for a reason, and a WHY ends with YOUR PLAN")
    check(not re.search(r"commander's|the commander|\.test\(|\.exec\(", cut_function(text, "  function hisNodesIn(")) and not re.search(r"commander's", cut_function(text, "  function hisNumbers(")),
          "R1: whose a node is is decided by numbers and `maker`: no word of a reason is read to say it, and not the words \"the commander's\"")
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("hisNodesIn", "writtenMarks"))
    heads = ["orderWords", "plainCounts", "shareWords", "tiredBit", "plainFact", "cautionPart", "cautionWords", "cautionTexts", "cleanName", "num", "hexDist", "hexWords", "nameText", "intentWords", "plainWords",
             "tidyWorked", "paperPre", "paperText", "reasonWords", "cutTop", "oneFact", "threatsOf", "placeSpan", "taskWords", "rich", "plainLinks", "refTo", "nameOf", "hexRef", "hexPlace", "whyView",
             "newTask", "hisNumbers", "hisNodesIn", "writtenMarks", "hisHex", "hisLanding", "hisLink", "planLinks", "goToHis", "draftHis", "landingWords", "landingCap", "limitLinks", "limitSetOn",
             "limitLinkWords", "limitInState", "isDraftNo", "markHis", "paperLine", "orderWordsOf", "orderLink", "orderFound", "commas", "sayIntent", "sameHex", "limitJump"]
    word = re.search(r"^  var WORD = \{.*?\};$", text, re.M).group(0)
    code = "".join(cut_function(text, f"  function {h}(") + "\n" for h in heads) + word + "\n" + re.search(r"  var MENTION = .*\n", text).group(0)
    code += text[text.index("  var LIMIT_LINK ="):text.index("  function limitLinkWords(")] + r"""
Node.prototype.addEventListener = function () {}; document.createElement = function (t) { return new Node(t); };
const hexes = [], config = { islands: [] }; let hexRaw = false, PAPER_TASKS = null, PAPER_ROOT = null, PLACE_HEX = null, paper = null, previewIntent = null, planState = { written: [] };
const paperOpen = {}; function paperArcsDraw() {} function paperArc() { return null; } function planEnd() { return null; } function dateOf() { return ''; }
M.contacts = []; M.bases = []; M.enemyBases = []; M.forces = []; M.units = [];
function walk(n, f) { f(n); (n.kids || []).forEach(k => walk(k, f)); }
// A reason shown under the line of task `own`, as the plan's paper shows it: through whyView, a block at a time.
function view(state, hexNames, own, reason) {
  M.state = state; M.queue = { entries: [] }; M.hexNames = hexNames; Object.keys(hexNames).forEach(k => { hexAt[k] = [0, 0, 0, 0, 0]; });
  const t = newTask(own, 'Order'); t.reasons = [reason]; t.nothing = [reason.worked];
  const w = whyView(t), blocks = [];
  if (!w) return { blocks: [] };
  w.kids.forEach(k => {
    if (k.className === 'wsub') blocks.push({ title: k.textContent, items: [] });
    else if (k.className === 'rpoints' && blocks.length) k.kids.forEach(li => { const links = []; walk(li, n => { if (n.className && /\bref\b/.test(n.className)) links.push(n.textContent); }); blocks[blocks.length - 1].items.push({ text: li.textContent, links: links }); });
  });
  return { blocks: blocks, text: w.textContent };
}
function main(x) {
  const out = {};
  if (x.his_in) out.his_in = x.his_in.map(r => hisNodesIn(r, x.his));
  if (x.marks) out.marks = x.marks.map(p => [writtenMarks(p[0], n => x.mine.indexOf(n) >= 0), writtenMarks(p[0], () => false)]);
  if (x.views) out.views = x.views.map(v => view(x.state, x.hexNames, v.own, v.reason));
  if (x.jump) {
    M.state = x.state; const rows = {}, picked = [], fake = k => ({ k: k, closest: () => null, scrollIntoView() {}, classList: { add() { picked.push(k); } }, querySelector: () => null });
    $('sheet-doc').querySelector = sel => { const m = /data-his="(\d+)"\]\[data-key="(\w+)"/.exec(sel), p = /^\.limrow\[data-key="(\w+)"\]$/.exec(sel); const k = m ? 'his:' + m[1] + ':' + m[2] : p ? 'order:' + p[1] : null; return k && x.jump.rows.indexOf(k) >= 0 ? fake(k) : null; };
    limitJump(x.jump.kind, x.jump.at); out.picked = picked;
  }
  return out;
}
"""
    his_nodes = HIS_NODES
    pairs = REASONS_AND_HIS_NODES
    bare = [{k: v for k, v in r.items() if not (k == "limits" and v is None)} for r, _ in pairs]
    first = run_in_node(code, {"his": his_nodes, "his_in": [r for r, _ in pairs] + bare, "marks": [[a, b] for a, b in HIS_LANDING_READ], "mine": [3]})
    want = [nodes for _, nodes in pairs]
    check(first["his_in"] == want + want, f"R1: hisNodesIn gives, for each reason, the numbers beside it, ascending, and the same with its limits key taken out ({first['his_in']})")
    check([m[0] for m in first["marks"]] == [b for _, b in HIS_LANDING_READ], f"R2: writtenMarks of each of HIS_LANDING_READ's first, with his nodes [3], is exactly its second ({[m[0] for m in first['marks'] if m[0] not in [b for _, b in HIS_LANDING_READ]]})")
    check([m[1] for m in first["marks"]] == [a for a, _ in HIS_LANDING_READ], "R2: with no node of his it gives every text back unchanged")
    # R2a, R3, R4: a reason as the plan's paper shows it
    lunga_hex, tulagi_hex = [2, -4], [3, -5]
    state = {"intents": [{"number": 1, "kind": "control_island", "maker": "commander", "parent": None, "hex": None, "in_force": True, "island": 3},
                         {"number": 2, "kind": "take_place", "maker": "commander", "parent": 1, "hex": tulagi_hex, "in_force": True},
                         {"number": 3, "kind": "take_place", "maker": "commander", "parent": 1, "hex": lunga_hex, "in_force": True},
                         {"number": 4, "kind": "hold", "maker": "staff", "parent": 1, "hex": None, "in_force": True, "base": {"handle": "base:1", "name": "Lunga"}},
                         {"number": 30, "kind": "hold", "maker": "staff", "parent": 1, "hex": None, "in_force": True, "base": {"handle": "base:1", "name": "Lunga"}}]}
    names = {"2,-4": "Lunga", "3,-5": "Tulagi"}
    reasons = [r for r, _ in pairs]
    said = run_in_node(code, {"state": state, "hexNames": names, "views": [{"own": 1, "reason": reasons[0]}, {"own": 3, "reason": reasons[1]}, {"own": 4, "reason": reasons[3]}, {"own": 1, "reason": reasons[4]}]})["views"]
    one, two, four, five = said
    lower = lambda s: s[:1].lower() + s[1:]
    held = [i for b in one["blocks"] if b["title"] == "HOLDS IT BACK" for i in b["items"]]
    check(held and lower(held[0]["text"]) == "your landing at Lunga stands: the staff forms no take beside it." and held[0]["links"] and lower(held[0]["links"][0]) == "your landing at Lunga",
          f"R2a: the first reason under the order's line says your landing at Lunga stands: the staff forms no take beside it., where your landing at Lunga is a link ({held[:1]})")
    check(not re.search(r"intent:|the commander|, your,", one["text"]) and one["blocks"][-1]["title"] == "YOUR PLAN" and [i["links"] for i in one["blocks"][-1]["items"]] == [["The landing at Lunga"]],
          f"R2a: no intent:, no the commander and no , your, is left in what is shown, and its WHY ends with YOUR PLAN and one link, The landing at Lunga ({one['blocks'][-1]})")
    waits = [i for b in five["blocks"] if b["title"] == "HOLDS IT BACK" for i in b["items"]]
    check(waits and lower(waits[0]["text"]) == "your landing at Lunga waits for your landing at Tulagi." and [lower(l) for l in waits[0]["links"]] == ["your landing at Lunga", "your landing at Tulagi"],
          f"R2a: the fifth says your landing at Lunga waits for your landing at Tulagi, each a link ({waits[:1]})")
    check([b["title"] for b in two["blocks"]].count("YOUR PLAN") == 0 and any(b["title"] == "YOUR LIMITS" for b in two["blocks"]),
          f"R3: the second, under his own landing's line, has no YOUR PLAN link to the line it stands under, and its limit is under YOUR LIMITS ({[b['title'] for b in two['blocks']]})")
    jump = run_in_node(code, {"state": state, "jump": {"kind": "risk", "at": 3, "rows": ["order:risk", "his:3:risk"]}})["picked"]
    plain = run_in_node(code, {"state": state, "jump": {"kind": "risk", "at": None, "rows": ["order:risk", "his:3:risk"]}})["picked"]
    check(jump == ["his:3:risk"] and plain == ["order:risk"], f"R3: that limit's link leads to the Risk line of his landing, and a limit of an order itself to the order's ({jump}, {plain})")
    check(not any(b["title"] == "YOUR PLAN" for b in four["blocks"]), f"R4: the fourth has no YOUR PLAN ({[b['title'] for b in four['blocks']]})")
    # Through the bridge: the worked draft signed, leave to launch given, one day resolved
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    n = person.call("preview", command)["ok"]["intent"]
    drafted = person.call("preview", dict(command, under=worked_under(n, h1, h4)))["ok"]
    check(person.call("approve", {"preview": drafted["preview"]})["ok"]["status"] == "queued", "the worked draft is signed")
    person.call("set_leave", {"leave": "launch", "given": True})
    person.call("end_day")
    given = person.call("staff_orders")["ok"]
    every = [o["reason"] for o in given.get("orders", [])] + [c["reason"] for c in given.get("commands", [])] + list(given.get("no_orders", []))
    every = [r for r in every if isinstance(r, dict)]
    tulagi, lunga = n + 1, n + 2
    # R6
    check(any((r.get("serves") or {}).get("intent") == n and f"intent:{lunga}" in str(r.get("worked")) for r in every) and any((r.get("serves") or {}).get("intent") == lunga for r in every),
          f"R6: among the {len(every)} reasons staff_orders gives, one serves the order and carries the handle of intent {lunga} in its words, and one serves intent {lunga} itself")

    def by_the_three_tests(r):
        found = set()
        if (r.get("serves") or {}).get("intent") in (tulagi, lunga):
            found.add(r["serves"]["intent"])
        found |= {l["intent"] for l in (r.get("limits") or []) if l.get("intent") in (tulagi, lunga)}
        found |= {int(k) for k in re.findall(r"\bintent:(\d+)\b", str(r.get("worked"))) if int(k) in (tulagi, lunga)}
        return sorted(found)
    real = run_in_node(code, {"his": [tulagi, lunga], "his_in": every})["his_in"]
    check(real == [by_the_three_tests(r) for r in every], f"R6: for each of the {len(every)} reasons hisNodesIn gives what the three tests of item 27 give when worked from serves.intent, limits and the handles in worked")
    # R7
    state_now = person.call("command_state")["ok"]
    first_kind = [r for r in every if (r.get("serves") or {}).get("intent") == n and f"intent:{lunga}" in str(r.get("worked"))][0]
    seen = run_in_node(code, {"state": state_now, "hexNames": {f"{h1[0]},{h1[1]}": "Lunga", f"{h4[0]},{h4[1]}": "Tulagi"}, "views": [{"own": n, "reason": first_kind}]})["views"][0]
    check(seen["blocks"] and seen["blocks"][-1]["title"] == "YOUR PLAN" and [i["links"] for i in seen["blocks"][-1]["items"]] == [["The landing at Lunga"]],
          f"R7: the page's view of the real reason ends its WHY with YOUR PLAN and the link The landing at Lunga ({seen['blocks'][-1:] })")
    check(not re.search(r"intent:\d|the commander|, your,", seen["text"]), f"R7: and shows no handle, no the commander and no , your, ({seen['text'][:200]})")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


# ---- Decision 0232, step 3, pull request C: a landing written under an order already in force ("Planned: step 3, C" of
# docs/decisions/as-built/0232-a-plan-the-commander-writes.md, items C1 to C12). Written by the architect before any code of C, on main
# at 100a748c and agent interface 58. The function below is empty and fails "not built"; it is not called from main() until C's first
# commit adds its call, after `a_reason_leads_to_his_line(bridge)`. A developer fills its body to the statements in its doc comment and
# changes no word of them, of the two functions here, or of B's five functions and their fixtures: a statement that cannot be filled
# as written comes back to the architect.
#
# THE WORKED LANDINGS, each written alone under the order N in force (seize and hold island 3, signed alone and a day resolved):
#   the landing at Tulagi: Ships and forces: use only the task force numbered 10. Not before: day 5.
#   the landing at Lunga:  After: the landing at Tulagi (numbered K). Risk: preserve. By the end of: day 9.


def worked_tulagi_alone(n, h4):
    """What the worked landing at Tulagi sends to `preview`, under an order numbered `n` (item C7). The architect's: not to be changed."""
    return {"command": "set_intent", "parent": n, "intent_kind": "take_place", "hex": list(h4),
            "limits": [{"limit": "forces", "task_forces": [10], "ships": []}, {"limit": "not_before", "from_day": 5}]}


def worked_lunga_alone(n, k, h1):
    """What the worked landing at Lunga sends, his landing at Tulagi being numbered `k` (item C7). The architect's: not to be changed."""
    return {"command": "set_intent", "parent": n, "intent_kind": "take_place", "hex": list(h1), "by_day": 9,
            "limits": [{"limit": "after", "other_intent": k}, {"limit": "risk", "risk": "preserve"}]}


def a_landing_written_under_an_order_in_force(bridge):
    """Rule 22 on the table: a landing written under an order already in force, from the plan's paper (items C1 to C12).

    Lunga is at (2,-4) and Tulagi at (3,-5) in the node checks; through the bridge H1 and H4 are read from `list_bases` as B's checks
    read them. "The fixture" is this tree, as `propose` gives one for the order numbered 1 whose plan waits:
        sub_intents: {2, take_place, hex (3,-5), parent 1, maker "commander", standing true};
                     {3, take_place, hex (2,-4), parent 1, maker "staff"}; {4, hold, parent 1, maker "staff"};
                     {12, ready_landing, hex (3,-5), parent 2, maker "staff"}; {13, ready_landing, hex (2,-4), parent 3, maker "staff"};
                     {14, take_place, hex (9,9), parent 3, maker "staff"}.

    In the page's text:
    U1. The page holds the functions `writeLines(`, `writtenAlone(`, `hisOtherLandings(` and `readingOfLanding(`, and a paper of the
        kind 'landing'. WRITE IT is put on a line of the plan's paper only for a line `writeLines` gives of the tree `propose` gave
        (a plan that waits), and only where the seat may sign (`canSign()`): never for a plan drawn from the command state. The
        bodies of `hisBlock`, `hisLine` and `hisLineRows` hold no "WRITE IT".
    U2. The paper's reading is `preview` with the body `writtenAlone` gives and nothing added. Its SIGN AND SEND is one `approve` of
        that preview, and the function that signs makes no other call. Its DRY RUN sends `trial` with `preview` and `size` and holds
        no `compare`, `dates` or `intent`. The functions of this paper call the tools `set_intent`, `set_limit`, `form_intent` and
        `withdraw_intent` nowhere.

    The page's own functions, cut from its text and run in node:
    U3. `writeLines(tree, order)` of the fixture and 1 gives exactly one line, the hex (2,-4): the standing landing of his gives none,
        nor does an entry that is not a `take_place`, nor a `take_place` whose parent is not the order (14). With entry 2 made the
        staff's and not standing it gives (3,-5) and then (2,-4), in the tree's order. Of null, of a tree with no `sub_intents` and
        of a tree with no such entry it gives an empty list.
    U4. `writtenAlone(1, landing)`, the landing being the model's own (`newLanding`) set as the worked landing at Tulagi, is exactly
        `worked_tulagi_alone(1, (3, -5))`; set as the worked landing at Lunga with its After the number 2, exactly
        `worked_lunga_alone(1, 2, (2, -4))`. Neither carries `intent`. A landing with nothing set sends `limits` empty and no
        `by_day`. For every landing, what it gives is `writtenUnder(1, [landing])[0]` less its `intent`, with `command` and
        `parent`: a risk never set sends none, balanced once set is sent, and no `none_of`, single ship, air group, area, target
        or protected ship is ever sent (P5 and P6 hold of this paper).
    U5. `hisOtherLandings(order, hex, intents, entries)`, for the order 1 and the paper's hex (2,-4), with `intents`
        [{1, control_island, commander, parent null, in force}, {2, take_place, commander, parent 1, hex (3,-5), in force},
         {5, take_place, commander, parent 1, hex (2,-4), in force}, {6, take_place, commander, parent 1, hex (8,8), not in force},
         {7, take_place, staff, parent 1, hex (7,7), in force}, {8, take_place, commander, parent null, hex (6,6), in force},
         {9, take_place, commander, parent 20, hex (4,4), in force}]
        and `entries` holding one queued `set_intent` {intent 12, take_place, parent 1, hex (5,-5)}, one {intent 13, take_place,
        parent 20, hex (4,5)} and one `set_limit` of intent 12, gives exactly [{number 2, hex (3,-5)}, {number 12, hex (5,-5)}]:
        his landings under this order, in force or queued, by number, and not the one at the paper's own place, one not in force,
        the staff's, a root, or one under another order.
    U6. The words. THE ORDER's text for a landing at Tulagi under an order the page says as "your order to seize and hold
        Guadalcanal and Tulagi" is exactly "Write the landing at Tulagi under your order to seize and hold Guadalcanal and
        Tulagi." `refusalWords("intent:2 already plans the place at (3,-5)")` is exactly "It is already planned.", and
        `refusalWords("the limit is wider than what its parent intent:1 allows")` is still "It is wider than the order above it
        allows." A refusal at `of_batch.position` 1 is said as the reason alone; at position 4 of the worked landing at Tulagi
        with Risk press, exactly "Risk: It is wider than the order above it allows."; at position 2 of the same, "Ships and
        forces: " and the reason. The status line once signed is exactly "Signed: the landing at Tulagi is yours. The staff takes
        it up at 1800."
    U7. The page's function for SIGNED, given the queue of U14 as the engine gives it for an order numbered 1 (the `set_intent` with
        `parent` 1 at (3,-5), which `goes_with` its two `set_limit`), the order said as in U6 and task force 10 named "Transport
        Group Yoke", begins "The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi · Ships and
        forces: Transport Group Yoke · Not before: " and then the day as the page says a day; with no limit queued it is exactly
        "The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi". It holds no "Order:" and no
        "Capture". A queued `set_intent` with no parent is said as today, and `signedOrders` still gives no entry with a parent.
    U8. `readingOfLanding(p, known)`, for p with `intent` 3, `command` {hex (2,-4), parent 1} and the tree
        [{4, hold, parent 1, staff}, {12, ready_landing, parent 2, staff}, {13, ready_landing, parent 3, staff}], and `known`
        [{1, control_island, commander, parent null}, {2, take_place, commander, parent 1, hex (3,-5)},
         {7, take_place, staff, parent 1, hex (7,7)}, {9, take_place, commander, parent 20, hex (4,4)}],
        gives five entries numbered 3, 2, 4, 12, 13 in that order: the first {number 3, kind "take_place", hex (2,-4), parent 1,
        maker "commander"}; the second his landing 2 with `standing` true; the rest the tree's own, unchanged. `marksOf` of it is
        YOURS for 3 and 2 and THE STAFF’S for 4, 12 and 13. With `known` empty it gives 3, 4, 12, 13. Where the tree already lists
        2 it is not added again. For p with no tree it gives null.
    U9. `landingTried(j, hex)`, for an answer j with `asked.tried_intent` 9 and `beside.order` 1, intent 9 being in neither
        `command_state` nor `queued`: with the hex (3,-5) it gives "Tulagi"; with no second argument "that place", as today; where
        `asked.tried_intent` is `beside.order` it gives null whatever the hex. `twoPlans(j, "Tulagi")` of that answer with
        `beside.of` "left_to_the_staff" says exactly "The staff’s plan is this order with the landing at Tulagi left to the staff."
    U10. YOUR LIMITS (item C12), by the view of B's check of a reason (`a_reason_leads_to_his_line`, its state and names): the
        second of `REASONS_AND_HIS_NODES`, shown under his own landing's line (3), lists its limit as exactly "its risk", one link
        and no " on "; the fifth, shown under the order's line (1), lists exactly "the forces it may use on your landing at
        Tulagi", whose links are "the forces it may use" and "your landing at Tulagi"; no item of either holds "your order to".
        The fourth, a limit of the order itself shown under a task of the staff's, is said exactly as the page on main says it.
    U11. The reading of the paper in hand (item C8): with the paper writing the landing at Tulagi and its reading numbered 2, the
        line "intent:2, the commander's, plans the place at (3,-5): the staff forms no take beside it." is shown, but for the case
        of its first letter, as "your landing at Tulagi stands: the staff forms no take beside it.", where "your landing at
        Tulagi" is a link; no "intent:", no "the commander" and no ", your," is left; and its WHY has no YOUR PLAN link to the
        landing whose paper it is.

    Through the bridge: the 39 days, the person Allied against the engine; `preview` of {"command": "set_intent", "intent_kind":
    "control_island", "island": 3} alone gives the order its number N and is approved; one day is resolved, leave to launch being
    as the game gives it:
    U12. In `command_state` the order N is in force, leave to launch is not given, its plan is preparing or a draft and no intent
        stands under it. `propose` for N gives a `take_place` at H1 and at H4, each with `maker` "staff" and not `standing`, and
        `writeLines` of that tree and N gives two lines, at H1 and H4.
    U13. `preview` of `worked_tulagi_alone(N, H4)` is valid: its `command` has `parent` N and `hex` H4, its `intent` is a number K
        above N, its `under` is empty. Its tree lists no entry numbered K, no `take_place` at H4, a `take_place` at H1 with `maker`
        "staff" and `parent` N, and a `ready_landing` whose `parent` is K; among its `no_orders` is a line that begins "intent:K,
        the commander's, plans the place at". `marksOf(readingOfLanding(p, known))`, `known` from that `command_state` and
        `queued`, is YOURS for K and THE STAFF’S for every other entry.
    U14. The same with {"limit": "risk", "risk": "press"} added to its limits is refused with `of_batch` {position 4, size 4}, the
        engine's reason "the limit is wider than what its parent intent:N allows".
    U15. `trial` of U13's preview at the size small, with no `compare`, is answered with `asked.second_trial` true,
        `asked.tried_intent` K, `beside.of` "left_to_the_staff" and `beside.order` N; `twoPlans` of it and "Tulagi", the name the
        page gives from the paper's own hex, says "the landing at Tulagi left to the staff". `trial` of the same preview with
        `dates` "chosen" is refused in words that hold 'A written landing is held to a day by its "not before"'. The preview is
        still approved after it.
    U16. `approve` of U13's preview answers `status` "queued" and `limits_queued` 2. `queued` then holds three entries: a
        `set_intent` with `parent` N and `hex` H4 whose `goes_with` names the other two, a `set_limit` `forces` and a `set_limit`
        `not_before` of K. The page's function for SIGNED says the first in the words of U7 and `signedOrders` of that queue is
        empty. `propose` for N now gives the `take_place` at H4 as number K, `maker` "commander", `standing`, and the staff's at H1:
        `writeLines` gives H1 alone. `preview` of `worked_tulagi_alone(N, H4)` again is refused, the engine's reason "intent:K
        already plans the place at " and H4 as the engine writes a hex, which the page says "It is already planned."
    U17. One `take_back` of the first entry is not refused and leaves nothing queued; `propose` gives both takes as the staff's again
        and `writeLines` both lines.
    U18. Previewed and approved again, and a day resolved: intent K is in force under N with `maker` "commander", its plan a draft
        or preparing while the order's plan still waits; `hisLine` of it, from `command_state` with nothing queued and the seat
        able to sign, is marked YOURS, offers GIVE IT BACK TO THE STAFF, and shows "Ships and forces" and "Not before" each with
        CHANGE and BACK TO THE STAFF’S CHOICE and every other line but the day with CHANGE (item C5). `hisOtherLandings` for N and
        H1 gives K at H4 alone. `preview` of `worked_lunga_alone(N, K, H1)` is valid, with `parent` N and a number above K;
        nothing is approved. `preview` of {"command": "withdraw_intent", "intent": K} is valid with `intent` N.
    U19. Leave to launch given and a day resolved: a `take_place` at H1 with `maker` "staff" is in force under N and the order's plan
        is released; `preview` of `worked_lunga_alone(N, K, H1)` is now refused, the engine's reason beginning "intent:" and
        ending " already plans the place at " and H1; the page says "It is already planned." The page's paper for N is then
        drawn from the command state, where no line is offered WRITE IT (U1).
    The game is put away."""
    import re
    print("-- a landing written under an order in force: the page's functions in node, its text, and the engine taking what it sends", flush=True)
    text = page_text()
    tool_calls = r"api\('(set_intent|set_limit|form_intent|withdraw_intent)'|act\('(set_intent|set_limit|form_intent|withdraw_intent)'"
    # U1, U2: in the page's text
    check(all(f"function {f}(" in text for f in ("writeLines", "writtenAlone", "hisOtherLandings", "readingOfLanding")) and "openPaper('landing'" in text and "paper.kind === 'landing'" in text,
          "U1: the page holds writeLines, writtenAlone, hisOtherLandings and readingOfLanding, and a paper of the kind 'landing'")
    plan = cut_function(text, "  function readPlan(")
    waits = plan[plan.index("if (proposed) {"):plan.index("PAPER_TASKS = {};\n        var make")]
    nodes = "".join(cut_function(text, f"  function {h}(") for h in ("hisBlock", "hisLine", "hisLineRows"))
    check("writeLines(pr.tree, root)" in waits and "canSign()" in waits and "t.write = {" in waits and plan.count("t.write = {") == 1 and "WRITE IT" not in nodes
          and "t.write && !bare" in cut_function(text, "  function paperNode(") and "WRITE IT" in cut_function(text, "  function writeBlock(") and "WRITE IT" not in cut_function(text, "  function paperNode("),
          "U1: WRITE IT is put on a line only for a line writeLines gives of the tree propose gave (a plan that waits), only where canSign(), and the bodies of hisBlock, hisLine and hisLineRows hold none")
    ask, dry, sign = (cut_function(text, f"  function {h}(") for h in ("askLanding", "landingDry", "signLanding"))
    check("body = writtenAlone(c.order, c.l)" in ask and ask.count("api('preview', body)") == 1 and ask.count("api(") == 1, "U2: the paper's reading is preview with the body writtenAlone gives, and nothing added")
    check(sign.count("api('approve', { preview: no })") == 1 and sign.count("api(") == 1 and "askLanding" in cut_function(text, "  function askPreview(") and "signLanding()" in text[text.index("$('paper').addEventListener('submit'"):][:400],
          "U2: SIGN AND SEND is one approve of that preview, and the function that signs makes no other call")
    check("playDry({ preview: previewNo, size: drySize }" in dry and not any(w in dry for w in ("compare", "dates", "intent:", "intent =", "{ intent")) and "landingDry()" in text[text.index("$('p-dry').addEventListener('click'"):][:400],
          "U2: its DRY RUN sends trial with preview and size, and holds no compare, dates or intent")
    own = "".join(cut_function(text, f"  function {h}(") for h in ("writeBlock", "openLanding", "landingHead", "drawLandingPaper", "landingFocus", "knownLandings", "askLanding", "landingDry", "signLanding"))
    check(not re.search(tool_calls, own), "U2: the functions of this paper call the tools set_intent, set_limit, form_intent and withdraw_intent nowhere")
    # The page's own functions, in node
    helpers = "".join(cut_function(text, f"  function {h}(") + "\n" for h in ("cleanName", "num", "hexDist", "hexWords", "nm", "unitNo", "nameText", "sayIntent", "signedWith", "signedOwnLimits", "signedPlanWords", "signedLine",
                                                                              "signedOrders", "whoseLine", "marksOf", "landingTried", "twoPlans", "commas", "lowerFirst", "spreadOf", "runsIn", "orderWordsOf", "orderFound",
                                                                              "intentWords", "knownLandings", "hisLineRows", "hisLine", "giveBackQueued"))
    word = re.search(r"^  var WORD = \{.*?\};$", text, re.M).group(0)
    model = text[text.index("  var LIMITS_OFFERED"):text.index("  // ---- end of the written plan model ----")]
    refusal = (text[text.index("  function cleanName"):text.index("  function sayIntent")] + text[text.index("  function orderWords"):text.index("  function nameOf(")]
               + cut_function(text, "  function commas(n)") + "\n" + text[text.index("  var REFUSALS = ["):text.index("  // What the status line says of orders taken in")])
    code = helpers + word + "\n" + model + refusal + text[text.index("  var COSTS ="):text.index("  function spreadOf")] + r"""
const hexes = [], config = { islands: [{ number: 3, name: 'Guadalcanal and Tulagi', base: 1, places: [1, 4] }] }; let hexRaw = false, PAPER_TASKS = null, paper = null;
function dateOf(d) { return 'AUG ' + (d + 1); } function rich(tag, cls, w) { return el(tag, cls, w); } function placeLink() { return el('span', '', '?'); } function refTo() { return el('span', '', '?'); }
function baseByNumber() { return null; } function plainWords(t) { return t; }
function tulagi(h) { const l = newLanding(h); l.lim.ships = { mode: 'only', task_forces: [10], ships: [] }; l.lim.not_before = 5; return l; }
function lunga(h, after) { const l = newLanding(h); l.lim.after = after; l.lim.risk = 'preserve'; l.by_day = 9; return l; }
function main(x) {
  const out = {}; M.hexNames[x.h1[0] + ',' + x.h1[1]] = 'Lunga'; M.hexNames[x.h4[0] + ',' + x.h4[1]] = 'Tulagi'; M.names['task_force:10'] = 'Transport Group Yoke';
  M.state = x.state || { intents: [] }; M.queue = { entries: x.queue || [] };
  if (x.pure) {
    const fx = x.pure.fixture, clone = o => JSON.parse(JSON.stringify(o));
    // U3
    const staffs = clone(fx); staffs.sub_intents[0].maker = 'staff'; delete staffs.sub_intents[0].standing;
    out.u3 = { fixture: writeLines(fx, 1), staffs: writeLines(staffs, 1), nulls: [writeLines(null, 1), writeLines(undefined, 1), writeLines({}, 1), writeLines({ sub_intents: [fx.sub_intents[0], fx.sub_intents[2]] }, 1)] };
    // U4
    const odd = newLanding(x.h4); odd.by_day = 5; odd.lim.ships = { mode: 'none', task_forces: [], ships: [7] }; odd.lim.land_units = { mode: 'none', list: [4] }; odd.lim.air_groups = { mode: 'only', list: [3] };
    odd.lim.submarines = { mode: 'only', list: [1] }; odd.lim.area = { q: 0, r: 0, dist: 2 }; odd.lim.targets = { kinds: ['carrier'], waters_first: true }; odd.lim.protected = [7]; odd.lim.in_place = true;
    const both = newLanding(x.h4); both.lim.ships = { mode: 'only', task_forces: [10], ships: [7] }; both.lim.land_units = { mode: 'only', list: [4] }; both.lim.source = 1;
    const bal = tulagi(x.h4); bal.lim.risk = 'balanced';
    const set = [tulagi(x.h4), lunga(x.h1, 2), newLanding(x.h1), odd, both, bal, tulagi(x.h4)];
    out.u4 = { tulagi: writtenAlone(1, set[0]), lunga: writtenAlone(1, set[1]), bare: writtenAlone(1, set[2]), odd: writtenAlone(1, set[3]), both: writtenAlone(1, set[4]), balanced: writtenAlone(1, set[5]),
               everyone: set.map(l => { const u = writtenUnder(1, [l])[0]; delete u.intent; return [writtenAlone(1, l), Object.assign({ command: 'set_intent', parent: 1 }, u)]; }) };
    // U5
    out.u5 = hisOtherLandings(1, x.h1, x.pure.u5.intents, x.pure.u5.entries);
    // U6
    const press = tulagi(x.h4); press.lim.risk = 'press'; const why = refusalWords('the limit is wider than what its parent intent:1 allows');
    out.u6 = { planned: refusalWords("intent:2 already plans the place at (3,-5)"), wider: why, said: orderWordsOf(1, orderFound(1)), refused: [1, 2, 4].map(k => refusedAlone(press, k, why)), signed: signedAloneWords(x.h4),
               write: writeWords(x.h4, 'your order to seize and hold Guadalcanal and Tulagi') };
    // U8
    const p = x.pure.u8.p, known = x.pure.u8.known, listed = clone(p); listed.with_change.tree.sub_intents.push({ number: 2, kind: 'take_place', hex: [3, -5], parent: 1, maker: 'commander', standing: true });
    const r = readingOfLanding(p, known), bare = readingOfLanding(p, []), twice = readingOfLanding(listed, known);
    out.u8 = { r: r, marks: marksOf(r), bare: bare && bare.map(e => e.number), twice: twice && twice.map(e => e.number), none: [readingOfLanding({ intent: 3, command: p.command, with_change: {} }, known), readingOfLanding({ intent: 3, command: p.command }, known), readingOfLanding(null, known)] };
    // U9
    M.state = { intents: [] }; M.queue = { entries: [] }; const j = x.pure.u9.j, same = clone(j); same.asked.tried_intent = 1;
    out.u9 = { named: landingTried(j, x.h4), bare: landingTried(j), same: [landingTried(same, x.h4), landingTried(same)], two: twoPlans(j, landingTried(j, x.h4)).staffs_is };
  }
  if (x.signed) {
    M.state = x.state; const q = x.signed.queue; M.queue = { entries: q }; const first = q.filter(e => e.command.kind === 'set_intent' && e.command.parent != null)[0];
    out.signed = { line: signedLine(first).textContent, orders: signedOrders().length, order_line: x.signed.order ? signedLine(q.filter(e => e.command.kind === 'set_intent' && e.command.parent == null)[0]).textContent : null };
    M.queue = { entries: [first] }; out.signed.bare = signedLine(Object.assign({}, first, { goes_with: [] })).textContent;
  }
  if (x.lines) out.lines = writeLines(x.lines.tree, x.lines.order).map(l => l.hex);
  if (x.reading) {
    const rd = readingOfLanding(x.reading.p, knownLandings()); out.reading = { marks: marksOf(rd), numbers: rd.map(e => e.number) };
  }
  if (x.his) {
    const by = {}; x.state.intents.forEach(i => { by[i.number] = i; }); const h = hisLine(by[x.his.k], x.queue || [], true);
    out.his = h && { mark: h.mark, giveBack: h.giveBack, rows: h.rows.map(r => [r.key, r.set, r.change, r.back]) };
    out.others = hisOtherLandings(x.his.n, x.his.hex, x.state.intents, x.queue || []);
  }
  if (x.trial) { const ct = landingTried(x.trial, x.h4); out.trial = { land: ct, two: twoPlans(x.trial, ct).staffs_is }; }
  return out;
}
"""
    # The engine's names for the places, and the architect's fixtures
    check(begin(bridge, "guadalcanal-39-days", "engine", "person") == {"ok": True}, "begun: the person Japanese, to read the hexes of Lunga and Tulagi")
    placed = {b["base"]["handle"]: [b["place"]["q"], b["place"]["r"]] for b in Person(bridge.table).call("list_bases")["ok"]["bases"]}
    h1, h4 = placed["base:1"], placed["base:4"]
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    sub = lambda number, kind, parent, hex_=None, maker="staff", **more: dict({"number": number, "kind": kind, "parent": parent, "hex": hex_, "maker": maker}, **more)
    fixture = {"sub_intents": [sub(2, "take_place", 1, [3, -5], "commander", standing=True), sub(3, "take_place", 1, [2, -4]), sub(4, "hold", 1), sub(12, "ready_landing", 2, [3, -5]),
                               sub(13, "ready_landing", 3, [2, -4]), sub(14, "take_place", 3, [9, 9])]}
    intents5 = [{"number": 1, "kind": "control_island", "maker": "commander", "parent": None, "hex": None, "in_force": True},
                {"number": 2, "kind": "take_place", "maker": "commander", "parent": 1, "hex": [3, -5], "in_force": True},
                {"number": 5, "kind": "take_place", "maker": "commander", "parent": 1, "hex": [2, -4], "in_force": True},
                {"number": 6, "kind": "take_place", "maker": "commander", "parent": 1, "hex": [8, 8], "in_force": False},
                {"number": 7, "kind": "take_place", "maker": "staff", "parent": 1, "hex": [7, 7], "in_force": True},
                {"number": 8, "kind": "take_place", "maker": "commander", "parent": None, "hex": [6, 6], "in_force": True},
                {"number": 9, "kind": "take_place", "maker": "commander", "parent": 20, "hex": [4, 4], "in_force": True}]
    entries5 = [{"command": {"kind": "set_intent", "intent": 12, "intent_kind": "take_place", "parent": 1, "hex": [5, -5]}}, {"command": {"kind": "set_intent", "intent": 13, "intent_kind": "take_place", "parent": 20, "hex": [4, 5]}},
                {"command": {"kind": "set_limit", "intent": 12, "limit": "risk", "risk": "preserve"}}]
    p8 = {"intent": 3, "command": {"hex": [2, -4], "parent": 1}, "with_change": {"tree": {"sub_intents": [sub(4, "hold", 1), sub(12, "ready_landing", 2), sub(13, "ready_landing", 3)]}}}
    known8 = [sub(1, "control_island", None, None, "commander"), sub(2, "take_place", 1, [3, -5], "commander"), sub(7, "take_place", 1, [7, 7], "staff"), sub(9, "take_place", 20, [4, 4], "commander")]
    spread = {"lowest": 0, "middle": 0, "highest": 0}
    world = lambda: {"worlds": [1, 2], "met_and_kept": [], "met_and_lost_again": [], "not_met": [1, 2], "met_days": None}
    j9 = {"asked": {"tried_intent": 9}, "last_day": 8, "cost": {}, "enemy_lost": {}, "beside": {"of": "left_to_the_staff", "order": 1, "with_your_plan": world(), "how_it_went": world(), "cost": {}, "enemy_lost": {},
                                                                                              "world_for_world": {"apart": [], "both": [], "neither": [1, 2], "with_the_staffs_alone": [], "with_your_plan_alone": []}}}
    state7 = {"intents": [{"number": 1, "kind": "control_island", "island": 3, "maker": "commander", "parent": None, "hex": None, "in_force": True}]}
    queue7 = [{"command": {"kind": "set_intent", "intent": 2, "intent_kind": "take_place", "parent": 1, "hex": h4}, "place": {"queue": "commands", "number": 1}, "goes_with": [{"queue": "commands", "number": 2}, {"queue": "commands", "number": 3}], "acts": {"at": "dusk"}},
              {"command": {"kind": "set_limit", "intent": 2, "limit": "forces", "task_forces": [10], "ships": []}, "place": {"queue": "commands", "number": 2}, "goes_with": [], "acts": {"at": "dusk"}},
              {"command": {"kind": "set_limit", "intent": 2, "limit": "not_before", "from_day": 5}, "place": {"queue": "commands", "number": 3}, "goes_with": [], "acts": {"at": "dusk"}}]
    pure = run_in_node(code, {"h1": h1, "h4": h4, "state": state7, "queue": queue7, "pure": {"fixture": fixture, "u5": {"intents": intents5, "entries": entries5}, "u8": {"p": p8, "known": known8}, "u9": {"j": j9}}, "signed": {"queue": queue7}})
    # U3
    u3 = pure["u3"]
    check(u3["fixture"] == [{"hex": [2, -4], "number": 3}] and u3["staffs"] == [{"hex": [3, -5], "number": 2}, {"hex": [2, -4], "number": 3}] and u3["nulls"] == [[], [], [], []],
          f"U3: writeLines gives the one line at (2,-4) of the fixture (not the standing landing of his, a hold, or a take under another task), both once entry 2 is the staff's, and none of no tree ({u3})")
    # U4
    u4 = pure["u4"]
    check(u4["tulagi"] == worked_tulagi_alone(1, (3, -5)) and u4["lunga"] == worked_lunga_alone(1, 2, (2, -4)) and all("intent" not in u4[k] for k in ("tulagi", "lunga", "bare")),
          f"U4: writtenAlone of the worked landings is exactly the architect's bodies, and carries no intent ({u4['tulagi']}, {u4['lunga']})")
    check(u4["bare"] == {"command": "set_intent", "parent": 1, "intent_kind": "take_place", "hex": [2, -4], "limits": []} and "by_day" not in u4["bare"],
          f"U4: a landing with nothing set sends limits empty and no by_day ({u4['bare']})")
    check(all(a == b for a, b in u4["everyone"]) and not any(k in json.dumps([u4["odd"]]) for k in ("none_of", "air_groups", "submarines", "area", "targets", "protected", "in_place")) and u4["odd"]["limits"] == []
          and u4["both"]["limits"] == [{"limit": "forces", "task_forces": [10], "ships": [], "source": 1, "land_units": [4]}] and u4["balanced"]["limits"][-1] == {"limit": "risk", "risk": "balanced"}
          and not any(l["limit"] == "risk" for l in u4["tulagi"]["limits"]),
          "U4: for every landing writtenAlone is writtenUnder less its intent, with command and parent: a risk never set sends none, balanced once set is sent, and no none_of, single ship, air group, area, target or protected ship is sent")
    # U5
    check(pure["u5"] == [{"number": 2, "hex": [3, -5]}, {"number": 12, "hex": [5, -5]}], f"U5: hisOtherLandings gives his landings under the order, in force or queued, by number, and no other ({pure['u5']})")
    # U6
    u6 = pure["u6"]
    check(u6["said"] == "your order to seize and hold Guadalcanal and Tulagi" and u6["write"] == "Write the landing at Tulagi under your order to seize and hold Guadalcanal and Tulagi."
          and "writeWords(hex, said)" in cut_function(text, "  function openLanding(") and "orderLink(order)" in cut_function(text, "  function openLanding("),
          f"U6: THE ORDER's text is exactly Write the landing at Tulagi under your order to seize and hold Guadalcanal and Tulagi. ({u6['write']})")
    check(u6["planned"] == "It is already planned." and u6["wider"] == "It is wider than the order above it allows.", f"U6: refusalWords says It is already planned. and, as ever, that a limit is wider than the order above it allows ({u6['planned']}, {u6['wider']})")
    check(u6["refused"][0]["words"] == u6["wider"] and u6["refused"][1]["words"] == "Ships and forces: " + u6["wider"] and u6["refused"][2]["words"] == "Risk: It is wider than the order above it allows." and u6["refused"][2]["key"] == "risk",
          f"U6: a refusal at position 1 is said as the reason alone, at position 2 as Ships and forces: and the reason, at position 4 of the Tulagi landing with Risk press exactly Risk: It is wider than the order above it allows. ({u6['refused']})")
    check(u6["signed"] == "Signed: the landing at Tulagi is yours. The staff takes it up at 1800." and "signedAloneWords(c.hex)" in cut_function(text, "  function signLanding("),
          f"U6: the status line once signed is exactly Signed: the landing at Tulagi is yours. The staff takes it up at 1800. ({u6['signed']})")
    # U7
    s7 = pure["signed"]
    begins = "The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi · Ships and forces: Transport Group Yoke · Not before: "
    check(s7["line"] == begins + "AUG 5" and not re.search(r"Order:|Capture", s7["line"]) and s7["orders"] == 0,
          f"U7: SIGNED says the landing as The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi, then its limits, with no Order: and no Capture, and signedOrders gives none ({s7['line']})")
    check(s7["bare"] == "The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi", f"U7: with no limit queued it is exactly that ({s7['bare']})")
    own_orders = run_in_node(code, {"h1": h1, "h4": h4, "state": state7, "queue": [], "signed": {"order": True, "queue": [{"command": {"kind": "set_intent", "intent": 1, "intent_kind": "control_island", "island": 3}, "place": {"queue": "commands", "number": 1}, "goes_with": [], "acts": {"at": "dusk"}},
                                                                                                                            queue7[0]]}})["signed"]
    check(own_orders["order_line"].startswith("Order: ") and own_orders["orders"] == 1, f"U7: a queued set_intent with no parent is said as today, and signedOrders still gives no entry with a parent ({own_orders['order_line']})")
    # U8
    u8 = pure["u8"]
    check([e["number"] for e in u8["r"]] == [3, 2, 4, 12, 13] and u8["r"][0] == {"number": 3, "kind": "take_place", "hex": [2, -4], "parent": 1, "maker": "commander"}
          and u8["r"][1] == {"number": 2, "kind": "take_place", "hex": [3, -5], "parent": 1, "maker": "commander", "standing": True} and u8["r"][2:] == p8["with_change"]["tree"]["sub_intents"],
          f"U8: readingOfLanding gives five entries numbered 3, 2, 4, 12, 13: his landing, his other landing standing, and the tree's own unchanged ({[e['number'] for e in u8['r']]})")
    check(u8["marks"] == {"3": "YOURS", "2": "YOURS", "4": "THE STAFF’S", "12": "THE STAFF’S", "13": "THE STAFF’S"} and u8["bare"] == [3, 4, 12, 13] and u8["twice"] == [3, 4, 12, 13, 2] and u8["none"] == [None, None, None],
          f"U8: marksOf of it is YOURS for 3 and 2 and THE STAFF’S for the rest; with known empty 3, 4, 12, 13; a landing the tree lists is not added again; no tree gives null ({u8['marks']}, {u8['bare']}, {u8['twice']}, {u8['none']})")
    # U9
    u9 = pure["u9"]
    check(u9["named"] == "Tulagi" and u9["bare"] == "that place" and u9["same"] == [None, None], f"U9: landingTried names the paper's own place, says that place with none given, and is null for the order itself ({u9})")
    check(u9["two"] == "The staff’s plan is this order with the landing at Tulagi left to the staff.", f"U9: twoPlans says exactly The staff’s plan is this order with the landing at Tulagi left to the staff. ({u9['two']})")
    # U10, U11: a reason as the plan's paper shows it (the view of B's check of a reason)
    heads = ["orderWords", "plainCounts", "shareWords", "tiredBit", "plainFact", "cautionPart", "cautionWords", "cautionTexts", "cleanName", "num", "hexDist", "hexWords", "nameText", "intentWords", "plainWords",
             "tidyWorked", "paperPre", "paperText", "reasonWords", "cutTop", "oneFact", "threatsOf", "placeSpan", "taskWords", "rich", "plainLinks", "refTo", "nameOf", "hexRef", "hexPlace", "whyView",
             "newTask", "hisNumbers", "hisNodesIn", "writtenMarks", "hisHex", "hisLanding", "hisLink", "planLinks", "goToHis", "draftHis", "landingWords", "landingCap", "limitLinks", "limitSetOn",
             "limitLinkWords", "limitInState", "isDraftNo", "markHis", "paperLine", "orderWordsOf", "orderLink", "orderFound", "commas", "sayIntent", "sameHex", "limitJump"]
    vcode = "".join(cut_function(text, f"  function {h}(") + "\n" for h in heads) + word + "\n" + re.search(r"  var MENTION = .*\n", text).group(0)
    vcode += text[text.index("  var LIMIT_LINK ="):text.index("  function limitLinkWords(")] + r"""
Node.prototype.addEventListener = function () {}; document.createElement = function (t) { return new Node(t); };
const hexes = [], config = { islands: [] }; let hexRaw = false, PAPER_TASKS = null, PAPER_ROOT = null, PLACE_HEX = null, paper = null, previewIntent = null, planState = { written: [] };
const paperOpen = {}; function paperArcsDraw() {} function paperArc() { return null; } function planEnd() { return null; } function dateOf() { return ''; }
M.contacts = []; M.bases = []; M.enemyBases = []; M.forces = []; M.units = [];
function walk(n, f) { f(n); (n.kids || []).forEach(k => walk(k, f)); }
function view(state, hexNames, own, reason, landing) {
  M.state = state; M.queue = { entries: [] }; M.hexNames = hexNames; Object.keys(hexNames).forEach(k => { hexAt[k] = [0, 0, 0, 0, 0]; }); paper = landing ? { kind: 'landing', write: landing } : null;
  const t = newTask(own, 'Order'); t.reasons = [reason]; t.nothing = [reason.worked];
  const w = whyView(t), blocks = [];
  if (!w) return { blocks: [] };
  w.kids.forEach(k => {
    if (k.className === 'wsub') blocks.push({ title: k.textContent, items: [] });
    else if (k.className === 'rpoints' && blocks.length) k.kids.forEach(li => { const links = []; walk(li, n => { if (n.className && /\bref\b/.test(n.className)) links.push(n.textContent); }); blocks[blocks.length - 1].items.push({ text: li.textContent, links: links }); });
  });
  return { blocks: blocks, text: w.textContent };
}
function main(x) { return x.views.map(v => view(x.state, x.hexNames, v.own, v.reason, v.landing)); }
"""
    state = {"intents": [{"number": 1, "kind": "control_island", "maker": "commander", "parent": None, "hex": None, "in_force": True, "island": 3},
                         {"number": 2, "kind": "take_place", "maker": "commander", "parent": 1, "hex": [3, -5], "in_force": True},
                         {"number": 3, "kind": "take_place", "maker": "commander", "parent": 1, "hex": [2, -4], "in_force": True},
                         {"number": 4, "kind": "hold", "maker": "staff", "parent": 1, "hex": None, "in_force": True, "base": {"handle": "base:1", "name": "Lunga"}},
                         {"number": 30, "kind": "hold", "maker": "staff", "parent": 1, "hex": None, "in_force": True, "base": {"handle": "base:1", "name": "Lunga"}}]}
    reasons = [r for r, _ in REASONS_AND_HIS_NODES]
    kept = {"intent": 2, "hex": [3, -5]}
    said = run_in_node(vcode, {"state": state, "hexNames": {"2,-4": "Lunga", "3,-5": "Tulagi"}, "views": [
        {"own": 3, "reason": reasons[1]}, {"own": 1, "reason": reasons[4]}, {"own": 4, "reason": reasons[3]},
        {"own": 2, "reason": {"serves": {"intent": 1}, "limits": None, "worked": "intent:2, the commander's, plans the place at (3,-5): the staff forms no take beside it."}, "landing": {"number": 2, "hex": [3, -5]}},
        {"own": 4, "reason": {"serves": {"intent": 4}, "limits": None, "worked": "intent:2, the commander's, plans the place at (3,-5): the staff forms no take beside it."}, "landing": {"number": 2, "hex": [3, -5]}}]})
    two, five, four, line_own, line_other = said
    limits = lambda v: [i for b in v["blocks"] if b["title"] == "YOUR LIMITS" for i in b["items"]]
    check([(i["text"], i["links"]) for i in limits(two)] == [("its risk", ["its risk"])] and " on " not in limits(two)[0]["text"],
          f"U10: the second reason, under his own landing's line, lists its limit as exactly its risk, one link and no on ({limits(two)})")
    check([(i["text"], i["links"]) for i in limits(five)] == [("the forces it may use on your landing at Tulagi", ["the forces it may use", "your landing at Tulagi"])]
          and not any("your order to" in i["text"] for v in (two, five) for i in limits(v)), f"U10: the fifth, under the order's line, lists exactly the forces it may use on your landing at Tulagi, two links, and no your order to ({limits(five)})")
    check([(i["text"], i["links"]) for i in limits(four)] == [("the forces it may use on your order to seize and hold", ["the forces it may use", "your order to seize and hold"])], f"U10: the fourth, a limit of the order itself under a task of the staff's, is said as the page on main says it ({limits(four)})")
    # U11
    held = [i for b in line_own["blocks"] if b["title"] == "HOLDS IT BACK" for i in b["items"]]
    lower = lambda s: s[:1].lower() + s[1:]
    check(held and lower(held[0]["text"]) == "your landing at Tulagi stands: the staff forms no take beside it." and [lower(l) for l in held[0]["links"]] == ["your landing at Tulagi"]
          and not re.search(r"intent:|the commander|, your,", line_own["text"] + line_other["text"]), f"U11: the engine's line about the landing in hand is shown as your landing at Tulagi stands: the staff forms no take beside it., the place a link ({held[:1]})")
    check(not any(b["title"] == "YOUR PLAN" for v in (line_own, line_other) for b in v["blocks"]), f"U11: its WHY has no YOUR PLAN link to the landing whose paper it is ({[b['title'] for v in (line_own, line_other) for b in v['blocks']]})")
    # Through the bridge: the 39 days, the person Allied against the engine; the order alone signed, one day resolved
    check(begin(bridge, "guadalcanal-39-days", "person", "engine") == {"ok": True}, "begun: the person Allied against the engine")
    person = Person(bridge.table)
    command = {"command": "set_intent", "intent_kind": "control_island", "island": 3}
    n = person.call("preview", command)["ok"]["intent"]
    check(person.call("approve", {"preview": person.call("preview", command)["ok"]["preview"]})["ok"]["status"] == "queued", "the order to seize and hold island 3 is signed alone")
    person.call("end_day")

    def island(**more):
        return dict({"h1": h1, "h4": h4, "state": person.call("command_state")["ok"], "queue": person.call("queued")["ok"]["entries"]}, **more)

    # U12
    st = person.call("command_state")["ok"]
    mine = [i for i in st["intents"] if i["number"] == n][0]
    plan_of = {p_["number"]: p_ for p_ in st["plans"]}
    check(mine["in_force"] and not st["leaves"]["launch"] and plan_of[mine["plan"]]["state"] in ("preparing", "draft") and not [i for i in st["intents"] if i["parent"] == n],
          f"U12: the order {n} is in force, leave to launch is not given, its plan is {plan_of[mine['plan']]['state']} and no intent stands under it")
    tree = person.call("propose", {"intent": n})["ok"]["tree"]
    takes = [(i["hex"], i["maker"], i.get("standing")) for i in tree["sub_intents"] if i["kind"] == "take_place" and i["parent"] == n]
    check(sorted(t[0] for t in takes) == sorted([h1, h4]) and all(t[1] == "staff" and not t[2] for t in takes),
          f"U12: propose for {n} gives a take_place at the hex of Lunga and of Tulagi, each the staff's and not standing ({takes})")
    lines = run_in_node(code, island(lines={"tree": tree, "order": n}))["lines"]
    check(sorted(lines) == sorted([h1, h4]) and len(lines) == 2, f"U12: writeLines of that tree and {n} gives two lines, at the hex of Lunga and of Tulagi ({lines})")
    # U13
    shown = person.call("preview", worked_tulagi_alone(n, h4))["ok"]
    k = shown["intent"]
    sub_ = shown["with_change"]["tree"]["sub_intents"]
    check(shown.get("valid") is True and shown["command"]["parent"] == n and shown["command"]["hex"] == h4 and k > n and shown["under"] == [],
          f"U13: preview of the worked landing at Tulagi is valid: its command has parent {n} and the hex of Tulagi, its intent is {k}, its under is empty")
    check(not [i for i in sub_ if i["number"] == k] and not [i for i in sub_ if i["kind"] == "take_place" and i["hex"] == h4]
          and [i for i in sub_ if i["kind"] == "take_place" and i["hex"] == h1 and i["maker"] == "staff" and i["parent"] == n] and [i for i in sub_ if i["kind"] == "ready_landing" and i["parent"] == k]
          and [t for t in shown["with_change"]["tree"]["no_orders"] if t.startswith(f"intent:{k}, the commander's, plans the place at")],
          f"U13: its tree lists no entry numbered {k}, no take_place at the hex of Tulagi, the staff's take at the hex of Lunga and a ready_landing under {k}; among its no_orders is the line about intent:{k}")
    got = run_in_node(code, island(reading={"p": shown}))["reading"]
    check(got["marks"][str(k)] == "YOURS" and all(v == "THE STAFF’S" for key, v in got["marks"].items() if key != str(k)) and got["numbers"][0] == k,
          f"U13: marksOf of readingOfLanding is YOURS for {k} and THE STAFF’S for every other entry ({got['marks']})")
    # U14
    press = [{"limit": "forces", "task_forces": [10], "ships": []}, {"limit": "not_before", "from_day": 5}, {"limit": "risk", "risk": "press"}]
    refused = person.call("preview", dict(worked_tulagi_alone(n, h4), limits=press))["ok"]
    check(refused.get("status") == "refused" and refused["of_batch"] == {"position": 4, "size": 4} and refused["reason"] == f"the limit is wider than what its parent intent:{n} allows",
          f"U14: the same with Risk press is refused with of_batch position 4, size 4, the engine's reason being the limit is wider than what its parent allows ({refused.get('of_batch')}, {refused.get('reason')})")
    shown = person.call("preview", worked_tulagi_alone(n, h4))["ok"]
    # U15
    trial = person.call("trial", {"preview": shown["preview"], "size": "small"})["ok"]
    asked, beside = trial.get("asked") or {}, trial.get("beside") or {}
    check(asked.get("second_trial") is True and asked.get("tried_intent") == k and beside.get("of") == "left_to_the_staff" and beside.get("order") == n,
          f"U15: trial of that preview at the size small is answered with second_trial true, tried_intent {k}, beside.of left_to_the_staff and beside.order {n} ({asked.get('second_trial')}, {asked.get('tried_intent')}, {beside.get('of')}, {beside.get('order')})")
    said_two = run_in_node(code, island(trial=trial))["trial"]
    check(said_two["land"] == "Tulagi" and "the landing at Tulagi left to the staff" in said_two["two"], f"U15: twoPlans of it and Tulagi says the landing at Tulagi left to the staff ({said_two})")
    dated = person.call("trial", {"preview": shown["preview"], "size": "small", "dates": "chosen"})
    check('A written landing is held to a day by its "not before"' in str(dated.get("refused", "")), f"U15: trial of the same preview with dates chosen is refused in the words of the tool ({str(dated.get('refused', ''))[:120]})")
    # U16
    approved = person.call("approve", {"preview": shown["preview"]})["ok"]
    check(approved["status"] == "queued" and approved["limits_queued"] == 2, f"U16: approve of that preview answers queued and limits_queued 2 ({approved.get('status')}, {approved.get('limits_queued')})")
    held = person.call("queued")["ok"]
    top = [e for e in held["entries"] if e["command"]["kind"] == "set_intent"]
    check(len(held["entries"]) == 3 and len(top) == 1 and top[0]["command"].get("parent") == n and top[0]["command"]["hex"] == h4 and len(top[0]["goes_with"]) == 2
          and sorted(e["command"]["limit"] for e in held["entries"] if e["command"]["kind"] == "set_limit" and e["command"]["intent"] == k) == ["forces", "not_before"],
          f"U16: queued then holds a set_intent with parent {n} and the hex of Tulagi whose goes_with names a forces and a not_before limit of {k}")
    signed = run_in_node(code, island(signed={"queue": held["entries"]}))["signed"]
    check(signed["line"].startswith("The landing at Tulagi: written under your order to seize and hold Guadalcanal and Tulagi · Ships and forces: ") and " · Not before: " in signed["line"] and signed["orders"] == 0 and not re.search(r"Order:|Capture", signed["line"]),
          f"U16: the page's function for SIGNED says the first in the words of U7, and signedOrders of that queue is empty ({signed['line']})")
    after = person.call("propose", {"intent": n})["ok"]["tree"]
    takes = {i["number"]: i for i in after["sub_intents"] if i["kind"] == "take_place" and i["parent"] == n}
    check(takes[k]["maker"] == "commander" and takes[k].get("standing") and takes[k]["hex"] == h4 and [i for i in takes.values() if i["maker"] == "staff" and i["hex"] == h1],
          f"U16: propose for {n} now gives the take_place at the hex of Tulagi as number {k}, the commander's and standing, and the staff's at the hex of Lunga")
    check(run_in_node(code, island(lines={"tree": after, "order": n}))["lines"] == [h1], "U16: writeLines gives the hex of Lunga alone")
    again = person.call("preview", worked_tulagi_alone(n, h4))["ok"]
    check(again.get("status") == "refused" and again["reason"] == f"intent:{k} already plans the place at ({h4[0]},{h4[1]})" and pure["u6"]["planned"] == "It is already planned.",
          f"U16: previewing the worked landing at Tulagi again is refused, the engine's reason being intent:{k} already plans the place at that hex, which the page says It is already planned. ({again.get('reason')})")
    # U17
    entry = top[0]
    taken = person.call("take_back", {"queue": entry["place"]["queue"], "number": entry["place"]["number"], "as_of": held["as_of"]})["ok"]
    back = person.call("propose", {"intent": n})["ok"]["tree"]
    check(taken.get("status") != "refused" and person.call("queued")["ok"]["entries"] == [] and all(i["maker"] == "staff" for i in back["sub_intents"] if i["kind"] == "take_place" and i["parent"] == n)
          and sorted(run_in_node(code, island(lines={"tree": back, "order": n}))["lines"]) == sorted([h1, h4]),
          "U17: one take_back of the first entry is not refused and leaves nothing queued; propose gives both takes as the staff's again and writeLines both lines")
    # U18
    shown = person.call("preview", worked_tulagi_alone(n, h4))["ok"]
    k = shown["intent"]
    check(person.call("approve", {"preview": shown["preview"]})["ok"]["status"] == "queued", "U18: the landing at Tulagi is previewed and approved again")
    person.call("end_day")
    st = person.call("command_state")["ok"]
    his = [i for i in st["intents"] if i["number"] == k]
    plans = {p_["number"]: p_ for p_ in st["plans"]}
    check(his and his[0]["in_force"] and his[0]["maker"] == "commander" and his[0]["parent"] == n and plans[his[0]["plan"]]["state"] in ("draft", "preparing") and plans[[i for i in st["intents"] if i["number"] == n][0]["plan"]]["state"] in ("draft", "preparing"),
          f"U18: intent {k} is in force under {n}, the commander's, its plan {plans[his[0]['plan']]['state']} while the order's plan still waits")
    line = run_in_node(code, island(his={"k": k, "n": n, "hex": h1}))
    rows = {r[0]: r[1:] for r in line["his"]["rows"]}
    check(line["his"]["mark"] == "YOURS" and line["his"]["giveBack"] is True and rows["ships"] == [True, True, True] and rows["not_before"] == [True, True, True]
          and all(v[1] for key, v in rows.items() if key != "by_day") and all(not v[2] for key, v in rows.items() if v[0] is False) and "by_day" not in rows,
          f"U18: hisLine of it is YOURS, offers GIVE IT BACK TO THE STAFF, and shows Ships and forces and Not before with CHANGE and BACK TO THE STAFF’S CHOICE and every other line with CHANGE ({rows})")
    check("t.his = hisLine(byNum[t.n]" in waits, "U18 (C5): the plan's paper of a plan that waits gives a landing of his the lines of one in force")
    check(line["others"] == [{"number": k, "hex": h4}], f"U18: hisOtherLandings for the order and the hex of Lunga gives {k} at the hex of Tulagi alone ({line['others']})")
    lu = person.call("preview", worked_lunga_alone(n, k, h1))["ok"]
    check(lu.get("valid") is True and lu["command"]["parent"] == n and lu["intent"] > k and person.call("queued")["ok"]["entries"] == [],
          f"U18: preview of the worked landing at Lunga is valid, with parent {n} and a number above {k}, and nothing is approved")
    wd = person.call("preview", {"command": "withdraw_intent", "intent": k})["ok"]
    check(wd.get("valid") is True and wd["intent"] == n, f"U18: preview of withdraw_intent {k} is valid with intent {n}")
    # U19
    person.call("set_leave", {"leave": "launch", "given": True})
    person.call("end_day")
    st = person.call("command_state")["ok"]
    plans = {p_["number"]: p_ for p_ in st["plans"]}
    took = [i for i in st["intents"] if i["kind"] == "take_place" and i["parent"] == n and i["hex"] == h1 and i["in_force"]]
    check(took and all(i["maker"] == "staff" for i in took) and plans[[i for i in st["intents"] if i["number"] == n][0]["plan"]]["state"] == "released",
          f"U19: a take_place at the hex of Lunga with maker staff is in force under {n} and the order's plan is released")
    late = person.call("preview", worked_lunga_alone(n, k, h1))["ok"]
    check(late.get("status") == "refused" and re.fullmatch(r"intent:\d+ already plans the place at \(%d,%d\)" % tuple(h1), late["reason"]) and pure["u6"]["planned"] == "It is already planned.",
          f"U19: preview of the worked landing at Lunga is now refused, the engine's reason beginning intent: and ending already plans the place at that hex, which the page says It is already planned. ({late.get('reason')})")
    check(plans[[i for i in st["intents"] if i["number"] == n][0]["plan"]]["state"] == "released" and any(i["parent"] == n for i in st["intents"])
          and "mp.state === 'released' && !(a[0].intents || []).some(function (i) { return i.parent === root; })" in plan,
          "U19: the page's paper for the order is then drawn from the command state (its plan is released and has tasks), where no line is offered WRITE IT")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


# ---- Rung 1 on the table (decision 0237). The statements are the architect's, written before the code with
# docs/decisions/as-built/0237-the-first-three-rungs.md, "Planned: rung 1 on the table", which has the entry, the directive and the
# verdict's words in full. None of the four is called from main() until it is built. An expected value below that the build cannot
# meet goes back to the architect: it is not changed to pass.

RUNG_ONE = "ladder-1-one-ship"
RUNG_ONE_MISSION = {"measure": "every_night_fed", "last_day": 14,
                    "places": [{"q": 0, "r": 0, "name": "Saltmere"}, {"q": 5, "r": 0, "name": "Thornwick"}]}
RUNG_ONE_MET = ["Mission met.", "Every man was fed every night to the end of day 14, at Saltmere and at Thornwick."]


def rung_one_order(floor, interval):
    """The supply order as the paper sends it for a reading: base 2 (T), with ship 1 and base 1 (S) as its forces."""
    return {"command": "set_intent", "intent_kind": "keep_supplied", "base": 2, "floor_days": floor, "interval_days": interval,
            "limits": [{"limit": "forces", "ships": [1], "task_forces": [], "source": 1}]}


def rung_one_on_the_list(bridge):
    """Rung 1 on the list, its directive, the screen and a game with no date.

    In the files:
    L1. The first entry of `table/scenarios.json` has the id "ladder-1-one-ship", the name "Rung 1: One ship", `dates` "An invented
        sea", `days` 14, `placeholder` true, the map "data/map/placeholder_ladder.toml", no key `start`, `islands` empty, three `lines`
        of which the last begins "A placeholder game", `sides` with the key `allied` alone, `directives` with the key `allied` alone,
        `mission` equal to RUNG_ONE_MISSION, and one pairing: {allied "person", japanese "engine", engine "absent", file
        "data/scenarios/ladder/138_one_ship.toml"}. The whole entry is the one written in the as-built section.
    L2. Every other entry is what it was: none has the key `mission` or `placeholder`, none has a pairing whose `engine` is "absent",
        and each has a `start`.
    L3. `table/directives/ladder_1_allied.json` has one directive, number 1, `when` "start", from "Area Commander", to "Commander,
        Saltmere", subject "Both garrisons fed", with three parts headed "1. Situation", "2. Mission" and "3. Guidance". The text of its mission
        is one paragraph and begins with the entry's `sides.allied`, which is its first sentence, "Keep both garrisons fed." (the card on
        the new game screen says the mission in brief and the directive says it in full: as-built, "The card on the new game screen, cut short"). Its guidance names "Thornwick", "Saltmere" and "two numbers", and its second paragraph is "Then tell the staff that plans launch when ready, under YOUR ORDERS,
        and carry on: it will not sail the ship without that leave."

    In the page's text:
    L4. `ENGINE_IS` has the key `absent`, whose words are "There is no enemy in this game: the Japanese side has nothing." for the
        Japanese side, and keeps `staff`, `script` and `none` as they were.
    L5. `dateOf`, cut from the page and run in node with a config whose `start` is the empty string, gives the empty string for days 0,
        1 and 13. No place where the page writes a day shows nothing for it: each use of `dateOf` whose result is shown has a fallback
        in words ("day 3", "Day 3"), the three that had none included ("Not before" on a force's card, and the two lines of what comes
        within the week in the staff's summary).

    Through the bridge:
    L6. The screen's list has the rung first, with one pairing, and names no file and no `mission` (what the screen is given of a
        scenario is unchanged). The game begins: `/config` has `seat` "allied", `start` the empty string, `islands` empty, the one
        directive, and `mission` equal to RUNG_ONE_MISSION. The seat's `list_bases` has two bases; for each place of the mission, the
        page's `cleanName` (in node) of the name of the base in that hex is the place's `name`.
    L7. The seat's tools do not include `trial`. `get_briefing` gives `heading.last_day` 14.
    L8. A game of another scenario begun after it (the Eastern Solomons, a person Allied against the engine) has no key `mission` in
        its `/config`.
    L9. (The designer's answer of Nov 27, 2026: leave to launch is his to give on the first evening, by the page's own switch.) In the
        page's text, YOUR ORDERS has the line "Plans launch: " with a button that reads "WHEN READY" while the leave is withheld and
        sends `set_leave` {leave "launch", given true}; it is disabled only where `canSign()` is false, and `canSign` is false only in
        a watched game, a game that is over, or while time runs. In the game of L6, before any day is fought, `command_state` gives
        `leaves.launch` false and `leaves.prepare` true; `set_leave` {leave "launch", given true} is answered `status` "queued"; and
        `command_state` then gives `leaves.launch` true.
    Each game is put away after it."""
    print("-- rung 1 on the list", flush=True)
    root = os.path.dirname(HERE)
    entries = json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]
    first = entries[0]
    check(first["id"] == RUNG_ONE and first["name"] == "Rung 1: One ship" and first["dates"] == "An invented sea" and first["days"] == 14 and first["placeholder"] is True
          and first["map"] == "data/map/placeholder_ladder.toml" and "start" not in first and first["islands"] == [],
          "L1: the first entry is rung 1, with its id, name, dates, days, placeholder and map, no start and no islands")
    check(len(first["lines"]) == 3 and first["lines"][2].startswith("A placeholder game") and list(first["sides"]) == ["allied"] and list(first["directives"]) == ["allied"]
          and first["directives"]["allied"] == "table/directives/ladder_1_allied.json" and first["mission"] == RUNG_ONE_MISSION
          and first["pairings"] == [{"allied": "person", "japanese": "engine", "engine": "absent", "file": "data/scenarios/ladder/138_one_ship.toml"}]
          and sorted(first) == sorted(["id", "map", "name", "dates", "days", "placeholder", "lines", "sides", "islands", "directives", "mission", "pairings"]),
          "L1: three lines, the Allied side alone in `sides` and `directives`, the mission, and the one pairing with no agent and the engine absent")
    check(all("mission" not in e and "placeholder" not in e and "start" in e and all(p.get("engine") != "absent" for p in e["pairings"]) for e in entries[1:]),
          "L2: every other entry has no `mission` or `placeholder`, no pairing with the engine absent, and a start")
    directive = json.load(open(os.path.join(root, first["directives"]["allied"])))["directives"]
    d = directive[0]
    check(len(directive) == 1 and d["number"] == 1 and d["when"] == "start" and d["from"] == "Area Commander" and d["to"] == "Commander, Saltmere" and d["subject"] == "Both garrisons fed"
          and [p["heading"] for p in d["parts"]] == ["1. Situation", "2. Mission", "3. Guidance"],
          "L3: the directive is number 1, at the start, from the Area Commander to the Commander, Saltmere, in three parts")
    guidance = " ".join(d["parts"][2]["text"])
    check(len(d["parts"][1]["text"]) == 1 and first["sides"]["allied"] == "Keep both garrisons fed." and d["parts"][1]["text"][0].startswith(first["sides"]["allied"] + " "),
          "L3: the mission is one paragraph and begins with the entry's words for the side, its first sentence")
    check(all(w in guidance for w in ("Thornwick", "Saltmere", "two numbers")) and d["parts"][2]["text"][1] == "Then tell the staff that plans launch when ready, under YOUR ORDERS, and carry on: it will not sail the ship without that leave.",
          "L3: the guidance names Thornwick, Saltmere and the two numbers, and its second paragraph is the designer's")
    # L1a. Decision 0241: rung 1's scenario names its two bases "Saltmere" and "Thornwick", its land units "Thornwick garrison" and
    # "Saltmere garrison" and its ship "Merrowdale", in that order in the file; it is marked `placeholder = true`; and none of the
    # words Saltmere, Thornwick and Merrowdale is in any file of `data/oob`, whatever its case.
    scen = open(os.path.join(root, "data", "scenarios", "ladder", "138_one_ship.toml")).read()
    named = [l.split('"')[1] for l in scen.splitlines() if l.startswith('name = "')]
    oob = os.path.join(root, "data", "oob")
    record = " ".join(open(os.path.join(d, f), errors="replace").read().lower() for d, _, fs in os.walk(oob) for f in fs if f.endswith((".csv", ".toml")))
    check(named[1:] == ["Saltmere", "Thornwick", "Thornwick garrison", "Saltmere garrison", "Merrowdale"] and named[0].startswith("Placeholder scenario 138")
          and "\nplaceholder = true\n" in scen and not any(w in record for w in ("saltmere", "thornwick", "merrowdale")),
          "L1a: rung 1's bases, garrisons and ship carry invented names that are in no file of data/oob, and the scenario is still marked a placeholder")
    # L3a. The islands of the ladder's chart, for the eye only: `table/chart/placeholder_ladder_coast.json` has a note that begins
    # "PLACEHOLDER" and six rings, each closed, and every point of every ring lies within 0.333 degrees (the hex's inner radius,
    # 20 nm at 60 nm to the degree) of the centre of one of the map's four coast hexes, (0,0), (5,0), (0,14) and (3,3), as
    # serve.py places a hand-written map's hexes: so no island reaches into a hex of open sea. Each of the four has a ring about it.
    # The page defines `openOnMission` once and calls it once, after the chart is laid and before the camera is first placed.
    coast = json.load(open(os.path.join(HERE, "chart", "placeholder_ladder_coast.json")))
    centres = [(40 * (q + r / 2) / 60, -34.641016 * r / 60) for q, r in ((0, 0), (5, 0), (0, 14), (3, 3))]
    near = lambda x, y: [n for n, (cx, cy) in enumerate(centres) if (x - cx) ** 2 + (y - cy) ** 2 <= 0.333 ** 2]
    homes = [{tuple(near(ring[n], ring[n + 1])) for n in range(0, len(ring), 2)} for ring in coast["rings"]]
    check(coast["note"].startswith("PLACEHOLDER") and len(coast["rings"]) == 6 and all(len(r) % 2 == 0 and r[:2] == r[-2:] for r in coast["rings"])
          and all(len(h) == 1 and len(next(iter(h))) == 1 for h in homes) and {next(iter(h))[0] for h in homes} == {0, 1, 2, 3},
          "L3a: the ladder's chart has six invented islands, marked a placeholder, each inside the hex of one of the four places, and each place has one")
    page = page_text()
    check(page.count("function openOnMission()") == 1 and page.count("openOnMission()") == 2 and "layChart(); cur.zoom = 0; openOnMission(); placeCamera();" in page,
          "L3a: the page opens a game with a mission looking at the mission's places: openOnMission is defined once and called once, as the chart is first laid")
    text = page_text()
    said = run_in_node(text[text.index("  var SIDES = ['allied', 'japanese'], ENGINE_IS = {"):text.index("  function copyRow(")].replace("var SIDES = ['allied', 'japanese'], ENGINE_IS", "var ENGINE_IS")
                       + "\nfunction main(x) { return { keys: Object.keys(ENGINE_IS), absent: ENGINE_IS.absent('Japanese'), none: ENGINE_IS.none('Japanese'), staff: ENGINE_IS.staff('Allied'), script: ENGINE_IS.script('Allied') }; }", [])
    check(said["keys"] == ["staff", "script", "none", "absent"] and said["absent"] == "There is no enemy in this game: the Japanese side has nothing."
          and said["none"] == "The Japanese side is the engine's, and the scenario gives it no orders: its units do only what they do by themselves."
          and said["staff"] == "The Allied side is the engine's: its staff carries out the orders the scenario gives that side, day by day."
          and said["script"] == "The Allied side is the engine's: its forces follow the orders the scenario gives them, as written.",
          "L4: ENGINE_IS has `absent`, \"There is no enemy in this game: the Japanese side has nothing.\", and keeps staff, script and none as they were")
    said = run_in_node("const config = { start: '' }; const MONTHS = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'];\n" + cut_function(text, "  function dateOf(")
                       + "\nfunction main(x) { return x.map(n => dateOf(n)); }", [0, 1, 13])
    check(said == ["", "", ""], "L5: with no start date, dateOf gives the empty string for days 0, 1 and 13")
    check("(dateOf(x.day - 1) || 'Day ' + x.day) + ': '" in text and "(dateOf(f.available_from - 1) || 'Day ' + f.available_from) + ': '" in text
          and "dateOf(f.available_from - 1) ? dateOf(f.available_from - 1) + ' (day ' + f.available_from + ')' : 'day ' + f.available_from" in text,
          "L5: the three places that had no fallback for a date have one in words: \"Not before\" on a force's card and the two lines of what comes within the week")

    print("-- the game begun, and the switch for leave to launch", flush=True)
    listed = get(bridge.table + "/scenarios")["scenarios"]
    check(listed[0]["id"] == RUNG_ONE and listed[0]["pairings"] == [{"allied": "person", "japanese": "engine", "engine": "absent"}] and "mission" not in listed[0] and "file" not in listed[0],
          "L6: the screen's list has the rung first with one pairing, and names no file and no mission")
    check(begin(bridge, RUNG_ONE, "person", "engine") == {"ok": True}, "L6: rung 1 begins")
    config = get(bridge.table + "/config")
    check(config["seat"] == "allied" and config["start"] == "" and config["islands"] == [] and len(config["directives"]) == 1 and config["mission"] == RUNG_ONE_MISSION,
          "L6: /config has the seat, an empty start, no islands, the one directive and the mission")
    person = Person(bridge.table)
    bases = person.call("list_bases")["ok"]["bases"]
    names = run_in_node(text[text.index("  function cleanName"):text.index("  function nameText")] + "\nfunction main(x) { return x.map(cleanName); }",
                        [next(b["base"]["name"] for b in bases if (b["place"]["q"], b["place"]["r"]) == (pl["q"], pl["r"])) for pl in RUNG_ONE_MISSION["places"]])
    check(len(bases) == 2 and names == [pl["name"] for pl in RUNG_ONE_MISSION["places"]],
          f"L6: list_bases has two bases, and the page's name for each place of the mission is the mission's ({names})")
    check("trial" not in get(bridge.table + "/tools") and person.call("get_briefing", {"text": False})["ok"]["heading"]["last_day"] == 14,
          "L7: the seat's tools do not include trial, and the briefing's last day is 14")
    lines = text[text.index("var given = !!s.leaves.launch"):]
    lines = lines[:lines.index("line.appendChild(l);")]
    check("'Plans launch: ' + (given ? 'when ready' : 'on my word')" in lines and "given ? 'ON MY WORD' : 'WHEN READY'" in lines and "sw.disabled = !canSign()" in lines
          and "act('set_leave', { leave: 'launch', given: !given }" in lines and "function canSign() { return !WATCH && !M.over && !playing; }" in text,
          "L9: YOUR ORDERS has \"Plans launch: \" and a button WHEN READY that sends set_leave for launch, disabled only where canSign() is false (a watched game, a game that is over, time running)")
    state = person.call("command_state")["ok"]["leaves"]
    check(state["launch"] is False and state["prepare"] is True, "L9: before any day is fought leave to prepare is given and leave to launch is withheld")
    queued = person.call("set_leave", {"leave": "launch", "given": True})["ok"]
    check(queued["status"] == "queued" and person.call("command_state")["ok"]["leaves"]["launch"] is True,
          "L9: set_leave for launch is queued on the first evening, and command_state then gives leave to launch")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")
    check(begin(bridge, "eastern-solomons", "person", "engine") == {"ok": True} and "mission" not in get(bridge.table + "/config"),
          "L8: a game of another scenario begun after it has no mission in its /config")
    check(post(bridge.table + "/new") == {"ok": True}, "the game is put away")


def the_supply_orders_two_numbers(bridge):
    """The floor and the interval on the order paper, for the end "keep it supplied".

    In the page's text:
    O1. The paper has a row with the id `o-supply` holding two inputs of type number, `o-floor` (min 1, max 99, value 30) and
        `o-interval` (min 1, max 99, no value), and the words "Never below", "days of supply", "Each run brings", "days more" and "left
        to the staff: 15 days". The page does not hold "Keep it supplied, 30 days"; the end reads "Keep it supplied".
    O2. The row is shown where the end chosen is `keep_supplied` and hidden for every other, by the same path that shows `o-patrol`
        for a patrol; a change of either input asks the reading again, as a change of `o-dist` does.

    The page's own functions, cut from its text and run in node, with a stand-in for the two inputs:
    O3. Nothing touched (30, empty): what the paper adds to the order is `floor_days` 30 and no key `interval_days`, so the arguments of
        the reading are exactly {command "set_intent", intent_kind "keep_supplied", base n, floor_days 30}: what the page sent before.
    O4. (2, 5) gives `floor_days` 2 and `interval_days` 5. An empty floor, or one that is no number, gives 30; 0 gives 1; 150 gives 99.
        An interval of 0 gives 1 and of 150 gives 99; an empty one gives no key. For an end of any other kind neither key is added.
    O5. What a place's card says of an order in force: {kind keep_supplied, floor_days 2, interval_days 5} gives "Kept supplied, 2
        days, each run 5 days more"; {floor_days 30} with no interval gives "Kept supplied, 30 days", as before.

    Through the bridge, in a game of rung 1:
    O6. A preview of `rung_one_order(2, 5)` is valid and not refused, its `command` has `floor_days` 2 and `interval_days` 5, and its
        `shown_limits` has one entry. A preview of the same order with no `interval_days` is valid and its `command` has no interval.
        Nothing is signed, and the game is put away."""
    import re
    print("-- the supply order's two numbers", flush=True)
    text = page_text()
    floor_tag = re.search(r'<input type="number" id="o-floor"[^>]*>', text).group(0)
    interval_tag = re.search(r'<input type="number" id="o-interval"[^>]*>', text).group(0)
    row = text[text.index('<div id="o-supply"'):text.index('<fieldset id="o-risk">')]
    check('min="1"' in floor_tag and 'max="99"' in floor_tag and 'value="30"' in floor_tag and 'min="1"' in interval_tag and 'max="99"' in interval_tag and "value=" not in interval_tag
          and all(w in row for w in ("Never below", "days of supply", "Each run brings", "days more")) and "left to the staff: 15 days" in text
          and "Keep it supplied, 30 days" not in text and "['Keep it supplied', { intent_kind: 'keep_supplied'" in text,
          "O1: the paper has the row o-supply with o-floor (1 to 99, 30) and o-interval (1 to 99, empty), their words and \"left to the staff: 15 days\"; the end reads \"Keep it supplied\"")
    check("$('o-supply').hidden = !on" in text and "function supplyChosen()" in text and "e[1].intent_kind === 'keep_supplied'" in text
          and "showPatrol(); showSupply(); drawMarks(); endChanged();" in text and "e.target.id === 'o-floor' || e.target.id === 'o-interval'" in text and "{ showSupply(); askPreview(); }" in text
          and "$('o-floor').value = '30'; $('o-interval').value = '';" in text,
          "O2: the row is shown where the end chosen is keep_supplied and hidden for every other, by the path that shows o-patrol; a change of either input asks the reading again")
    code = ("".join(cut_function(text, h) + "\n" for h in ("  function supplyFloor(", "  function supplyInterval(", "  function addSupplyNumbers(", "  function commas(n)", "  function dayCount(", "  function suppliedWords("))
            + "function main(list) { return list.map(function (x) { made['o-floor'] = { value: x.floor }; made['o-interval'] = { value: x.interval };"
              " return { args: addSupplyNumbers(Object.assign({ command: 'set_intent' }, x.end)), card: x.card ? suppliedWords(x.card) : null }; }); }")
    keep = {"intent_kind": "keep_supplied", "base": 2}
    out = run_in_node(code, [
        {"floor": "30", "interval": "", "end": keep}, {"floor": "2", "interval": "5", "end": keep}, {"floor": "", "interval": "", "end": keep},
        {"floor": "abc", "interval": "abc", "end": keep}, {"floor": "0", "interval": "0", "end": keep}, {"floor": "150", "interval": "150", "end": keep},
        {"floor": "2", "interval": "5", "end": {"intent_kind": "hold", "base": 2}},
        {"floor": "30", "interval": "", "end": keep, "card": {"kind": "keep_supplied", "floor_days": 2, "interval_days": 5}},
        {"floor": "30", "interval": "", "end": keep, "card": {"kind": "keep_supplied", "floor_days": 30}}])
    args = [o["args"] for o in out]
    check(args[0] == {"command": "set_intent", "intent_kind": "keep_supplied", "base": 2, "floor_days": 30},
          "O3: nothing touched (30, empty) adds floor_days 30 and no interval_days: the arguments are what the page sent before")
    check(args[1]["floor_days"] == 2 and args[1]["interval_days"] == 5 and args[2]["floor_days"] == 30 and "interval_days" not in args[2] and args[3]["floor_days"] == 30 and "interval_days" not in args[3]
          and args[4]["floor_days"] == 1 and args[4]["interval_days"] == 1 and args[5]["floor_days"] == 99 and args[5]["interval_days"] == 99
          and args[6] == {"command": "set_intent", "intent_kind": "hold", "base": 2},
          "O4: (2, 5) gives 2 and 5; empty or no number gives a floor of 30 and no interval; 0 gives 1; 150 gives 99; an end of another kind gets neither key")
    check(out[7]["card"] == "Kept supplied, 2 days, each run 5 days more" and out[8]["card"] == "Kept supplied, 30 days",
          "O5: a place's card says \"Kept supplied, 2 days, each run 5 days more\", and with no interval \"Kept supplied, 30 days\"")
    check(begin(bridge, RUNG_ONE, "person", "engine") == {"ok": True}, "begun: rung 1")
    person = Person(bridge.table)
    both = person.call("preview", rung_one_order(2, 5))["ok"]
    check(both["valid"] is True and both["command"]["floor_days"] == 2 and both["command"]["interval_days"] == 5 and len(both["shown_limits"]) == 1,
          "O6: a preview of the supply order at 2 and 5 is valid, carries both numbers and shows one limit")
    bare = rung_one_order(2, None)
    del bare["interval_days"]
    only = person.call("preview", bare)["ok"]
    check(only["valid"] is True and only["command"]["floor_days"] == 2 and "interval_days" not in only["command"],
          "O6: the same with no interval is valid and its command has no interval")
    check(person.call("queued")["ok"]["entries"] == [] and post(bridge.table + "/new") == {"ok": True}, "nothing was signed, and the game is put away")


def the_verdict_in_words():
    """The verdict's words: the page's own `missionLines(mission, days)`, cut from its text with `commas` and run in node. It draws
    nothing and asks nothing. `days` is a list, day 1 first, each the list of that day's events as the tool `events` gives them, or
    null for a day that was not answered. The events below are made up in the tool's shape: {"day", "event_kind": "men_fed", "pulse":
    "night", "text"}, the text being the engine's, "Allied men ashore at (5,0): 0 of 1000 fed from base:2, 1000 unfed". `fed(q, r)` is
    such an event with every one of 1,000 men fed, `unfed(q, r, n)` one with n of 1,000 unfed. The mission is RUNG_ONE_MISSION.

    W1. Fourteen days, each with fed(0,0) and fed(5,0): RUNG_ONE_MET.
    W2. The same with unfed(5,0,1000) on day 14: ["Mission not met.", "1,000 men at Thornwick went unfed on the night of day 14."].
    W3. The same as W1 with unfed(0,0,1000) on days 13 and 14: ["Mission not met.", "1,000 men at Saltmere went unfed on the nights of
        days 13 and 14."].
    W4. unfed(5,0,1000) on days 7, 8, 9, 12, 13 and 14: the second line is "1,000 men at Thornwick went unfed on the nights of days 7, 8,
        9, 12, 13 and 14."
    W5. Both places, and two counts: unfed(5,0,400) on day 12, unfed(5,0,1000) on days 13 and 14, unfed(0,0,1) on day 14 gives, in
        this order: "Mission not met.", "1 man at Saltmere went unfed on the night of day 14.", "400 men at Thornwick went unfed on the
        night of day 12.", "1,000 men at Thornwick went unfed on the nights of days 13 and 14." (the measure's order of places, then each
        count in the order of its first night).
    W6. What does not count: an event of another kind whose text says "1000 unfed"; a `men_fed` at a hex the mission does not name
        ((3,3), 600 unfed); a `men_fed` of a named hex whose pulse is not "night". Each added to W1's days leaves RUNG_ONE_MET. The
        form with no base, "Allied men ashore at (5,0): no base of the side, none of 1000 fed, 1000 unfed", on day 6 counts: "1,000 men
        at Thornwick went unfed on the night of day 6." Two events of (5,0) on one night, 300 and 200 unfed, are said as 500 men.
    W7. Not judged, and never "met": day 9 null gives ["The mission could not be judged.", "Day 9 could not be read."]; thirteen days
        given where the mission's last day is 14 gives "Day 14 could not be read."; a `men_fed` of day 9 whose text is "Allied men
        ashore somewhere" gives ["The mission could not be judged.", "A report of the men fed on day 9 could not be read."]. A night
        unfed on an earlier day does not turn either into "Mission not met.": what could not be read whole is not judged.
    W8. A mission of one place, (5,0) "Thornwick", all fed: the second line ends "at Thornwick."; of three places named A, B and C: "at A,
        B and C."

    In the page's text:
    W9. `missionLines(` is called in one place, inside `readEnd`, and only where `config.mission` is set; what it is given is read
        through `dayEvents` for the days 1 to the mission's `last_day` and through no other call of the seat. The paper holds "MISSION
        MET", "MISSION NOT MET" and "Reading the nights.", and still holds "This game set no objectives, so the umpire gives no result."
        for a game with no mission."""
    import re
    print("-- the verdict's words", flush=True)
    text = page_text()

    def event(day, q, r, unfed):
        base = "base:1" if (q, r) == (0, 0) else "base:2"
        return {"day": day, "event_kind": "men_fed", "pulse": "night",
                "text": f"Allied men ashore at ({q},{r}): {1000 - unfed} of 1000 fed from {base}, {unfed} unfed"}

    def fourteen(unfed=(), extra=(), places=((0, 0), (5, 0)), length=14, null=(), replaced=()):
        """The days: each with every place fed, an unfed (day, q, r, n) put in the place of the fed event of its place, `extra` events
        added to their day, `replaced` events (day, q, r, event) put in place of a place's, a day in `null` not answered."""
        bad = {(d, q, r): n for d, q, r, n in unfed}
        swap = {(d, q, r): e for d, q, r, e in replaced}
        days = []
        for d in range(1, length + 1):
            if d in null:
                days.append(None)
                continue
            events = []
            for q, r in places:
                events.append(swap[(d, q, r)] if (d, q, r) in swap else event(d, q, r, bad.get((d, q, r), 0)))
            events += [e for day, e in extra if day == d]
            days.append(events)
        return days

    base_t = {"q": 5, "r": 0, "name": "Thornwick"}
    cases = []
    def case(days, mission=RUNG_ONE_MISSION):
        cases.append({"mission": mission, "days": days})
    case(fourteen())                                                                    # W1
    case(fourteen(unfed=[(14, 5, 0, 1000)]))                                            # W2
    case(fourteen(unfed=[(13, 0, 0, 1000), (14, 0, 0, 1000)]))                          # W3
    case(fourteen(unfed=[(d, 5, 0, 1000) for d in (7, 8, 9, 12, 13, 14)]))              # W4
    case(fourteen(unfed=[(12, 5, 0, 400), (13, 5, 0, 1000), (14, 5, 0, 1000), (14, 0, 0, 1)]))   # W5
    other = {"day": 4, "event_kind": "cargo_unloaded", "pulse": "night", "text": "1000 unfed"}
    case(fourteen(extra=[(4, other)]))                                                  # W6 other kind
    case(fourteen(extra=[(5, event(5, 3, 3, 600))]))                                    # W6 a hex the mission does not name
    late = dict(event(6, 5, 0, 1000), pulse="morning")
    case(fourteen(extra=[(6, late)]))                                                   # W6 not the night pulse
    nobase = {"day": 6, "event_kind": "men_fed", "pulse": "night", "text": "Allied men ashore at (5,0): no base of the side, none of 1000 fed, 1000 unfed"}
    case(fourteen(replaced=[(6, 5, 0, nobase)]))                                        # W6 no base
    case(fourteen(replaced=[(9, 5, 0, event(9, 5, 0, 300))], extra=[(9, event(9, 5, 0, 200))]))   # W6 two events of one night
    earlier = [(3, 5, 0, 1000)]
    case(fourteen(null=[9], unfed=earlier))                                             # W7 a day not answered
    case(fourteen(length=13, unfed=earlier))                                            # W7 thirteen days
    somewhere = {"day": 9, "event_kind": "men_fed", "pulse": "night", "text": "Allied men ashore somewhere"}
    case(fourteen(unfed=earlier, extra=[(9, somewhere)]))                               # W7 a report that does not fit
    case(fourteen(places=((5, 0),)), {"measure": "every_night_fed", "last_day": 14, "places": [base_t]})                                       # W8 one place
    three = {"measure": "every_night_fed", "last_day": 14, "places": [{"q": 0, "r": 0, "name": "A"}, {"q": 5, "r": 0, "name": "B"}, {"q": 3, "r": 3, "name": "C"}]}
    case(fourteen(places=((0, 0), (5, 0), (3, 3))), three)                              # W8 three places
    code = text[text.index("  function missionLines("):text.index("  var endSeq = 0;")] + cut_function(text, "  function commas(n)") + "\nfunction main(list) { return list.map(x => missionLines(x.mission, x.days)); }"
    got = run_in_node(code, cases)
    check(got[0] == RUNG_ONE_MET, "W1: fourteen days, each with both places fed: " + " ".join(got[0]))
    check(got[1] == ["Mission not met.", "1,000 men at Thornwick went unfed on the night of day 14."], "W2: " + " ".join(got[1]))
    check(got[2] == ["Mission not met.", "1,000 men at Saltmere went unfed on the nights of days 13 and 14."], "W3: " + " ".join(got[2]))
    check(got[3][1] == "1,000 men at Thornwick went unfed on the nights of days 7, 8, 9, 12, 13 and 14.", "W4: " + got[3][1])
    check(got[4] == ["Mission not met.", "1 man at Saltmere went unfed on the night of day 14.", "400 men at Thornwick went unfed on the night of day 12.",
                     "1,000 men at Thornwick went unfed on the nights of days 13 and 14."], "W5: both places and two counts, in the measure's order")
    check(got[5] == RUNG_ONE_MET and got[6] == RUNG_ONE_MET and got[7] == RUNG_ONE_MET, "W6: an event of another kind, a hex the mission does not name and a pulse that is not the night do not count")
    check(got[8] == ["Mission not met.", "1,000 men at Thornwick went unfed on the night of day 6."], "W6: the form with no base counts: " + got[8][1])
    check(got[9] == ["Mission not met.", "500 men at Thornwick went unfed on the night of day 9."], "W6: two events of one hex in one night are added: " + got[9][1])
    check(got[10] == ["The mission could not be judged.", "Day 9 could not be read."] and got[11] == ["The mission could not be judged.", "Day 14 could not be read."]
          and got[12] == ["The mission could not be judged.", "A report of the men fed on day 9 could not be read."],
          "W7: a day not answered, thirteen days of fourteen and a report that does not fit are not judged, and a night unfed before them does not make it \"not met\"")
    check(got[13] == ["Mission met.", "Every man was fed every night to the end of day 14, at Thornwick."] and got[14] == ["Mission met.", "Every man was fed every night to the end of day 14, at A, B and C."],
          "W8: the places are joined as English joins a list: \"at Thornwick.\" and \"at A, B and C.\"")
    end = cut_function(text, "  function readEnd(")
    check(end.rstrip().endswith("}") and "api('get_briefing'" in end, "W9: readEnd is cut whole")
    mission_branch = end[:end.index("    var show = function () {")]
    check(text.count("missionLines(") == 2 and end.count("missionLines(") == 1 and mission_branch.count("missionLines(") == 1 and mission_branch.split("\n", 1)[1].lstrip().startswith("if (config.mission) {"),
          "W9: missionLines( is defined once and called once, in readEnd, only where config.mission is set")
    check(mission_branch.count("dayEvents(") == 1 and "for (d = 1; d <= mission.last_day; d++) asks.push(dayEvents(d)" in mission_branch and "api(" not in mission_branch and "M.result" not in mission_branch,
          "W9: what it is given is read through dayEvents for the days 1 to the last day, and through no other call of the seat")
    check("'MISSION MET'" in end and "'MISSION NOT MET'" in end and "'Reading the nights.'" in end and "This game set no objectives, so the umpire gives no result." in end[end.index("    var show = function () {"):],
          "W9: the paper holds MISSION MET, MISSION NOT MET and Reading the nights., and still says there is no result for a game with no mission")


def rung_one_played_three_ways(bridge):
    """Rung 1 played through the bridge to its end three times, and the verdict of each: the page's own `missionLines` in node on what
    the seat's `events` answered. Each game is cases A, B and C of the hand calculation in `tests/scenario_138_one_ship.rs` (its
    statements 3, 5 and 6), which gives both leaves on day 1; leave to prepare is given when a game on the table begins, and leave to launch is given here as
    the player gives it: before day 1 is fought (the designer's answer of Nov 27, 2026).

    Each game: begun from the list (rung 1, a person Allied against the engine); a preview of `rung_one_order(floor, interval)`,
    approved; `set_leave` {leave "launch", given true}; then `run` {days 14} again and again until an answer says the game is over
    (a stop is answered by `run` again, and nothing else is ordered); then `events` {day d} for d from 1 to 14, in order.

    V1. A floor of 2 days and an interval of 5: the lines are RUNG_ONE_MET.
    V2. 2 and 6: ["Mission not met.", "1,000 men at Saltmere went unfed on the nights of days 13 and 14."].
    V3. 2 and 4: ["Mission not met.", "1,000 men at Thornwick went unfed on the night of day 14."].
    V4. In each game: the last `run` answers `game_over` true and `result` null; every one of the fourteen `events` answers has `begun`
        true; the check's own reading of those answers (an event of kind `men_fed` whose text does not end ", 0 unfed", by day and hex)
        agrees with the lines; and the lines are the same when the fourteen answers are asked a second time, after the end. In V1 the
        first stop is the staff's "cannot meet" of day 13 (the fuel), which shows the mission is not touched by it.
    Each game is put away after it."""
    import re
    print("-- rung 1 played to its end three ways", flush=True)
    text = page_text()
    code = text[text.index("  function missionLines("):text.index("  var endSeq = 0;")] + cut_function(text, "  function commas(n)") + "\nfunction main(list) { return list.map(x => missionLines(x.mission, x.days)); }"
    games = [("V1", 2, 5, RUNG_ONE_MET, {}),
             ("V2", 2, 6, ["Mission not met.", "1,000 men at Saltmere went unfed on the nights of days 13 and 14."], {(0, 0): [13, 14]}),
             ("V3", 2, 4, ["Mission not met.", "1,000 men at Thornwick went unfed on the night of day 14."], {(5, 0): [14]})]
    for name, floor, interval, expected, unfed in games:
        check(begin(bridge, RUNG_ONE, "person", "engine") == {"ok": True}, f"{name}: rung 1 begins")
        person = Person(bridge.table)
        shown = person.call("preview", rung_one_order(floor, interval))["ok"]
        check(shown["valid"] is True and person.call("approve", {"preview": shown["preview"]})["ok"]["status"] == "queued", f"{name}: the order at {floor} and {interval} is previewed and approved")
        check(person.call("set_leave", {"leave": "launch", "given": True})["ok"]["status"] == "queued", f"{name}: leave to launch is given before day 1 is fought")
        runs = []
        while not runs or not runs[-1].get("game_over"):
            check(len(runs) < 8, f"{name}: the game ends in a few runs")
            runs.append(person.call("run", {"days": 14})["ok"])

        def asked():
            return [person.call("events", {"day": d})["ok"] for d in range(1, 15)]
        answers = asked()
        lines = run_in_node(code, [{"mission": RUNG_ONE_MISSION, "days": [a["events"] for a in answers]}])[0]
        check(lines == expected, f"{name}: the verdict is " + " ".join(lines))
        check(runs[-1]["game_over"] is True and runs[-1]["result"] is None and all(a["begun"] is True for a in answers) and len(answers) == 14,
              f"{name}: the last run answers game_over with no result, and every one of the fourteen days is answered as begun")
        reading = {}
        for a in answers:
            for e in a["events"]:
                if e["event_kind"] == "men_fed" and not e["text"].endswith(", 0 unfed"):
                    q, r = map(int, re.search(r"\((-?\d+),(-?\d+)\)", e["text"]).groups())
                    reading.setdefault((q, r), []).append(a["day"])
        check(reading == unfed and (lines[0] == "Mission met.") == (not reading), f"{name}: the check's own reading of the events agrees with the lines ({reading or 'every night fed'})")
        again = run_in_node(code, [{"mission": RUNG_ONE_MISSION, "days": [a["events"] for a in asked()]}])[0]
        check(again == lines, f"{name}: asked a second time, after the end, the lines are the same")
        if name == "V1":
            check(runs[0]["day"] == 13 and any(f.get("standing") == "cannot_meet" for f in runs[0]["fired"]),
                  "V1: the first stop is the staff's \"cannot meet\" of day 13 (the fuel), and the mission is not touched by it")
        check(post(bridge.table + "/new") == {"ok": True}, f"{name}: the game is put away")


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument("--port", type=int, default=8780, help="the first of six ports used")
    parser.add_argument("--hold", type=float, default=3, help="the doors' wait, shortened for the check")
    args = parser.parse_args()
    page_scripts_parse()
    outcomes_said_one_way()
    the_stop_paper()
    orders_and_signed()
    one_order_paper()
    the_paper_reads_as_the_order()
    stops_speak_as_the_staff()
    cards_show_no_handle()
    the_trail_holds_no_duplicate()
    the_plan_paper_has_no_rule_caption()
    the_fold_and_the_keys()
    the_words_and_what_the_papers_say()
    the_submarines()
    the_intelligence_paper()
    a_force_ordered()
    the_verdict_in_words()
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        bridge = Bridge(args.port, args.sim_cli, args.hold, keep)
        try:
            listed = the_screen(bridge)
            every_game(bridge, listed)
            orders_on_the_card(bridge)
            a_refusal_through_the_page(bridge)
            no_points_in_play(bridge)
            queue_through_the_bridge(bridge)
            the_paper_in_words(bridge)
            the_staffs_reading(bridge)
            the_limits_on_the_paper(bridge)
            limits_in_force(bridge)
            the_orders_paper(bridge)
            reason_limits_as_links(bridge)
            the_intelligence_of_the_long_game(bridge)
            midway_on_its_chart(bridge)
            person_against_agent(bridge, args.hold)
            seat_mcp(bridge)
            the_island_of_two_places(bridge)
            your_plan_on_the_order_paper(bridge)
            the_draft_is_read_and_signed_as_one(bridge)
            the_dry_run_shows_two_plans(bridge)
            the_plan_paper_marks_whose_line(bridge)
            a_reason_leads_to_his_line(bridge)
            a_landing_written_under_an_order_in_force(bridge)
            rung_one_on_the_list(bridge)
            the_supply_orders_two_numbers(bridge)
            rung_one_played_three_ways(bridge)
        finally:
            bridge.stop()
        check(bridge.process.returncode is not None, "the bridge has left")
        games = sorted(os.listdir(keep))
        check(len(games) >= 12, f"each game is kept under its own number ({len(games)} folders)")
        versus = [g for g in games if os.path.exists(os.path.join(keep, g, "transcript_japanese.jsonl"))
                  and os.path.exists(os.path.join(keep, g, "transcript_allied.jsonl"))]
        check(versus, f"a game of a person against an agent keeps each seat's transcript ({versus[0] if versus else ''})")
    the_submarines_through_the_page(args.sim_cli, args.port, args.hold)
    the_patrol_through_the_page(args.sim_cli, args.port)
    the_intelligence_through_a_bridge(args.sim_cli, args.port)
    a_force_followed_through_a_bridge(args.sim_cli, args.port)
    a_refusal_at_dusk_through_a_bridge(args.sim_cli, args.port)
    print("-- the bridge started with a scenario: table/check_watch.py", flush=True)
    done = subprocess.run([sys.executable, os.path.join(HERE, "check_watch.py"), "--sim-cli", args.sim_cli,
                           "--port", str(args.port + 3), "--hold", str(args.hold)])
    check(done.returncode == 0, "check_watch.py holds: with --watch and without, the bridge does as before")
    print(f"all held ({COUNT['held']} checks)", flush=True)


if __name__ == "__main__":
    try:
        main()
    except AssertionError as error:
        print("FAILED  " + str(error), flush=True)
        sys.exit(1)
