#!/usr/bin/env python3
"""The table's bridge: one seat of one game, served to a browser on this machine.

It starts `sim-cli mcp SCENARIO --seat SIDE` and passes the page's tool calls
to it, one at a time. The page gets what the seat's tools answer and nothing
else: this file never reads a replay, a save, a batch or the event log, and it
does not read the scenario. It reads one data file, the map (for drawing the
coast), which both sides know.

Run from the top of the repository:

    python3 table/serve.py data/scenarios/stage-3-proof/07b_slice.toml \
        --seat allied --start 1942-08-07 \
        --island "1=Guadalcanal@1" --island "2=Tulagi@4"

then open http://127.0.0.1:8742/. Standard library only. It listens on this
machine alone. Decision 0154.

With `--watch` it holds a watched game instead (decision 0182, step 2): it
starts `sim-cli serve SCENARIO --watched`, each side named by `--seat` is an
agent's, and the other is the script's. Each agent has a door of its own and
the watcher has the page, each behind a key made at start:

    python3 table/serve.py data/scenarios/78_orders_inside_a_day.toml \
        --watch --seat allied --seat japanese

prints the watcher's address and one door for each agent's seat. A door
answers only its own seat's calls; nothing the watcher reads reaches it.
"""

import argparse
import csv
import json
import os
import secrets
import signal
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.parse import parse_qs, urlsplit

HERE = os.path.dirname(os.path.abspath(__file__))
SIDES = ("allied", "japanese")
# A watched game cannot be begun again while an agent has called its door
# within this many seconds (decision 0182, amended 20).
RECENT = 300
# The tool of the door's own: it holds again for the answer of a call of time
# (decision 0182, amended 17).
WAIT_TOOL = {
    "name": "wait",
    "description": (
        "Wait for the answer of your call of end_day, run or advance. In this game a watcher "
        "holds the clock, and such a call is answered when the clock has reached the stop you "
        "asked for. If its answer was {\"waiting\": true}, call wait (no arguments) until the "
        "answer comes; it is the answer the call would have given."
    ),
    "inputSchema": {"type": "object", "properties": {}, "additionalProperties": False},
}
DOOR_WORDS = (
    "\n\nThis game is watched. A watcher holds the clock and moves it a step at a time. Your "
    "calls of end_day, run and advance are answered when the clock has reached the stop you "
    "asked for, which may take a while; meanwhile you can make no other call. If such a call "
    "answers {\"waiting\": true}, call the tool wait, again as often as it answers so, until "
    "the answer comes."
)


class Game:
    """The seat's server, spoken to over its standard input and output."""

    def __init__(self, command):
        self.lock = threading.Lock()
        self.next_id = 0
        self.process = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
        )
        self.request(
            "initialize",
            {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {"name": "flag-plot-table", "version": "0"},
            },
        )
        self.send({"jsonrpc": "2.0", "method": "notifications/initialized"})

    def send(self, message):
        self.process.stdin.write(json.dumps(message) + "\n")
        self.process.stdin.flush()

    def request(self, method, params):
        self.next_id += 1
        self.send({"jsonrpc": "2.0", "id": self.next_id, "method": method, "params": params})
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError("the game's server has stopped")
        return json.loads(line)

    def tools(self):
        """The names of the tools this seat is offered in this game."""
        with self.lock:
            reply = self.request("tools/list", {})
        return [tool["name"] for tool in reply.get("result", {}).get("tools", [])]

    def call(self, tool, arguments):
        """One tool call. Answers {"ok": the answer} or {"refused": the words}."""
        with self.lock:
            reply = self.request("tools/call", {"name": tool, "arguments": arguments})
        if "error" in reply:
            return {"refused": reply["error"].get("message", "refused")}
        result = reply.get("result", {})
        text = "".join(part.get("text", "") for part in result.get("content", []))
        if result.get("isError"):
            return {"refused": text}
        try:
            return {"ok": json.loads(text)}
        except ValueError:
            return {"ok": {"text": text}}


