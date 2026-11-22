#!/usr/bin/env python3
"""A check, by script, of the table's bridge in a watched game (decision 0182, step 2).

It starts `table/serve.py --watch` and plays two short watched games through
its doors as scripted agents would, with the watcher stepping the clock, and
then the bridge without `--watch` as the player's table has it. It needs
`sim-cli` built (`cargo build --release -p sim-cli`) and is run from the top of
the repository:

    python3 table/check_watch.py

It prints one line for each thing checked and stops at the first that does
not hold. Every process it starts it stops. Standard library only. It is not
run by CI: the table has no test there.
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
ES = "data/scenarios/orders-inside-proof/05_eastern_solomons_on.toml"
S78 = "data/scenarios/78_orders_inside_a_day.toml"


def ok(words):
    print("ok  " + words, flush=True)


def check(holds, words):
    if not holds:
        raise AssertionError(words)
    ok(words)


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


class Bridge:
    """`table/serve.py`, started and read until it has printed its addresses."""

    def __init__(self, arguments, lines):
        self.process = subprocess.Popen(
            [sys.executable, os.path.join(HERE, "serve.py")] + arguments,
            stdout=subprocess.PIPE,
            text=True,
        )
        self.printed = []
        for _ in range(lines):
            line = self.process.stdout.readline()
            if not line:
                raise AssertionError("the bridge stopped before it printed its addresses")
            self.printed.append(line.strip())

    def url(self, starts):
        for line in self.printed:
            if line.startswith(starts):
                return line.split()[-1].rstrip("/")
        raise AssertionError(f"the bridge printed no line starting {starts!r}: {self.printed}")

    def stop(self):
        if self.process.poll() is None:
            self.process.send_signal(signal.SIGTERM)
            try:
                self.process.wait(90)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait()


class Agent:
    """A scripted agent at one door."""

    def __init__(self, door):
        self.door = door

    def call(self, tool, arguments=None):
        return http("POST", self.door + "/call", {"tool": tool, "arguments": arguments or {}})[1]

    def later(self, tool, arguments=None):
        """A call made on a thread of its own; `.join()` it, then read `.answer`."""
        thread = threading.Thread(target=lambda: setattr(thread, "answer", self.call(tool, arguments)))
        thread.start()
        return thread


class Watcher:
    def __init__(self, base):
        self.base = base

    def read(self, side, tool, arguments=None):
        return http("POST", self.base + "/call", {"side": side, "tool": tool, "arguments": arguments or {}})[1]

    def step(self):
        return http("POST", self.base + "/step", {})[1]

    def standing(self):
        return http("GET", self.base + "/standing")[1]["ok"]

    def seat_calls(self, side):
        return http("GET", self.base + "/seat_calls?side=" + side)[1]["ok"]


def wait_until(test, seconds=20):
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        if test():
            return True
        time.sleep(0.2)
    return False


def two_agents(sim_cli, port, hold, keep):
    """Eastern Solomons, orders inside a day, both seats agents'."""
    print(f"-- a watched game of two agents: {ES}", flush=True)
    bridge = Bridge(
        [ES, "--watch", "--seat", "allied", "--seat", "japanese", "--port", str(port),
         "--sim-cli", sim_cli, "--hold", str(hold), "--keep", keep],
        4,
    )
    try:
        watcher = Watcher(bridge.url("A watched game."))
        allied = Agent(bridge.url("The allied seat's door"))
        japanese = Agent(bridge.url("The japanese seat's door"))
        root = f"http://127.0.0.1:{port}"

        # Keys.
        check(http("POST", root + "/seat/not-a-key/call", {"tool": "list_forces"})[0] == 403,
              "a call at a door with a wrong key is refused (403)")
        check(http("GET", root + "/watch/not-a-key/standing")[0] == 403,
              "the watcher's reads with a wrong key are refused (403)")
        check(http("GET", root + "/call")[0] == 403 and "watched" in http("GET", root + "/")[1],
              "the page without the watcher's key is not served")
        check(allied.door.split("/")[-1] != japanese.door.split("/")[-1] != watcher.base.split("/")[-1],
              "each door and the watcher have keys of their own")

        # A door's tools and words.
        status, tools = http("GET", allied.door + "/tools")
        names = [t["name"] for t in tools["ok"]["tools"]]
        check("wait" in names and "advance" in names and "issue_orders" in names,
              f"the allied door lists its seat's tools and wait ({len(names)} tools)")
        words = http("GET", japanese.door + "/instructions")[1]["ok"]["instructions"]
        check("watcher holds the clock" in words, "the japanese door gives the seat's opening words and the door's")
        page = http("GET", watcher.base + "/")[1]
        check(isinstance(page, str) and "<html" in page, "the watcher's address serves the page")
        config = http("GET", watcher.base + "/config")[1]
        check(config["watch"]["agents"] == ["allied", "japanese"] and config["watch"]["inside_day"],
              "the page is told the game is watched, by two agents, a pulse at a time")
        wtools = http("GET", watcher.base + "/tools?side=japanese")[1]
        check("get_briefing" in wtools and "issue_orders" not in wtools and "preview" not in wtools,
              "the watcher's tools from the japanese side are reads only")

        # Reads and an order.
        forces = allied.call("list_forces")["ok"]["task_forces"]
        check(len(forces) > 0, f"the allied agent reads its forces ({len(forces)})")
        check("heading" in allied.call("get_briefing")["ok"], "the allied agent reads its briefing")
        jforces = japanese.call("list_forces")["ok"]["task_forces"]
        check(len(jforces) > 0, f"the japanese agent reads its forces ({len(jforces)})")
        unit = forces[0]["task_force"]["handle"]
        order = allied.call("issue_orders", {
            "orders": [{"unit": unit, "order": {"kind": "set_readiness", "readiness": "battle"}}],
            "reason": "the enemy carriers may be near",
        })
        results = order["ok"]["results"]
        check(results[0]["status"] == "queued", f"the allied agent's order is queued: {unit} to battle readiness")

        # What the watcher may not do.
        refused = watcher.read("allied", "issue_orders", {"orders": []})
        check("refused" in refused, "the watcher is refused a tool that orders: " + refused.get("refused", "")[:70])
        check("refused" in watcher.read("allied", "preview", {"command": "set_leave", "leave": "launch", "given": True}),
              "the watcher is refused a preview, which leaves a record")
        check("refused" in watcher.read("allied", "end_day"), "the watcher is refused a tool of time")
        check("heading" in watcher.read("japanese", "get_briefing", {"text": False})["ok"],
              "the watcher reads the japanese briefing as its seat would")

        # The step waits for both seats.
        step = watcher.step()
        check("refused" in step and "allied" in step["refused"] and "japanese" in step["refused"],
              "a step is refused while neither seat is ready: " + step.get("refused", "")[:80])
        standing = watcher.standing()
        check(standing["waiting_for"] == ["allied", "japanese"], "the standing says both seats are waited for")

        # Two calls of time: the allied one answered at the step, the
        # japanese one held past the door's wait.
        a1 = allied.later("advance", {"pulses": 1, "reason": "one pulse at a time"})
        j1 = japanese.later("end_day", {"reason": "nothing to change"})
        check(wait_until(lambda: watcher.standing()["waiting_for"] == []), "both seats are ready once they call")
        calls = watcher.seat_calls("allied")
        check([c["tool"] for c in calls["calls"]][-1:] == ["issue_orders"] and calls["held"]["tool"] == "advance"
              and calls["calls"][-1]["reason"] == "the enemy carriers may be near" and calls["calls"][-1]["refused"] is False,
              "the watcher sees the allied seat's calls, its order with its reason, and its held advance")
        check(a1.is_alive(), "the allied advance is held: no answer before the step")
        step = watcher.step()["ok"]
        check(step["resolved"]["pulses"] == ["night"] and step["stops"] == ["allied"],
              "the step resolves the night; the allied seat's stop came")
        a1.join(10)
        check(not a1.is_alive() and a1.answer["ok"]["pulses_resolved"][0]["pulse"] == "night",
              "the allied advance is answered after the step, in its open call")
        j1.join(hold + 10)
        check(j1.answer.get("waiting") is True, f"the japanese end_day, not answered in {hold} s, says it is waiting")
        held = japanese.call("list_forces")
        check("refused" in held, "a seat that is held makes no other call: " + held.get("refused", "")[:70])
        step = watcher.step()
        check("refused" in step and "allied" in step["refused"] and "japanese" not in step["refused"],
              "the next step waits for the allied seat alone")

        a2 = allied.later("advance", {"pulses": 2})
        check(wait_until(lambda: watcher.standing()["waiting_for"] == []), "the allied seat is ready again")
        j2 = japanese.later("wait")
        step = watcher.step()["ok"]
        check(step["resolved"]["pulses"] == ["morning"] and step["stops"] == [], "the step resolves the morning; no stop came")
        check(watcher.read("allied", "events", {"day": 1, "pulse": "night"})["ok"]["kind"] == "events",
              "the watcher reads the night's events as the allied side")
        step = watcher.step()["ok"]
        check(step["resolved"]["pulses"] == ["afternoon"] and step["game_over"] and step["stops"] == ["allied", "japanese"],
              "the step resolves the afternoon, the game is over and both stops came")
        a2.join(10)
        check(a2.answer["ok"]["game_over"] is True, "the allied advance is answered at the game's end")
        j2.join(hold + 10)
        if j2.answer.get("waiting"):
            j2.answer = japanese.call("wait")
        check(j2.answer["ok"]["resolved"] is True, "the japanese seat's wait gives its end_day answer: day resolved")
        check("refused" in japanese.call("wait"), "a wait with nothing held is refused")
        check(watcher.standing()["game_over"] is True, "the standing says the game is over")
        restart = http("POST", watcher.base + "/restart", {})[1]
        check("refused" in restart, "no restart while an agent has called in the last few minutes")
    finally:
        bridge.stop()
    files = sorted(os.listdir(keep))
    check({"save.json", "transcript.jsonl", "transcript_allied.jsonl", "transcript_japanese.jsonl"} <= set(files),
          "on leaving the game and each seat's transcript are kept: " + ", ".join(files))
    for side in ("allied", "japanese"):
        with open(os.path.join(keep, f"transcript_{side}.jsonl")) as handle:
            seats = {json.loads(line)["seat"] for line in handle}
        check(seats == {side}, f"the {side} transcript holds the {side} seat's calls alone")
    with open(os.path.join(keep, "transcript.jsonl")) as handle:
        tools = [json.loads(line)["tool"] for line in handle]
    check("get_briefing" in tools and all(t != "events" for t in tools),
          "the watcher's reads are in no transcript")
    replay = subprocess.run([sim_cli, "replay", os.path.join(keep, "save.json")], capture_output=True, text=True)
    check(replay.returncode == 0, "the kept game replays: " + (replay.stdout.strip().splitlines() or [""])[-1][:80])


