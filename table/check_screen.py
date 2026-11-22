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
    check("api('queued').catch(function () { return null; })" in text and "api('take_back', { queue: e.place.queue, number: e.place.number, as_of: M.queue.as_of })" in text
          and "'TAKE BACK'" in text and "NOT TAKEN BACK" in text,
          "SIGNED is read from queued at each refresh, TAKE BACK withdraws one entry by its place and as_of, and a refusal is said beside the entry")
    check("function canSign() { return !WATCH && !M.over && !playing; }" in text and "if (canSign()) {" in text and "(e.goes_with || [])" in text,
          "TAKE BACK is offered only while the clock is stopped and the seat may sign; what goes with an entry is not a line of its own")
    check("'Acts at '" in text and "'Waits for 1800'" in text, "each entry says when it acts, \"Acts at 1200\" or \"Waits for 1800\"")
    check("refusedIn(home, words)" in text and "'NOT SENT'" in text and "act(tool, args, done, home)" in text,
          "a refusal is said as NOT SENT in the form, paper or entry that was signed")
    check("else if (kind === 'confirm') $('game-says')" in text and "else if (kind === 'running')" in text and "'clock-doing'" in text
          and "showPrompt(kind, words)" in text and "else if (kind === 'draft') $('sr')" in text,
          "a confirm is said in the GAME menu, what the clock is doing under the head's state, a pick and the first hint in a prompt by the chart's controls, a draft to the live region")
    check("act(tool, { plan: p.number }, done, row)" in text and "tallied[word]" not in text and "el('h3', '', 'The brief')" not in text,
          "the plan's paper has no tally and no brief: the plan's line is one, the tree is the rest")
    paper = text[text.index("// ---- the plan's paper as one tree ----"):text.index("// ---- the dry run (decisions")]
    check("function paperNode(t)" in paper and "t.kids.forEach(function (k) { n.appendChild(paperNode(k)); })" in paper
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
    check("function atStop() { setDrawer(true); }" in text and text.count("atStop();") >= 4 and "function startRun() { setDrawer(false);" in text,
          "the drawer opens at load and at every stop of the clock; the clock's starting closes it")
    check(re.search(r"\.stoppaper \.sp-body \{ display: none; \}", text) and not re.search(r"\.stoppaper \.sp-foot \{ display: none", text)
          and 'id="sp-foot"' in column and 'id="btn-day"' in column and 'id="btn-go"' in column,
          "with the drawer closed the two controls that run time stay in sight under the head: the same buttons of the foot")


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
    check("'landorder'" in page and "NOT SENT" in page and "Acts at 1800, for the coming day." in page,
          "the page holds the land unit's order form, the line for a refusal and the line for an order kept to dusk")
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
    if kind == "hold":
        limit["air_groups"] = []
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
    stops_speak_as_the_staff()
    cards_show_no_handle()
    the_trail_holds_no_duplicate()
    the_plan_paper_has_no_rule_caption()
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        bridge = Bridge(args.port, args.sim_cli, args.hold, keep)
        try:
            listed = the_screen(bridge)
            every_game(bridge, listed)
            orders_on_the_card(bridge)
            queue_through_the_bridge(bridge)
            the_paper_in_words(bridge)
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