# ---------------------------------------------------------------------------
# A watched game (decision 0182, step 2).
# ---------------------------------------------------------------------------


class Slot:
    """One request to `serve`, and its answer line once it has come."""

    def __init__(self):
        self.done = threading.Event()
        self.line = None

    def put(self, line):
        self.line = line
        self.done.set()

    def answer(self):
        """The answer as the bridge gives it: {"ok": ..} or {"refused": ..}."""
        line = self.line
        if "error" in line:
            error = line["error"]
            out = {"refused": error.get("message", "refused")}
            if error.get("allowed"):
                out["allowed"] = error["allowed"]
            return out
        return {"ok": line.get("result")}


class Watched:
    """`sim-cli serve --watched`, spoken to in JSON lines. Its answers are read
    on a thread of their own and matched to their requests by `id`, since a
    held call's answer comes after later requests' (decision 0182, amended 18)."""

    def __init__(self, command, journal=None):
        # Every request that can change the game is written here as it is
        # sent, so a game killed outright can be played again from it.
        self.journal = open(journal, "w") if journal else None
        self.write_lock = threading.Lock()
        self.table_lock = threading.Lock()
        self.pending = {}
        self.next_id = 0
        self.ended = None
        self.process = subprocess.Popen(
            command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True
        )
        self.reader = threading.Thread(target=self.read, daemon=True)
        self.reader.start()

    def read(self):
        for raw in self.process.stdout:
            try:
                line = json.loads(raw)
            except ValueError:
                continue
            with self.table_lock:
                slot = self.pending.pop(line.get("id"), None)
            if slot is not None:
                slot.put(line)
        # The server has stopped: every call still waiting is told so.
        with self.table_lock:
            self.ended = "the game's server has stopped"
            left, self.pending = self.pending, {}
        for slot in left.values():
            slot.put({"error": {"message": self.ended}})

    def send(self, request):
        """Sends one request and gives its slot, whose answer may come later."""
        slot = Slot()
        with self.table_lock:
            if self.ended:
                slot.put({"error": {"message": self.ended}})
                return slot
            self.next_id += 1
            request = dict(request, id=self.next_id)
            self.pending[self.next_id] = slot
        with self.write_lock:
            try:
                self.process.stdin.write(json.dumps(request) + "\n")
                self.process.stdin.flush()
                if self.journal and request.get("watcher") in (None, "step"):
                    self.journal.write(json.dumps(request) + "\n")
                    self.journal.flush()
            except (BrokenPipeError, ValueError, OSError):
                with self.table_lock:
                    self.pending.pop(request["id"], None)
                slot.put({"error": {"message": "the game's server has stopped"}})
        return slot

    def ask(self, request, timeout=120):
        """A request answered at once (any but a seat's call of time)."""
        slot = self.send(request)
        if not slot.done.wait(timeout):
            return {"refused": "the game's server did not answer"}
        return slot.answer()

    def finish(self, timeout=60):
        """Ends the game: `serve` keeps what it was told to on leaving."""
        with self.write_lock:
            try:
                self.process.stdin.close()
            except OSError:
                pass
        try:
            self.process.wait(timeout)
        except subprocess.TimeoutExpired:
            self.process.terminate()


class Door:
    """One agent's seat: its key, the call of time it holds, and when it last
    called (decision 0182, amended 15 and 17)."""

    def __init__(self, side, key):
        self.side = side
        self.key = key
        self.lock = threading.Lock()
        self.held = None
        self.held_tool = None
        self.last_call = 0.0

    def reset(self):
        with self.lock:
            self.held = None
            self.held_tool = None


def waiting_words(side, tool):
    return {
        "waiting": True,
        "tool": tool,
        "words": (
            f"The clock has not yet reached the stop of your {tool}. Call the tool wait for "
            "its answer."
        ),
    }