def one_agent(sim_cli, port, hold):
    """Scenario 78, orders inside a day, the allied seat an agent's and the japanese the script's."""
    print(f"-- a watched game of one agent: {S78}", flush=True)
    bridge = Bridge([S78, "--watch", "--seat", "allied", "--port", str(port), "--sim-cli", sim_cli,
                     "--hold", str(hold)], 3)
    try:
        watcher = Watcher(bridge.url("A watched game."))
        door = bridge.url("The allied seat's door")
        allied = Agent(door)
        root = f"http://127.0.0.1:{port}"
        check(http("GET", root + "/seat/" + "x" * 24 + "/tools")[0] == 403, "a wrong key is refused")
        check(len(bridge.printed) == 3 and "japanese" in bridge.printed[2] and "script" in bridge.printed[2],
              "the bridge prints one door, and that the japanese side is the script's")
        check("refused" in http("GET", watcher.base + "/seat_calls?side=japanese")[1],
              "the watcher looks only from a side an agent plays")
        forces = allied.call("list_forces")["ok"]["task_forces"]
        unit = forces[0]["task_force"]["handle"]
        order = allied.call("issue_orders", {"orders": [{"unit": unit, "order": {"kind": "set_speed", "speed": "full"}}]})
        check(order["ok"]["results"][0]["status"] == "queued", f"the agent's order is queued: {unit} at full speed")

        # The seat's program, over a door.
        mcp = subprocess.Popen([sys.executable, os.path.join(HERE, "seat_mcp.py"), "--url", door],
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            def rpc(n, method, params):
                mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": n, "method": method, "params": params}) + "\n")
                mcp.stdin.flush()
                return json.loads(mcp.stdout.readline())
            init = rpc(1, "initialize", {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "check", "version": "0"}})
            check(init["result"]["protocolVersion"] == "2025-06-18" and "watched" in init["result"]["instructions"],
                  "seat_mcp.py answers initialize with the door's words")
            mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "method": "notifications/initialized"}) + "\n")
            listed = rpc(2, "tools/list", {})
            names = [t["name"] for t in listed["result"]["tools"]]
            check("wait" in names and "end_day" in names, f"seat_mcp.py lists the door's tools ({len(names)}, wait among them)")
            brief = rpc(3, "tools/call", {"name": "get_briefing", "arguments": {}})
            text = brief["result"]["content"][0]["text"]
            check(brief["result"]["isError"] is False and "FLAG PLOT BRIEFING" in text, "seat_mcp.py calls get_briefing and gives its text")
            bad = rpc(4, "tools/call", {"name": "no_such_tool", "arguments": {}})
            check(bad["result"]["isError"] is True, "seat_mcp.py gives a refusal as an error result")
        finally:
            mcp.stdin.close()
            mcp.wait(10)

        e1 = allied.later("end_day", {"reason": "steam on"})
        check(wait_until(lambda: watcher.standing()["waiting_for"] == []), "the agent is ready; the script's seat always is")
        for pulse in ("night", "morning", "afternoon"):
            step = watcher.step()["ok"]
            check(step["resolved"]["pulses"] == [pulse], f"the watcher steps the {pulse}")
            if pulse != "afternoon":
                check(e1.is_alive() or e1.answer.get("waiting"), "the end_day is not answered before dusk")
        e1.join(hold + 10)
        answer = e1.answer if not e1.answer.get("waiting") else allied.call("wait")
        check(answer["ok"]["resolved"] is True and answer["ok"]["day"] == 1,
              "the end_day is answered at dusk, as the day resolved")
        told = watcher.read("allied", "events", {"day": 1})["ok"]
        check(told["count"] > 0, f"the watcher reads day 1 as the allied side ({told['count']} events)")
    finally:
        bridge.stop()


