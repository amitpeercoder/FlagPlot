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
    check([s["id"] for s in listed] == ["guadalcanal-39-days", "eastern-solomons"],
          "the screen lists the 39 days and the Eastern Solomons")
    check(all("file" not in p for s in listed for p in s["pairings"]), "the list given to the screen names no file")
    check(len(listed[0]["pairings"]) == 7 and len(listed[1]["pairings"]) == 4,
          "the 39 days offers 7 games and the Eastern Solomons 4")
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
    check("directive" not in jap["brief"] and "Your mission: Hold Lunga and Tulagi" in jap["brief"],
          "a side with no directive is given the scenario's mission for it in its brief")
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
                for t in held:
                    t.join(60)
                check(all(t.answer["ok"]["resolved"] is True for t in held) and config["watch"]["agents"] == agents,
                      f"{name}: the watcher steps, each agent's end_day answered")
    check(post(bridge.table + "/new") == {"ok": True} and get(bridge.table + "/config") == {"screen": True},
          "New game ends the game in hand and the screen is shown again")


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
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        bridge = Bridge(args.port, args.sim_cli, args.hold, keep)
        try:
            listed = the_screen(bridge)
            every_game(bridge, listed)
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