def seat_call(game, door, tool, arguments, hold):
    """One call at a seat's door. A call is passed to `serve` as the seat's;
    one not answered within `hold` seconds is held, and `wait` collects it."""
    door.last_call = time.monotonic()
    if tool == "wait":
        with door.lock:
            slot, held_tool = door.held, door.held_tool
        if slot is None:
            return {"refused": "nothing waits: you have no call of end_day, run or advance held"}
    else:
        with door.lock:
            if door.held is not None and door.held.done.is_set():
                return {
                    "refused": (
                        f"your call of {door.held_tool} has been answered: call wait to "
                        "collect its answer first"
                    )
                }
        slot = game.send(
            {"seat": door.side, "tool": tool, "arguments": arguments if arguments else {}}
        )
        held_tool = tool
    if not slot.done.wait(hold):
        with door.lock:
            if door.held is None:
                door.held, door.held_tool = slot, held_tool
        return waiting_words(door.side, held_tool)
    with door.lock:
        if door.held is slot:
            door.held, door.held_tool = None, None
    return slot.answer()


def read_map(path):
    hexes = []
    with open(path, newline="") as handle:
        rows = csv.DictReader(line for line in handle if not line.startswith("#"))
        for row in rows:
            hexes.append(
                [
                    int(row["q"]),
                    int(row["r"]),
                    row["kind"],
                    float(row["lat"]),
                    float(row["lon"]),
                    int(row["land_percent"] or 0),
                ]
            )
    return hexes


def read_directives(path):
    with open(path) as handle:
        return json.load(handle)["directives"]


def main():
    parser = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    parser.add_argument("scenario")
    parser.add_argument(
        "--seat",
        action="append",
        choices=list(SIDES),
        help="the side played: one, or with --watch each side an agent plays",
    )
    parser.add_argument("--map", default="data/map/south_pacific.csv")
    parser.add_argument("--start", default="", help="the date of day 1, as 1942-08-07")
    parser.add_argument(
        "--island",
        action="append",
        default=[],
        help='an island the scenario names, as "1=Guadalcanal@1": its number, a name, '
        "and the number of a base that stands on it. No tool lists islands yet.",
    )
    parser.add_argument(
        "--directives",
        action="append",
        default=[],
        help="a file of directives from the commander above, as table/directives/ has them; "
        'with --watch, "allied=FILE" or "japanese=FILE" for each side that has one',
    )
    parser.add_argument("--port", type=int, default=8742)
    parser.add_argument("--sim-cli", default="target/release/sim-cli")
    parser.add_argument(
        "--watch",
        action="store_true",
        help="a watched game: agents at the seats named by --seat, the page at the clock",
    )
    parser.add_argument(
        "--keep",
        default="",
        help="with --watch: a folder to keep the saved game and each seat's transcript in on leaving",
    )
    parser.add_argument(
        "--hold",
        type=float,
        default=50,
        help="with --watch: seconds a door holds a call of time before it answers that it waits",
    )
    args = parser.parse_args()

    if not os.path.exists(args.sim_cli):
        sys.exit(f"{args.sim_cli} is not built: cargo build --release -p sim-cli")
    islands = []
    for item in args.island:
        number, rest = item.split("=", 1)
        name, base = rest.rsplit("@", 1)
        islands.append({"number": int(number), "name": name, "base": int(base)})
    chart = read_map(args.map)
    # The coastline drawn on the chart, where the map has one beside this
    # script: for the eye only. The game plays on the hexes and reads none of it.
    coast = None
    coast_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "chart", os.path.splitext(os.path.basename(args.map))[0] + "_coast.json")
    if os.path.exists(coast_path):
        with open(coast_path) as handle:
            coast = json.load(handle)
    # The names lettered on the chart, where the map has a file of them: for
    # the eye only, as the coastline is.
    names = None
    names_path = coast_path.replace("_coast.json", "_names.json")
    if os.path.exists(names_path):
        with open(names_path) as handle:
            names = json.load(handle)["names"]
    if args.watch:
        watch(args, islands, chart, coast, names)
        return
    if args.seat and len(args.seat) > 1:
        sys.exit("one --seat only: two seats are a watched game (--watch)")
    if args.keep:
        sys.exit("--keep is for a watched game (--watch)")
    seat = args.seat[0] if args.seat else "allied"
    directives = []
    if args.directives:
        directives = read_directives(args.directives[-1])
    config = {
        "seat": seat,
        "start": args.start,
        "islands": islands,
        "directives": directives,
    }
    command = [args.sim_cli, "mcp", args.scenario, "--seat", seat]
    # The game in play. A restart ends it and begins the scenario again from
    # its first evening: nothing of the old game is kept.
    held = {"game": Game(command)}
    swap = threading.Lock()

    class Handler(BaseHTTPRequestHandler):
        def answer(self, status, kind, body):
            self.send_response(status)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, status=200):
            self.answer(status, "application/json", json.dumps(value).encode())

        def do_GET(self):
            if self.path in ("/", "/index.html"):
                with open(os.path.join(HERE, "index.html"), "rb") as page:
                    self.answer(200, "text/html; charset=utf-8", page.read())
            elif self.path == "/config":
                self.json(config)
            elif self.path == "/tools":
                self.json(held["game"].tools())
            elif self.path == "/map":
                self.json(chart)
            elif self.path == "/coast" and coast is not None:
                self.json(coast)
            elif self.path == "/names" and names is not None:
                self.json(names)
            else:
                self.answer(404, "text/plain", b"not found")

        def do_POST(self):
            if self.path not in ("/call", "/restart"):
                self.answer(404, "text/plain", b"not found")
                return
            # Only this machine's own page may call: a page of another site
            # cannot send JSON here without asking first, and is not answered.
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                self.json({"refused": "send JSON"}, 415)
                return
            if self.path == "/restart":
                with swap:
                    old = held["game"]
                    try:
                        held["game"] = Game(command)
                    except (RuntimeError, OSError) as error:
                        self.json({"refused": str(error)}, 500)
                        return
                    old.process.terminate()
                self.json({"ok": True})
                return
            try:
                length = int(self.headers.get("Content-Length", "0"))
                body = json.loads(self.rfile.read(length))
                tool, arguments = body["tool"], body.get("arguments", {})
            except (ValueError, KeyError):
                self.json({"refused": "a call is {tool, arguments}"}, 400)
                return
            try:
                self.json(held["game"].call(tool, arguments))
            except RuntimeError as error:
                self.json({"refused": str(error)}, 500)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    print(f"The table is at http://127.0.0.1:{args.port}/  (seat: {seat})", flush=True)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        held["game"].process.terminate()