SUB = "data/scenarios/table-submarines/128_a_submarine_in_the_way.toml"


def submarines_watched(sim_cli, port, hold):
    """Submarines in a watched game (decision 0218, stage 3): the table's copy of scenario 128 (a Japanese boat on a station where an
    Allied force lies, seed 6: it fires on the carrier on the first morning and is hunted), both seats agents'. What each seat and the
    watcher read through the doors is what the page draws and says: the Japanese boat is its side's, exactly, with her hands; the
    Allied side holds a report of a submarine and the waters it is told of and nothing of the boat; a direct order and a hold on a
    boat are queued at the Japanese door and seen by the watcher in the seat's calls."""
    print(f"-- submarines in a watched game of two agents: {SUB}", flush=True)
    bridge = Bridge([SUB, "--watch", "--seat", "allied", "--seat", "japanese", "--port", str(port), "--sim-cli", sim_cli, "--hold", str(hold)], 3)
    try:
        watcher = Watcher(bridge.url("A watched game."))
        allied = Agent(bridge.url("The allied seat's door"))
        japanese = Agent(bridge.url("The japanese seat's door"))
        boats = japanese.call("list_forces")["ok"].get("submarines", [])
        check(len(boats) == 1 and boats[0]["state"] == "on station" and boats[0]["hands"]["by"] == "free" and boats[0]["salvoes_left"] == 2,
              "the Japanese agent reads its boat: on station, in no one's hands, two salvoes left")
        check("submarines" not in allied.call("list_forces")["ok"] and allied.call("list_contacts")["ok"].get("suspected_waters"),
              "the Allied agent reads no boat of the enemy's and the waters it is told of")
        check("submarines" in watcher.read("japanese", "list_forces")["ok"] and "submarines" not in watcher.read("allied", "list_forces")["ok"],
              "the watcher reads the boat from the Japanese side and none from the Allied")
        held = japanese.call("hold", {"submarine": 1})
        check("command" in held["ok"] and held["ok"]["command"]["submarine"] == 1, "the Japanese agent holds the boat out of the staff's plans")
        order = japanese.call("issue_orders", {"orders": [{"unit": "submarine:1", "order": {"kind": "station", "hex": [5, 0]}}], "reason": "stay on the force"})
        check(order["ok"]["results"][0]["status"] == "queued", "and orders her a station")
        calls = watcher.seat_calls("japanese")
        check(calls["calls"][-1]["tool"] == "issue_orders" and calls["calls"][-1]["reason"] == "stay on the force" and any(c["tool"] == "hold" for c in calls["calls"]),
              "the watcher sees the seat's hold and its order to the boat, with its reason")
        pending = [allied.later("end_day"), japanese.later("end_day")]
        check(wait_until(lambda: watcher.standing()["waiting_for"] == []), "both seats are ready")
        for _ in range(4):
            step = watcher.step()
            if "ok" in step and step["ok"].get("resolved") and (step["ok"]["resolved"].get("day") or "afternoon" in step["ok"]["resolved"].get("pulses", [])):
                break
        for thread in pending:
            thread.join(hold + 10)
        events = watcher.read("allied", "events", {"day": 1})["ok"]["events"]
        kinds = [e["event_kind"] for e in events]
        check("salvo_from_submarine" in kinds and "submarine_reported" in kinds and "submarine_hunted_by" in kinds,
              f"the Allied side is told that one of its ships was fired on from a submarine, of a report of one and of its own force's hunt ({sorted(set(kinds))})")
        contacts = watcher.read("allied", "list_contacts")["ok"]
        report = contacts["submarine_reports"][0]
        explained = watcher.read("allied", "explain", {"event": report["event"]})["ok"]
        check([n["event_kind"] for n in explained["nodes"]] == ["submarine_reported", "salvo_from_submarine"],
              "the Allied report of a submarine is explained by the salvo on its ship, which is how its card says it was made")
        jevents = watcher.read("japanese", "events", {"day": 1})["ok"]["events"]
        jkinds = [e["event_kind"] for e in jevents]
        check("own_submarine_fired" in jkinds and "own_submarine_hunted" in jkinds and "submarine_reported" not in jkinds and "salvo_from_submarine" not in jkinds,
              f"the Japanese side is told its boat fired and was hunted, and nothing of the Allied side's report or its ship's fright ({sorted(set(jkinds))})")
        check("submarine_reports" not in watcher.read("japanese", "list_contacts")["ok"], "the Japanese side holds no report of a submarine")
    finally:
        bridge.stop()


