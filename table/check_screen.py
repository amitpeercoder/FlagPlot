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
process it starts it stops. Standard library only. It is not run by CI: the
table has no test there.
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
    check(all(f'id="{b}"' in view for b in ("btn-plans", "btn-pieces", "btn-labels", "btn-hexes", "btn-view", "btn-above", "btn-lean", "btn-back"))
          and "bottom: 16px" in css(".view"),
          "the chart's controls are at its foot: plans, pieces, names, hexes, table or from above, and the zoom")
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
    parts = ['id="p-to"', "THE ORDER", "IN WHOSE HANDS", "ACTS", 'id="o-pv"', "THE STAFF'S READING", 'id="p-says"', "SIGN AND SEND", "TEAR UP"]
    at = [form.find(p) for p in parts]
    check(all(a >= 0 for a in at) and at == sorted(at), "the paper's parts are in order: TO, THE ORDER, IN WHOSE HANDS, ACTS, the staff's reading, what was sent, SIGN AND SEND, TEAR UP")
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
            out.write(NODE_PRELUDE + code + "\nconst input = JSON.parse(process.argv[2]); console.log(JSON.stringify(main(input)));\n")
        done = subprocess.run([node, path, json.dumps(data)], capture_output=True, text=True)
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
          and "body.papered #prompt { right: calc(min(360px, 100% - var(--sp-w) - 48px) + 32px); }" in text
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
    check('<span class="fl">Patrol these waters,</span><input type="number" id="o-dist"' in text
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
    check([s["id"] for s in listed] == ["eastern-solomons", "coral-sea", "midway", "eastern-solomons-five-days", "guadalcanal-39-days"],
          "the screen lists the scenarios simplest first: the Eastern Solomons, the Coral Sea, Midway, the Eastern Solomons over five days and the 39 days")
    check(all("file" not in p for s in listed for p in s["pairings"]), "the list given to the screen names no file")
    check([len(s["pairings"]) for s in listed] == [4, 7, 7, 7, 7],
          "the Eastern Solomons offers 4 games, the Coral Sea 7, Midway 7, the five-day Eastern Solomons 7 and the 39 days 7")
    # Midway has its submarines (decision 0225, the designer's answer): one game of the battle, and it plays the copies with
    # boats in table-boats/; its directives name each side's boats in the situation's last sentence.
    midway = {e["id"]: e for e in json.load(open(os.path.join(HERE, "scenarios.json")))["scenarios"]}["midway"]
    check(midway["name"] == "Midway" and sorted({p["file"] for p in midway["pairings"]}) == ["data/scenarios/table-boats/01" + l + "_midway_submarines.toml" for l in "bcd"],
          "Midway is one game, named Midway, and its pairings play the copies with submarines of files b, c and d")
    for side, words in (("allied", "Four of our submarines are on station"), ("japanese", "The submarine I-168 is off Midway")):
        situation = json.load(open(os.path.join(os.path.dirname(HERE), midway["directives"][side])))["directives"][0]["parts"][0]["text"]
        check(situation[-1].startswith(words) and sum("submarine" in t for t in situation) == 1,
              f"Midway: the {side} directive's situation ends with the one sentence naming the side's boats")
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
    check(page_supply_limit(person, 3, "keep_supplied") is None,
          "with no other base of the side to be a source, the page sets no forces limit on a keep-supplied order")

    # The men put ashore at Lunga, and a keep-supplied order signed as the page
    # signs it: its forces limit names a source and a pool the staff can send.
    person.call("issue_orders", {"orders": [
        {"unit": "task_force:9", "order": {"kind": "set_route", "waypoints": [[2, -4]], "act_at": [[2, -4]]}},
        {"unit": "task_force:9", "order": {"kind": "unload"}}]})
    person.call("end_day")
    signed = person.call("set_intent", {"intent_kind": "keep_supplied", "base": 1, "floor_days": 30})["ok"]
    limit = page_supply_limit(person, 1, "keep_supplied")
    check(limit is not None and limit["source"] == 3 and limit["task_forces"],
          f"with men ashore at Lunga the page names Espiritu Santo as the source and the free transports as the pool ({limit})")
    limit["intent"] = signed["intent"]
    check(person.call("set_limit", limit)["ok"]["status"] == "queued", "the forces limit is taken")
    said = person.call("staff_orders")["ok"]
    mine = [r for r in said["no_orders"] if r.get("serves", {}).get("intent") == signed["intent"]]
    check(not any("no forces limit" in r["worked"] or "No ship of the pool" in r["worked"] for r in mine)
          and any(c["kind"] == "approve" and c["reason"]["serves"]["intent"] == signed["intent"] for c in said["commands"]),
          "the staff takes the order up: it makes the plan ready, and does not say it has no forces or no ship")
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
    check("lim.chosen.source = lim.chosen.ships = true" in text and "WE CHOSE IT FOR YOU" in text and "limit.air_groups" not in text,
          "the table's own source is put in the limits as a choice the player may change, and a hold no longer names its air groups")
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
    check(sources and all(l.get("intent") == held["intent"] for _, l in sources), "the hold with no source: its reason carries {kind: source, intent: <the hold>}")
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
    # Midway's submarines (decision 0225): each seat has its own boats and none of the other's, and a boat takes a station order.
    check(begin(bridge, "midway", "person", "engine") == {"ok": True}, "Midway begun again, the person Allied, for the boats")
    person = Person(bridge.table)
    boats = person.call("list_forces")["ok"].get("submarines", [])
    check([(b["submarine"].get("name"), b["state"]) for b in boats] == [("USS Tambor", "on station"), ("USS Trout", "on station"), ("USS Nautilus", "on station"), ("USS Gato", "on station")],
          f"the Allied seat has its four boats, each on station, and nothing of I-168 ({[b['submarine'] for b in boats]})")
    order = person.call("issue_orders", {"orders": [{"unit": "submarine:3", "order": {"kind": "station", "hex": [5, -3]}}]})["ok"]
    check(order["results"][0]["status"] == "queued", "a direct order gives a boat of Midway's a station")
    check(begin(bridge, "midway", "engine", "person") == {"ok": True}, "Midway begun again, the person Japanese, for the boats")
    boats = Person(bridge.table).call("list_forces")["ok"].get("submarines", [])
    check([(b["submarine"].get("name"), b["state"]) for b in boats] == [("I-168", "on station")], "the Japanese seat has I-168 alone, on station")
    check("base_attacked_against: 'bombing of a base'" in page and "(n === 1 ? 'report' : 'reports')" in page,
          "the plot folds the rest of a kind of report under a plain name, not the event's own")


def page_supply_limit(person, base, kind):
    """The forces limit index.html sets when a keep-supplied or hold order is
    signed (supplyForces), worked from the same answers, or None."""
    st = person.call("command_state")["ok"]
    bases = person.call("list_bases")["ok"]["bases"]
    forces = person.call("list_forces")["ok"]
    num = lambda h: int(h.split(":")[1])
    same = lambda a, b: a and b and a["q"] == b["q"] and a["r"] == b["r"]
    dest = next((b for b in bases if num(b["base"]["handle"]) == base), None)
    if dest is None:
        return None
    named = set()
    for i in st["intents"]:
        f = i["in_force"] and i["kind"] not in ("keep_supplied", "hold") and i["limits"]["forces"]
        for t in (f or {}).get("task_forces", []) if f else []:
            named.add(num(t["handle"]) if isinstance(t, dict) else int(t))
    for h in st.get("holds", []):
        if h.get("target", {}).get("task_force") is not None:
            named.add(int(h["target"]["task_force"]))
    reserve = {r["base"]["handle"]: float(r["supply_reserve_t"]) for r in st.get("reserves", [])}
    above = lambda b: float(b["supply_t"]) - reserve.get(b["base"]["handle"], 0)
    sources = [b for b in bases if b is not dest and not same(b["place"], dest["place"]) and above(b) > 0]
    if not sources:
        return None
    aboard = {a["ship"]["handle"] for u in forces["land_units"] for a in u.get("aboard", [])}
    free = [f for f in forces["task_forces"] if f["mission"] == "transport" and num(f["task_force"]["handle"]) not in named
            and not any(s["ship"]["handle"] in aboard for s in person.call("get_entity", {"handle": f["task_force"]["handle"]})["ok"]["entity"].get("ships", []))]
    in_port = {b["base"]["handle"]: [num(s["handle"]) for s in person.call("get_entity", {"handle": b["base"]["handle"]})["ok"]["entity"].get("ships_in_port") or []] for b in sources}
    day = st["for_day"]
    manned = lambda b: any(u["men_ashore"] and same(u["place"], b["place"]) for u in forces["land_units"])
    ready = lambda b: bool(in_port[b["base"]["handle"]]) or any(same(f["place"], b["place"]) and not f.get("destination") and not (f.get("available_from") or 0) > day for f in free)
    source = sorted(sources, key=lambda b: (manned(b), not ready(b), -above(b), num(b["base"]["handle"])))[0]
    if not free and not in_port[source["base"]["handle"]]:
        return None
    limit = {"limit": "forces", "source": num(source["base"]["handle"]), "task_forces": [num(f["task_force"]["handle"]) for f in free],
             "ships": in_port[source["base"]["handle"]]}
    return limit


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
    stops_speak_as_the_staff()
    cards_show_no_handle()
    the_trail_holds_no_duplicate()
    the_plan_paper_has_no_rule_caption()
    the_fold_and_the_keys()
    the_words_and_what_the_papers_say()
    the_submarines()
    the_intelligence_paper()
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