def watch(args, islands, chart, coast, names):
    """A watched game: the page holds the clock and reads, each agent has a door."""
    agents = []
    for side in args.seat or []:
        if side in agents:
            sys.exit(f"--seat names the {side} side twice")
        agents.append(side)
    if not agents:
        sys.exit("--watch needs --seat for each side an agent plays: --seat allied, --seat japanese or both")
    agents.sort(key=SIDES.index)
    directives = {}
    for item in args.directives:
        side, _, path = item.partition("=")
        if side not in SIDES or not path:
            side, path = agents[0], item
        directives[side] = read_directives(path)
    keep = os.path.abspath(args.keep) if args.keep else ""
    if keep:
        os.makedirs(keep, exist_ok=True)

    # The keys: one for each agent's door and one for the watcher, new at
    # every start. They keep a program from calling a door not its own by
    # mistake; they do not stop a program on this machine that sets out to.
    watcher_key = secrets.token_urlsafe(18)
    doors = {side: Door(side, secrets.token_urlsafe(18)) for side in agents}
    by_key = {door.key: door for door in doors.values()}
    games = {"count": 0}

    def paths(number):
        suffix = "" if number == 1 else f"-{number}"
        return (
            os.path.join(keep, f"save{suffix}.json"),
            os.path.join(keep, f"transcript{suffix}.jsonl"),
            suffix,
        )

    def start():
        games["count"] += 1
        command = [args.sim_cli, "serve", args.scenario, "--watched"]
        for side in agents:
            command += ["--seat", f"{side}=player"]
        if keep:
            save, transcript, suffix = paths(games["count"])
            command += ["--save", save, "--transcript", transcript]
            return Watched(command, os.path.join(keep, f"journal{suffix}.jsonl"))
        return Watched(command)

    def finish(game, number):
        """Ends a game and splits its transcript into one file a seat."""
        game.finish()
        if not keep:
            return
        _, transcript, suffix = paths(number)
        if not os.path.exists(transcript):
            return
        lines = {side: [] for side in agents}
        with open(transcript) as handle:
            for line in handle:
                try:
                    seat = json.loads(line).get("seat")
                except ValueError:
                    continue
                if seat in lines:
                    lines[seat].append(line)
        for side, kept in lines.items():
            with open(os.path.join(keep, f"transcript_{side}{suffix}.jsonl"), "w") as handle:
                handle.writelines(kept)

    held = {"game": start(), "number": 1}
    swap = threading.Lock()

    # Whether the scenario has orders inside a day: the agents' seats then have
    # `advance`, and a step is a pulse.
    first = held["game"].ask({"seat": agents[0], "request": "tools"})
    tool_names = [t["name"] for t in (first.get("ok") or {}).get("tools", [])]
    if "refused" in first:
        held["game"].finish()
        sys.exit(f"the game could not be opened: {first['refused']}")
    config = {
        "seat": agents[0],
        "start": args.start,
        "islands": islands,
        "directives": directives.get(agents[0], []),
        "watch": {
            "agents": agents,
            "inside_day": "advance" in tool_names,
            "directives": directives,
        },
    }

    def watcher_tools(side):
        # The watcher's list, as `serve` names it when asked for no tool.
        answer = held["game"].ask({"watcher": "read", "side": side, "tool": ""})
        return answer.get("allowed", [])

    def same(given, key):
        return secrets.compare_digest(given.encode(), key.encode())

    def recently_called():
        now = time.monotonic()
        return [d.side for d in doors.values() if d.last_call and now - d.last_call < RECENT]

    class Handler(BaseHTTPRequestHandler):
        def answer(self, status, kind, body):
            self.send_response(status)
            self.send_header("Content-Type", kind)
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Cache-Control", "no-store")
            self.end_headers()
            self.wfile.write(body)

        def json(self, value, status=200):
            self.answer(status, "application/json", json.dumps(value).encode())

        def where(self):
            """(kind, door or None, the rest of the path, the query), or None
            for a path that is no door's. A wrong key is no door."""
            parts = urlsplit(self.path)
            words = parts.path.split("/")
            query = parse_qs(parts.query)
            if len(words) >= 3 and words[1] == "watch" and same(words[2], watcher_key):
                return "watch", None, "/" + "/".join(words[3:]), query
            if len(words) >= 3 and words[1] == "seat":
                door = next((d for k, d in by_key.items() if same(words[2], k)), None)
                if door is not None:
                    return "seat", door, "/" + "/".join(words[3:]), query
            return None

        def refuse(self):
            self.json({"refused": "no door here: use the address the table printed when it started"}, 403)

        def body(self):
            if self.headers.get("Content-Type", "").split(";")[0] != "application/json":
                raise ValueError("send JSON")
            length = int(self.headers.get("Content-Length", "0"))
            value = json.loads(self.rfile.read(length) or b"{}")
            if not isinstance(value, dict):
                raise ValueError("send a JSON object")
            return value

        def side_of(self, query, body=None):
            side = ((body or {}).get("side")) or (query.get("side") or [agents[0]])[0]
            if side not in agents:
                raise ValueError(f"the watcher looks from a side an agent plays: {', '.join(agents)}")
            return side

        def do_GET(self):
            found = self.where()
            if found is None:
                if urlsplit(self.path).path == "/":
                    self.answer(
                        200,
                        "text/plain; charset=utf-8",
                        b"This game is watched. Open the watcher's address the table printed when it started.\n",
                    )
                else:
                    self.refuse()
                return
            kind, door, path, query = found
            game = held["game"]
            if kind == "seat":
                if path == "/tools":
                    answer = game.ask({"seat": door.side, "request": "tools"})
                    if "ok" in answer:
                        answer = {"ok": {"tools": answer["ok"]["tools"] + [WAIT_TOOL]}}
                    self.json(answer)
                elif path == "/instructions":
                    answer = game.ask({"seat": door.side, "request": "instructions"})
                    if "ok" in answer:
                        answer = {"ok": {"instructions": answer["ok"]["instructions"] + DOOR_WORDS}}
                    self.json(answer)
                else:
                    self.answer(404, "text/plain", b"not found")
                return
            try:
                if path in ("/", "/index.html"):
                    with open(os.path.join(HERE, "index.html"), "rb") as page:
                        self.answer(200, "text/html; charset=utf-8", page.read())
                elif path == "/config":
                    self.json(config)
                elif path == "/tools":
                    self.json(watcher_tools(self.side_of(query)))
                elif path == "/map":
                    self.json(chart)
                elif path == "/coast" and coast is not None:
                    self.json(coast)
                elif path == "/names" and names is not None:
                    self.json(names)
                elif path == "/standing":
                    self.json(game.ask({"watcher": "standing"}))
                elif path == "/seat_calls":
                    self.json(game.ask({"watcher": "seat_calls", "side": self.side_of(query)}))
                else:
                    self.answer(404, "text/plain", b"not found")
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def do_POST(self):
            found = self.where()
            if found is None:
                self.refuse()
                return
            kind, door, path, query = found
            try:
                body = self.body()
            except ValueError as error:
                self.json({"refused": str(error)}, 415)
                return
            game = held["game"]
            if kind == "seat":
                if path != "/call" or not isinstance(body.get("tool"), str):
                    self.json({"refused": "a call is POST /call with {tool, arguments}"}, 400)
                    return
                arguments = body.get("arguments") or {}
                self.json(seat_call(game, door, body["tool"], arguments, args.hold))
                return
            try:
                if path == "/call":
                    if not isinstance(body.get("tool"), str):
                        raise ValueError("a read is {tool, arguments, side}")
                    self.json(
                        game.ask(
                            {
                                "watcher": "read",
                                "side": self.side_of(query, body),
                                "tool": body["tool"],
                                "arguments": body.get("arguments") or {},
                            }
                        )
                    )
                elif path == "/step":
                    self.json(game.ask({"watcher": "step"}, timeout=600))
                elif path == "/restart":
                    with swap:
                        busy = recently_called()
                        if busy:
                            self.json(
                                {
                                    "refused": "an agent has called in the last few minutes ("
                                    + ", ".join(busy)
                                    + "): the game is not begun again under it"
                                }
                            )
                            return
                        old, number = held["game"], held["number"]
                        finish(old, number)
                        held["game"], held["number"] = start(), number + 1
                        for each in doors.values():
                            each.reset()
                    self.json({"ok": True})
                else:
                    self.answer(404, "text/plain", b"not found")
            except ValueError as error:
                self.json({"refused": str(error)}, 400)

        def log_message(self, *_):
            pass

    server = ThreadingHTTPServer(("127.0.0.1", args.port), Handler)
    server.daemon_threads = True
    base = f"http://127.0.0.1:{args.port}"
    print(f"A watched game. The watcher's table is at {base}/watch/{watcher_key}/", flush=True)
    for side in agents:
        print(f"The {side} seat's door is {base}/seat/{doors[side].key}", flush=True)
    others = [side for side in SIDES if side not in agents]
    if others:
        print(f"The {others[0]} side is the script's.", flush=True)
    if keep:
        print(f"On leaving, the game and the transcripts are kept in {keep}", flush=True)

    def leave(*_):
        raise KeyboardInterrupt

    signal.signal(signal.SIGTERM, leave)
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        server.server_close()
        finish(held["game"], held["number"])


if __name__ == "__main__":
    main()
