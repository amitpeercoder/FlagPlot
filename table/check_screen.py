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
]


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
    check("function say(kind, words)" in text and "setStatus" not in text,
          "the page says every outcome through say(kind, words), and nothing else writes the status line")
    kinds = set(re.findall(r"\b(\w+): 1", re.search(r"var SAY_KINDS = \{([^}]*)\}", text).group(1)))
    used = set(re.findall(r"\bsay\('(\w+)'", text))
    check(used and used <= kinds, f"every say names a kind of SAY_KINDS ({', '.join(sorted(used))})")


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
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        bridge = Bridge(args.port, args.sim_cli, args.hold, keep)
        try:
            listed = the_screen(bridge)
            every_game(bridge, listed)
            orders_on_the_card(bridge)
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