def not_watched(sim_cli, port):
    print("-- the player's table, not watched", flush=True)
    bridge = Bridge([S78, "--seat", "allied", "--port", str(port), "--sim-cli", sim_cli], 1)
    try:
        root = f"http://127.0.0.1:{port}"
        check("<html" in http("GET", root + "/")[1], "the page is served at / as before")
        check("advance" in http("GET", root + "/tools")[1], "/tools lists the seat's tools as before")
        brief = http("POST", root + "/call", {"tool": "get_briefing", "arguments": {"text": False}})[1]
        check(brief["ok"]["heading"]["day"] == 1, "get_briefing through /call")
        end = http("POST", root + "/call", {"tool": "end_day", "arguments": {}})[1]
        check(end["ok"]["resolved"] is True, "end_day through /call resolves the day at once")
    finally:
        bridge.stop()


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument("--port", type=int, default=8760, help="the first of three ports used")
    parser.add_argument("--hold", type=float, default=3, help="the doors' wait, shortened for the check")
    args = parser.parse_args()
    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    with tempfile.TemporaryDirectory() as keep:
        two_agents(args.sim_cli, args.port, args.hold, keep)
    one_agent(args.sim_cli, args.port + 1, args.hold)
    not_watched(args.sim_cli, args.port + 2)
    submarines_watched(args.sim_cli, args.port + 2, args.hold)
    print("all held", flush=True)


if __name__ == "__main__":
    try:
        main()
    except AssertionError as error:
        print("FAILED  " + str(error), flush=True)
        sys.exit(1)
